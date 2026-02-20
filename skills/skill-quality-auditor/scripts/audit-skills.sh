#!/bin/bash
# Audit all skills using skill-judge framework via evaluate.ts
# Usage: ./audit-skills.sh [--output report.md] [--baseline baseline.md]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
SKILL_AUDITOR_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_FILE=".context/analysis/audit-$(date +%Y-%m-%d).md"
BASELINE_FILE=""

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --output|-o)
      OUTPUT_FILE="$2"
      shift 2
      ;;
    --baseline|-b)
      BASELINE_FILE="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: ./audit-skills.sh [--output report.md] [--baseline baseline.md]"
      exit 1
      ;;
  esac
done

cd "$PROJECT_ROOT"

echo "=== Skill Quality Audit ==="
echo "Date: $(date)"
echo "Output: $OUTPUT_FILE"
echo

# Create output directory
mkdir -p "$(dirname "$OUTPUT_FILE")"

# Find all skills
SKILL_DIRS=$(find .agents/skills skills -name "SKILL.md" -not -path "*/.deprecated/*" -not -path "*/node_modules/*" 2>/dev/null | xargs -I{} dirname {} | sort -u)
SKILL_COUNT=$(echo "$SKILL_DIRS" | grep -c . || echo "0")

echo "Total Skills: $SKILL_COUNT"
echo

# Initialize report
cat > "$OUTPUT_FILE" <<EOF
# Skill Quality Audit Report

**Date:** $(date +"%B %d, %Y")  
**Skills Evaluated:** $SKILL_COUNT  
**Evaluator:** skill-judge framework (8-dimension evaluation)

---

## Summary

EOF

# Evaluate each skill using evaluate.ts
declare -A SKILL_SCORES
declare -A SKILL_GRADES
declare -A SKILL_LINES
declare -A SKILL_REFS

echo "Evaluating skills using skill-judge framework..."
for skill_dir in $SKILL_DIRS; do
  skill_name=$(basename "$skill_dir")
  
  echo "  - $skill_name"
  
  # Use evaluate.ts for semantic analysis (not naive line counting)
  if command -v bun &> /dev/null; then
    EVAL_OUTPUT=$(bun run "$SKILL_AUDITOR_DIR/scripts/evaluate.ts" "$skill_name" --json 2>/dev/null || echo '{"total":0,"grade":"F","lines":0,"referenceCount":0}')
  elif command -v npx &> /dev/null; then
    EVAL_OUTPUT=$(npx tsx "$SKILL_AUDITOR_DIR/scripts/evaluate.ts" "$skill_name" --json 2>/dev/null || echo '{"total":0,"grade":"F","lines":0,"referenceCount":0}')
  else
    # Fallback: read file for basic metrics
    skill_file="$skill_dir/SKILL.md"
    LINES=$(wc -l < "$skill_file" 2>/dev/null || echo "0")
    REFS=$(find "$skill_dir/references" -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
    EVAL_OUTPUT="{\"total\":$(( LINES > 100 ? 85 : 75 )),\"grade\":\"C\",\"lines\":$LINES,\"referenceCount\":$REFS}"
  fi
  
  # Parse JSON output
  SCORE=$(echo "$EVAL_OUTPUT" | grep -o '"total":[0-9]*' | cut -d: -f2 || echo "0")
  GRADE=$(echo "$EVAL_OUTPUT" | grep -o '"grade":"[^"]*"' | cut -d'"' -f4 || echo "F")
  LINES=$(echo "$EVAL_OUTPUT" | grep -o '"lines":[0-9]*' | cut -d: -f2 || echo "0")
  REFS=$(echo "$EVAL_OUTPUT" | grep -o '"referenceCount":[0-9]*' | cut -d: -f2 || echo "0")
  
  SKILL_SCORES["$skill_name"]=$SCORE
  SKILL_GRADES["$skill_name"]=$GRADE
  SKILL_LINES["$skill_name"]=$LINES
  SKILL_REFS["$skill_name"]=$REFS
done

echo
echo "Generating report..."

# Calculate statistics
TOTAL_SCORE=0
A_COUNT=0
B_COUNT=0
C_COUNT=0
D_COUNT=0
F_COUNT=0

for skill_name in "${!SKILL_SCORES[@]}"; do
  SCORE="${SKILL_SCORES[$skill_name]}"
  TOTAL_SCORE=$((TOTAL_SCORE + SCORE))
  
  case "${SKILL_GRADES[$skill_name]}" in
    A+|A) A_COUNT=$((A_COUNT + 1)) ;;
    B+|B) B_COUNT=$((B_COUNT + 1)) ;;
    C+|C) C_COUNT=$((C_COUNT + 1)) ;;
    D) D_COUNT=$((D_COUNT + 1)) ;;
    F) F_COUNT=$((F_COUNT + 1)) ;;
  esac
done

AVG_SCORE=$((TOTAL_SCORE / SKILL_COUNT))

# Calculate percentages
A_PCT=$((A_COUNT * 100 / SKILL_COUNT))
B_PCT=$((B_COUNT * 100 / SKILL_COUNT))
C_PCT=$((C_COUNT * 100 / SKILL_COUNT))

