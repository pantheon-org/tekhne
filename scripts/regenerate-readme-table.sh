#!/usr/bin/env sh
# Generate README skills table from new domain structure

set -e

echo "Generating skills table..."
echo ""
echo "| Skill | Description | Rating | Audit | Tessl |"
echo "| --- | --- | --- | --- | --- |"

find skills -name "SKILL.md" -type f | sort | while read -r skill_file; do
  skill_path="$(dirname "$skill_file")"
  skill_rel_path="${skill_path#skills/}"
  
  # Extract description from SKILL.md frontmatter (first 80 chars)
  description=$(sed -n '/^description:/p' "$skill_file" | sed 's/description: //' | head -1 | cut -c1-80)
  if [ ${#description} -eq 80 ]; then
    description="${description}..."
  fi
  
  # Determine audit path (find most recent audit)
  audit_base=".context/audits/${skill_rel_path}"
  if [ -d "$audit_base" ]; then
    latest_audit=$(find "$audit_base" -name "audit.json" -type f | sort -r | head -1)
    if [ -n "$latest_audit" ]; then
      audit_date=$(basename "$(dirname "$latest_audit")")
      audit_path="$audit_base/$audit_date/audit.json"
      
      # Extract rating from audit
      if [ -f "$latest_audit" ]; then
        grade=$(jq -r '.grade' "$latest_audit" 2>/dev/null || echo "?")
        
        # Determine badge color
        case "$grade" in
          A) badge_color="green" ;;
          B+) badge_color="yellowgreen" ;;
          B) badge_color="yellow" ;;
          C+) badge_color="orange" ;;
          C) badge_color="red" ;;
          *) badge_color="lightgrey" ;;
        esac
        
        rating_badge="![${grade}](https://img.shields.io/badge/Rating-${grade}-${badge_color})"
      else
        rating_badge="![?](https://img.shields.io/badge/Rating-?-lightgrey)"
      fi
    else
      audit_path=""
      rating_badge="![?](https://img.shields.io/badge/Rating-?-lightgrey)"
    fi
  else
    audit_path=""
    rating_badge="![?](https://img.shields.io/badge/Rating-?-lightgrey)"
  fi
  
  # Extract registry name from tile.json if exists
  tile_file="$skill_path/tile.json"
  if [ -f "$tile_file" ]; then
    registry_name=$(jq -r '.name' "$tile_file" | sed 's|pantheon-ai/||')
    tessl_link="[Published](https://tessl.io/registry/skills/pantheon-ai/$registry_name)"
  else
    # Generate from path (flatten with hyphens)
    registry_name=$(echo "$skill_rel_path" | tr '/' '-')
    tessl_link="[Published](https://tessl.io/registry/skills/pantheon-ai/$registry_name)"
  fi
  
  # Generate display name (flatten path for readability)
  display_name=$(echo "$skill_rel_path" | tr '/' '-')
  
  # Generate row
  if [ -n "$audit_path" ]; then
    echo "| [\`$display_name\`](skills/$skill_rel_path/SKILL.md) | $description | $rating_badge | [$audit_date]($audit_path) | $tessl_link |"
  else
    echo "| [\`$display_name\`](skills/$skill_rel_path/SKILL.md) | $description | $rating_badge | - | $tessl_link |"
  fi
done

echo ""
echo "✅ Table generated successfully"
