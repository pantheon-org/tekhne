#!/usr/bin/env sh
# Audit all skills using skill-judge framework
# Usage:
#   ./audit-skills.sh [--pr-changes-only] [--latest-symlink] [--output-dir <path>]

set -e

SCRIPT_DIR=$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)
PROJECT_ROOT=$(cd "$SCRIPT_DIR/../../.." >/dev/null 2>&1 && pwd)
EVALUATE_SCRIPT="$SCRIPT_DIR/evaluate.sh"

AUDIT_DATE=$(date +%Y-%m-%d)
AUDIT_DIR=".context/audits/skill-audit/$AUDIT_DATE"
LATEST_DIR=".context/audits/skill-audit/latest"
PR_CHANGES_ONLY=0
UPDATE_LATEST=0
SKILLS_DIRS="skills .agents/skills"

usage() {
  cat <<EOF
Usage: ./audit-skills.sh [options]

Options:
  --pr-changes-only      Only audit skills modified in current branch vs main/master
  --latest-symlink       Update latest/ symlink after audit
  --output-dir <path>    Custom output directory (default: $AUDIT_DIR)
  --help, -h            Show this help

Output:
  Creates .context/audits/skill-audit/YYYY-MM-DD/ with:
    - audit.json    (machine-readable data)
    - analysis.md   (human-readable summary)
    - remediation-plan.md (aggregated remediation needs)
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --pr-changes-only) PR_CHANGES_ONLY=1 ;;
    --latest-symlink) UPDATE_LATEST=1 ;;
    --output-dir)
      if [ "$#" -ge 2 ]; then
        AUDIT_DIR="$2"
        shift
      fi
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
  shift
done

cd "$PROJECT_ROOT"

echo "=== Skill Quality Audit ==="
echo "Date: $(date)"
echo "Output: $AUDIT_DIR"
echo

mkdir -p "$AUDIT_DIR"

PREVIOUS_AUDIT=""
if [ -L "$LATEST_DIR" ] || [ -d "$LATEST_DIR" ]; then
  PREVIOUS_AUDIT="$LATEST_DIR/audit.json"
  if [ ! -f "$PREVIOUS_AUDIT" ]; then
    PREVIOUS_AUDIT=""
  fi
fi

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM

collect_skills() {
  skills_file="$1"
  : > "$skills_file"
  
  for root in $SKILLS_DIRS; do
    [ -d "$root" ] || continue
    find "$root" -type f -name "SKILL.md" -not -path "*/.deprecated/*" -not -path "*/node_modules/*" 2>/dev/null |
      while IFS= read -r skill_md; do
        skill_name=$(basename "$(dirname "$skill_md")")
        printf '%s\n' "$skill_name"
      done
  done | sort -u > "$skills_file"
}

if [ "$PR_CHANGES_ONLY" -eq 1 ]; then
  echo "Detecting changed skills in current branch..."
  
  BASE_BRANCH="main"
  if git rev-parse "origin/main" >/dev/null 2>&1; then
    BASE_BRANCH="origin/main"
  elif git rev-parse "origin/master" >/dev/null 2>&1; then
    BASE_BRANCH="origin/master"
  fi
  
  CHANGED_FILES=$(git diff --name-only "$BASE_BRANCH" HEAD 2>/dev/null || git diff --name-only HEAD~1 HEAD 2>/dev/null || echo "")
  
  : > "$TMP_DIR/skills.txt"
  for file in $CHANGED_FILES; do
    case "$file" in
      skills/*/SKILL.md)
        skill_name=$(echo "$file" | sed 's|skills/||' | sed 's|/SKILL.md||')
        printf '%s\n' "$skill_name" >> "$TMP_DIR/skills.txt"
        ;;
      .agents/skills/*/SKILL.md)
        skill_name=$(echo "$file" | sed 's|.agents/skills/||' | sed 's|/SKILL.md||')
        printf '%s\n' "$skill_name" >> "$TMP_DIR/skills.txt"
        ;;
    esac
  done
  
  if [ ! -s "$TMP_DIR/skills.txt" ]; then
    echo "No skill files changed. Running full audit."
    collect_skills "$TMP_DIR/skills.txt"
  else
    sort -u "$TMP_DIR/skills.txt" -o "$TMP_DIR/skills.txt"
  fi
else
  collect_skills "$TMP_DIR/skills.txt"
fi

SKILL_COUNT=$(grep -c . "$TMP_DIR/skills.txt" || true)
if [ "$SKILL_COUNT" -eq 0 ]; then
  echo "No skills found for audit." >&2
  exit 1
fi

echo "Evaluating $SKILL_COUNT skill(s)..."

TOTAL_SCORE=0
A_COUNT=0
B_COUNT=0
C_COUNT=0
D_COUNT=0
F_COUNT=0

: > "$TMP_DIR/results.jsonl"
: > "$TMP_DIR/rows.txt"

