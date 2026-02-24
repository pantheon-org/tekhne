#!/usr/bin/env bash
# shell: bash
# validate-journal-entry.sh
# Validator for journal entry Markdown files. Supports validating multiple files.
# Usage: ./validate-journal-entry.sh [--allow-nondated] <file.md> [more-files...]

set -euo pipefail

allow_nondated=false
# Collect args
args=()
while [[ ${#} -gt 0 ]]; do
  case "${1}" in
    --allow-nondated)
      allow_nondated=true
      shift
      ;;
    --)
      shift
      break
      ;;
    *)
      args+=("$1")
      shift
      ;;
  esac
done

if [[ ${#args[@]} -eq 0 ]]; then
  echo "Usage: $0 [--allow-nondated] <markdown-file> [more-files...]" >&2
  exit 2
fi

# Helper: validate a single file
validate_single() {
  local file="$1"

  if [[ ! -f "$file" ]]; then
    echo "File not found: $file" >&2
    return 3
  fi

  # Prepare file_date and formatted expected date in shell
  local base file_date formatted_date
  base=$(basename "$file")
  if [[ "$base" =~ ^([0-9]{4}-[0-9]{2}-[0-9]{2}) ]]; then
    file_date="${BASH_REMATCH[1]}"
  else
    file_date=""
  fi
  formatted_date=""
  if [[ -n "$file_date" ]]; then
    # Prefer GNU date, fall back to BSD date (macOS)
    if date -d "$file_date" "+%B %-d, %Y" >/dev/null 2>&1; then
      formatted_date=$(date -d "$file_date" "+%B %-d, %Y")
    else
      # BSD date on macOS
      if date -j -f "%Y-%m-%d" "$file_date" "+%B %-d, %Y" >/dev/null 2>&1; then
        formatted_date=$(date -j -f "%Y-%m-%d" "$file_date" "+%B %-d, %Y")
      else
        # Fallback: try day without dash padding
        formatted_date=$(date -j -f "%Y-%m-%d" "$file_date" "+%B %e, %Y" 2>/dev/null | sed -e 's/^ //')
      fi
    fi
    # Normalize spaces
    formatted_date=$(echo "$formatted_date" | tr -s ' ')
  fi

  # Detect if path is inside .templates
  local is_template=false
  case "$file" in
    */.templates/* | .templates/* ) is_template=true ;;
  esac

  # If this is a template file, skip validation — validator is for journal entries only
  if [[ "$is_template" == "true" ]]; then
    echo "SKIP: template file; validator intended for journal entries: $file" >&2
    return 0
  fi

  # Check that file is in a year directory and has correct naming format
  # Files should look like: <YEAR>/<MM>/<YYYY-MM-DD-slug.md>
  if ! echo "$file" | rg -q '(^|/)[0-9]{4}/' 2>/dev/null; then
    # Not inside a year directory — skip validation (may be a template or other doc)
    return 0
  fi
  if ! [[ "$(basename "$file")" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2} ]]; then
    # Basename doesn't start with YYYY-MM-DD format — fail validation
    echo "Invalid: filename must start with YYYY-MM-DD format (got: $(basename "$file"))" >&2
    return 10
  fi
  
  # Check that filename slug (after date) contains no uppercase letters
  local basename_file slug_part
  basename_file="$(basename "$file")"
  # Extract slug: everything after the date prefix (YYYY-MM-DD-)
  slug_part="${basename_file#[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]-}"
  if [[ "$slug_part" =~ [A-Z] ]]; then
    echo "Invalid: filename slug must be lowercase-only (found uppercase in: $basename_file)" >&2
    return 11
  fi

  # Determine AWK to use. Prefer the `awk` found in PATH (which awk); ensure it's GNU Awk (gawk).
  # If PATH awk is not GNU Awk, fall back to `gawk` if available. Fail if GNU Awk cannot be found.
  local AWK_CMD AWK_CANDIDATE
  AWK_CMD=""
  AWK_CANDIDATE="$(command -v awk 2>/dev/null || true)"
  if [[ -n "$AWK_CANDIDATE" ]]; then
    if "$AWK_CANDIDATE" --version 2>/dev/null | grep -qi 'gnu awk'; then
      AWK_CMD="$AWK_CANDIDATE"
    fi
  fi
  if [[ -z "$AWK_CMD" ]]; then
    if command -v gawk >/dev/null 2>&1; then
      AWK_CMD="$(command -v gawk)"
    fi
  fi
  if [[ -z "$AWK_CMD" ]]; then
    echo "gawk (GNU Awk) is required but not found; please install gawk (Homebrew: brew install gawk)" >&2
    return 2
  fi
  export AWK_CMD

  # Portable validator: remove fenced code blocks and validate outside content
  # Create a temporary file containing only lines outside fenced code blocks
  local tmp_out
  tmp_out=$(mktemp /tmp/journal_out.XXXXXX)
  # Use perl to toggle fenced blocks (```), preserving line numbers via nl later
  perl -ne 'if (/^(`{3,})/) { $in = !$in; next } print unless $in' -- "$file" | nl -ba -w1 -s $'\t' > "$tmp_out"

  # 1) Ensure exactly one H1
  local h1_count h1_text h1_line
  # shellcheck disable=SC2034
  IFS=$'\t' read -r h1_count h1_text h1_line < <(awk -F"\t" '$2 ~ /^# / {count++; h=$2; ln=$1} END{print (count?count:0) "\t" (h?h:"") "\t" (ln?ln:"")}' "$tmp_out")
  unset IFS
  if [[ "$h1_count" -ne 1 ]]; then
    echo "Invalid: expected exactly one H1 (found $h1_count) in $file" >&2
    rm -f "$tmp_out"
    return 4
  fi
  # Strip leading '# ' from h1_text
  h1_text="${h1_text#\# }"

  # 2) Filename date check
  if [[ -z "$file_date" && "$allow_nondated" != "true" ]]; then
    echo "Invalid: filename does not start with YYYY-MM-DD (got: $file)" >&2
    rm -f "$tmp_out"
    return 5
  fi

  # 3) Required sections (unless nondated or template)
  if [[ "$allow_nondated" != "true" && "$is_template" != "true" ]]; then
    for title in "## Session Overview" "## Compliance" "## Tags"; do
      if ! awk -F"\t" -v t="$title" '$2 == t { found=1; exit } END{ if(!found) exit 1 }' "$tmp_out"; then
        echo "Invalid: missing $title section in $file" >&2
        rm -f "$tmp_out"
        case "$title" in
          "## Session Overview") return 7 ;;
          "## Compliance") return 8 ;;
          "## Tags") return 9 ;;
        esac
      fi
    done
  fi

  # 4) Opening code fences without language in original file
  local bad_lines
  bad_lines=$(perl -ne 'if (/^(`{3,})(.*)$/) { if (!$in) { $in=1; $lang=$2; if ($lang =~ /^\s*$/) { push @b,$. } } else { $in=0 } } END{ print join(" ",@b) }' -- "$file")
  if [[ -n "$bad_lines" ]]; then
    echo "Invalid: found opening code fence(s) without language specifier at lines: $bad_lines in $file" >&2
    rm -f "$tmp_out"
    return 10
  fi

  # 5) Headings trailing punctuation (outside blocks)
  local bad_heads
  bad_heads=$(awk -F"\t" '$2 ~ /^#{1,6} / && $2 ~ /[\.:;!?]$/ { printf("%s:%s\\n", $1, $2); found=1 } END{ if(found) exit 0 }' "$tmp_out" || true)
  if [[ -n "$bad_heads" ]]; then
    echo -n "Invalid: found heading(s) with trailing punctuation at lines: " >&2
    echo "$bad_heads" >&2
    rm -f "$tmp_out"
    return 11
  fi

  # 6) Tags validation
  # Find line index of ## Tags in the outside file (nl-ed)
  local tag_ln tag_block tokens invalid_list
  tag_ln=$(awk -F"\t" '$2 == "## Tags" { print NR; exit }' "$tmp_out")
  if [[ -n "$tag_ln" ]]; then
    # Collect following lines until next '## ' or EOF
    tag_block=$(awk -F"\t" -v start="$tag_ln" 'NR>start { if ($2 ~ /^## /) { exit } else print $2 }' "$tmp_out")
    tag_block=$(echo "$tag_block" | sed '/^\s*$/d')
    if [[ -z "$tag_block" ]]; then
      echo "Invalid: ## Tags section is empty in $file" >&2
      rm -f "$tmp_out"
      return 12
    fi
    # Extract tokens
    read -r -a tokens < <(echo "$tag_block" | grep -oE '#?[A-Za-z0-9][A-Za-z0-9_\/ -]*' || true)
    if [[ ${#tokens[@]} -eq 0 ]]; then
      echo "Invalid: no tag tokens detected in ## Tags section in $file" >&2
      rm -f "$tmp_out"
      return 13
    fi
    invalid_list=""
    for t in "${tokens[@]}"; do
      local clean cl
      clean="$t"
      if [[ "${clean:0:1}" == "#" ]]; then clean="${clean:1}"; fi
      cl="$(echo "$clean" | tr '[:upper:]' '[:lower:]')"
      if ! [[ $cl =~ ^([a-z0-9]+(-[a-z0-9]+)*)(/([a-z0-9]+(-[a-z0-9]+)*))*$ ]]; then
        invalid_list="$invalid_list $t"
      fi
    done
    if [[ -n "$invalid_list" ]]; then
      echo "Invalid: found tag(s) that do not follow Obsidian conventions (lowercase, hyphen-separated, optional parent/child): $invalid_list in $file" >&2
      rm -f "$tmp_out"
      return 14
    fi
  fi

  # 7) H1 date ending check
  if [[ -n "$file_date" && -n "$formatted_date" ]]; then
    # Trim whitespace from h1_text
    local h1_text_trimmed
    h1_text_trimmed="$(echo "$h1_text" | sed -e 's/^\s*//;s/\s*$//')"
    if [[ "$h1_text_trimmed" != *"$formatted_date" ]]; then
      echo "Invalid: H1 must end with the full date formatted as 'Month D, YYYY' (expected ending: $formatted_date) in $file" >&2
      rm -f "$tmp_out"
      return 15
    fi
  fi

  # All checks passed
  echo "OK: $file passes enhanced checks"
  rm -f "$tmp_out"
  return 0
}

# Iterate through provided files
for f in "${args[@]}"; do
  validate_single "$f" || exit $?
done

exit 0
