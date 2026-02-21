#!/usr/bin/env sh
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEMPLATES_DIR="${SCRIPT_DIR}/../templates"

print_usage() {
    cat << 'EOF'
Usage: 
  sh generate-structure.sh --plan <plan.json>     Generate from JSON plan
  sh generate-structure.sh --scan <phases-dir>    Scan existing phases
  sh generate-structure.sh --example              Print example JSON format

Options:
  --plan <file>    Path to JSON plan definition
  --scan <dir>     Scan directory for existing phase files
  --example        Print example JSON format

Templates are loaded from: ../templates/*.yaml
  - phase-readme.yaml      # Phase-level README
  - intermediate-readme.yaml # activities/steps directory README
  - group-readme.yaml      # Group directory README
  - step-file.yaml         # Step/activity leaf file
EOF
}

print_example() {
    cat << 'EOF'
{
  "outputPath": "docs/refactoring/phases",
  "phases": [
    {
      "number": "1",
      "name": "Analysis",
      "description": "Analyze codebase and design architecture",
      "type": "activities",
      "items": [
        {
          "number": "1",
          "name": "Analysis and Design",
          "description": "Complete analysis workflow",
          "subItems": [
            {
              "number": "1.1",
              "name": "Current Codebase Analysis",
              "description": "Map dependencies and identify logic leakage",
              "checklist": ["Map all imports between apps/ and libs/", "Identify Colyseus room logic"],
              "acceptanceCriteria": ["Dependency graph complete", "Logic leakage identified"],
              "status": "Not Started",
              "dependencies": []
            }
          ]
        }
      ]
    }
  ]
}
EOF
}

to_kebab_case() {
    printf '%s' "$1" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/^-*//;s/-*$//'
}

# shellcheck disable=SC2329
to_title_case() {
    first_char="$(printf '%s' "$1" | cut -c1 | tr '[:lower:]' '[:upper:]')"
    rest="$(printf '%s' "$1" | cut -c2-)"
    printf '%s%s' "$first_char" "$rest"
}

# shellcheck disable=SC2329
singularize_type() {
    if [ "$1" = "activities" ]; then
        printf 'activity'
    else
        printf 'step'
    fi
}

escape_sed() {
    printf '%s' "$1" | sed 's/[&/\]/\\&/g'
}

json_get_string() {
    _json_file="$1"
    _key="$2"
    grep "\"$_key\"" "$_json_file" | head -1 | sed 's/.*: *"\([^"]*\)".*/\1/' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//'
}

# shellcheck disable=SC2329
json_get_value() {
    _json_file="$1"
    _key="$2"
    grep "\"$_key\"" "$_json_file" | head -1 | sed 's/.*: *\(.*\)/\1/' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' | sed 's/,$//'
}

get_template_dir() {
    printf '%s' "$TEMPLATES_DIR"
}

load_yaml_field() {
    _file="$1"
    _field="$2"
    grep "^${_field}:" "$_file" | head -1 | sed "s/^${_field}: *//" | sed "s/^['\"]//;s/['\"]$//"
}

load_yaml_template_section() {
    _file="$1"
    _section_name="$2"
    
    awk -v section="$_section_name" '
        BEGIN { in_section = 0; in_template = 0; found = 0 }
        /^  - name:/ {
            if (in_template && found) { exit }
            name = $0
            gsub(/^  - name: */, "", name)
            gsub(/^["'"'"']/, "", name)
            gsub(/["'"'"']$/, "", name)
            if (name == section) { in_section = 1; found = 1; next }
            else { in_section = 0 }
        }
        /^    template: *\|/ {
            if (in_section) { in_template = 1; next }
        }
        /^      / {
            if (in_template && in_section) {
                print substr($0, 7)
            }
        }
        /^  - name:/ && in_template && !in_section { in_template = 0 }
        /^sections:/ { in_section = 0; in_template = 0 }
    ' "$_file"
}