load_previous_scores() {
  if [ -z "$PREVIOUS_AUDIT" ] || [ ! -f "$PREVIOUS_AUDIT" ]; then
    return
  fi
  grep -o '"skill": *"[^"]*"' "$PREVIOUS_AUDIT" | sed 's/"skill": *"\([^"]*\)"/\1/' | while read -r skill; do
    score=$(grep "\"skill\": *\"$skill\"" "$PREVIOUS_AUDIT" | sed -n 's/.*"total": *\([0-9]*\).*/\1/p' | head -1)
    if [ -n "$score" ]; then
      printf '%s\t%s\n' "$skill" "$score"
    fi
  done > "$TMP_DIR/previous_scores.txt"
}

load_previous_scores

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
  
  printf '%s\n' "$EVAL_OUTPUT" >> "$TMP_DIR/results.jsonl"
  printf '%s|%s|%s|%s|%s\n' "$SCORE" "$skill_name" "$GRADE" "$LINES" "$REFS" >> "$TMP_DIR/rows.txt"
  
  case "$GRADE" in
    A+|A) A_COUNT=$((A_COUNT + 1)) ;;
    B+|B) B_COUNT=$((B_COUNT + 1)) ;;
    C+|C) C_COUNT=$((C_COUNT + 1)) ;;
    D) D_COUNT=$((D_COUNT + 1)) ;;
    F) F_COUNT=$((F_COUNT + 1)) ;;
  esac
done < "$TMP_DIR/skills.txt"

AVG_SCORE=$((TOTAL_SCORE / SKILL_COUNT))
A_PCT=$((A_COUNT * 100 / SKILL_COUNT))
B_PCT=$((B_COUNT * 100 / SKILL_COUNT))
C_PCT=$((C_COUNT * 100 / SKILL_COUNT))

generate_json() {
  {
    echo "{"
    echo "  \"auditDate\": \"$AUDIT_DATE\","
    echo "  \"skillsCount\": $SKILL_COUNT,"
    echo "  \"averageScore\": $AVG_SCORE,"
    echo "  \"gradeDistribution\": {"
    echo "    \"A\": $A_COUNT,"
    echo "    \"B\": $B_COUNT,"
    echo "    \"C\": $C_COUNT,"
    echo "    \"D\": $D_COUNT,"
    echo "    \"F\": $F_COUNT"
    echo "  },"
    echo "  \"skills\": ["
    
    first=1
    while IFS= read -r line; do
      [ -n "$line" ] || continue
      skill_name=$(printf '%s' "$line" | sed -n 's/.*"skill": *"\([^"]*\)".*/\1/p')
      [ -n "$first" ] && first=0 || echo ","
      printf '    %s' "$line" | sed 's/^//'
    done < "$TMP_DIR/results.jsonl"
    
    echo ""
    echo "  ]"
    echo "}"
  } > "$AUDIT_DIR/audit.json"
}

