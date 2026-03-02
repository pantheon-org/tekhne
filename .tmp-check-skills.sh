#!/usr/bin/env bash
# Temporary script to check skill versions against registry

set -euo pipefail

SKILLS_DIR="skills"
WORKSPACE="pantheon-ai"

echo "=== Checking registry status for all skills ==="
echo ""

# Get all published skills from pantheon-ai workspace
echo "Fetching published skills from registry..."
PUBLISHED=$(tessl search pantheon-ai 2>&1 | grep "^pantheon-ai/" || true)

# Track skills needing processing
declare -a NEEDS_PROCESSING=()

# Check each skill directory
for skill_dir in "$SKILLS_DIR"/*/ ; do
  skill_name=$(basename "$skill_dir")
  tile_json="$skill_dir/tile.json"
  
  # Skip if no tile.json exists
  if [[ ! -f "$tile_json" ]]; then
    echo "⚠️  $skill_name: No tile.json found - needs import"
    NEEDS_PROCESSING+=("$skill_name:no-tile")
    continue
  fi
  
  # Extract version from tile.json
  local_version=$(jq -r '.version' "$tile_json" 2>/dev/null || echo "unknown")
  full_name=$(jq -r '.name' "$tile_json" 2>/dev/null || echo "unknown")
  
  # Check if published in registry
  if echo "$PUBLISHED" | grep -q "^$full_name "; then
    # Extract published version
    published_version=$(echo "$PUBLISHED" | grep "^$full_name " | awk '{print $2}')
    
    if [[ "$local_version" == "$published_version" ]]; then
      echo "✅ $skill_name: v$local_version (up to date)"
    else
      echo "🔄 $skill_name: v$local_version (local) vs v$published_version (published) - needs update"
      NEEDS_PROCESSING+=("$skill_name:version-mismatch")
    fi
  else
    echo "📦 $skill_name: v$local_version (not published) - needs publish"
    NEEDS_PROCESSING+=("$skill_name:not-published")
  fi
done

echo ""
echo "=== Summary ==="
echo "Total skills needing processing: ${#NEEDS_PROCESSING[@]}"
echo ""

# Output list for processing
if [[ ${#NEEDS_PROCESSING[@]} -gt 0 ]]; then
  echo "Skills to process:"
  for item in "${NEEDS_PROCESSING[@]}"; do
    echo "  - $item"
  done
fi
