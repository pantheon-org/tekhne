#!/usr/bin/env sh
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SCHEMAS_DIR="${SCRIPT_DIR}/../schemas"

error_count=0
warning_count=0
phase_count=0
group_count=0
leaf_count=0
readme_count=0
schema_validated_count=0
total_file_count=0

error_messages=""
warning_messages=""

log_error() {
    error_type="$1"
    path="$2"
    message="$3"
    error_count=$((error_count + 1))
    error_messages="${error_messages}   [${error_type}] ${path}
      ${message}
"
}

# shellcheck disable=SC2329
log_warning() {
    warning_type="$1"
    path="$2"
    message="$3"
    warning_count=$((warning_count + 1))
    warning_messages="${warning_messages}   [${warning_type}] ${path}
      ${message}
"
}

check_jq() {
    if ! command -v jq >/dev/null 2>&1; then
        printf "Error: 'jq' is required for JSON schema validation.\\n" >&2
        printf "Install with: brew install jq (macOS) or apt install jq (Linux)\\n" >&2
        exit 1
    fi
}

matches_pattern() {
    string="$1"
    pattern="$2"
    printf "%s" "$string" | grep -Eq "$pattern"
}

is_phase_dir() {
    matches_pattern "$1" '^phase-[0-9]+(\.[0-9]+)*-[a-z0-9-]+$'
}

# shellcheck disable=SC2329
is_activity_dir() {
    matches_pattern "$1" '^activity-[0-9]+(\.[0-9]+)*-[a-z0-9-]+$'
}

# shellcheck disable=SC2329
is_step_dir() {
    matches_pattern "$1" '^step-[0-9]+(\.[0-9]+)*-[a-z0-9-]+$'
}

is_leaf_file() {
    matches_pattern "$1" '^(activity|step)-[0-9]+(\.[0-9]+)+-[a-z0-9-]+\.md$'
}

has_description() {
    name="$1"
    case "$name" in
        phase-[0-9] | phase-[0-9].[0-9]* | \
        activity-[0-9] | activity-[0-9].[0-9]* | \
        step-[0-9] | step-[0-9].[0-9]*)
            return 1
            ;;
        *)
            return 0
            ;;
    esac
}

parse_markdown_title() {
    file="$1"
    if [ -f "$file" ]; then
        head -20 "$file" | grep -m1 '^# ' | sed 's/^# //'
    fi
}

parse_section_content() {
    file="$1"
    section_name="$2"
    in_section=0
    content=""
    
    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in
            "## $section_name" | "## ${section_name}"*)
                in_section=1
                continue
                ;;
            "## "*)
                if [ "$in_section" -eq 1 ]; then
                    break
                fi
                ;;
            *)
                if [ "$in_section" -eq 1 ]; then
                    content="${content}${line}
"
                fi
                ;;
        esac
    done < "$file"
    printf "%s" "$content"
}

parse_checklist() {
    file="$1"
    count=0
    in_checklist=0
    
    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in
            "## Checklist" | "## checklist"*)
                in_checklist=1
                continue
                ;;
            "## "*)
                if [ "$in_checklist" -eq 1 ]; then
                    break
                fi
                ;;
            *)
                if [ "$in_checklist" -eq 1 ]; then
                    case "$line" in
                        *"- [ ]"* | *"- [x]"*)
                            count=$((count + 1))
                            ;;
                    esac
                fi
                ;;
        esac
    done < "$file"
    printf "%d" "$count"
}

parse_acceptance_criteria() {
    file="$1"
    count=0
    in_section=0
    
    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in
            "## Acceptance Criteria" | "## Acceptance criteria" | "## acceptance criteria"*)
                in_section=1
                continue
                ;;
            "## "*)
                if [ "$in_section" -eq 1 ]; then
                    break
                fi
                ;;
            *)
                if [ "$in_section" -eq 1 ]; then
                    case "$line" in
                        *"- "*)
                            count=$((count + 1))
                            ;;
                    esac
                fi
                ;;
        esac
    done < "$file"
    printf "%d" "$count"
}

parse_status() {
    file="$1"
    status=""
    in_section=0
    
    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in
            "## Status" | "## status"*)
                in_section=1
                continue
                ;;
            "## "*)
                if [ "$in_section" -eq 1 ]; then
                    break
                fi
                ;;
            *)
                if [ "$in_section" -eq 1 ]; then
                    case "$line" in
                        *"status:"* | *"Status:"*)
                            status=$(printf "%s" "$line" | sed -n 's/.*[Ss]tatus:[[:space:]]*\([^[:space:]]*\).*/\1/p')
                            if [ -n "$status" ]; then
                                printf "%s" "$status"
                                return
                            fi
                            ;;
                    esac
                fi
                ;;
        esac
    done < "$file"
    printf "pending"
}

