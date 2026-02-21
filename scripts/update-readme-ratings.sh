#!/usr/bin/env sh
# Update README.md with skill ratings from audit reports
# Usage: ./scripts/update-readme-ratings.sh [--dry-run]

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
README_PATH="$PROJECT_ROOT/README.md"
AUDITS_DIR="$PROJECT_ROOT/.context/audits"

DRY_RUN=false
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
    esac
done

get_grade_color() {
    grade="$1"
    case "$grade" in
        A+) echo "brightgreen" ;;
        A)  echo "green" ;;
        B+) echo "yellowgreen" ;;
        B)  echo "yellow" ;;
        C+) echo "orange" ;;
        C)  echo "red" ;;
        D)  echo "purple" ;;
        F)  echo "purple" ;;
        *)  echo "lightgrey" ;;
    esac
}

extract_rating_from_report() {
    file="$1"
    
    if [ ! -f "$file" ]; then
        return 1
    fi
    
    score_line=$(grep '\*\*Total Score\*\*' "$file" | head -1)
    grade_line=$(grep '\*\*Grade\*\*' "$file" | head -1)
    
    if [ -z "$score_line" ]; then
        return 1
    fi
    
    score=$(echo "$score_line" | sed 's/.*\*\*Total Score\*\*[[:space:]]*|[[:space:]]*\([0-9]*\)\/\([0-9]*\).*/\1/')
    max_score=$(echo "$score_line" | sed 's/.*\*\*Total Score\*\*[[:space:]]*|[[:space:]]*\([0-9]*\)\/\([0-9]*\).*/\2/')
    
    if [ -z "$score" ] || [ -z "$max_score" ]; then
        return 1
    fi
    
    grade=$(echo "$grade_line" | sed 's/.*\*\*Grade\*\*[[:space:]]*|[[:space:]]*\([A-F+]*\).*/\1/')
    
    if [ -z "$grade" ]; then
        grade="N/A"
    fi
    
    echo "$score|$max_score|$grade"
    return 0
}

get_latest_audit_info() {
    skill_name="$1"
    
    if [ ! -d "$AUDITS_DIR" ]; then
        echo ""
        return
    fi
    
    latest_file=""
    latest_timestamp=""
    
    for file in "$AUDITS_DIR"/"${skill_name}-"*; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            timestamp=$(echo "$filename" | sed "s/${skill_name}-//" | sed 's/\.md$//')
            
            if [ -z "$latest_timestamp" ] || expr "$timestamp" ">" "$latest_timestamp" >/dev/null; then
                latest_timestamp="$timestamp"
                latest_file="$file"
            fi
        fi
    done
    
    if [ -n "$latest_file" ]; then
        rating=$(extract_rating_from_report "$latest_file")
        echo "${latest_timestamp}|${rating}"
    fi
}

get_skill_badge() {
    rating="$1"
    
    if [ -z "$rating" ]; then
        echo "N/A"
        return
    fi
    
    score=$(echo "$rating" | cut -d'|' -f1)
    max_score=$(echo "$rating" | cut -d'|' -f2)
    grade=$(echo "$rating" | cut -d'|' -f3)
    
    color=$(get_grade_color "$grade")
    
    case "$grade" in
        *+) badge_grade="$grade" ;;
        *)  badge_grade="${grade}%20" ;;
    esac
    
    badge="![${grade}](https://img.shields.io/badge/Rating-${badge_grade}-${color})"
    printf '%s' "$badge"
}

get_audit_link() {
    timestamp="$1"
    skill_name="$2"
    
    if [ -z "$timestamp" ]; then
        echo "N/A"
        return
    fi
    
    audit_file=".context/audits/${skill_name}-${timestamp}.md"
    printf '[%s](%s)' "$timestamp" "$audit_file"
}

is_skill_table_row() {
    line="$1"
    
    # shellcheck disable=SC2016
    printf '%s\n' "$line" | grep -qE '^\| *\[`[^`]*`\]'
}

