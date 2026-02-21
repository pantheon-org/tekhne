#!/usr/bin/env sh

set -e

DEFAULT_REPORT=".context/audits/skill-quality-auditor-review.md"
DEFAULT_TEMPLATE="skills/skill-quality-auditor/templates/review-report-template.yaml"
DEFAULT_SCHEMA="skills/skill-quality-auditor/schemas/review-report.schema.json"
DEFAULT_REQUIREMENTS="skills/skill-quality-auditor/references/review-report.requirements.json"

strict_recommended=0
report_arg=""

for arg in "$@"; do
  case "$arg" in
    --strict-recommended)
      strict_recommended=1
      ;;
    -*)
      ;;
    *)
      if [ -z "$report_arg" ]; then
        report_arg="$arg"
      fi
      ;;
  esac
done

report_path="${report_arg:-$DEFAULT_REPORT}"
template_path="$DEFAULT_TEMPLATE"
schema_path="$DEFAULT_SCHEMA"
requirements_path="$DEFAULT_REQUIREMENTS"

missing_files=""
for f in "$report_path" "$template_path" "$schema_path" "$requirements_path"; do
  if [ ! -f "$f" ]; then
    missing_files="$missing_files $f"
  fi
done

if [ -n "$missing_files" ]; then
  printf "Missing required file:%s\n" "$missing_files" >&2
  exit 1
fi

report_content=$(cat "$report_path")
template_content=""

load_yaml_markdown_template() {
  _file="$1"
  awk '
    BEGIN { in_block = 0 }
    /^report_template_markdown:[[:space:]]*\|[[:space:]]*$/ {
      in_block = 1
      next
    }
    in_block == 1 {
      if ($0 ~ /^  /) {
        print substr($0, 3)
      } else if ($0 ~ /^[[:space:]]*$/) {
        print ""
      } else {
        exit
      }
    }
  ' "$_file"
}

case "$template_path" in
  *.yaml|*.yml)
    template_content=$(load_yaml_markdown_template "$template_path")
    if [ -z "$template_content" ]; then
      printf "Template YAML missing 'report_template_markdown: |' block: %s\n" "$template_path" >&2
      exit 1
    fi
    ;;
  *)
    template_content=$(cat "$template_path")
    ;;
esac

json_get_string() {
  _file="$1"
  _key="$2"
  sed -n "s/.*\"${_key}\"[[:space:]]*:[[:space:]]*\"\([^\"]*\)\".*/\1/p" "$_file" | head -1
}

