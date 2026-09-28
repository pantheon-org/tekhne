#!/usr/bin/env bash
# shell: bash
# validate-handover.sh
# Validator for .context/handovers/YYYY-MM-DD-<slug>.md documents.
# Checks: filename shape and date, required frontmatter fields (schema:
# assets/schemas/handover-frontmatter.schema.json), frontmatter date matches the
# filename's date segment, and all required sections are present.
#
# Usage: ./validate-handover.sh <.context/handovers/ directory | a single .md file> [more paths...]

set -euo pipefail

if [[ $# -eq 0 ]]; then
  echo "Usage: $0 <.context/handovers/ directory | .md file> [more paths...]" >&2
  exit 2
fi

errors=0

REQUIRED_SECTIONS=(
  "## Session Summary"
  "## Completed"
  "## Outstanding"
  "## Current State"
  "## Next Steps"
  "## References"
)

# Helper: read a scalar field from the YAML frontmatter block (between the first two '---' lines).
# Returns empty if the file has no frontmatter or the field is absent -- never fails the caller.
frontmatter_field() {
  local file="$1" field="$2"
  awk '/^---$/{c++; next} c==1' "$file" 2>/dev/null \
    | grep -E "^${field}:" \
    | head -1 \
    | sed -E "s/^${field}:[[:space:]]*\"?([^\"]*)\"?[[:space:]]*\$/\\1/" \
    || true
}

fail() {
  echo "  ERROR: $1" >&2
  errors=$((errors + 1))
}

# Validate a single handover document. Prints its own file path as a header.
validate_file() {
  local file="$1"
  echo "Checking: $file"

  if [[ ! -f "$file" ]]; then
    fail "file not found"
    return
  fi

  local base
  base=$(basename "$file")

  # 1. Filename must match YYYY-MM-DD-<slug>.md.
  local filename_date=""
  if [[ "$base" =~ ^([0-9]{4}-[0-9]{2}-[0-9]{2})-[a-z0-9-]+\.md$ ]]; then
    filename_date="${BASH_REMATCH[1]}"
  else
    fail "filename '$base' does not match YYYY-MM-DD-<slug>.md"
  fi

  # 2. Frontmatter must exist and carry all required fields.
  local title type date branch status
  title=$(frontmatter_field "$file" "title")
  type=$(frontmatter_field "$file" "type")
  date=$(frontmatter_field "$file" "date")
  branch=$(frontmatter_field "$file" "branch")
  status=$(frontmatter_field "$file" "status")

  [[ -z "$title" ]] && fail "frontmatter missing 'title'"
  [[ -z "$branch" ]] && fail "frontmatter missing 'branch'"

  if [[ -z "$type" ]]; then
    fail "frontmatter missing 'type'"
  elif [[ "$type" != "handover" ]]; then
    fail "frontmatter 'type' is '$type', expected 'handover'"
  fi

  if [[ -z "$status" ]]; then
    fail "frontmatter missing 'status'"
  elif [[ "$status" != "active" && "$status" != "done" && "$status" != "superseded" ]]; then
    fail "frontmatter 'status' is '$status', expected one of: active, done, superseded"
  fi

  if [[ -z "$date" ]]; then
    fail "frontmatter missing 'date'"
  elif [[ ! "$date" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
    fail "date '$date' is not an ISO date (YYYY-MM-DD)"
  elif [[ -n "$filename_date" && "$date" != "$filename_date" ]]; then
    fail "frontmatter date ($date) does not match filename date ($filename_date)"
  fi

  # 3. Required sections must all be present.
  local section
  for section in "${REQUIRED_SECTIONS[@]}"; do
    if ! grep -qF "$section" "$file"; then
      fail "missing required section: '$section'"
    fi
  done
}

# Validate every handover document in a directory.
validate_directory() {
  local dir="$1"
  echo "Checking directory: $dir"

  local found=0
  local f
  for f in "$dir"/*.md; do
    [[ -e "$f" ]] || continue
    found=1
    validate_file "$f"
  done

  if [[ "$found" -eq 0 ]]; then
    echo "  (no .md files found in $dir)"
  fi
}

for path in "$@"; do
  if [[ -d "$path" ]]; then
    validate_directory "$path"
  elif [[ -f "$path" ]]; then
    validate_file "$path"
  else
    echo "ERROR: $path not found" >&2
    errors=$((errors + 1))
  fi
done

echo ""
if [[ $errors -gt 0 ]]; then
  echo "FAILED: $errors error(s)" >&2
  exit 1
fi
echo "OK"
