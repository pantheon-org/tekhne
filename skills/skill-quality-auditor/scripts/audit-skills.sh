#!/usr/bin/env sh
# Audit skills using skill-judge framework via evaluate.sh
# Usage:
#   ./audit-skills.sh [--output report.md] [--baseline baseline.md]
#                     [--skills-dir path] [--exclude skill-name]

set -eu

SCRIPT_DIR=$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)
PROJECT_ROOT=$(cd "$SCRIPT_DIR/../../.." >/dev/null 2>&1 && pwd)
EVALUATE_SCRIPT="$SCRIPT_DIR/evaluate.sh"

OUTPUT_FILE=".context/analysis/audit-$(date +%Y-%m-%d).md"
BASELINE_FILE=""
SKILLS_DIRS="skills .agents/skills"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM
EXCLUDES_FILE="$TMP_DIR/excludes.txt"
SKILLS_FILE="$TMP_DIR/skills.txt"
ROWS_FILE="$TMP_DIR/rows.txt"
: > "$EXCLUDES_FILE"
: > "$SKILLS_FILE"
: > "$ROWS_FILE"

usage() {
  cat <<EOF
Usage: ./audit-skills.sh [--output report.md] [--baseline baseline.md] [--skills-dir path] [--exclude skill-name]
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --output|-o)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      OUTPUT_FILE="$2"
      shift 2
      ;;
    --baseline|-b)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      BASELINE_FILE="$2"
      shift 2
      ;;
    --skills-dir|-s)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      SKILLS_DIRS="$SKILLS_DIRS $2"
      shift 2
      ;;
    --exclude|-x)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      printf '%s\n' "$2" >> "$EXCLUDES_FILE"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

cd "$PROJECT_ROOT"

echo "=== Skill Quality Audit ==="
echo "Date: $(date)"
echo "Output: $OUTPUT_FILE"
if [ -s "$EXCLUDES_FILE" ]; then
  echo "Excludes: $(paste -sd ', ' "$EXCLUDES_FILE")"
fi
echo

mkdir -p "$(dirname "$OUTPUT_FILE")"

for root in $SKILLS_DIRS; do
  [ -d "$root" ] || continue
  find "$root" -type f -name "SKILL.md" -not -path "*/.deprecated/*" -not -path "*/node_modules/*" 2>/dev/null |
    while IFS= read -r skill_md; do
      skill_name=$(basename "$(dirname "$skill_md")")
      printf '%s\n' "$skill_name" >> "$SKILLS_FILE"
    done
done

sort -u "$SKILLS_FILE" -o "$SKILLS_FILE"
if [ -s "$EXCLUDES_FILE" ]; then
  grep -v -x -F -f "$EXCLUDES_FILE" "$SKILLS_FILE" > "$TMP_DIR/skills.filtered.txt" || true
  mv "$TMP_DIR/skills.filtered.txt" "$SKILLS_FILE"
fi

SKILL_COUNT=$(grep -c . "$SKILLS_FILE" || true)
if [ "$SKILL_COUNT" -eq 0 ]; then
  echo "No skills found for audit." >&2
  exit 1
fi

cat > "$OUTPUT_FILE" <<EOF
# Skill Quality Audit Report

**Date:** $(date +"%B %d, %Y")  
**Skills Evaluated:** $SKILL_COUNT  
**Evaluator:** skill-judge framework (8-dimension evaluation)

---

## Summary

EOF

echo "Evaluating skills using skill-judge framework..."

TOTAL_SCORE=0
A_COUNT=0
B_COUNT=0
C_COUNT=0
D_COUNT=0
F_COUNT=0

while IFS= read -r skill_name; do
  [ -n "$skill_name" ] || continue
  echo "  - $skill_name"
  EVAL_OUTPUT=$(sh "$EVALUATE_SCRIPT" "$skill_name" --json 2>/dev/null || echo '{"total":0,"grade":"F","lines":0,"referenceCount":0}')

  SCORE=$(printf '%s' "$EVAL_OUTPUT" | sed -n 's/.*"total"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' | head -1)
  GRADE=$(printf '%s' "$EVAL_OUTPUT" | sed -n 's/.*"grade"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -1)
  LINES=$(printf '%s' "$EVAL_OUTPUT" | sed -n 's/.*"lines"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' | head -1)
  REFS=$(printf '%s' "$EVAL_OUTPUT" | sed -n 's/.*"referenceCount"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' | head -1)

  [ -n "$SCORE" ] || SCORE=0
  [ -n "$GRADE" ] || GRADE=F
  [ -n "$LINES" ] || LINES=0
  [ -n "$REFS" ] || REFS=0

  TOTAL_SCORE=$((TOTAL_SCORE + SCORE))
  printf '%s|%s|%s|%s|%s\n' "$SCORE" "$skill_name" "$GRADE" "$LINES" "$REFS" >> "$ROWS_FILE"

  case "$GRADE" in
    A+|A) A_COUNT=$((A_COUNT + 1)) ;;
    B+|B) B_COUNT=$((B_COUNT + 1)) ;;
    C+|C) C_COUNT=$((C_COUNT + 1)) ;;
    D) D_COUNT=$((D_COUNT + 1)) ;;
    F) F_COUNT=$((F_COUNT + 1)) ;;
  esac
