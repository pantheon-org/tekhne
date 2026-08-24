#!/usr/bin/env bash
# shell: bash
# Validates every data row in docs/RISK_REGISTER.md against
# assets/schemas/risk-register-row.schema.json. Constraints (the type enum, the added/
# status patterns, minLength) are read from the schema file at runtime via jq, not
# duplicated as separate hardcoded values here -- see .claude/instructions/skill-authoring.md
# for why: a hand-duplicated copy silently drifts from the schema the moment one changes
# without the other.
#
# Usage: validate-risk-register-schema.sh [<file>]  (defaults to docs/RISK_REGISTER.md)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/risk-register-row.schema.json"
FILE="${1:-docs/RISK_REGISTER.md}"

if [ ! -f "$FILE" ]; then
  echo "validate-risk-register-schema.sh: $FILE not found" >&2
  exit 1
fi
if [ ! -f "$SCHEMA" ]; then
  echo "validate-risk-register-schema.sh: $SCHEMA not found" >&2
  exit 1
fi

# Sanity check that this script's positional column order still matches the schema's
# required list -- a tripwire for exactly the drift skill-authoring.md warns against.
expected_fields="id,item,type,description,risk,added,status,date,decision"
required_fields="$(jq -r '.required | join(",")' "$SCHEMA")"
if [ "$required_fields" != "$expected_fields" ]; then
  echo "validate-risk-register-schema.sh: schema required fields ($required_fields) no longer" >&2
  echo "match this script's column order ($expected_fields) -- update the script to match." >&2
  exit 1
fi

# Escape each enum value's regex metacharacters before joining into a "|"-separated
# alternation: these values are schema-declared literal strings, not regex fragments,
# so a future enum value containing e.g. "(" or "." must still match literally instead
# of gaining regex meaning when spliced into the `[[ =~ ^(...)$ ]]` checks below.
escape_enum='map(gsub("(?<c>[.^$*+?(){}\\[\\]|\\\\])"; "\\\(.c)")) | join("|")'
type_enum="$(jq -r ".properties.type.enum | $escape_enum" "$SCHEMA")"
status_enum="$(jq -r ".properties.status.enum | $escape_enum" "$SCHEMA")"
added_pattern="$(jq -r '.properties.added.pattern' "$SCHEMA")"
date_pattern="$(jq -r '.properties.date.pattern' "$SCHEMA")"
item_minlen="$(jq -r '.properties.item.minLength' "$SCHEMA")"
description_minlen="$(jq -r '.properties.description.minLength' "$SCHEMA")"
risk_minlen="$(jq -r '.properties.risk.minLength' "$SCHEMA")"
decision_minlen="$(jq -r '.properties.decision.minLength' "$SCHEMA")"

# jq -r prints the literal string "null" (exit 0) for a path that doesn't exist --
# it does not error or leave the variable unset, so a renamed/removed pattern or
# minLength key beneath .properties (not caught by the .required tripwire above,
# which only checks field *names*) would otherwise silently validate every row
# against the string "null" instead of failing loudly (pr-agent review of MR !63).
for name in type_enum status_enum added_pattern date_pattern item_minlen description_minlen risk_minlen decision_minlen; do
  if [ "${!name}" = "null" ]; then
    echo "validate-risk-register-schema.sh: $SCHEMA is missing a property this script reads" >&2
    echo "($name resolved to jq's \"null\") -- update the script or the schema to match." >&2
    exit 1
  fi
done