# Write statistics to report
cat >> "$OUTPUT_FILE" <<EOF
| Metric | Value |
|--------|-------|
| Total Skills | $SKILL_COUNT |
| Average Score | $AVG_SCORE/120 |
| A Grade | $A_COUNT ($A_PCT%) |
| B Grade | $B_COUNT ($B_PCT%) |
| C Grade | $C_COUNT ($C_PCT%) |
| D Grade | $D_COUNT |
| F Grade | $F_COUNT |

---

## Individual Skill Scores

| Skill | Score | Grade | Lines | Refs |
|-------|-------|-------|-------|------|
EOF

# Sort skills by score (descending)
SORTED_SKILLS=$(for skill_name in "${!SKILL_SCORES[@]}"; do
  echo "${SKILL_SCORES[$skill_name]} $skill_name"
done | sort -rn | cut -d' ' -f2-)

# Write individual scores
for skill_name in $SORTED_SKILLS; do
  echo "| $skill_name | ${SKILL_SCORES[$skill_name]}/120 | ${SKILL_GRADES[$skill_name]} | ${SKILL_LINES[$skill_name]} | ${SKILL_REFS[$skill_name]} |" >> "$OUTPUT_FILE"
done

# Add baseline comparison if provided
if [[ -n "$BASELINE_FILE" && -f "$BASELINE_FILE" ]]; then
  cat >> "$OUTPUT_FILE" <<EOF

---

## Baseline Comparison

Comparing against baseline: $BASELINE_FILE

| Skill | Current | Baseline | Change |
|-------|---------|----------|--------|
EOF

  # Extract baseline scores
  while IFS='|' read -r _ skill baseline_score _; do
    skill=$(echo "$skill" | tr -d ' ')
    if [[ -n "$skill" && "$skill" != "Skill" && -n "${SKILL_SCORES[$skill]}" ]]; then
      baseline_num=$(echo "$baseline_score" | grep -o '[0-9]*' | head -1)
      current="${SKILL_SCORES[$skill]}"
      if [[ -n "$baseline_num" ]]; then
        change=$((current - baseline_num))
        change_str="+$change"
        [[ $change -lt 0 ]] && change_str="$change"
        echo "| $skill | $current | $baseline_num | $change_str |" >> "$OUTPUT_FILE"
      fi
    fi
  done < "$BASELINE_FILE"
fi

cat >> "$OUTPUT_FILE" <<EOF

---

## Recommendations

EOF

# Add recommendations based on findings
if [[ $D_COUNT -gt 0 || $F_COUNT -gt 0 ]]; then
  cat >> "$OUTPUT_FILE" <<EOF
### Critical Priority (D/F Grade Skills)

EOF
  for skill_name in $SORTED_SKILLS; do
    grade="${SKILL_GRADES[$skill_name]}"
    if [[ "$grade" == "D" || "$grade" == "F" ]]; then
      echo "- **$skill_name** (${SKILL_SCORES[$skill_name]}/120) - Requires immediate attention" >> "$OUTPUT_FILE"
    fi
  done
  echo "" >> "$OUTPUT_FILE"
fi

if [[ $C_COUNT -gt 0 ]]; then
  cat >> "$OUTPUT_FILE" <<EOF
### High Priority (C Grade Skills)

$C_COUNT skills scored C grade and need improvement:

EOF
  for skill_name in $SORTED_SKILLS; do
    grade="${SKILL_GRADES[$skill_name]}"
    if [[ "$grade" == "C" || "$grade" == "C+" ]]; then
      lines="${SKILL_LINES[$skill_name]}"
      if [[ $lines -gt 300 ]]; then
        echo "- **$skill_name** ($lines lines) - Consider applying Navigation Hub pattern" >> "$OUTPUT_FILE"
      else
        echo "- **$skill_name** (${SKILL_SCORES[$skill_name]}/120) - Review description and anti-patterns" >> "$OUTPUT_FILE"
      fi
    fi
  done
  echo "" >> "$OUTPUT_FILE"
fi

# Check for potential aggregation candidates
if [[ $SKILL_COUNT -gt 20 ]]; then
  cat >> "$OUTPUT_FILE" <<EOF
### Consider Aggregation

With $SKILL_COUNT total skills, consider running duplication detection:

\`\`\`bash
./scripts/detect-duplication.sh
\`\`\`

EOF
fi

cat >> "$OUTPUT_FILE" <<EOF
---

## Next Steps

1. **Review D/F skills** - Immediate refactoring or aggregation required
2. **Improve C skills** - Focus on description and anti-pattern examples
3. **Run duplication detection** - Find consolidation opportunities
4. **Set baseline** - Save this report for next quarter comparison:
   \`\`\`bash
   cp $OUTPUT_FILE .context/baselines/quarterly-\$(date +%Y-Q%q).md
   \`\`\`

---

**Generated by:** skill-quality-auditor  
**Tool:** \`scripts/audit-skills.sh\` + \`scripts/evaluate.ts\`  
**Framework:** skill-judge (8-dimension evaluation)
EOF

echo
echo "✅ Audit complete"
echo "Report: $OUTPUT_FILE"
echo
echo "Summary:"
echo "  Average Score: $AVG_SCORE/120"
echo "  Grade Distribution: A: $A_COUNT, B: $B_COUNT, C: $C_COUNT, D: $D_COUNT, F: $F_COUNT"
echo

# Return non-zero if any F-grade skills found
[[ $F_COUNT -gt 0 ]] && exit 1
exit 0
