#!/usr/bin/env sh
# Update README.md with skill ratings from audit reports
# Usage: ./scripts/update-readme-ratings.sh [--dry-run]

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
README_PATH="$PROJECT_ROOT/README.md"
AUDITS_DIR="$PROJECT_ROOT/.context/audits"
SKILLS_DIR="$PROJECT_ROOT/skills"

DRY_RUN=false
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
    esac
done

get_all_skills() {
    if [ ! -d "$SKILLS_DIR" ]; then
        echo ""
        return
    fi
    
    for dir in "$SKILLS_DIR"/*/; do
        if [ -d "$dir" ] && [ -f "$dir/SKILL.md" ]; then
            basename "$dir" | sed 's/\/$//'
        fi
    done
}

get_skill_description() {
    skill_name="$1"
    skill_file="$SKILLS_DIR/$skill_name/SKILL.md"
    
    if [ ! -f "$skill_file" ]; then
        echo ""
        return
    fi
    
    # Try to extract from frontmatter description field (handle multiline YAML scalar)
    in_frontmatter=false
    in_description=false
    desc=""
    
    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in
            ---)
                if [ "$in_frontmatter" = false ]; then
                    in_frontmatter=true
                else
                    break
                fi
                ;;
            description:*)
                # Single-line description
                desc=$(echo "$line" | sed 's/^description:[[:space:]]*//' | sed 's/^"//' | sed 's/"$//')
                if [ -n "$desc" ]; then
                    break
                fi
                ;;
            "    "*)
                # Indented content - might be continuation of multiline description
                if [ "$in_description" = true ]; then
                    line_desc=$(echo "$line" | sed 's/^[[:space:]]*//' | sed 's/^"//' | sed 's/"$//')
                    if [ -n "$line_desc" ]; then
                        desc="$desc $line_desc"
                    fi
                fi
                ;;
            *)
                # Check if we hit another top-level key
                if [ "$in_frontmatter" = true ] && [ "$in_description" = true ]; then
                    if echo "$line" | grep -qE '^[a-z]'; then
                        break
                    fi
                fi
                ;;
        esac
    done < "$skill_file"
    
    if [ -n "$desc" ]; then
        # Truncate if too long
        if [ ${#desc} -gt 80 ]; then
            printf '%s...' "$(echo "$desc" | cut -c1-77)"
        else
            printf '%s' "$desc"
        fi
        return
    fi
    
    # No description found - return empty (caller will use existing)
    echo ""
}

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
    
    # New format: analysis.md with table
    if echo "$file" | grep -q "analysis.md"; then
        extract_from_analysis_table "$file"
        return $?
    fi
    
    # Legacy format: standalone audit markdown files
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

extract_from_analysis_table() {
    file="$1"
    skill_name="$2"
    
    # Table format: | Skill | Score | Grade | Lines | Refs |
    # Example: | nx-executors | 114/120 | A+ | 135 | 2 |
    line=$(grep "|[[:space:]]*${skill_name}[[:space:]]*|" "$file" | head -1)
    
    if [ -z "$line" ]; then
        return 1
    fi
    
    # Extract score (e.g., "114/120")
    score=$(echo "$line" | sed -n 's/.*|[[:space:]]*\([0-9]*\)\/\([0-9]*\)[[:space:]]*|.*/\1/p')
    max_score=$(echo "$line" | sed -n 's/.*|[[:space:]]*\([0-9]*\)\/\([0-9]*\)[[:space:]]*|.*/\2/p')
    grade=$(echo "$line" | sed -n 's/.*|[[:space:]]*[A-F+]*[[:space:]]*|[[:space:]]*\([A-F+]*\)[[:space:]]*|.*/\1/p')
    
    if [ -z "$score" ] || [ -z "$max_score" ] || [ -z "$grade" ]; then
        return 1
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
    
    # Search in both root audits dir and subdirectories (e.g., .context/audits/skill-audit/2026-02-24/)
    search_dirs="$AUDITS_DIR"
    for subdir in "$AUDITS_DIR"/*/; do
        if [ -d "$subdir" ]; then
            search_dirs="$search_dirs $subdir"
        fi
    done
    
    # New format: .context/audits/<skill>/<date>/audit.json
    skill_audit_dir="$AUDITS_DIR/$skill_name"
    if [ -d "$skill_audit_dir" ]; then
        # Find the latest dated subdirectory
        for date_dir in "$skill_audit_dir"/*/; do
            if [ -d "$date_dir" ]; then
                date_name=$(basename "$date_dir")
                if echo "$date_name" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}$'; then
                    audit_json="${date_dir}audit.json"
                    if [ -f "$audit_json" ]; then
                        # Extract from JSON: totalScore, grade
                        score=$(grep -oP '"totalScore":\s*\K[0-9]+' "$audit_json" | head -1)
                        grade=$(grep -oP '"grade":\s*"\K[A-F+]+' "$audit_json" | head -1)
                        max_score=120

                        if [ -n "$score" ] && [ -n "$grade" ]; then
                            if [ -z "$latest_date" ] || [ "$(printf '%s\n' "$date_name" "$latest_date" | sort -V | head -n1)" != "$latest_date" ]; then
                                latest_date="$date_name"
                                latest_file="$audit_json"
                                latest_rating="$score|$max_score|$grade"
                            fi
                        fi
                    fi
                fi
            fi
        done
    fi
    
    # If we found a rating from analysis.md, return it
    if [ -n "$latest_file" ]; then
        echo "${latest_date}|${latest_file}|${latest_rating}"
        return
    fi
    
    # Fallback: legacy format - individual audit files in root
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
    
    # Get all skills from filesystem
    all_skills=$(get_all_skills)
    
    # Build a map of existing skills in README (skill_name -> description)
    existing_skills=""
    while IFS= read -r line || [ -n "$line" ]; do
        if is_skill_table_row "$line"; then
            skill_name=$(extract_skill_name "$line")
            if [ -n "$skill_name" ]; then
                description=$(printf '%s\n' "$line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/, "", $3); gsub(/\n/, " ", $3); print $3}' | sed 's/  */ /g' | sed 's/ $//')
                existing_skills="${existing_skills}${skill_name}|${description}\n"
            fi
        fi
    done < "$README_PATH"
    
    temp_file=$(mktemp)
    current_line=0
    in_table=false
    skills_processed=""
    
    while IFS= read -r line || [ -n "$line" ]; do
        current_line=$((current_line + 1))
        
        case "$line" in
            *\|\ Skill*\|\ Description*\|*)
                in_table=true
                # Write header
                printf '| Skill | Description | Rating | Audit |\n' >> "$temp_file"
                printf '| --- | --- | --- | --- |\n' >> "$temp_file"
                
                # Process all skills from filesystem
                for skill_name in $all_skills; do
                    # Get description - prefer existing, otherwise extract from SKILL.md
                    existing_desc=$(printf '%s' "$existing_skills" | grep "^${skill_name}|" | head -1 | cut -d'|' -f2)
                    # Force re-extract if existing is empty or malformed
                    if [ -z "$existing_desc" ] || echo "$existing_desc" | grep -qE '^\s*\|' || echo "$existing_desc" | grep -qE '^\s*-$' ; then
                        description=$(get_skill_description "$skill_name")
                    else
                        description="$existing_desc"
                    fi
                    
                    # Get audit info
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
                    
                    printf '| %s | %s | %s | %s |\n' \
                        "[\`$skill_name\`](skills/$skill_name/SKILL.md)" \
                        "$description" \
                        "$badge" \
                        "$audit_link" >> "$temp_file"
                done
                
                # Skip original table rows
                continue
                ;;
        esac
        
        if [ "$in_table" = true ]; then
            # Skip until we exit the table
            case "$line" in
                "|"*)
                    # Skip table rows - we've already written them
                    ;;
                "")
                    # Empty line - might be end of table
                    in_table=false
                    printf '\n' >> "$temp_file"
                    ;;
                *)
                    in_table=false
                    printf '%s\n' "$line" >> "$temp_file"
                    ;;
            esac
        else
            printf '%s\n' "$line" >> "$temp_file"
        fi
    done < "$README_PATH"
    
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
