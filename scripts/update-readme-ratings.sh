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

get_latest_rating() {
    skill_name="$1"
    
    if [ ! -d "$AUDITS_DIR" ]; then
        echo ""
        return
    fi
    
    latest_file=""
    latest_timestamp=""
    
    for file in "$AUDITS_DIR"/"${skill_name}-skill-quality-audit-"*; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            timestamp=$(echo "$filename" | sed "s/${skill_name}-skill-quality-audit-//" | sed 's/\.md$//')
            
            if [ -z "$latest_timestamp" ] || expr "$timestamp" ">" "$latest_timestamp" >/dev/null; then
                latest_timestamp="$timestamp"
                latest_file="$file"
            fi
        fi
    done
    
    if [ -n "$latest_file" ]; then
        extract_rating_from_report "$latest_file"
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
    printf '%-65s' "$badge"
}

is_skill_table_row() {
    line="$1"
    
    # shellcheck disable=SC2016
    printf '%s\n' "$line" | grep -q '^|  *`[^`]*` *|'
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
    has_rating_column=false
    line_num=0
    
    while IFS= read -r line || [ -n "$line" ]; do
        line_num=$((line_num + 1))
        
        case "$line" in
            *\|\ Skill*\|\ Description*\|*)
                table_start_line=$line_num
                case "$line" in
                    *Rating*) has_rating_column=true ;;
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
            if [ "$has_rating_column" = false ]; then
                printf '%s Rating |\n' "$line"
            else
                printf '%s\n' "$line"
            fi
            continue
        fi
        
        if [ "$in_table" = true ] && [ "$current_line" -eq $((table_start_line + 1)) ]; then
            if [ "$has_rating_column" = false ]; then
                printf '%s ------ |\n' "$line"
            else
                printf '%s\n' "$line"
            fi
            continue
        fi
        
        if [ "$in_table" = true ]; then
            if is_skill_table_row "$line"; then
                skill_name=$(extract_skill_name "$line")
                
                if [ -n "$skill_name" ]; then
                    rating=$(get_latest_rating "$skill_name")
                    badge=$(get_skill_badge "$rating")
                    
                    if [ "$has_rating_column" = false ]; then
                        printf '%s %s |\n' "$line" "$badge"
                    else
                        printf '%s\n' "$line" | sed 's|!\[.*\](https://img.shields.io/badge/Rating-[^)]*)[[:space:]]*|'"$badge"'|'
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
        echo "README.md updated with skill ratings"
    fi
}

update_readme
