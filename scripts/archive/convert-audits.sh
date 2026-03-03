#!/usr/bin/env sh
# Convert markdown audit files to JSON format

AUDIT_DATE="${1:-2026-02-23}"
AUDIT_DIR=".context/audits/skill-audit/$AUDIT_DATE"

mkdir -p "$AUDIT_DIR"

SKILLS_JSON="["
first=1

for mdfile in .context/audits/*-audit-$AUDIT_DATE.md; do
  [ -f "$mdfile" ] || continue
  
  skill_name=$(basename "$mdfile" | sed 's/-audit-.*$//')
  
  total_score=$(grep -oP '\*\*Total Score\*\*\s*\|\s*\K[0-9]+' "$mdfile" | head -1)
  grade=$(grep -oP '\*\*Grade\*\*\s*\|\s*\K[A-F][+]?' "$mdfile" | head -1)
  lines=$(grep -oP 'File size is\s*\K[0-9]+' "$mdfile" | head -1)
  refs=$(grep -oP 'references count is\s*\K[0-9]+' "$mdfile" | head -1)
  
  [ -z "$total_score" ] && total_score=0
  [ -z "$grade" ] && grade="F"
  [ -z "$lines" ] && lines=0
  [ -z "$refs" ] && refs=0
  
  if [ $first -eq 1 ]; then
    first=0
  else
    SKILLS_JSON="$SKILLS_JSON,"
  fi
  
  SKILLS_JSON="$SKILLS_JSON
    {
      \"skill\": \"$skill_name\",
      \"total\": $total_score,
      \"grade\": \"$grade\",
      \"lines\": $lines,
      \"referenceCount\": $refs
    }"
done

SKILLS_JSON="$SKILLS_JSON
  ]"

A_COUNT=$(echo "$SKILLS_JSON" | grep -o '"grade": *"A[A+]*"' | wc -l | tr -d ' ')
B_COUNT=$(echo "$SKILLS_JSON" | grep -o '"grade": *"B[A+]*"' | wc -l | tr -d ' ')
C_COUNT=$(echo "$SKILLS_JSON" | grep -o '"grade": *"C[A+]*"' | wc -l | tr -d ' ')
D_COUNT=$(echo "$SKILLS_JSON" | grep -o '"grade": *"D"' | wc -l | tr -d ' ')
F_COUNT=$(echo "$SKILLS_JSON" | grep -o '"grade": *"F"' | wc -l | tr -d ' ')

SKILL_COUNT=$(echo "$SKILLS_JSON" | grep -o '"skill":' | wc -l | tr -d ' ')

TOTAL_SCORE=0
for score in $(echo "$SKILLS_JSON" | grep -oP '"total":\s*\K[0-9]+'); do
  TOTAL_SCORE=$((TOTAL_SCORE + score))
done

if [ "$SKILL_COUNT" -gt 0 ]; then
  AVG_SCORE=$((TOTAL_SCORE / SKILL_COUNT))
else
  AVG_SCORE=0
fi

[ -z "$A_COUNT" ] && A_COUNT=0
[ -z "$B_COUNT" ] && B_COUNT=0
[ -z "$C_COUNT" ] && C_COUNT=0
[ -z "$D_COUNT" ] && D_COUNT=0
[ -z "$F_COUNT" ] && F_COUNT=0

cat > "$AUDIT_DIR/audit.json" <<EOF
{
  "auditDate": "$AUDIT_DATE",
  "skillsCount": $SKILL_COUNT,
  "averageScore": $AVG_SCORE,
  "gradeDistribution": {
    "A": $A_COUNT,
    "B": $B_COUNT,
    "C": $C_COUNT,
    "D": $D_COUNT,
    "F": $F_COUNT
  },
  "skills": $SKILLS_JSON
}
EOF

echo "Created $AUDIT_DIR/audit.json with $SKILL_COUNT skills"