render_simple_vars() {
    _text="$1"
    shift
    
    while [ $# -gt 0 ]; do
        _key="$1"
        _val="$2"
        shift 2
        _escaped_val="$(escape_sed "$_val")"
        _text="$(printf '%s' "$_text" | sed "s/{{ *${_key} *}}/${_escaped_val}/g")"
    done
    
    printf '%s' "$_text"
}

# shellcheck disable=SC2329
render_each_block() {
    _text="$1"
    _key="$2"
    shift 2
    
    _result=""
    for _item in "$@"; do
        _block="$_text"
        
        _item_name="$(printf '%s' "$_item" | cut -d'|' -f1)"
        _item_val="$(printf '%s' "$_item" | cut -d'|' -f2-)"
        
        _escaped_val="$(escape_sed "$_item_val")"
        _escaped_kebab="$(escape_sed "$(to_kebab_case "$_item_val")")"
        
        _block="$(printf '%s' "$_block" | sed "s/{{ *${_item_name} *}}/${_escaped_val}/g")"
        _block="$(printf '%s' "$_block" | sed "s/{{kebab *${_item_name}}}/${_escaped_kebab}/g")"
        
        _result="${_result}${_block}"
    done
    
    printf '%s' "$_result"
}

render_template_file() {
    _template_path="$1"
    shift
    
    _title="$(load_yaml_field "$_template_path" "title")"
    _title="$(render_simple_vars "$_title" "$@")"
    
    _output="# ${_title}"
    
    _section_names="$(awk '
        /^  - name:/ {
            name = $0
            gsub(/^  - name: */, "", name)
            gsub(/^["'"'"']/, "", name)
            gsub(/["'"'"']$/, "", name)
            print name
        }
    ' "$_template_path")"
    
    for _section in $_section_names; do
        _section_title="$(render_simple_vars "$_section" "$@")"
        _section_body="$(load_yaml_template_section "$_template_path" "$_section")"
        _section_body="$(render_simple_vars "$_section_body" "$@")"
        
        _output="${_output}

## ${_section_title}
${_section_body}"
    done
    
    printf '%s' "$_output"
}

# shellcheck disable=SC2329
parse_json_string_array() {
    _json_file="$1"
    _start_line="$2"
    _end_pattern="$3"
    
    awk -v start="$_start_line" -v end="$_end_pattern" '
        NR >= start && /'"$_end_pattern"'/ { exit }
        NR >= start && /"[^"]*"/ {
            gsub(/.*"/, "")
            gsub(/".*/, "")
            if (length > 0) print
        }
    ' "$_json_file"
}

# shellcheck disable=SC2329
count_json_array_items() {
    _json_file="$1"
    _context="$2"
    
    awk -v ctx="$_context" '
        index($0, ctx) > 0 { in_array = 1; count = 0 }
        in_array && /{/ { brace++; if (brace == 1) count++ }
        in_array && /}/ { brace-- }
        in_array && brace == 0 && /]/ { in_array = 0; print count; exit }
    ' "$_json_file"
}

