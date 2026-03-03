#!/usr/bin/env sh
# Update README.md with skill ratings organized by domain
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

# Define domain order and descriptions
get_domain_info() {
    domain="$1"
    case "$domain" in
        ci-cd)
            echo "CI/CD|CI/CD pipelines & deployment automation"
            ;;
        infrastructure)
            echo "Infrastructure|Infrastructure as Code"
            ;;
        repository-mgmt)
            echo "Repository Management|Repository & workspace management"
            ;;
        development)
            echo "Development|Development tooling"
            ;;
        agentic-harness)
            echo "Agentic Harness|Agent framework configurations"
            ;;
        testing)
            echo "Testing|Testing methodologies & quality"
            ;;
        software-engineering)
            echo "Software Engineering|Software engineering principles"
            ;;
        observability)
            echo "Observability|Monitoring, logging & debugging"
            ;;
        documentation)
            echo "Documentation|Writing & communication"
            ;;
        package-mgmt)
            echo "Package Management|Package & version management"
            ;;
        project-mgmt)
            echo "Project Management|Planning & organization"
            ;;
        specialized)
            echo "Specialized|Domain-specific tools"
            ;;
        *)
            echo "Unknown|Unknown domain"
            ;;
    esac
}

# Get all skills organized by domain
get_skills_by_domain() {
    if [ ! -d "$SKILLS_DIR" ]; then
        echo ""
        return
    fi
    
    # Find all SKILL.md files and extract domain/skill-path
    find "$SKILLS_DIR" -name "SKILL.md" -type f | while read -r skill_file; do
        skill_dir="$(dirname "$skill_file")"
        skill_rel_path="${skill_dir#"$SKILLS_DIR"/}"
        
        # Extract domain (first directory component)
        domain=$(echo "$skill_rel_path" | cut -d'/' -f1)
        
        echo "$domain|$skill_rel_path"
    done | sort
}

