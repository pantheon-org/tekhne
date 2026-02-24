#!/usr/bin/env sh
# Generate analysis.md for each audit date

generate_analysis() {
  AUDIT_DATE="$1"
  AUDIT_DIR=".context/audits/skill-audit/$AUDIT_DATE"
  AUDIT_JSON="$AUDIT_DIR/audit.json"
  
  [ -f "$AUDIT_JSON" ] || return
  
  SKILL_COUNT=$(grep -oP '"skillsCount":\s*\K[0-9]+' "$AUDIT_JSON")
  AVG_SCORE=$(grep -oP '"averageScore":\s*\K[0-9]+' "$AUDIT_JSON")
  A_COUNT=$(grep -oP '"A":\s*\K[0-9]+' "$AUDIT_JSON" | head -1)
  B_COUNT=$(grep -oP '"B":\s*\K[0-9]+' "$AUDIT_JSON" | head -1)
  C_COUNT=$(grep -oP '"C":\s*\K[0-9]+' "$AUDIT_JSON" | head -1)
  D_COUNT=$(grep -oP '"D":\s*\K[0-9]+' "$AUDIT_JSON" | head -1)
  F_COUNT=$(grep -oP '"F":\s*\K[0-9]+' "$AUDIT_JSON" | head -1)
  
  A_PCT=$((A_COUNT * 100 / SKILL_COUNT))
  B_PCT=$((B_COUNT * 100 / SKILL_COUNT))
  C_PCT=$((C_COUNT * 100 / SKILL_COUNT))
  
  MONTH=$(date -j -f "%Y-%m-%d" "$AUDIT_DATE" +"%B %d, %Y" 2>/dev/null || echo "$AUDIT_DATE")
  
  cat > "$AUDIT_DIR/analysis.md" <<EOF
# Skill Quality Audit Report

**Date:** $MONTH  
**Skills Evaluated:** $SKILL_COUNT  
**Evaluator:** skill-judge framework (8-dimension evaluation)

---

## Summary

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

| Skill | Score | Grade | Lines | References |
|-------|------|-------|-------|------------|
EOF
  
  grep -oP '"skill":\s*"\K[^"]+' "$AUDIT_JSON" | nl -w1 -s"| " | while read -r skill; do
    score=$(grep -A4 "\"skill\": \"$skill\"" "$AUDIT_JSON" | grep -oP '"total":\s*\K[0-9]+' | head -1)
    grade=$(grep -A4 "\"skill\": \"$skill\"" "$AUDIT_JSON" | grep -oP '"grade":\s*"\K[^"]+' | head -1)
    lines=$(grep -A4 "\"skill\": \"$skill\"" "$AUDIT_JSON" | grep -oP '"lines":\s*\K[0-9]+' | head -1)
    refs=$(grep -A4 "\"skill\": \"$skill\"" "$AUDIT_JSON" | grep -oP '"referenceCount":\s*\K[0-9]+' | head -1)
    echo "| $skill | $score | $grade | $lines | $refs |" >> "$AUDIT_DIR/analysis.md"
  done
  
  cat >> "$AUDIT_DIR/analysis.md" <<EOF

---

## Recommendations

EOF
  
  D_SKILLS=$(grep -B1 '"D"' "$AUDIT_JSON" | grep -oP '"skill":\s*"\K[^"]+' | tr '\n' ', ' | sed 's/,$//')
  F_SKILLS=$(grep -B1 '"F"' "$AUDIT_JSON" | grep -oP '"skill":\s*"\K[^"]+' | tr '\n' ', ' | sed 's/,$//')
  C_SKILLS=$(grep -B1 '"C[A+]*"' "$AUDIT_JSON" | grep -oP '"skill":\s*"\K[^"]+' | tr '\n' ', ' | sed 's/,$//')
  
  if [ -n "$D_SKILLS" ] || [ -n "$F_SKILLS" ]; then
    echo "### Priority: Address Underperforming Skills" >> "$AUDIT_DIR/analysis.md"
    echo "" >> "$AUDIT_DIR/analysis.md"
    [ -n "$F_SKILLS" ] && echo "Skills with F grade require immediate attention: $F_SKILLS" >> "$AUDIT_DIR/analysis.md"
    [ -n "$D_SKILLS" ] && echo "Skills with D grade need improvement: $D_SKILLS" >> "$AUDIT_DIR/analysis.md"
    echo "" >> "$AUDIT_DIR/analysis.md"
  fi
  
  if [ -n "$C_SKILLS" ]; then
    echo "### Consider Improving" >> "$AUDIT_DIR/analysis.md"
    echo "" >> "$AUDIT_DIR/analysis.md"
    echo "Skills with C grade have room for improvement: $C_SKILLS" >> "$AUDIT_DIR/analysis.md"
    echo "" >> "$AUDIT_DIR/analysis.md"
  fi
  
  cat >> "$AUDIT_DIR/analysis.md" <<EOF

---

## Next Steps

1. Review individual skill audit reports in \`.context/audits/\`
2. Address critical issues in underperforming skills
3. Run remediation plans from \`.context/plans/<skill>-remediation-plan.md\`
4. Re-audit after improvements

---

*Generated from audit.json on $AUDIT_DATE*
EOF
  
  echo "Created $AUDIT_DIR/analysis.md"
}

generate_analysis "2026-02-21"
generate_analysis "2026-02-22"
generate_analysis "2026-02-23"
