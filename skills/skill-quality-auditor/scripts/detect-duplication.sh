#!/usr/bin/env bash
# Detect duplication across skills using simple text similarity

set -euo pipefail

SKILLS_DIR="${1:-.agents/skills}"
OUTPUT_FILE=".context/analysis/duplication-report-$(date +%Y-%m-%d).md"

echo "Detecting duplication in $SKILLS_DIR..."
echo "# Skill Duplication Report - $(date +%Y-%m-%d)" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Find all SKILL.md files
mapfile -t SKILLS < <(find "$SKILLS_DIR" -name "SKILL.md" -not -path "*/.deprecated/*" | sort)

echo "Found ${#SKILLS[@]} skills to analyze"
echo "## Summary" >> "$OUTPUT_FILE"
echo "- Skills analyzed: ${#SKILLS[@]}" >> "$OUTPUT_FILE"
echo "- Threshold: >20% similarity" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## High Duplication Pairs" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Simple duplication detection (comparing line overlap)
for ((i=0; i<${#SKILLS[@]}; i++)); do
  for ((j=i+1; j<${#SKILLS[@]}; j++)); do
    skill1="${SKILLS[$i]}"
    skill2="${SKILLS[$j]}"
    
    # Get skill names
    name1=$(basename "$(dirname "$skill1")")
    name2=$(basename "$(dirname "$skill2")")
    
    # Count common lines (simple heuristic)
    common=$(comm -12 <(sort "$skill1") <(sort "$skill2") | wc -l)
    total1=$(wc -l < "$skill1")
    total2=$(wc -l < "$skill2")
    
    # Calculate similarity
    avg=$(( (total1 + total2) / 2 ))
    if [ "$avg" -gt 0 ]; then
      similarity=$(( common * 100 / avg ))
      
      if [ "$similarity" -gt 20 ]; then
        echo "### $name1 ↔ $name2" >> "$OUTPUT_FILE"
        echo "- Similarity: ${similarity}%" >> "$OUTPUT_FILE"
        echo "- Common lines: $common" >> "$OUTPUT_FILE"
        echo "- Recommendation: Consider aggregation" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
      fi
    fi
  done
done

echo "## Next Steps" >> "$OUTPUT_FILE"
echo "1. Review high-similarity pairs (>35% critical)" >> "$OUTPUT_FILE"
echo "2. Read aggregation-pattern.md for consolidation guide" >> "$OUTPUT_FILE"
echo "3. Calculate ROI for aggregation candidates" >> "$OUTPUT_FILE"

echo "Report generated: $OUTPUT_FILE"
cat "$OUTPUT_FILE"