json_get_string_array_items() {
  _file="$1"
  _key="$2"
  awk -v key="\"$_key\"" '
    BEGIN { in_array = 0; in_string = 0; item = ""; }
    $0 ~ key"[[:space:]]*:[[:space:]]*\\[" {
      in_array = 1
      line = $0
      sub(/.*:[[:space:]]*\[/, "", line)
      $0 = line
    }
    in_array {
      for (i = 1; i <= length($0); i++) {
        c = substr($0, i, 1)
        if (c == "\"" && (i == 1 || substr($0, i-1, 1) != "\\")) {
          in_string = !in_string
          if (in_string) { item = ""; continue }
          else { if (item != "") { print item; item = "" } continue }
        }
        if (in_string) item = item c
        if (!in_string && c == "]") { in_array = 0; exit }
      }
    }
  ' "$_file"
}

json_get_nested_string_arrays() {
  _file="$1"
  _key="$2"
  awk -v key="\"$_key\"" '
    BEGIN {
      in_outer = 0
      depth = 0
      buffer = ""
    }
    $0 ~ key"[[:space:]]*:[[:space:]]*\\[" {
      in_outer = 1
      depth = 1
      line = $0
      sub(/.*:[[:space:]]*\[/, "", line)
      $0 = line
    }
    in_outer {
      for (i = 1; i <= length($0); i++) {
        c = substr($0, i, 1)
        if (c == "[") {
          if (depth == 1) buffer = ""
          depth++
        }
        if (c == "]") {
          depth--
          if (depth == 1 && buffer != "") {
            print buffer
            buffer = ""
          }
        }
        if (depth > 1 && c != "[" && c != "]") {
          buffer = buffer c
        }
      }
      if (depth == 0) { in_outer = 0; exit }
    }
  ' "$_file" | awk '
    {
      line = $0
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", line)
      
      in_string = 0
      current = ""
      result = ""
      
      for (i = 1; i <= length(line); i++) {
        c = substr(line, i, 1)
        if (c == "\"") {
          in_string = !in_string
          if (!in_string && current != "") {
            if (result != "") result = result "|"
            result = result current
            current = ""
          }
          continue
        }
        if (in_string) current = current c
      }
      
      if (result != "") print result
    }
  '
}

required_title_prefix=$(json_get_string "$requirements_path" "required_title_prefix")

if [ -z "$required_title_prefix" ]; then
  printf "Requirements schema validation failed:\n- missing required property: required_title_prefix\n" >&2
  exit 1
fi

extract_h1() {
  printf "%s\n" "$1" | grep "^# " | head -1 | sed 's/^# //'
}

extract_h2_headings() {
  printf "%s\n" "$1" | grep "^## " | sed 's/^## //'
}

extract_frontmatter() {
  awk '
    BEGIN { delim_count = 0 }
    NR == 1 && $0 == "---" {
      delim_count = 1
      next
    }
    delim_count == 1 {
      if ($0 == "---") {
        delim_count = 2
        exit
      }
      print
    }
  ' <<EOF
$1
EOF
}

has_frontmatter_key() {
  _content="$1"
  _key="$2"
  printf "%s\n" "$_content" | grep -q "^${_key}:[[:space:]]*"
}

report_title=$(extract_h1 "$report_content")
template_title=$(extract_h1 "$template_content")
report_h2=$(extract_h2_headings "$report_content" | sort -u)
report_h2_list=$(extract_h2_headings "$report_content")
report_frontmatter=$(extract_frontmatter "$report_content")
template_frontmatter=$(extract_frontmatter "$template_content")

errors_file=$(mktemp)
warnings_file=$(mktemp)
trap 'rm -f "$errors_file" "$warnings_file"' EXIT

if [ -z "$report_title" ]; then
  echo "missing H1 title" >> "$errors_file"
else
  case "$report_title" in
    "$required_title_prefix"*) ;;
    *)
      echo "title must start with '${required_title_prefix}'" >> "$errors_file"
      ;;
  esac
fi

case "$template_title" in
  "$required_title_prefix"*) ;;
  *)
    echo "template title is out of sync with required title prefix" >> "$errors_file"
    ;;
esac

json_get_string_array_items "$requirements_path" "required_frontmatter_keys" | while IFS= read -r key; do
  [ -z "$key" ] && continue
  if ! has_frontmatter_key "$report_frontmatter" "$key"; then
    echo "missing required frontmatter key: ${key}" >> "$errors_file"
  fi
  if ! has_frontmatter_key "$template_frontmatter" "$key"; then
    echo "template missing required frontmatter key: ${key}" >> "$errors_file"
  fi
done

json_get_string_array_items "$requirements_path" "recommended_frontmatter_keys" | while IFS= read -r key; do
  [ -z "$key" ] && continue
  if ! has_frontmatter_key "$report_frontmatter" "$key"; then
    msg="missing recommended frontmatter key: ${key}"
    if [ "$strict_recommended" -eq 1 ]; then
      echo "$msg" >> "$errors_file"
    else
      echo "$msg" >> "$warnings_file"
    fi
  fi
done

json_get_string_array_items "$requirements_path" "required_metadata_labels" | while IFS= read -r label; do
  [ -z "$label" ] && continue
  case "$report_content" in
    *"**${label}**:"*) ;;
    *)
      echo "missing metadata label: ${label}" >> "$errors_file"
      ;;
  esac
done

json_get_string_array_items "$requirements_path" "recommended_metadata_labels" | while IFS= read -r label; do
  [ -z "$label" ] && continue
  case "$report_content" in
    *"**${label}**:"*) ;;
    *)
      msg="missing recommended metadata label: ${label}"
      if [ "$strict_recommended" -eq 1 ]; then
        echo "$msg" >> "$errors_file"
      else
        echo "$msg" >> "$warnings_file"
      fi
      ;;
  esac
done

