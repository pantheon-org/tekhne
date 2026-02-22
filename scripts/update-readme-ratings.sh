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
    latest_date=""
    
    for file in "$AUDITS_DIR"/"${skill_name}-"*.md; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")

            # Support both naming conventions:
            #   <skill>-YYYY-MM-DD.md
            #   <skill>-audit-YYYY-MM-DD.md
            date_part=$(echo "$filename" | sed -n "s/^${skill_name}-audit-\([0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]\)\.md$/\1/p")
            if [ -z "$date_part" ]; then
                date_part=$(echo "$filename" | sed -n "s/^${skill_name}-\([0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]\)\.md$/\1/p")
            fi

            [ -n "$date_part" ] || continue

            if [ -z "$latest_date" ]; then
                latest_date="$date_part"
                latest_file="$file"
            else
                max_date=$(printf '%s\n%s\n' "$date_part" "$latest_date" | sort | tail -n 1)
                if [ "$max_date" = "$date_part" ] && [ "$date_part" != "$latest_date" ]; then
                    latest_date="$date_part"
                    latest_file="$file"
                fi
            fi
        fi
    done
    
    if [ -n "$latest_file" ]; then
        rating=$(extract_rating_from_report "$latest_file")
        echo "${latest_date}|${latest_file}|${rating}"
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
    date_part="$1"
    audit_path="$2"

    if [ -z "$date_part" ] || [ -z "$audit_path" ]; then
        echo "N/A"
        return
    fi

    rel_path=$(echo "$audit_path" | sed "s|^$PROJECT_ROOT/||")
    printf '[%s](%s)' "$date_part" "$rel_path"
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
    
    skill_link="[\`$skill_name\`](skills/$skill_name/SKILL.md)"
    printf '| %s | %s | %s | %s |' "$skill_link" "$description" "$badge" "$audit_link"
}

extract_skill_name() {
    line="$1"
    # Works for both:
    #   | `skill` | ...
    #   | [`skill`](skills/skill/SKILL.md) | ...
    # shellcheck disable=SC2016
    printf '%s\n' "$line" | awk -F'|' '{print $2}' | sed -n 's/.*`\([^`]*\)`.*/\1/p'
}

update_readme() {
    if [ ! -f "$README_PATH" ]; then
        echo "Error: README.md not found" >&2
        exit 1
    fi
    
    table_start_line=0
    line_num=0
    
    while IFS= read -r line || [ -n "$line" ]; do
        line_num=$((line_num + 1))
        
        case "$line" in
            *\|\ Skill*\|\ Description*\|*)
                table_start_line=$line_num
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
            printf '| Skill | Description | Rating | Audit |\n'
            continue
        fi
        
        if [ "$in_table" = true ] && [ "$current_line" -eq $((table_start_line + 1)) ]; then
            printf '| --- | --- | --- | --- |\n'
            continue
        fi
        
        if [ "$in_table" = true ]; then
            if is_skill_table_row "$line"; then
                skill_name=$(extract_skill_name "$line")
                
                if [ -n "$skill_name" ]; then
                    audit_info=$(get_latest_audit_info "$skill_name")
                    
                    if [ -n "$audit_info" ]; then
                        date_part=$(echo "$audit_info" | cut -d'|' -f1)
                        audit_path=$(echo "$audit_info" | cut -d'|' -f2)
                        rating=$(echo "$audit_info" | cut -d'|' -f3-)
                        badge=$(get_skill_badge "$rating")
                        audit_link=$(get_audit_link "$date_part" "$audit_path")
                    else
                        badge="N/A"
                        audit_link="N/A"
                    fi
                    
                    description=$(printf '%s\n' "$line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/, "", $3); print $3}')

                    printf '%s\n' "$(build_skill_row "$skill_name" "$description" "$badge" "$audit_link")"
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
