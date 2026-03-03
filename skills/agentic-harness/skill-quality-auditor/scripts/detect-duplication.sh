#!/usr/bin/env sh
# shellcheck disable=SC2129
# Detect duplication across skills using simple text similarity

set -eu

SKILLS_DIR="${1:-skills}"
OUTPUT_FILE=".context/analysis/duplication-report-$(date +%Y-%m-%d).md"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM
SKILLS_FILE="$TMP_DIR/skills.txt"

echo "Detecting duplication in $SKILLS_DIR..."
echo "# Skill Duplication Report - $(date +%Y-%m-%d)" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Find all SKILL.md files
find "$SKILLS_DIR" -name "SKILL.md" -not -path "*/.deprecated/*" | sort > "$SKILLS_FILE"
SKILL_COUNT=$(grep -c . "$SKILLS_FILE" || true)

echo "Found ${SKILL_COUNT} skills to analyze"
echo "## Summary" >> "$OUTPUT_FILE"
echo "- Skills analyzed: ${SKILL_COUNT}" >> "$OUTPUT_FILE"
echo "- Threshold: >20% similarity" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "## High Duplication Pairs" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Simple duplication detection (comparing line overlap)
i=1
while [ "$i" -le "$SKILL_COUNT" ]; do
  skill1=$(sed -n "${i}p" "$SKILLS_FILE")
  j=$((i + 1))
  while [ "$j" -le "$SKILL_COUNT" ]; do
    skill2=$(sed -n "${j}p" "$SKILLS_FILE")

    # Get skill names
    name1=$(basename "$(dirname "$skill1")")
    name2=$(basename "$(dirname "$skill2")")

    # Count common lines (simple heuristic)
    sort "$skill1" > "$TMP_DIR/s1.sorted"
    sort "$skill2" > "$TMP_DIR/s2.sorted"
    common=$(comm -12 "$TMP_DIR/s1.sorted" "$TMP_DIR/s2.sorted" | wc -l | tr -d ' ')
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
    j=$((j + 1))
  done
  i=$((i + 1))
done

echo "## Next Steps" >> "$OUTPUT_FILE"
echo "1. Review high-similarity pairs (>35% critical)" >> "$OUTPUT_FILE"
echo "2. Read aggregation-pattern.md for consolidation guide" >> "$OUTPUT_FILE"
echo "3. Calculate ROI for aggregation candidates" >> "$OUTPUT_FILE"

echo "Report generated: $OUTPUT_FILE"
cat "$OUTPUT_FILE"