extract_phase_info() {
    _json_file="$1"
    _phase_num="$2"
    _field="$3"
    
    awk -v phase="$_phase_num" -v field="$_field" '
        /"phases":/ { in_phases = 1; phase_count = 0; brace = 0 }
        in_phases && /{/ { brace++ }
        in_phases && /}/ { brace-- }
        in_phases && brace == 1 { phase_count++ }
        in_phases && phase_count == phase && index($0, "\""field"\"") > 0 {
            val = $0
            gsub(/.*: */, "", val)
            gsub(/,$/, "", val)
            gsub(/^"/, "", val)
            gsub(/"$/, "", val)
            print val
            exit
        }
    ' "$_json_file"
}

count_phases() {
    _json_file="$1"
    awk '
        /"phases":/ { in_phases = 1; count = 0; brace = 0 }
        in_phases && /{/ { brace++; if (brace == 1) count++ }
        in_phases && /}/ { brace-- }
        in_phases && /]/ { print count; exit }
    ' "$_json_file"
}

extract_phase_type() {
    _json_file="$1"
    _phase_num="$2"
    extract_phase_info "$_json_file" "$_phase_num" "type"
}

extract_phase_number() {
    _json_file="$1"
    _phase_num="$2"
    extract_phase_info "$_json_file" "$_phase_num" "number"
}

extract_phase_name() {
    _json_file="$1"
    _phase_num="$2"
    extract_phase_info "$_json_file" "$_phase_num" "name"
}

extract_phase_description() {
    _json_file="$1"
    _phase_num="$2"
    extract_phase_info "$_json_file" "$_phase_num" "description"
}

count_phase_items() {
    _json_file="$1"
    _phase_num="$2"
    
    awk -v phase="$_phase_num" '
        /"phases":/ { in_phases = 1; phase_count = 0 }
        in_phases && /{/ { brace++ }
        in_phases && /}/ { brace-- }
        in_phases && brace == 1 { phase_count++ }
        in_phases && phase_count == phase && /"items":/ { in_items = 1; item_count = 0; item_brace = 0 }
        in_items && /{/ { item_brace++; if (item_brace == 1) item_count++ }
        in_items && /}/ { item_brace-- }
        in_items && item_brace == 0 && /]/ { print item_count; exit }
    ' "$_json_file"
}