generate_analysis() {
  cat > "$AUDIT_DIR/analysis.md" <<EOF
# Skill Quality Audit Report

**Date:** $(date +"%B %d, %Y")  
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

| Skill | Score | Grade | Lines | Refs |
|-------|-------|-------|-------|------|
EOF

  sort -t '|' -k1,1nr "$TMP_DIR/rows.txt" |
    while IFS='|' read -r score skill_name grade lines refs; do
      echo "| $skill_name | $score/120 | $grade | $lines | $refs |" >> "$AUDIT_DIR/analysis.md"
    done

  if [ -n "$PREVIOUS_AUDIT" ] && [ -f "$PREVIOUS_AUDIT" ]; then
    cat >> "$AUDIT_DIR/analysis.md" <<EOF

---

## Score Evolution

| Skill | Previous | Current | Delta |
|-------|----------|---------|-------|
EOF
    sort -t '|' -k1,1nr "$TMP_DIR/rows.txt" |
      while IFS='|' read -r score skill_name _grade _lines _refs; do
        prev_score=$(grep -w "^$skill_name" "$TMP_DIR/previous_scores.txt" 2>/dev/null | cut -f2 || echo "")
        if [ -n "$prev_score" ]; then
          delta=$((score - prev_score))
          if [ "$delta" -gt 0 ]; then
            delta_str="+$delta"
          elif [ "$delta" -lt 0 ]; then
            delta_str="$delta"
          else
            delta_str="0"
          fi
          echo "| $skill_name | $prev_score | $score | $delta_str |" >> "$AUDIT_DIR/analysis.md"
        fi
      done
  fi

  cat >> "$AUDIT_DIR/analysis.md" <<EOF

---

## Recommendations

EOF

  if [ "$D_COUNT" -gt 0 ] || [ "$F_COUNT" -gt 0 ]; then
    cat >> "$AUDIT_DIR/analysis.md" <<EOF
### Critical Priority (D/F Grade Skills)

EOF
    sort -t '|' -k1,1nr "$TMP_DIR/rows.txt" |
      while IFS='|' read -r score skill_name grade _lines _refs; do
        if [ "$grade" = "D" ] || [ "$grade" = "F" ]; then
          echo "- **$skill_name** ($score/120) - Requires immediate attention" >> "$AUDIT_DIR/analysis.md"
        fi
      done
    echo "" >> "$AUDIT_DIR/analysis.md"
  fi

  if [ "$C_COUNT" -gt 0 ]; then
    cat >> "$AUDIT_DIR/analysis.md" <<EOF
### High Priority (C Grade Skills)

EOF
    sort -t '|' -k1,1nr "$TMP_DIR/rows.txt" |
      while IFS='|' read -r score skill_name grade lines _refs; do
        if [ "$grade" = "C" ] || [ "$grade" = "C+" ]; then
          if [ "$lines" -gt 300 ]; then
            echo "- **$skill_name** ($lines lines) - Consider applying Navigation Hub pattern" >> "$AUDIT_DIR/analysis.md"
          else
            echo "- **$skill_name** ($score/120) - Review description and anti-patterns" >> "$AUDIT_DIR/analysis.md"
          fi
        fi
      done
    echo "" >> "$AUDIT_DIR/analysis.md"
  fi

  if [ "$SKILL_COUNT" -gt 20 ]; then
    cat >> "$AUDIT_DIR/analysis.md" <<EOF
### Consider Aggregation

With $SKILL_COUNT total skills, consider running duplication detection:

\`\`\`bash
sh skills/skill-quality-auditor/scripts/detect-duplication.sh
\`\`\`

EOF
  fi

  cat >> "$AUDIT_DIR/analysis.md" <<EOF
---

## Next Steps

1. **Review D/F skills** - Immediate refactoring or aggregation required
2. **Improve C skills** - Focus on description and anti-pattern examples
3. **Run duplication detection** - Find consolidation opportunities

---

**Generated by:** skill-quality-auditor  
**Tool:** \`scripts/audit-skills.sh\` + \`scripts/evaluate.sh\`  
**Framework:** skill-judge (8-dimension evaluation)
EOF
}

generate_remediation_plan() {
  cat > "$AUDIT_DIR/remediation-plan.md" <<EOF
# Skill Remediation Plan

**Date:** $(date +"%B %d, %Y")  
**Audited Skills:** $SKILL_COUNT

---

## Priority Queue

EOF

  sort -t '|' -k1,1nr "$TMP_DIR/rows.txt" |
    while IFS='|' read -r score skill_name grade _lines _refs; do
      case "$grade" in
        D|F)
          echo "### $skill_name (Score: $score/120, Grade: $grade) - CRITICAL" >> "$AUDIT_DIR/remediation-plan.md"
          echo "" >> "$AUDIT_DIR/remediation-plan.md"
          echo "Run: \`sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json\` for details" >> "$AUDIT_DIR/remediation-plan.md"
          echo "" >> "$AUDIT_DIR/remediation-plan.md"
          ;;
      esac
    done

  cat >> "$AUDIT_DIR/remediation-plan.md" <<EOF
---

## Medium Priority

EOF

  sort -t '|' -k1,1nr "$TMP_DIR/rows.txt" |
    while IFS='|' read -r score skill_name grade _lines _refs; do
      case "$grade" in
        C|C+)
          echo "- **$skill_name** ($score/120) - Review D3 (anti-patterns) and D4 (description)" >> "$AUDIT_DIR/remediation-plan.md"
          ;;
      esac
    done

  cat >> "$AUDIT_DIR/remediation-plan.md" <<EOF
---

**Generated by:** skill-quality-auditor  
**Run with:** \`sh skills/skill-quality-auditor/scripts/generate-remediation-plan.sh --audit-dir $AUDIT_DIR/\`
EOF
}

echo
echo "Generating audit files..."
generate_json
generate_analysis
generate_remediation_plan

if [ "$UPDATE_LATEST" -eq 1 ]; then
  rm -f "$LATEST_DIR"
  ln -s "$AUDIT_DIR" "$LATEST_DIR" 2>/dev/null || cp -r "$AUDIT_DIR" "$LATEST_DIR"
  echo "Updated: $LATEST_DIR"
fi

echo
echo "Audit complete"
echo "Report: $AUDIT_DIR/analysis.md"
echo "Data: $AUDIT_DIR/audit.json"
echo
echo "Summary:"
echo "  Average Score: $AVG_SCORE/120"
echo "  Grade Distribution: A: $A_COUNT, B: $B_COUNT, C: $C_COUNT, D: $D_COUNT, F: $F_COUNT"
echo

if [ "$F_COUNT" -gt 0 ]; then
  exit 1
fi
exit 0