get_skill_description() {
    skill_rel_path="$1"
    skill_file="$SKILLS_DIR/$skill_rel_path/SKILL.md"
    
    if [ ! -f "$skill_file" ]; then
        echo ""
        return
    fi
    
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
                first_part=$(echo "$line" | sed 's/^description:[[:space:]]*//')
                if echo "$first_part" | grep -qE '^[|>]-?[[:space:]]*$'; then
                    in_description=true
                elif [ -n "$first_part" ]; then
                    desc=$(echo "$first_part" | sed 's/^"//' | sed 's/"$//')
                    if [ -n "$desc" ]; then
                        break
                    else
                        in_description=true
                    fi
                else
                    in_description=true
                fi
                ;;
            "    "*|"  "*)
                if [ "$in_description" = true ]; then
                    line_desc=$(echo "$line" | sed 's/^[[:space:]]*//' | sed 's/^"//' | sed 's/"$//' | sed 's/  */ /g')
                    if [ -n "$line_desc" ]; then
                        if [ -n "$desc" ]; then
                            desc="$desc $line_desc"
                        else
                            desc="$line_desc"
                        fi
                    fi
                fi
                ;;
            *)
                if [ "$in_frontmatter" = true ] && [ "$in_description" = true ]; then
                    if echo "$line" | grep -qE '^[a-z]'; then
                        break
                    fi
                fi
                ;;
        esac
    done < "$skill_file"
    
    if [ -n "$desc" ]; then
        # Escape pipe characters to prevent markdown table issues
        desc=$(echo "$desc" | sed 's/|/\\|/g')
        if [ ${#desc} -gt 80 ]; then
            printf '%s...' "$(echo "$desc" | cut -c1-77)"
        else
            printf '%s' "$desc"
        fi
        return
    fi
    
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

get_latest_audit_info() {
    skill_rel_path="$1"
    
    if [ ! -d "$AUDITS_DIR" ]; then
        echo ""
        return
    fi
    
    latest_file=""
    latest_date=""
    latest_rating=""
    
    # New format: .context/audits/<domain>/<skill-path>/<date>/audit.json
    skill_audit_dir="$AUDITS_DIR/$skill_rel_path"
    if [ -d "$skill_audit_dir" ]; then
        for date_dir in "$skill_audit_dir"/*/; do
            if [ -d "$date_dir" ]; then
                date_name=$(basename "$date_dir")
                if echo "$date_name" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}$'; then
                    audit_json="${date_dir}audit.json"
                    if [ -f "$audit_json" ]; then
                        if command -v jq >/dev/null 2>&1; then
                            score=$(jq -r '.total // empty' "$audit_json" 2>/dev/null)
                            grade=$(jq -r '.grade // empty' "$audit_json" 2>/dev/null)
                        else
                            score=$(grep -o '"total"[[:space:]]*:[[:space:]]*[0-9]*' "$audit_json" | sed 's/.*:[[:space:]]*//' | head -1)
                            grade=$(grep -o '"grade"[[:space:]]*:[[:space:]]*"[A-F+]*"' "$audit_json" | sed 's/.*"grade"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/' | head -1)
                        fi
                        max_score=120

                        if [ -n "$score" ] && [ -n "$grade" ]; then
                            if [ -z "$latest_date" ] || [ "$(printf '%s\n%s\n' "$date_name" "$latest_date" | sort -r | head -1)" = "$date_name" ]; then
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
    
    if [ -n "$latest_file" ]; then
        echo "${latest_date}|${latest_file}|${latest_rating}"
    fi
}

get_skill_badge() {
    rating="$1"
    
    if [ -z "$rating" ]; then
        echo "![?](https://img.shields.io/badge/Rating-?-lightgrey)"
        return
    fi
    
    score=$(echo "$rating" | cut -d'|' -f1)
    max_score=$(echo "$rating" | cut -d'|' -f2)
    grade=$(echo "$rating" | cut -d'|' -f3)
    
    color=$(get_grade_color "$grade")
    
    badge="![${grade}](https://img.shields.io/badge/Rating-${grade}-${color})"
    printf '%s' "$badge"
}

get_tessl_status() {
    skill_rel_path="$1"
    tile_path="$SKILLS_DIR/$skill_rel_path/tile.json"
    
    if [ ! -f "$tile_path" ]; then
        echo "No"
        return
    fi
    
    # Check if private field exists and is false (published)
    private=$(grep -o '"private"[[:space:]]*:[[:space:]]*[^,}]*' "$tile_path" | sed 's/.*:[[:space:]]*//' | tr -d ' ')
    
    if [ "$private" = "false" ]; then
        tessl_name=$(grep -o '"name"[[:space:]]*:[[:space:]]*"[^"]*"' "$tile_path" | sed 's/.*"name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/')
        if [ -n "$tessl_name" ]; then
            echo "[Public](https://tessl.io/registry/skills/$tessl_name)"
        else
            echo "Public"
        fi
    elif [ "$private" = "true" ]; then
        echo "Private"
    else
        echo "Configured"
    fi
}

get_audit_link() {
    date_part="$1"
    audit_path="$2"

    if [ -z "$date_part" ] || [ -z "$audit_path" ]; then
        echo "-"
        return
    fi

    rel_path=$(echo "$audit_path" | sed "s|^$PROJECT_ROOT/||")
    printf '[%s](%s)' "$date_part" "$rel_path"
}

generate_domain_tables() {
    temp_file="$1"
    
    # Get all skills grouped by domain (redirect to avoid output in README)
    all_skills=$(get_skills_by_domain 2>/dev/null)
    
    # Define domain order
    domains="ci-cd infrastructure repository-mgmt development agentic-harness testing software-engineering observability documentation package-mgmt project-mgmt specialized"
    
    for domain in $domains; do
        # Get domain info
        domain_info=$(get_domain_info "$domain")
        domain_title=$(echo "$domain_info" | cut -d'|' -f1)
        domain_desc=$(echo "$domain_info" | cut -d'|' -f2)
        
        # Filter skills for this domain
        domain_skills=$(echo "$all_skills" | grep "^${domain}|" || true)
        
        if [ -z "$domain_skills" ]; then
            continue
        fi
        
        # Count skills in domain
        skill_count=$(echo "$domain_skills" | wc -l | tr -d ' ')
        
        # Write domain header
        {
            printf '\n## %s (%s skills)\n\n' "$domain_title" "$skill_count"
            printf '%s\n\n' "$domain_desc"
            printf '| Skill | Description | Rating | Audit | Tessl |\n'
            printf '| --- | --- | --- | --- | --- |\n'
        } >> "$temp_file"
        
        # Process each skill in domain
        echo "$domain_skills" | while IFS='|' read -r _ skill_rel_path; do
            # Get skill display name (include subdomain but not top domain)
            # Example: repository-mgmt/nx/executors → nx-executors
            # Example: ci-cd/github-actions/generator → github-actions-generator
            # Example: documentation/markdown-authoring → markdown-authoring
            skill_path_without_domain=$(echo "$skill_rel_path" | cut -d'/' -f2-)
            if echo "$skill_path_without_domain" | grep -q '/'; then
                # Has subdomain - flatten with hyphens
                skill_display=$(echo "$skill_path_without_domain" | tr '/' '-')
            else
                # No subdomain - use as is
                skill_display="$skill_path_without_domain"
            fi
            
            # Get description
            description=$(get_skill_description "$skill_rel_path")
            if [ -z "$description" ]; then
                description="-"
            fi
            
            # Get audit info
            audit_info=$(get_latest_audit_info "$skill_rel_path")
            
            if [ -n "$audit_info" ]; then
                date_part=$(echo "$audit_info" | cut -d'|' -f1)
                audit_path=$(echo "$audit_info" | cut -d'|' -f2)
                rating=$(echo "$audit_info" | cut -d'|' -f3-)
                badge=$(get_skill_badge "$rating")
                audit_link=$(get_audit_link "$date_part" "$audit_path")
            else
                badge="![?](https://img.shields.io/badge/Rating-?-lightgrey)"
                audit_link="-"
            fi
            
            # Get Tessl status
            tessl_status=$(get_tessl_status "$skill_rel_path")
            
            # Write row
            printf '| [%s](skills/%s/SKILL.md) | %s | %s | %s | %s |\n' \
                "$skill_display" \
                "$skill_rel_path" \
                "$description" \
                "$badge" \
                "$audit_link" \
                "$tessl_status" >> "$temp_file"
        done
    done
}

update_readme() {
    if [ ! -f "$README_PATH" ]; then
        echo "Error: README.md not found" >&2
        exit 1
    fi
    
    temp_file=$(mktemp)
    in_skills_section=false
    
    # Copy README up to skills section
    while IFS= read -r line || [ -n "$line" ]; do
        # Detect start of skills section (first domain header or old table)
        if echo "$line" | grep -qE '^## (CI/CD|Infrastructure|Repository Management|Development|Agentic Harness|Testing|Software Engineering|Observability|Documentation|Package Management|Project Management|Specialized)'; then
            in_skills_section=true
            break
        elif echo "$line" | grep -qE '^\| Skill \| Description'; then
            in_skills_section=true
            break
        fi
        
        printf '%s\n' "$line" >> "$temp_file"
    done < "$README_PATH"
    
    # Generate new domain tables
    generate_domain_tables "$temp_file"
    
    # Skip old skills section if we found it
    if [ "$in_skills_section" = true ]; then
        # Read until we hit a top-level section that's not a domain
        while IFS= read -r line || [ -n "$line" ]; do
            # Stop when we hit a non-domain top-level section
            if echo "$line" | grep -qE '^## ' && ! echo "$line" | grep -qE '^## (CI/CD|Infrastructure|Repository Management|Development|Agentic Harness|Testing|Software Engineering|Observability|Documentation|Package Management|Project Management|Specialized)'; then
                printf '\n%s\n' "$line" >> "$temp_file"
                break
            fi
        done < "$README_PATH"
        
        # Copy rest of file
        while IFS= read -r line || [ -n "$line" ]; do
            printf '%s\n' "$line" >> "$temp_file"
        done
    fi
    
    if [ "$DRY_RUN" = true ]; then
        echo "=== DRY RUN - Changes that would be made ==="
        echo ""
        
        diff -u "$README_PATH" "$temp_file" 2>/dev/null || true
        
        echo ""
        echo "To apply changes, run without --dry-run"
        rm -f "$temp_file"
    else
        mv "$temp_file" "$README_PATH"
        echo "README.md updated with 12 domain-organized skill tables"
    fi
}

update_readme