done < "$SKILLS_FILE"

AVG_SCORE=$((TOTAL_SCORE / SKILL_COUNT))
A_PCT=$((A_COUNT * 100 / SKILL_COUNT))
B_PCT=$((B_COUNT * 100 / SKILL_COUNT))
C_PCT=$((C_COUNT * 100 / SKILL_COUNT))

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

sort -t '|' -k1,1nr "$ROWS_FILE" |
  while IFS='|' read -r score skill_name grade lines refs; do
    echo "| $skill_name | $score/120 | $grade | $lines | $refs |" >> "$OUTPUT_FILE"
  done

if [ -n "$BASELINE_FILE" ] && [ -f "$BASELINE_FILE" ]; then
  cat >> "$OUTPUT_FILE" <<EOF

---

## Baseline Comparison

Comparing against baseline: $BASELINE_FILE

| Skill | Current | Baseline | Change |
|-------|---------|----------|--------|
EOF

  while IFS='|' read -r score skill_name _grade _lines _refs; do
    baseline_num=$(awk -F'|' -v s="$skill_name" '
      $0 ~ /^\|/ {
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
        if ($2 == s) {
          gsub(/^[[:space:]]+|[[:space:]]+$/, "", $3)
          sub(/\/.*/, "", $3)
          print $3
          exit
        }
      }
    ' "$BASELINE_FILE")

    if [ -n "$baseline_num" ]; then
      change=$((score - baseline_num))
      change_str="+$change"
      if [ "$change" -lt 0 ]; then
        change_str="$change"
      fi
      echo "| $skill_name | $score | $baseline_num | $change_str |" >> "$OUTPUT_FILE"
    fi
  done < "$ROWS_FILE"
fi

cat >> "$OUTPUT_FILE" <<EOF

---

## Recommendations

EOF

if [ "$D_COUNT" -gt 0 ] || [ "$F_COUNT" -gt 0 ]; then
  cat >> "$OUTPUT_FILE" <<EOF
### Critical Priority (D/F Grade Skills)

EOF
  sort -t '|' -k1,1nr "$ROWS_FILE" |
    while IFS='|' read -r score skill_name grade _lines _refs; do
      if [ "$grade" = "D" ] || [ "$grade" = "F" ]; then
        echo "- **$skill_name** ($score/120) - Requires immediate attention" >> "$OUTPUT_FILE"
      fi
    done
  echo "" >> "$OUTPUT_FILE"
fi

if [ "$C_COUNT" -gt 0 ]; then
  cat >> "$OUTPUT_FILE" <<EOF
### High Priority (C Grade Skills)

$C_COUNT skills scored C grade and need improvement:

EOF
  sort -t '|' -k1,1nr "$ROWS_FILE" |
    while IFS='|' read -r score skill_name grade lines _refs; do
      if [ "$grade" = "C" ] || [ "$grade" = "C+" ]; then
        if [ "$lines" -gt 300 ]; then
          echo "- **$skill_name** ($lines lines) - Consider applying Navigation Hub pattern" >> "$OUTPUT_FILE"
        else
          echo "- **$skill_name** ($score/120) - Review description and anti-patterns" >> "$OUTPUT_FILE"
        fi
      fi
    done
  echo "" >> "$OUTPUT_FILE"
fi

if [ "$SKILL_COUNT" -gt 20 ]; then
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
   cp $OUTPUT_FILE .context/baselines/audit-\$(date +%Y-%m-%d).md
   \`\`\`

---

**Generated by:** skill-quality-auditor  
**Tool:** \`scripts/audit-skills.sh\` + \`scripts/evaluate.sh\`  
**Framework:** skill-judge (8-dimension evaluation)
EOF

echo
echo "Audit complete"
echo "Report: $OUTPUT_FILE"
echo
echo "Summary:"
echo "  Average Score: $AVG_SCORE/120"
echo "  Grade Distribution: A: $A_COUNT, B: $B_COUNT, C: $C_COUNT, D: $D_COUNT, F: $F_COUNT"
echo

if [ "$F_COUNT" -gt 0 ]; then
  exit 1
fi
exit 0
