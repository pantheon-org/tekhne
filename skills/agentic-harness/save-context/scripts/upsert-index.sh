#!/usr/bin/env bash
# shell: bash
# Upsert a context row into .context/session/INDEX.md
# Usage: upsert-index.sh <area> <project> <context> <status> <focus> <saved_date>
# Example: upsert-index.sh repos agent-skills mystream "building" "Working on X" 2026-02-24
#
# Behavior:
# - Finds INDEX.md at .context/session/INDEX.md under git repo root
# - Matches existing row by Area+Project+Context columns
# - Updates matched row or appends to Active Contexts table
# - Preserves Parked/Done/Archived sections unchanged

set -euo pipefail

AREA="${1:?Usage: upsert-index.sh <area> <project> <context> <status> <focus> <saved>}"
PROJECT="${2:?}"
CONTEXT="${3:?}"
STATUS="${4:?}"
FOCUS="${5:?}"
SAVED="${6:?}"

# Truncate focus to 80 chars
FOCUS="${FOCUS:0:80}"

# Find INDEX.md
REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || true)
INDEX="${REPO_ROOT}/.context/session/INDEX.md"

if [[ ! -f "$INDEX" ]]; then
  echo "SKIP: INDEX.md not found at ${INDEX}"
  exit 0
fi

NEW_ROW="| ${AREA} | ${PROJECT} | ${CONTEXT} | ${STATUS} | ${FOCUS} | ${SAVED} |"

# Escape special chars for grep/sed
# shellcheck disable=SC2016
AREA_ESC=$(printf '%s' "$AREA" | sed 's/[.[\*^\$()\+?{|]/\\&/g')
# shellcheck disable=SC2016
PROJECT_ESC=$(printf '%s' "$PROJECT" | sed 's/[.[\*^\$()\+?{|]/\\&/g')
# shellcheck disable=SC2016
CONTEXT_ESC=$(printf '%s' "$CONTEXT" | sed 's/[.[\*^\$()\+?{|]/\\&/g')

PATTERN="^\| ${AREA_ESC} \| ${PROJECT_ESC} \| ${CONTEXT_ESC} \|"

if grep -qE "$PATTERN" "$INDEX"; then
  # Replace existing row
  sed -i -E "s|${PATTERN}.*|$(printf '%s' "$NEW_ROW" | sed 's/[&/\]/\\&/g')|" "$INDEX"
  echo "UPDATED: ${AREA}/${PROJECT}/${CONTEXT}"
else
  # Append after last row in Active Contexts table (before empty line or ## Parked)
  # Find the line number of "## Parked" or "## Done" or "## Archived" — insert before it
  INSERT_BEFORE=$(grep -n "^## \(Parked\|Done\|Archived\)" "$INDEX" | head -1 | cut -d: -f1)
  if [[ -n "$INSERT_BEFORE" ]]; then
    # Insert before the section header (and its preceding blank line)
    sed -i "${INSERT_BEFORE}i\\${NEW_ROW}" "$INDEX"
  else
    # No Parked section — append to end of file
    echo "$NEW_ROW" >> "$INDEX"
  fi
  echo "APPENDED: ${AREA}/${PROJECT}/${CONTEXT}"
fi