extract_item_field() {
    _json_file="$1"
    _phase_num="$2"
    _item_num="$3"
    _field="$4"
    
    awk -v phase="$_phase_num" -v item="$_item_num" -v field="$_field" '
        /"phases":/ { in_phases = 1; phase_count = 0 }
        in_phases && /{/ { brace++ }
        in_phases && /}/ { brace-- }
        in_phases && brace == 1 { phase_count++ }
        in_phases && phase_count == phase && /"items":/ { in_items = 1; item_count = 0 }
        in_items && /{/ { item_brace++ }
        in_items && item_brace == 1 && /{/ { item_count++ }
        in_items && item_brace == 1 && item_count == item && index($0, "\""field"\"") > 0 {
            val = $0
            gsub(/.*: */, "", val)
            gsub(/,$/, "", val)
            gsub(/^"/, "", val)
            gsub(/"$/, "", val)
            print val
            exit
        }
        in_items && /}/ { item_brace-- }
    ' "$_json_file"
}

count_item_subitems() {
    _json_file="$1"
    _phase_num="$2"
    _item_num="$3"
    
    awk -v phase="$_phase_num" -v item="$_item_num" '
        /"phases":/ { in_phases = 1; phase_count = 0 }
        in_phases && /{/ { brace++ }
        in_phases && /}/ { brace-- }
        in_phases && brace == 1 { phase_count++ }
        in_phases && phase_count == phase && /"items":/ { in_items = 1; item_count = 0 }
        in_items && /{/ { item_brace++ }
        in_items && item_brace == 1 && /{/ { item_count++ }
        in_items && item_brace == 1 && item_count == item && /"subItems":/ { in_sub = 1; sub_count = 0; sub_brace = 0 }
        in_sub && /{/ { sub_brace++; if (sub_brace == 1) sub_count++ }
        in_sub && /}/ { sub_brace-- }
        in_sub && sub_brace == 0 && /]/ { print sub_count; exit }
        in_items && /}/ { item_brace-- }
    ' "$_json_file"
}

extract_subitem_field() {
    _json_file="$1"
    _phase_num="$2"
    _item_num="$3"
    _subitem_num="$4"
    _field="$5"
    
    awk -v phase="$_phase_num" -v item="$_item_num" -v sub="$_subitem_num" -v field="$_field" '
        /"phases":/ { in_phases = 1; phase_count = 0 }
        in_phases && /{/ { brace++ }
        in_phases && /}/ { brace-- }
        in_phases && brace == 1 { phase_count++ }
        in_phases && phase_count == phase && /"items":/ { in_items = 1; item_count = 0 }
        in_items && /{/ { item_brace++ }
        in_items && item_brace == 1 && /{/ { item_count++ }
        in_items && item_brace == 1 && item_count == item && /"subItems":/ { in_sub = 1; sub_count = 0 }
        in_sub && /{/ { sub_brace++ }
        in_sub && sub_brace == 1 && /{/ { sub_count++ }
        in_sub && sub_brace == 1 && sub_count == sub && index($0, "\""field"\"") > 0 {
            val = $0
            gsub(/.*: */, "", val)
            if (index(val, "[") == 1) {
                print "ARRAY"
                exit
            }
            gsub(/,$/, "", val)
            gsub(/^"/, "", val)
            gsub(/"$/, "", val)
            print val
            exit
        }
        in_sub && /}/ { sub_brace-- }
        in_items && /}/ { item_brace-- }
    ' "$_json_file"
}

extract_subitem_array() {
    _json_file="$1"
    _phase_num="$2"
    _item_num="$3"
    _subitem_num="$4"
    _field="$5"
    
    awk -v phase="$_phase_num" -v item="$_item_num" -v sub="$_subitem_num" -v field="$_field" '
        /"phases":/ { in_phases = 1; phase_count = 0 }
        in_phases && /{/ { brace++ }
        in_phases && /}/ { brace-- }
        in_phases && brace == 1 { phase_count++ }
        in_phases && phase_count == phase && /"items":/ { in_items = 1; item_count = 0 }
        in_items && /{/ { item_brace++ }
        in_items && item_brace == 1 && /{/ { item_count++ }
        in_items && item_brace == 1 && item_count == item && /"subItems":/ { in_sub = 1; sub_count = 0 }
        in_sub && /{/ { sub_brace++ }
        in_sub && sub_brace == 1 && /{/ { sub_count++ }
        in_sub && sub_brace == 1 && sub_count == sub && index($0, "\""field"\"") { in_array = 1; next }
        in_array && /]/ { in_array = 0; exit }
        in_array && /"[^"]*"/ {
            val = $0
            gsub(/.*"/, "", val)
            gsub(/".*/, "", val)
            if (length(val) > 0) print val
        }
        in_sub && /}/ { sub_brace-- }
        in_items && /}/ { item_brace-- }
    ' "$_json_file"
}

generate_phase_structure() {
    _base_path="$1"
    _json_file="$2"
    _phase_num="$3"
    
    _phase_number="$(extract_phase_number "$_json_file" "$_phase_num")"
    _phase_name="$(extract_phase_name "$_json_file" "$_phase_num")"
    _phase_desc="$(extract_phase_description "$_json_file" "$_phase_num")"
    _phase_type="$(extract_phase_type "$_json_file" "$_phase_num")"
    
    _type_singular="$(singularize_type "$_phase_type")"
    _phase_name_kebab="$(to_kebab_case "$_phase_name")"
    
    _phase_dir="${_base_path}/phase-${_phase_number}-${_phase_name_kebab}"
    
    printf 'Creating phase: %s\n' "$_phase_dir"
    mkdir -p "$_phase_dir"
    
    _templates_dir="$(get_template_dir)"
    
    printf '%s\n' "Generating phase README..."
    _phase_readme="$(render_template_file "${_templates_dir}/phase-readme.yaml" \
        "number" "$_phase_number" \
        "name" "$_phase_name" \
        "description" "$_phase_desc" \
        "type" "$_phase_type" \
        "type_singular" "$_type_singular")"
    
    printf '%s\n' "$_phase_readme" > "${_phase_dir}/README.md"
    
    _type_dir="${_phase_dir}/${_phase_type}"
    mkdir -p "$_type_dir"
    
    _intermediate_readme="$(render_template_file "${_templates_dir}/intermediate-readme.yaml" \
        "type" "$_phase_type")"
    printf '%s\n' "$_intermediate_readme" > "${_type_dir}/README.md"
    
    _item_count="$(count_phase_items "$_json_file" "$_phase_num")"
    _i=1
    while [ "$_i" -le "$_item_count" ]; do
        _item_number="$(extract_item_field "$_json_file" "$_phase_num" "$_i" "number")"
        _item_name="$(extract_item_field "$_json_file" "$_phase_num" "$_i" "name")"
        _item_desc="$(extract_item_field "$_json_file" "$_phase_num" "$_i" "description")"
        
        _item_name_kebab="$(to_kebab_case "$_item_name")"
        _group_dir="${_type_dir}/${_type_singular}-${_item_number}-${_item_name_kebab}"
        
        printf '  Creating group: %s\n' "$_group_dir"
        mkdir -p "$_group_dir"
        
        _subitem_count="$(count_item_subitems "$_json_file" "$_phase_num" "$_i")"
        
        _sub_items_list=""
        _j=1
        while [ "$_j" -le "$_subitem_count" ]; do
            _sub_number="$(extract_subitem_field "$_json_file" "$_phase_num" "$_i" "$_j" "number")"
            _sub_name="$(extract_subitem_field "$_json_file" "$_phase_num" "$_i" "$_j" "name")"
            _sub_desc="$(extract_subitem_field "$_json_file" "$_phase_num" "$_i" "$_j" "description")"
            
            if [ -n "$_sub_items_list" ]; then
                _sub_items_list="${_sub_items_list}|${_sub_number}|${_sub_name}|${_sub_desc}"
            else
                _sub_items_list="${_sub_number}|${_sub_name}|${_sub_desc}"
            fi
            _j=$((_j + 1))
        done
        
        _group_readme="$(render_template_file "${_templates_dir}/group-readme.yaml" \
            "type_singular" "$_type_singular" \
            "number" "$_item_number" \
            "name" "$_item_name" \
            "description" "$_item_desc" \
            "sub_items_count" "$_subitem_count")"
        printf '%s\n' "$_group_readme" > "${_group_dir}/README.md"
        
        _j=1
        while [ "$_j" -le "$_subitem_count" ]; do
            _sub_number="$(extract_subitem_field "$_json_file" "$_phase_num" "$_i" "$_j" "number")"
            _sub_name="$(extract_subitem_field "$_json_file" "$_phase_num" "$_i" "$_j" "name")"
            _sub_desc="$(extract_subitem_field "$_json_file" "$_phase_num" "$_i" "$_j" "description")"
            _sub_status="$(extract_subitem_field "$_json_file" "$_phase_num" "$_i" "$_j" "status")"
            
            if [ -z "$_sub_status" ]; then
                _sub_status="Not Started"
            fi
            
            _checklist_items=""
            _checklist_count=0
            for _check_item in $(extract_subitem_array "$_json_file" "$_phase_num" "$_i" "$_j" "checklist"); do
                _checklist_count=$((_checklist_count + 1))
                if [ -n "$_checklist_items" ]; then
                    _checklist_items="${_checklist_items}|this|${_check_item}"
                else
                    _checklist_items="this|${_check_item}"
                fi
            done
            
            _criteria_items=""
            _criteria_count=0
            for _crit_item in $(extract_subitem_array "$_json_file" "$_phase_num" "$_i" "$_j" "acceptanceCriteria"); do
                _criteria_count=$((_criteria_count + 1))
                if [ -n "$_criteria_items" ]; then
                    _criteria_items="${_criteria_items}|this|${_crit_item}"
                else
                    _criteria_items="this|${_crit_item}"
                fi
            done
            
            _dep_items=""
            _dep_count=0
            for _dep_item in $(extract_subitem_array "$_json_file" "$_phase_num" "$_i" "$_j" "dependencies"); do
                _dep_count=$((_dep_count + 1))
                if [ -n "$_dep_items" ]; then
                    _dep_items="${_dep_items}|this|${_dep_item}"
                else
                    _dep_items="this|${_dep_item}"
                fi
            done
            
            _sub_name_kebab="$(to_kebab_case "$_sub_name")"
            _filename="${_type_singular}-${_sub_number}-${_sub_name_kebab}.md"
            printf '    Creating file: %s\n' "$_filename"
            
            _step_content="# ${_type_singular} ${_sub_number}: ${_sub_name}

## Description
${_sub_desc}

## Checklist"
            
            for _check_item in $(extract_subitem_array "$_json_file" "$_phase_num" "$_i" "$_j" "checklist"); do
                _step_content="${_step_content}
- [ ] ${_check_item}"
            done
            
            _step_content="${_step_content}

## Acceptance Criteria"
            
            for _crit_item in $(extract_subitem_array "$_json_file" "$_phase_num" "$_i" "$_j" "acceptanceCriteria"); do
                _step_content="${_step_content}
- ${_crit_item}"
            done
            
            _step_content="${_step_content}

## Status
**Current State**: ${_sub_status}

## Notes
_Add implementation notes, decisions, and learnings here._

## Dependencies"
            
            _dep_found=0
            for _dep_item in $(extract_subitem_array "$_json_file" "$_phase_num" "$_i" "$_j" "dependencies"); do
                _dep_found=1
                _step_content="${_step_content}
- ${_dep_item}"
            done
            
            if [ "$_dep_found" -eq 0 ]; then
                _step_content="${_step_content}
None"
            fi
            
            printf '%s\n' "$_step_content" > "${_group_dir}/${_filename}"
            
            _j=$((_j + 1))
        done
        
        _i=$((_i + 1))
    done
}

generate_from_plan() {
    _plan_path="$1"
    
    if [ ! -f "$_plan_path" ]; then
        printf 'Error: Plan file not found: %s\n' "$_plan_path" >&2
        exit 1
    fi
    
    _output_path="$(json_get_string "$_plan_path" "outputPath")"
    
    if [ -z "$_output_path" ]; then
        printf 'Error: outputPath not found in plan file\n' >&2
        exit 1
    fi
    
    printf 'Generating structure at: %s\n' "$_output_path"
    mkdir -p "$_output_path"
    
    _phase_count="$(count_phases "$_plan_path")"
    
    if [ -z "$_phase_count" ] || [ "$_phase_count" -eq 0 ]; then
        printf 'Error: No phases found in plan file\n' >&2
        exit 1
    fi
    
    _p=1
    while [ "$_p" -le "$_phase_count" ]; do
        generate_phase_structure "$_output_path" "$_plan_path" "$_p"
        _p=$((_p + 1))
    done
    
    printf '\nGeneration complete!\n'
}

generate_from_existing_phases() {
    _phases_dir="$1"
    
    printf 'Scanning existing phases in: %s\n' "$_phases_dir"
    
    if [ ! -d "$_phases_dir" ]; then
        printf 'Directory not found: %s\n' "$_phases_dir" >&2
        exit 1
    fi
    
    _phase_files="$(find "$_phases_dir" -maxdepth 1 -name 'phase-*.md' -type f 2>/dev/null | head -20)"
    
    if [ -z "$_phase_files" ]; then
        printf 'No flat phase files found to split.\n'
        return 0
    fi
    
    _count="$(printf '%s\n' "$_phase_files" | wc -l | tr -d ' ')"
    printf 'Found %s phase files to split\n' "$_count"
    printf 'Note: Automatic parsing of markdown requires manual configuration.\n'
    printf 'Use --plan option with a JSON definition for full automation.\n'
}

if [ $# -eq 0 ]; then
    print_usage
    exit 0
fi

case "$1" in
    --help|-h)
        print_usage
        exit 0
        ;;
    --example)
        print_example
        exit 0
        ;;
    --plan)
        if [ -z "$2" ]; then
            printf 'Error: --plan requires a file path\n' >&2
            exit 1
        fi
        generate_from_plan "$2"
        exit 0
        ;;
    --scan)
        if [ -z "$2" ]; then
            printf 'Error: --scan requires a directory path\n' >&2
            exit 1
        fi
        generate_from_existing_phases "$2"
        exit 0
        ;;
    *)
        printf 'Error: Unknown option: %s\n' "$1" >&2
        print_usage
        exit 1
        ;;
esac