validate_step_file() {
    file="$1"
    rel_path="$2"
    
    if [ ! -f "$file" ]; then
        return
    fi
    
    schema_validated_count=$((schema_validated_count + 1))
    
    title=$(parse_markdown_title "$file")
    if [ -z "$title" ]; then
        log_error "schema_validation" "$rel_path" "Missing required field: title"
    fi
    
    description=$(parse_section_content "$file" "Description")
    if [ -z "$description" ]; then
        log_error "schema_validation" "$rel_path" "Missing required field: description"
    fi
    
    checklist_count=$(parse_checklist "$file")
    if [ "$checklist_count" -eq 0 ]; then
        log_error "schema_validation" "$rel_path" "Missing required field: checklist (must have at least 1 item)"
    fi
    
    ac_count=$(parse_acceptance_criteria "$file")
    if [ "$ac_count" -eq 0 ]; then
        log_error "schema_validation" "$rel_path" "Missing required field: acceptanceCriteria (must have at least 1 item)"
    fi
    
    status=$(parse_status "$file")
    case "$status" in
        pending | in-progress | completed | blocked | cancelled)
            ;;
        *)
            log_error "schema_validation" "$rel_path" "Invalid status value: $status (must be: pending, in-progress, completed, blocked, cancelled)"
            ;;
    esac
}

validate_readme_file() {
    file="$1"
    rel_path="$2"
    
    if [ ! -f "$file" ]; then
        return
    fi
    
    schema_validated_count=$((schema_validated_count + 1))
    
    title=$(parse_markdown_title "$file")
    if [ -z "$title" ]; then
        log_error "schema_validation" "$rel_path" "Missing required field: title"
    fi
    
    description=$(parse_section_content "$file" "Description")
    if [ -z "$description" ]; then
        log_error "schema_validation" "$rel_path" "Missing required field: description"
    fi
    
    contents=$(parse_section_content "$file" "Contents")
    if [ -z "$contents" ]; then
        log_error "schema_validation" "$rel_path" "Missing required field: contents"
    fi
}

check_broken_links() {
    readme="$1"
    base_dir="$2"
    rel_path="$3"
    
    if [ ! -f "$readme" ]; then
        return
    fi
    
    while IFS= read -r link; do
        link_text=$(printf "%s" "$link" | sed 's/^\[//' | sed 's/\].*//')
        link_path=$(printf "%s" "$link" | sed 's/.*](//' | sed 's/)$//')
        
        case "$link_path" in
            http://* | https://* | \#*)
                continue
                ;;
        esac
        
        full_path="${base_dir}/${link_path}"
        if [ ! -e "$full_path" ]; then
            log_error "broken_link" "$rel_path" "Broken link: [$link_text]($link_path)"
        fi
    done <<EOF
$(grep -oE '\[[^\]]+\]\([^)]+\)' "$readme" 2>/dev/null || true)
EOF
}

