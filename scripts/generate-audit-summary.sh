#!/usr/bin/env sh
# generate-audit-summary.sh - Generate comprehensive audit summary report
# Uses correct JSON fields, dimension names, and grade ranges

set -e

# Check dependencies
command -v jq >/dev/null 2>&1 || {
  echo "❌ Error: jq is required"
  exit 1
}

# Check audit directory exists
if [ ! -d ".context/audits" ]; then
  echo "❌ Error: .context/audits directory not found"
  exit 1
fi

DATE=$(date +%Y-%m-%d)
OUTPUT=".context/audits/summary-$DATE.md"
TEMP_DATA=$(mktemp)

echo "🔍 Generating audit summary report..."

# Single-pass data collection from all audits
find -L .context/audits -type f -name "audit.json" -path "*/latest/*" 2>/dev/null > "$TEMP_DATA.files"

TOTAL_AUDITS=$(wc -l < "$TEMP_DATA.files" | tr -d ' ')

if [ "$TOTAL_AUDITS" -eq 0 ]; then
  echo "❌ Error: No audit.json files found in .context/audits/*/latest/"
  rm -f "$TEMP_DATA" "$TEMP_DATA.files"
  exit 1
fi

echo "  Found $TOTAL_AUDITS audits"
echo "  Collecting audit data..."

# Collect all data in one pass
while IFS= read -r file; do
  # Extract skill path relative to .context/audits/ (supports nested structure)
  skill=$(echo "$file" | sed 's|.context/audits/||' | sed 's|/latest/.*||')
  score=$(jq -r '.total' "$file" 2>/dev/null || echo "0")
  grade=$(jq -r '.grade' "$file" 2>/dev/null || echo "?")
  
  # Extract all 8 dimensions
  d1=$(jq -r '.dimensions.d1_knowledge_delta' "$file" 2>/dev/null || echo "0")
  d2=$(jq -r '.dimensions.d2_mindset_procedures' "$file" 2>/dev/null || echo "0")
  d3=$(jq -r '.dimensions.d3_anti_pattern_quality' "$file" 2>/dev/null || echo "0")
  d4=$(jq -r '.dimensions.d4_specification_compliance' "$file" 2>/dev/null || echo "0")
  d5=$(jq -r '.dimensions.d5_progressive_disclosure' "$file" 2>/dev/null || echo "0")
  d6=$(jq -r '.dimensions.d6_freedom_calibration' "$file" 2>/dev/null || echo "0")
  d7=$(jq -r '.dimensions.d7_pattern_recognition' "$file" 2>/dev/null || echo "0")
  d8=$(jq -r '.dimensions.d8_practical_usability' "$file" 2>/dev/null || echo "0")
  
  echo "$score|$skill|$grade|$d1|$d2|$d3|$d4|$d5|$d6|$d7|$d8" >> "$TEMP_DATA"
done < "$TEMP_DATA.files"

echo "  Generating report..."

