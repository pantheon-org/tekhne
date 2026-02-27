#!/usr/bin/env sh

# tessl-publish-check.sh - Validate skills can be published to tessl without actually publishing
# Usage: ./scripts/tessl-publish-check.sh [file1] [file2] ...
# Triggers on tile.json changes to validate the entire skill directory

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <tile.json files...>"
    exit 1
fi

for tile_file in "$@"; do
    if [ ! -f "$tile_file" ]; then
        echo "Warning: $tile_file does not exist, skipping..."
        continue
    fi
    
    # Extract skill directory from tile.json path
    skill_dir=$(dirname "$tile_file")
    skill_name=$(basename "$skill_dir")
    
    echo "🔍 Validating skill: $skill_name"
    
    # Run tessl lint but only fail on actual errors, not orphaned file warnings
    echo "  → Running tessl lint..."
    lint_output=$(tessl skill lint "$skill_dir" 2>&1)
    if echo "$lint_output" | grep -q "✔ Tile.*is valid"; then
        echo "  ✓ Tile validation passed"
    else
        echo "❌ Skill $skill_name failed lint validation"
        echo "$lint_output"
        exit 1
    fi
    
    # Show orphaned file warnings but don't fail on them
    if echo "$lint_output" | grep -q "orphaned file"; then
        echo "  ⚠️  Orphaned files detected (non-blocking)"
    fi
    
    # Check required files exist
    echo "  → Checking skill structure..."
    if [ ! -f "$skill_dir/SKILL.md" ]; then
        echo "❌ Missing SKILL.md in $skill_dir"
        exit 1
    fi
    
    if [ ! -f "$skill_dir/tile.json" ]; then
        echo "❌ Missing tile.json in $skill_dir"
        exit 1
    fi
    
    # Validate eval scenarios if they exist
    if [ -d "$skill_dir/evals" ]; then
        echo "  → Validating eval scenarios..."
        for scenario_dir in "$skill_dir/evals"/*; do
            if [ -d "$scenario_dir" ]; then
                scenario_name=$(basename "$scenario_dir")
                
                # Check for required eval files
                if [ ! -f "$scenario_dir/criteria.json" ]; then
                    echo "❌ Missing criteria.json in eval $scenario_name"
                    exit 1
                fi
                
                # Validate criteria.json has required tessl format
                if ! grep -q '"type".*"weighted_checklist"' "$scenario_dir/criteria.json"; then
                    echo "❌ Eval $scenario_name criteria.json missing 'type: weighted_checklist'"
                    exit 1
                fi
                
                if ! grep -q '"context"' "$scenario_dir/criteria.json"; then
                    echo "❌ Eval $scenario_name criteria.json missing 'context' field"
                    exit 1
                fi
                
                if ! grep -q '"checklist"' "$scenario_dir/criteria.json"; then
                    echo "❌ Eval $scenario_name criteria.json missing 'checklist' field"
                    exit 1
                fi
                
                echo "    ✓ Eval scenario $scenario_name validated"
            fi
        done
    fi
    
    echo "✅ Skill $skill_name passed validation"
done

echo "🎯 All skills passed tessl publish validation!"