trim() {
  # Strip leading/trailing whitespace without xargs's word-splitting edge cases.
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

errors=0
rows_checked=0

# A data row looks like "| 3 | Item | Type | Description | Risk | Added | Status | Date | Decision |".
# The header ("| # | Item | ...") and separator ("| --- | --- | ...") rows have no
# leading digit and are correctly excluded by this same pattern used in
# validate-risk-register-append-only.sh.
while IFS= read -r line; do
  rows_checked=$((rows_checked + 1))

  # A well-formed 9-column row has exactly 10 '|' delimiters (leading + 8 internal +
  # trailing). IFS='|' read splits on every '|' with no regard for which one is a
  # real column boundary, so a literal '|' inside any cell (e.g. "ADR-004 | ADR-015"
  # in a description) silently shifts every later column left instead of erroring
  # (pr-agent review of MR !63, verified: the shifted values then get checked
  # against the wrong field's pattern, producing a confusing false violation
  # instead of failing on the actual cause). Fail loudly instead.
  pipes_only="${line//[^|]/}"
  pipe_count="${#pipes_only}"
  if [ "$pipe_count" -ne 10 ]; then
    echo "$FILE: row $rows_checked has $pipe_count '|' characters, expected 10 for 9 columns --" >&2
    echo "a cell likely contains a literal '|', which this parser can't split around;" >&2
    echo "rephrase the cell to avoid '|' entirely. Row: $line" >&2
    errors=$((errors + 1))
    continue
  fi

  IFS='|' read -r _ raw_id raw_item raw_type raw_description raw_risk raw_added raw_status raw_date raw_decision _ <<<"$line"
  id="$(trim "$raw_id")"
  item="$(trim "$raw_item")"
  type="$(trim "$raw_type")"
  description="$(trim "$raw_description")"
  risk="$(trim "$raw_risk")"
  added="$(trim "$raw_added")"
  status="$(trim "$raw_status")"
  date="$(trim "$raw_date")"
  decision="$(trim "$raw_decision")"

  label="row with id '${id}'"

  if ! [[ "$id" =~ ^[0-9]+$ ]]; then
    echo "$FILE: $label: id must be a positive integer" >&2
    errors=$((errors + 1))
  fi
  if [ "${#item}" -lt "$item_minlen" ]; then
    echo "$FILE: $label: item is empty (minLength $item_minlen)" >&2
    errors=$((errors + 1))
  fi
  if ! [[ "$type" =~ ^($type_enum)$ ]]; then
    echo "$FILE: $label: type '$type' is not one of: $type_enum" >&2
    errors=$((errors + 1))
  fi
  if [ "${#description}" -lt "$description_minlen" ]; then
    echo "$FILE: $label: description is empty (minLength $description_minlen)" >&2
    errors=$((errors + 1))
  fi
  if [ "${#risk}" -lt "$risk_minlen" ]; then
    echo "$FILE: $label: risk is empty (minLength $risk_minlen)" >&2
    errors=$((errors + 1))
  fi
  if ! [[ "$added" =~ $added_pattern ]]; then
    echo "$FILE: $label: added '$added' does not match $added_pattern" >&2
    errors=$((errors + 1))
  fi
  if ! [[ "$status" =~ ^($status_enum)$ ]]; then
    echo "$FILE: $label: status '$status' is not one of: $status_enum" >&2
    errors=$((errors + 1))
  fi
  if ! [[ "$date" =~ $date_pattern ]]; then
    echo "$FILE: $label: date '$date' does not match $date_pattern" >&2
    errors=$((errors + 1))
  fi
  if [ "${#decision}" -lt "$decision_minlen" ]; then
    echo "$FILE: $label: decision is empty (minLength $decision_minlen)" >&2
    errors=$((errors + 1))
  fi

  # Cross-field: status/date/decision are set together (schema.json's description for
  # each says so) -- checking each column in isolation would miss a row where status
  # flipped to Resolved but date or decision was left at the Open placeholder, or vice
  # versa, since each individual pattern above still accepts "--" or a real value.
  if [ "$status" = "Open" ]; then
    if [ "$date" != "--" ] || [ "$decision" != "--" ]; then
      echo "$FILE: $label: status is Open but date/decision aren't both '--'" >&2
      errors=$((errors + 1))
    fi
  elif [ "$status" = "Resolved" ]; then
    if [ "$date" = "--" ] || [ "$decision" = "--" ]; then
      echo "$FILE: $label: status is Resolved but date/decision are still '--'" >&2
      errors=$((errors + 1))
    fi
  fi
done < <(grep -E '^\| *[0-9]+ *\|' "$FILE")

if [ "$errors" -gt 0 ]; then
  echo "" >&2
  echo "$errors schema violation(s) found in $FILE" >&2
  exit 1
fi

echo "ok -- all $rows_checked row(s) in $FILE match risk-register-row.schema.json"