# Generate report header
{
cat <<EOF
# Skill Audit Summary Report

**Date:** $DATE  
**Framework:** skill-quality-auditor (skill-judge)  
**Total Skills Audited:** $TOTAL_AUDITS

---

## Score Statistics

EOF

# Calculate statistics
TOTAL_SCORE=$(awk -F'|' '{sum+=$1} END {print sum}' "$TEMP_DATA")
AVG_SCORE=$((TOTAL_SCORE / TOTAL_AUDITS))
MIN_SCORE=$(awk -F'|' '{print $1}' "$TEMP_DATA" | sort -n | head -1)
MAX_SCORE=$(awk -F'|' '{print $1}' "$TEMP_DATA" | sort -n | tail -1)
MEDIAN_SCORE=$(awk -F'|' '{print $1}' "$TEMP_DATA" | sort -n | awk '{a[NR]=$1} END {if(NR%2==1)print a[(NR+1)/2];else print int((a[NR/2]+a[NR/2+1])/2)}')

cat <<EOF
| Metric | Value |
|--------|-------|
| Average Score | $AVG_SCORE/120 |
| Median Score | $MEDIAN_SCORE/120 |
| Minimum Score | $MIN_SCORE/120 |
| Maximum Score | $MAX_SCORE/120 |

---

## Grade Distribution

EOF

# Grade distribution with correct ranges
cat <<EOF
| Grade | Range | Count | Percentage |
|-------|-------|-------|------------|
EOF

grade_ap=$(grep -c '|A+$' "$TEMP_DATA" || echo "0")
grade_a=$(grep '|A$' "$TEMP_DATA" | grep -cv '|A+$' || echo "0")
grade_bp=$(grep -c '|B+$' "$TEMP_DATA" || echo "0")
grade_b=$(grep '|B$' "$TEMP_DATA" | grep -cv '|B+$' || echo "0")
grade_cp=$(grep -c '|C+$' "$TEMP_DATA" || echo "0")
grade_c=$(grep '|C$' "$TEMP_DATA" | grep -cv '|C+$' || echo "0")
grade_d=$(grep -c '|D$' "$TEMP_DATA" || echo "0")
grade_f=$(grep -c '|F$' "$TEMP_DATA" || echo "0")

# Calculate percentages using awk with proper quoting
pct_ap=$(awk "BEGIN {printf \"%.1f\", ($grade_ap/$TOTAL_AUDITS)*100}")
pct_a=$(awk "BEGIN {printf \"%.1f\", ($grade_a/$TOTAL_AUDITS)*100}")
pct_bp=$(awk "BEGIN {printf \"%.1f\", ($grade_bp/$TOTAL_AUDITS)*100}")
pct_b=$(awk "BEGIN {printf \"%.1f\", ($grade_b/$TOTAL_AUDITS)*100}")
pct_cp=$(awk "BEGIN {printf \"%.1f\", ($grade_cp/$TOTAL_AUDITS)*100}")
pct_c=$(awk "BEGIN {printf \"%.1f\", ($grade_c/$TOTAL_AUDITS)*100}")
pct_d=$(awk "BEGIN {printf \"%.1f\", ($grade_d/$TOTAL_AUDITS)*100}")
pct_f=$(awk "BEGIN {printf \"%.1f\", ($grade_f/$TOTAL_AUDITS)*100}")

cat >> "$OUTPUT" <<EOF
| A+ | ≥114/120 | $grade_ap | ${pct_ap}% |
| A | 108-113/120 | $grade_a | ${pct_a}% |
| B+ | 102-107/120 | $grade_bp | ${pct_bp}% |
| B | 96-101/120 | $grade_b | ${pct_b}% |
| C+ | 90-95/120 | $grade_cp | ${pct_cp}% |
| C | 84-89/120 | $grade_c | ${pct_c}% |
| D | 78-83/120 | $grade_d | ${pct_d}% |
| F | <78/120 | $grade_f | ${pct_f}% |

---

## Top 10 Highest Scoring Skills

| Skill | Score | Grade |
|-------|-------|-------|
EOF

# Top 10 scores
sort -t'|' -k1 -rn "$TEMP_DATA" | head -10 | while IFS='|' read -r score skill grade rest; do
  echo "| $skill | $score/120 | $grade |" >> "$OUTPUT"
done

cat >> "$OUTPUT" <<EOF

---

## Top 10 Skills Needing Improvement

| Skill | Score | Grade | Weakest Dimensions |
|-------|-------|-------|-------------------|
EOF

# Bottom 10 with weakest dimensions identified
sort -t'|' -k1 -n "$TEMP_DATA" | head -10 | while IFS='|' read -r score skill grade d1 d2 d3 d4 d5 d6 d7 d8; do
  # Calculate percentage for each dimension
  d1_pct=$((d1 * 100 / 20))
  d2_pct=$((d2 * 100 / 15))
  d3_pct=$((d3 * 100 / 15))
  d4_pct=$((d4 * 100 / 15))
  d5_pct=$((d5 * 100 / 15))
  d6_pct=$((d6 * 100 / 15))
  d7_pct=$((d7 * 100 / 10))
  d8_pct=$((d8 * 100 / 15))
  
  # Find dimensions below 80%
  weak=""
  [ "$d1_pct" -lt 80 ] && weak="${weak}D1 "
  [ "$d2_pct" -lt 80 ] && weak="${weak}D2 "
  [ "$d3_pct" -lt 80 ] && weak="${weak}D3 "
  [ "$d4_pct" -lt 80 ] && weak="${weak}D4 "
  [ "$d5_pct" -lt 80 ] && weak="${weak}D5 "
  [ "$d6_pct" -lt 80 ] && weak="${weak}D6 "
  [ "$d7_pct" -lt 80 ] && weak="${weak}D7 "
  [ "$d8_pct" -lt 80 ] && weak="${weak}D8 "
  
  [ -z "$weak" ] && weak="None"
  
  echo "| $skill | $score/120 | $grade | $weak |" >> "$OUTPUT"
done

cat >> "$OUTPUT" <<EOF

---

## Dimensional Analysis

Average scores by dimension across all skills:

| Dimension | Avg Score | Max Points | Avg % |
|-----------|-----------|------------|-------|
EOF

# Calculate dimensional averages
d1_avg=$(awk -F'|' '{sum+=$4} END {printf "%.1f", sum/NR}' "$TEMP_DATA")
d2_avg=$(awk -F'|' '{sum+=$5} END {printf "%.1f", sum/NR}' "$TEMP_DATA")
d3_avg=$(awk -F'|' '{sum+=$6} END {printf "%.1f", sum/NR}' "$TEMP_DATA")
d4_avg=$(awk -F'|' '{sum+=$7} END {printf "%.1f", sum/NR}' "$TEMP_DATA")
d5_avg=$(awk -F'|' '{sum+=$8} END {printf "%.1f", sum/NR}' "$TEMP_DATA")
d6_avg=$(awk -F'|' '{sum+=$9} END {printf "%.1f", sum/NR}' "$TEMP_DATA")
d7_avg=$(awk -F'|' '{sum+=$10} END {printf "%.1f", sum/NR}' "$TEMP_DATA")
d8_avg=$(awk -F'|' '{sum+=$11} END {printf "%.1f", sum/NR}' "$TEMP_DATA")

d1_pct=$(awk "BEGIN {printf \"%.1f\", ($d1_avg/20)*100}")
d2_pct=$(awk "BEGIN {printf \"%.1f\", ($d2_avg/15)*100}")
d3_pct=$(awk "BEGIN {printf \"%.1f\", ($d3_avg/15)*100}")
d4_pct=$(awk "BEGIN {printf \"%.1f\", ($d4_avg/15)*100}")
d5_pct=$(awk "BEGIN {printf \"%.1f\", ($d5_avg/15)*100}")
d6_pct=$(awk "BEGIN {printf \"%.1f\", ($d6_avg/15)*100}")
d7_pct=$(awk "BEGIN {printf \"%.1f\", ($d7_avg/10)*100}")
d8_pct=$(awk "BEGIN {printf \"%.1f\", ($d8_avg/15)*100}")

cat >> "$OUTPUT" <<EOF
| D1: Knowledge Delta | $d1_avg | 20 | ${d1_pct}% |
| D2: Mindset + Procedures | $d2_avg | 15 | ${d2_pct}% |
| D3: Anti-Pattern Quality | $d3_avg | 15 | ${d3_pct}% |
| D4: Specification Compliance | $d4_avg | 15 | ${d4_pct}% |
| D5: Progressive Disclosure | $d5_avg | 15 | ${d5_pct}% |
| D6: Freedom Calibration | $d6_avg | 15 | ${d6_pct}% |
| D7: Pattern Recognition | $d7_avg | 10 | ${d7_pct}% |
| D8: Practical Usability | $d8_avg | 15 | ${d8_pct}% |

---

## Common Patterns in Low-Scoring Dimensions

Skills scoring below 80% in each dimension:

EOF

# Count skills below 80% for each dimension
d1_low=$(awk -F'|' '$4 < 16 {count++} END {print count+0}' "$TEMP_DATA")
d2_low=$(awk -F'|' '$5 < 12 {count++} END {print count+0}' "$TEMP_DATA")
d3_low=$(awk -F'|' '$6 < 12 {count++} END {print count+0}' "$TEMP_DATA")
d4_low=$(awk -F'|' '$7 < 12 {count++} END {print count+0}' "$TEMP_DATA")
d5_low=$(awk -F'|' '$8 < 12 {count++} END {print count+0}' "$TEMP_DATA")
d6_low=$(awk -F'|' '$9 < 12 {count++} END {print count+0}' "$TEMP_DATA")
d7_low=$(awk -F'|' '$10 < 8 {count++} END {print count+0}' "$TEMP_DATA")
d8_low=$(awk -F'|' '$11 < 12 {count++} END {print count+0}' "$TEMP_DATA")

cat >> "$OUTPUT" <<EOF
- **D1 Knowledge Delta:** $d1_low skills need more unique insights beyond basic documentation
- **D2 Mindset + Procedures:** $d2_low skills missing decision frameworks or workflow guidance
- **D3 Anti-Pattern Quality:** $d3_low skills need NEVER/ALWAYS constraints with examples
- **D4 Specification Compliance:** $d4_low skills have weak description fields (<200 chars)
- **D5 Progressive Disclosure:** $d5_low skills are too long or lack references/ structure
- **D6 Freedom Calibration:** $d6_low skills are over/under-prescriptive
- **D7 Pattern Recognition:** $d7_low skills have insufficient trigger keywords
- **D8 Practical Usability:** $d8_low skills need more code examples and syntax highlighting

---

## Recommendations

Based on dimensional analysis:

EOF

# Generate recommendations based on weakest dimensions
weakest_dim=""
weakest_count=0

if [ "$d3_low" -gt "$weakest_count" ]; then weakest_count=$d3_low; weakest_dim="D3: Anti-Pattern Quality"; fi
if [ "$d5_low" -gt "$weakest_count" ]; then weakest_count=$d5_low; weakest_dim="D5: Progressive Disclosure"; fi
if [ "$d8_low" -gt "$weakest_count" ]; then weakest_count=$d8_low; weakest_dim="D8: Practical Usability"; fi

cat >> "$OUTPUT" <<EOF
1. **Priority Focus:** $weakest_dim ($weakest_count skills below 80%)
   - Review remediation plans for affected skills
   - Create templates/examples for this dimension
   - Add enforcement checks if possible

2. **Overall Quality Threshold:** Aim for all skills ≥108/120 (A grade)
   - Currently: $grade_a A-grade, $grade_ap A+ grade
   - Target: All skills in A/A+ range

3. **Systematic Improvement:** Address low-scoring skills in priority order
   - Start with bottom 10 skills
   - Focus on weakest 2-3 dimensions per skill
   - Re-audit after improvements

---

## Next Steps

1. Review bottom 10 skills and their remediation plans
2. Execute remediation for skills scoring <108
3. Re-run audits after improvements
4. Track quality trends over time with this report format

---

**Report Generated:** $DATE  
**Output:** $OUTPUT
EOF
} > "$OUTPUT"

# Cleanup
rm -f "$TEMP_DATA" "$TEMP_DATA.files"

echo "✅ Generated: $OUTPUT"
