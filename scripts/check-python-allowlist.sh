#!/usr/bin/env bash
#
# check-python-allowlist.sh
#
# Guardrail: fail if any .py file lives outside a directory listed in
# python-allowlist.txt. Python is retained only for the allowlisted skills, so
# a .py appearing anywhere else is either a mis-placed skill script or an
# allowlist gap and must be reviewed before it lands. Deterministic, offline,
# no network or build steps.
#
# Carve-outs (not skill Python; mirror the linter exclusions already in hk.pkl):
#   - target/, node_modules/, .git/   build output, dependencies, VCS metadata
#   - .agents/                        vendored installed-skill copies (gitignored)
#   - */tests/golden-corpus/*         skill-validator-rs / skill-auditor fixtures
#
# Usage: ./scripts/check-python-allowlist.sh
# Exit:  0 = all .py under an allowlisted directory; 1 = violation(s) found.
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

allowlist_file="python-allowlist.txt"
if [ ! -f "$allowlist_file" ]; then
  echo "error: $allowlist_file not found at repo root ($repo_root)" >&2
  exit 1
fi

# Normalise the allowlist to a temp file: drop comments and blank lines, trim
# surrounding whitespace and any trailing carriage returns.
normalised_allowlist="$(mktemp)"
trap 'rm -f "$normalised_allowlist"' EXIT
sed -e 's/#.*$//' -e 's/[[:space:]]*$//' -e 's/^[[:space:]]*//' "$allowlist_file" \
  | tr -d '\r' \
  | grep -v '^$' > "$normalised_allowlist"

py_total=0
violations=0
while IFS= read -r f; do
  [ -z "$f" ] && continue
  f="${f#./}"
  py_total=$((py_total + 1))
  matched=0
  while IFS= read -r d; do
    [ -z "$d" ] && continue
    case "$f" in
      "$d"/*) matched=1; break ;;
    esac
  done < "$normalised_allowlist"
  if [ "$matched" -eq 0 ]; then
    if [ "$violations" -eq 0 ]; then
      echo "python-allowlist guardrail FAILED: .py file(s) not under an allowlisted directory (see python-allowlist.txt):" >&2
    fi
    echo "  $f" >&2
    violations=$((violations + 1))
  fi
done < <(
  find . -type f -name '*.py' \
    -not -path './target/*' \
    -not -path '*/node_modules/*' \
    -not -path './.agents/*' \
    -not -path './.git/*' \
    -not -path '*/tests/golden-corpus/*' \
  | sort
)

if [ "$violations" -gt 0 ]; then
  {
    echo ""
    echo "Move each script under an allowlisted skill, or add its skill directory"
    echo "to python-allowlist.txt after Compliance and plan review."
  } >&2
  exit 1
fi

echo "python-allowlist guardrail OK: ${py_total} .py file(s), all under allowlisted directories."