check_h2_group() {
  _group="$1"
  _h2="$2"
  
  echo "$_group" | tr '|' '\n' | while IFS= read -r alt; do
    [ -z "$alt" ] && continue
    if printf "%s\n" "$_h2" | grep -qx "$alt"; then
      echo "found"
      return 0
    fi
  done
}

json_get_nested_string_arrays "$requirements_path" "required_h2_groups" | while IFS= read -r group; do
  [ -z "$group" ] && continue
  found=$(check_h2_group "$group" "$report_h2")
  if [ "$found" != "found" ]; then
    alts=$(echo "$group" | tr '|' ', ')
    echo "missing required H2 heading group (one of): ${alts}" >> "$errors_file"
  fi
done

find_first_heading_index() {
  _group="$1"
  _list="$2"
  
  awk -v group="$_group" '
    BEGIN {
      split(group, alts, "|")
      first_idx = -1
      idx = 0
    }
    {
      heading = $0
      for (i in alts) {
        if (heading == alts[i]) {
          if (first_idx < 0) first_idx = idx
        }
      }
      idx++
    }
    END { print first_idx }
  ' <<EOF
$_list
EOF
}

previous_idx=-1
json_get_nested_string_arrays "$requirements_path" "required_h2_order" | while IFS= read -r group; do
  [ -z "$group" ] && continue
  group_idx=$(find_first_heading_index "$group" "$report_h2_list")
  if [ "$group_idx" -lt 0 ]; then
    alts=$(echo "$group" | tr '|' ', ')
    echo "missing ordered H2 heading group (one of): ${alts}" >> "$errors_file"
  elif [ "$previous_idx" -ge 0 ] && [ "$group_idx" -lt "$previous_idx" ]; then
    alts=$(echo "$group" | tr '|' ', ')
    echo "H2 order violation near group: ${alts}; expected after prior required section" >> "$errors_file"
  fi
  if [ "$group_idx" -ge 0 ]; then
    previous_idx=$group_idx
  fi
done

json_get_nested_string_arrays "$requirements_path" "recommended_h2_groups" | while IFS= read -r group; do
  [ -z "$group" ] && continue
  found=$(check_h2_group "$group" "$report_h2")
  if [ "$found" != "found" ]; then
    alts=$(echo "$group" | tr '|' ', ')
    msg="missing recommended H2 heading group (one of): ${alts}"
    if [ "$strict_recommended" -eq 1 ]; then
      echo "$msg" >> "$errors_file"
    else
      echo "$msg" >> "$warnings_file"
    fi
  fi
done

json_get_string_array_items "$requirements_path" "required_dimension_labels" | while IFS= read -r label; do
  [ -z "$label" ] && continue
  case "$report_content" in
    *"$label"*) ;;
    *)
      echo "missing dimension label: ${label}" >> "$errors_file"
      ;;
  esac
done

json_get_string_array_items "$requirements_path" "required_commands" | while IFS= read -r cmd; do
  [ -z "$cmd" ] && continue
  case "$report_content" in
    *"$cmd"*) ;;
    *)
      echo "missing required command: ${cmd}" >> "$errors_file"
      ;;
  esac
done

json_get_string_array_items "$requirements_path" "recommended_commands" | while IFS= read -r cmd; do
  [ -z "$cmd" ] && continue
  case "$report_content" in
    *"$cmd"*) ;;
    *)
      msg="missing recommended command: ${cmd}"
      if [ "$strict_recommended" -eq 1 ]; then
        echo "$msg" >> "$errors_file"
      else
        echo "$msg" >> "$warnings_file"
      fi
      ;;
  esac
done

if [ -s "$warnings_file" ]; then
  printf "Review format warnings for: %s\n" "$report_path" >&2
  while IFS= read -r line; do
    [ -n "$line" ] && printf '%s\n' "- $line" >&2
  done < "$warnings_file"
fi

if [ -s "$errors_file" ]; then
  printf "Review format validation failed for: %s\n" "$report_path" >&2
  while IFS= read -r line; do
    [ -n "$line" ] && printf '%s\n' "- $line" >&2
  done < "$errors_file"
  exit 1
fi

printf "Review format validation passed: %s\n" "$report_path"