validate_directory_naming() {
    dir="$1"
    base="$2"
    
    for subdir in "$dir"/*; do
        [ -d "$subdir" ] || continue
        
        name=$(basename "$subdir")
        rel_path=$(printf "%s" "$subdir" | sed "s|^${base}/||")
        
        case "$name" in
            phase-* | activity-* | step-*)
                if ! has_description "$name"; then
                    log_error "invalid_name" "$rel_path" "Directory name lacks description: $name"
                fi
                ;;
        esac
        
        validate_directory_naming "$subdir" "$base"
    done
}

validate_group() {
    group_dir="$1"
    # shellcheck disable=SC2034
    group_type="$2"
    base="$3"
    
    rel_path=$(printf "%s" "$group_dir" | sed "s|^${base}/||")
    
    readme="${group_dir}/README.md"
    if [ ! -f "$readme" ]; then
        log_error "missing_readme" "$rel_path" "Missing README.md"
    else
        readme_count=$((readme_count + 1))
        validate_readme_file "$readme" "$rel_path"
        check_broken_links "$readme" "$group_dir" "$rel_path"
    fi
    
    for entry in "$group_dir"/*; do
        [ -e "$entry" ] || continue
        
        name=$(basename "$entry")
        entry_rel_path=$(printf "%s" "$entry" | sed "s|^${base}/||")
        
        if [ -f "$entry" ]; then
            if is_leaf_file "$name"; then
                leaf_count=$((leaf_count + 1))
                total_file_count=$((total_file_count + 1))
                validate_step_file "$entry" "$entry_rel_path"
            fi
        fi
    done
}

validate_container() {
    container_dir="$1"
    container_type="$2"
    base="$3"
    
    rel_path=$(printf "%s" "$container_dir" | sed "s|^${base}/||")
    
    readme="${container_dir}/README.md"
    if [ ! -f "$readme" ]; then
        log_error "missing_readme" "$rel_path" "Missing README.md"
    else
        readme_count=$((readme_count + 1))
    fi
    
    pattern_func="is_${container_type}_dir"
    
    for subdir in "$container_dir"/*; do
        [ -d "$subdir" ] || continue
        
        name=$(basename "$subdir")
        
        if $pattern_func "$name"; then
            group_count=$((group_count + 1))
            validate_group "$subdir" "$container_type" "$base"
        fi
    done
}

validate_phase() {
    phase_dir="$1"
    base="$2"
    
    rel_path=$(printf "%s" "$phase_dir" | sed "s|^${base}/||")
    phase_count=$((phase_count + 1))
    
    readme="${phase_dir}/README.md"
    if [ ! -f "$readme" ]; then
        log_error "missing_readme" "$rel_path" "Missing README.md"
    else
        readme_count=$((readme_count + 1))
        validate_readme_file "$readme" "$rel_path"
        check_broken_links "$readme" "$phase_dir" "$rel_path"
    fi
    
    activities_dir="${phase_dir}/activities"
    steps_dir="${phase_dir}/steps"
    
    if [ -d "$activities_dir" ]; then
        validate_container "$activities_dir" "activity" "$base"
    elif [ -d "$steps_dir" ]; then
        validate_container "$steps_dir" "step" "$base"
    else
        log_error "invalid_structure" "$rel_path" "Missing 'activities' or 'steps' directory"
    fi
}

validate_structure() {
    root="$1"
    
    if [ ! -d "$root" ]; then
        log_error "invalid_structure" "$root" "Root path does not exist"
        return 1
    fi
    
    if [ ! -f "${SCHEMAS_DIR}/step-file.schema.json" ]; then
        log_error "schema_validation" "$SCHEMAS_DIR" "Schema not found: step-file.schema.json"
        return 1
    fi
    
    if [ ! -f "${SCHEMAS_DIR}/readme-file.schema.json" ]; then
        log_error "schema_validation" "$SCHEMAS_DIR" "Schema not found: readme-file.schema.json"
        return 1
    fi
    
    for entry in "$root"/*; do
        [ -d "$entry" ] || continue
        
        name=$(basename "$entry")
        
        if is_phase_dir "$name"; then
            validate_phase "$entry" "$root"
        fi
    done
    
    validate_directory_naming "$root" "$root"
}

print_report() {
    root="$1"
    
    printf "\\n"
    printf "============================================================\\n"
    printf "IMPLEMENTATION PLAN STRUCTURE VALIDATION REPORT\\n"
    printf "============================================================\\n"
    printf "\\nRoot: %s\\n" "$root"
    
    if [ "$error_count" -eq 0 ]; then
        printf "Status: VALID\\n"
    else
        printf "Status: INVALID\\n"
    fi
    
    printf "\\nStatistics:\\n"
    printf "   Phases:           %d\\n" "$phase_count"
    printf "   Groups:           %d\\n" "$group_count"
    printf "   Leaf Files:       %d\\n" "$leaf_count"
    printf "   READMEs:          %d\\n" "$readme_count"
    printf "   Schema Validated: %d\\n" "$schema_validated_count"
    printf "   Total Files:      %d\\n" "$total_file_count"
    
    if [ -n "$error_messages" ]; then
        printf "\\nErrors:\\n"
        printf "%s" "$error_messages"
    fi
    
    if [ -n "$warning_messages" ]; then
        printf "\\nWarnings:\\n"
        printf "%s" "$warning_messages"
    fi
    
    printf "\\n============================================================\\n"
    
    if [ "$error_count" -eq 0 ]; then
        printf "Structure is valid!\\n"
    else
        printf "Found %d error(s) that need to be fixed.\\n" "$error_count"
    fi
    
    printf "============================================================\\n\\n"
}

print_usage() {
    printf "\
Usage:
  %s <phases-directory>
  %s --help

Options:
  <phases-directory>   Path to the phases directory to validate
  --help, -h           Show this help message
  --schema             Show schema information

Examples:
  %s docs/refactoring/phases
  %s ./phases

Validation checks:
  * Every directory has a README.md
  * Directory names include descriptions (e.g., step-1-extract-logic)
  * Leaf files validated against step-file.schema.json
  * README files validated against readme-file.schema.json
  * Required fields: title, description, checklist, acceptanceCriteria, status
  * All links in READMEs resolve
  * Proper hierarchy (phase -> activities/steps -> groups -> leaf files)
" "$0" "$0" "$0" "$0"
}

print_schema_info() {
    printf "\
Schema Files:
  %s/step-file.schema.json
    - Validates step/activity markdown files
    - Required: title, description, checklist, acceptanceCriteria, status

  %s/readme-file.schema.json
    - Validates README.md files at each level
    - Phase README: phaseHeader, activities/steps, successCriteria
    - Group README: files, prerequisites
" "$SCHEMAS_DIR" "$SCHEMAS_DIR"
}

main() {
    if [ $# -eq 0 ]; then
        print_usage
        exit 0
    fi
    
    case "$1" in
        --help | -h)
            print_usage
            exit 0
            ;;
        --schema)
            print_schema_info
            exit 0
            ;;
    esac
    
    check_jq
    
    root_path="$1"
    printf "Validating structure at: %s\\n" "$root_path"
    
    validate_structure "$root_path"
    print_report "$root_path"
    
    if [ "$error_count" -gt 0 ]; then
        exit 1
    fi
    
    exit 0
}

main "$@"
