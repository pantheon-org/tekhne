#!/usr/bin/env bash
# shell: bash
# Blocks a commit that removes a previously-committed row from docs/RISK_REGISTER.md.
# Append-only in spirit: a row is marked Resolved in its Status cell, never deleted.
# Every numbered row id ("| 3 | ...") present in the file at HEAD must still be present
# in the staged/working version -- this only guards row *survival*, not whether an
# existing row's other columns were edited in a way that misrepresents its history.
#
# Usage: validate-risk-register-append-only.sh [<file>]  (defaults to docs/RISK_REGISTER.md)
set -euo pipefail

FILE="${1:-docs/RISK_REGISTER.md}"

if [ ! -f "$FILE" ]; then
  echo "validate-risk-register-append-only.sh: $FILE not found" >&2
  exit 1
fi

extract_ids() {
  # A numbered row looks like "| 3 | Item text | ...". Table separator/header rows
  # ("| --- |", "| # |") have no leading digit and are correctly excluded. Reads
  # "${1:-/dev/stdin}" so this works both as extract_ids <file> and piped input.
  # `|| true`: zero rows (a brand-new, still-empty register) is a legitimate result,
  # not an error -- grep exits 1 on "no match", which set -e/pipefail would otherwise
  # treat as a script failure.
  grep -oE '^\| *[0-9]+ *\|' "${1:-/dev/stdin}" 2>/dev/null | grep -oE '[0-9]+' || true
}

# Nothing to compare against on the commit that creates the file.
if ! git cat-file -e "HEAD:${FILE}" 2>/dev/null; then
  echo "ok -- $FILE is new, nothing to compare against"
  exit 0
fi

old_ids_file="$(mktemp)"
trap 'rm -f "$old_ids_file"' EXIT

git show "HEAD:${FILE}" | extract_ids > "$old_ids_file"
new_ids="$(extract_ids "$FILE")"

missing=""
while IFS= read -r id; do
  [ -n "$id" ] || continue
  if ! printf '%s\n' "$new_ids" | grep -qx "$id"; then
    missing="${missing} #${id}"
  fi
done < "$old_ids_file"

if [ -n "$missing" ]; then
  echo "$FILE: row(s)${missing} existed at HEAD but are missing now." >&2
  echo "The register is append-only -- mark a row Resolved in its Status column instead of deleting it." >&2
  exit 1
fi

echo "ok -- no previously-committed row was removed from $FILE"