build_skill_row() {
    skill_name="$1"
    description="$2"
    badge="$3"
    audit_link="$4"
    
    # shellcheck disable=SC2006
    skill_link="[`${skill_name}`](skills/${skill_name}/SKILL.md)"
    printf '| %-80s | %-78s | %-57s | %-72s |' "$skill_link" "$description" "$badge" "$audit_link"
}

extract_skill_name() {
    line="$1"
    # shellcheck disable=SC2016
    printf '%s\n' "$line" | sed -n 's/^| `\([^`]*\)`.*/\1/p'
}

update_readme() {
    if [ ! -f "$README_PATH" ]; then
        echo "Error: README.md not found" >&2
        exit 1
    fi
    
    table_start_line=0
    has_full_format=false
    line_num=0
    
    while IFS= read -r line || [ -n "$line" ]; do
        line_num=$((line_num + 1))
        
        case "$line" in
            *\|\ Skill*\|\ Description*\|*)
                table_start_line=$line_num
                case "$line" in
                    *Audit*) has_full_format=true ;;
                esac
                ;;
        esac
    done < "$README_PATH"
    
    if [ "$table_start_line" -eq 0 ]; then
        echo "Error: Could not find skills table in README.md" >&2
        exit 1
    fi
    
    temp_file=$(mktemp)
    current_line=0
    in_table=false
    
    while IFS= read -r line || [ -n "$line" ]; do
        current_line=$((current_line + 1))
        
        if [ "$current_line" -eq "$table_start_line" ]; then
            in_table=true
            if [ "$has_full_format" = false ]; then
                printf '%s Rating | Audit |\n' "$line"
            else
                printf '%s\n' "$line"
            fi
            continue
        fi
        
        if [ "$in_table" = true ] && [ "$current_line" -eq $((table_start_line + 1)) ]; then
            if [ "$has_full_format" = false ]; then
                printf '%s ------ | ------ |\n' "$line"
            else
                printf '%s\n' "$line"
            fi
            continue
        fi
        
        if [ "$in_table" = true ]; then
            if is_skill_table_row "$line"; then
                skill_name=$(extract_skill_name "$line")
                
                if [ -n "$skill_name" ]; then
                    audit_info=$(get_latest_audit_info "$skill_name")
                    
                    if [ -n "$audit_info" ]; then
                        timestamp=$(echo "$audit_info" | cut -d'|' -f1)
                        rating=$(echo "$audit_info" | cut -d'|' -f2-)
                        badge=$(get_skill_badge "$rating")
                        audit_link=$(get_audit_link "$timestamp" "$skill_name")
                    else
                        badge="N/A"
                        audit_link="N/A"
                    fi
                    
                    description=$(echo "$line" | sed 's/^[^|]*|[^|]*| *\([^|]*\) *|.*/\1/')
                    
                    if [ "$has_full_format" = false ]; then
                        # shellcheck disable=SC2006
                        skill_link="[`${skill_name}`](skills/${skill_name}/SKILL.md)"
                        printf '| %-80s | %-78s | %-57s | %-72s |\n' "$skill_link" "$description" "$badge" "$audit_link"
                    else
                        new_line=$(echo "$line" | sed 's|!\[.*\](https://img.shields.io/badge/Rating-[^)]*)|'"$badge"'|')
                        new_line=$(echo "$new_line" | sed 's|\[20[0-9][0-9-[0-9][0-9]-[0-9][0-9]\](\.context/audits/[^)|]*)|'"$audit_link"'|')
                        printf '%s\n' "$new_line"
                    fi
                    continue
                fi
            fi
            
            case "$line" in
                "|"*)
                    ;;
                "")
                    ;;
                *)
                    in_table=false
                    ;;
            esac
        fi
        
        printf '%s\n' "$line"
    done < "$README_PATH" > "$temp_file"
    
    if [ "$DRY_RUN" = true ]; then
        echo "=== DRY RUN - Changes that would be made ==="
        echo ""
        
        diff -u "$README_PATH" "$temp_file" 2>/dev/null || true
        
        echo ""
        echo "To apply changes, run without --dry-run"
        rm -f "$temp_file"
    else
        mv "$temp_file" "$README_PATH"
        echo "README.md updated with skill ratings and audit links"
    fi
}

update_readme
