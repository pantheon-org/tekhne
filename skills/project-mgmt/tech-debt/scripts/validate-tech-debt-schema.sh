#!/usr/bin/env bash
# shell: bash
# Validates every data row in docs/TECH_DEBT.md against
# assets/schemas/tech-debt-row.schema.json. Constraints (the effort/status enums, the
# since pattern, minLength) are read from the schema file at runtime via jq, not
# duplicated as separate hardcoded values here -- see .claude/instructions/skill-authoring.md
# for why: a hand-duplicated copy silently drifts from the schema the moment one changes
# without the other.
#
# This is a schema-only check -- unlike risk-register, there is no append-only
# companion script (docs/TECH_DEBT.md rows are deleted, not archived, once resolved;
# see SKILL.md Rules) and no cross-field check (risk-register's status/date/decision
# set-together rule doesn't apply to this schema).
#
# Usage: validate-tech-debt-schema.sh [<file>]  (defaults to docs/TECH_DEBT.md)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA="$SCRIPT_DIR/../assets/schemas/tech-debt-row.schema.json"
FILE="${1:-docs/TECH_DEBT.md}"

if [ ! -f "$FILE" ]; then
  echo "validate-tech-debt-schema.sh: $FILE not found" >&2
  exit 1
fi
if [ ! -f "$SCHEMA" ]; then
  echo "validate-tech-debt-schema.sh: $SCHEMA not found" >&2
  exit 1
fi

# Sanity check that this script's positional column order still matches the schema's
# required list -- a tripwire for exactly the drift skill-authoring.md warns against.
expected_fields="id,item,area,description,effort,since,status"
required_fields="$(jq -r '.required | join(",")' "$SCHEMA")"
if [ "$required_fields" != "$expected_fields" ]; then
  echo "validate-tech-debt-schema.sh: schema required fields ($required_fields) no longer" >&2
  echo "match this script's column order ($expected_fields) -- update the script to match." >&2
  exit 1
fi

# Escape each enum value's regex metacharacters before joining into a "|"-separated
# alternation: these values are schema-declared literal strings, not regex fragments,
# so a future enum value containing e.g. "(" or "." must still match literally instead
# of gaining regex meaning when spliced into the `[[ =~ ^(...)$ ]]` checks below.
escape_enum='map(gsub("(?<c>[.^$*+?(){}\\[\\]|\\\\])"; "\\\(.c)")) | join("|")'
effort_enum="$(jq -r ".properties.effort.enum | $escape_enum" "$SCHEMA")"
status_enum="$(jq -r ".properties.status.enum | $escape_enum" "$SCHEMA")"
since_pattern="$(jq -r '.properties.since.pattern' "$SCHEMA")"
item_minlen="$(jq -r '.properties.item.minLength' "$SCHEMA")"
area_minlen="$(jq -r '.properties.area.minLength' "$SCHEMA")"
description_minlen="$(jq -r '.properties.description.minLength' "$SCHEMA")"

# jq -r prints the literal string "null" (exit 0) for a path that doesn't exist --
# it does not error or leave the variable unset, so a renamed/removed pattern or
# minLength key beneath .properties (not caught by the .required tripwire above,
# which only checks field *names*) would otherwise silently validate every row
# against the string "null" instead of failing loudly (same class of bug pr-agent
# caught in risk-register's validator on MR !63).
for name in effort_enum status_enum since_pattern item_minlen area_minlen description_minlen; do
  if [ "${!name}" = "null" ]; then
    echo "validate-tech-debt-schema.sh: $SCHEMA is missing a property this script reads" >&2
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

# A data row looks like "| 3 | Item | Area | Description | Effort | Since | Status |".
# The header ("| # | Item | ...") and separator ("| --- | --- | ...") rows have no
# leading digit and are correctly excluded by this pattern.
while IFS= read -r line; do
  rows_checked=$((rows_checked + 1))

  # A well-formed 7-column row has exactly 8 '|' delimiters (leading + 6 internal +
  # trailing). IFS='|' read splits on every '|' with no regard for which one is a
  # real column boundary, so a literal '|' inside any cell (e.g. a code snippet or
  # path list in `description`) silently shifts every later column left instead of
  # erroring, producing a confusing false violation instead of failing on the actual
  # cause (same class of bug pr-agent caught in risk-register's validator on MR !63).
  # Fail loudly instead.
  pipes_only="${line//[^|]/}"
  pipe_count="${#pipes_only}"
  if [ "$pipe_count" -ne 8 ]; then
    echo "$FILE: row $rows_checked has $pipe_count '|' characters, expected 8 for 7 columns --" >&2
    echo "a cell likely contains a literal '|', which this parser can't split around;" >&2
    echo "rephrase the cell to avoid '|' entirely. Row: $line" >&2
    errors=$((errors + 1))
    continue
  fi

  IFS='|' read -r _ raw_id raw_item raw_area raw_description raw_effort raw_since raw_status _ <<<"$line"
  id="$(trim "$raw_id")"
  item="$(trim "$raw_item")"
  area="$(trim "$raw_area")"
  description="$(trim "$raw_description")"
  effort="$(trim "$raw_effort")"
  since="$(trim "$raw_since")"
  status="$(trim "$raw_status")"

  label="row with id '${id}'"

  if ! [[ "$id" =~ ^[0-9]+$ ]] || [ "$id" -lt 1 ]; then
    echo "$FILE: $label: id must be a positive integer (minimum 1)" >&2
    errors=$((errors + 1))
  fi
  if [ "${#item}" -lt "$item_minlen" ]; then
    echo "$FILE: $label: item is empty (minLength $item_minlen)" >&2
    errors=$((errors + 1))
  fi
  if [ "${#area}" -lt "$area_minlen" ]; then
    echo "$FILE: $label: area is empty (minLength $area_minlen)" >&2
    errors=$((errors + 1))
  fi
  if [ "${#description}" -lt "$description_minlen" ]; then
    echo "$FILE: $label: description is empty (minLength $description_minlen)" >&2
    errors=$((errors + 1))
  fi
  if ! [[ "$effort" =~ ^($effort_enum)$ ]]; then
    echo "$FILE: $label: effort '$effort' is not one of: $effort_enum" >&2
    errors=$((errors + 1))
  fi
  if ! [[ "$since" =~ $since_pattern ]]; then
    echo "$FILE: $label: since '$since' does not match $since_pattern" >&2
    errors=$((errors + 1))
  fi
  if ! [[ "$status" =~ ^($status_enum)$ ]]; then
    echo "$FILE: $label: status '$status' is not one of: $status_enum" >&2
    errors=$((errors + 1))
  fi
done < <(grep -E '^\| *[0-9]+ *\|' "$FILE")

if [ "$errors" -gt 0 ]; then
  echo "" >&2
  echo "$errors schema violation(s) found in $FILE" >&2
  exit 1
fi

echo "ok -- all $rows_checked row(s) in $FILE match tech-debt-row.schema.json"
