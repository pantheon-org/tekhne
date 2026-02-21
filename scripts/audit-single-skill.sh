#!/usr/bin/env bash
# Audit a single skill and generate a quality report
# Usage: ./scripts/audit-single-skill.sh <skill-name> [--commit]
#
# Examples:
#   ./scripts/audit-single-skill.sh colyseus-multiplayer
#   ./scripts/audit-single-skill.sh colyseus-multiplayer --commit

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
AUDITS_DIR="$PROJECT_ROOT/.context/audits"
SKILLS_DIR="$PROJECT_ROOT/skills"
DATE=$(date +%Y-%m-%d)
COMMIT=false

# Parse arguments
SKILL_NAME=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --commit|-c)
      COMMIT=true
      shift
      ;;
    --help|-h)
      echo "Usage: ./scripts/audit-single-skill.sh <skill-name> [--commit]"
      echo ""
      echo "Arguments:"
      echo "  skill-name       Name of the skill to audit"
      echo "  --commit, -c     Commit the audit report after generation"
      exit 0
      ;;
    *)
      if [[ -z "$SKILL_NAME" ]]; then
        SKILL_NAME="$1"
      fi
      shift
      ;;
  esac
done

if [[ -z "$SKILL_NAME" ]]; then
  echo "Error: Skill name required"
  echo "Usage: ./scripts/audit-single-skill.sh <skill-name> [--commit]"
  exit 1
fi

SKILL_PATH="$SKILLS_DIR/$SKILL_NAME/SKILL.md"
if [[ ! -f "$SKILL_PATH" ]]; then
  echo "Error: Skill not found: $SKILL_PATH"
  exit 1
fi

mkdir -p "$AUDITS_DIR"

REPORT_FILE="$AUDITS_DIR/${SKILL_NAME}-skill-quality-audit-${DATE}.md"

echo "Auditing: $SKILL_NAME"

# Run evaluation to get scores (JSON output for parsing)
EVAL_OUTPUT=$(cd "$PROJECT_ROOT" && bun run skills/skill-quality-auditor/scripts/evaluate.ts "$SKILL_NAME" --json 2>/dev/null)

if [[ -z "$EVAL_OUTPUT" ]]; then
  echo "Error: Evaluation failed for $SKILL_NAME"
  exit 1
fi

# Parse JSON scores using grep/sed (portable, no jq dependency)
TOTAL=$(echo "$EVAL_OUTPUT" | grep -oP '"total"\s*:\s*\K[0-9]+')
MAX_TOTAL=$(echo "$EVAL_OUTPUT" | grep -oP '"maxTotal"\s*:\s*\K[0-9]+')
GRADE=$(echo "$EVAL_OUTPUT" | grep -oP '"grade"\s*:\s*"\K[^"]+')
LINES=$(echo "$EVAL_OUTPUT" | grep -oP '"lines"\s*:\s*\K[0-9]+')
REF_COUNT=$(echo "$EVAL_OUTPUT" | grep -oP '"referenceCount"\s*:\s*\K[0-9]+')

D1=$(echo "$EVAL_OUTPUT" | grep -oP '"knowledgeDelta"\s*:\s*\K[0-9]+')
D2=$(echo "$EVAL_OUTPUT" | grep -oP '"mindsetProcedures"\s*:\s*\K[0-9]+')
D3=$(echo "$EVAL_OUTPUT" | grep -oP '"antiPatternQuality"\s*:\s*\K[0-9]+')
D4=$(echo "$EVAL_OUTPUT" | grep -oP '"specificationCompliance"\s*:\s*\K[0-9]+')
D5=$(echo "$EVAL_OUTPUT" | grep -oP '"progressiveDisclosure"\s*:\s*\K[0-9]+')
D6=$(echo "$EVAL_OUTPUT" | grep -oP '"freedomCalibration"\s*:\s*\K[0-9]+')
D7=$(echo "$EVAL_OUTPUT" | grep -oP '"patternRecognition"\s*:\s*\K[0-9]+')
D8=$(echo "$EVAL_OUTPUT" | grep -oP '"practicalUsability"\s*:\s*\K[0-9]+')

PERCENT=$(echo "scale=1; $TOTAL * 100 / $MAX_TOTAL" | bc)

# Determine pattern type
if [[ "$REF_COUNT" -gt 0 ]]; then
  PATTERN="Navigation Hub"
else
  PATTERN="Monolithic"
fi

# Check for markdown issues
MD_ISSUES=""
if grep -q "^\*\*" "$SKILL_PATH" 2>/dev/null; then
  MD_ISSUES="${MD_ISSUES}"$'\n'"- MD036: Emphasis used as heading"
fi
if grep -q '```$' "$SKILL_PATH" 2>/dev/null; then
  MD_ISSUES="${MD_ISSUES}"$'\n'"- MD040: Fenced code block without language"
fi

# Count code blocks
CODE_BLOCKS=$(grep -c '```' "$SKILL_PATH" 2>/dev/null || echo "0")
CODE_BLOCKS=$((CODE_BLOCKS / 2))

# Generate assessments for each dimension
D1_ASSESSMENT="Contains skill-specific guidance"
D2_ASSESSMENT="Procedural guidance present"
D3_ASSESSMENT="Anti-pattern coverage"
D4_ASSESSMENT="Frontmatter compliant"
D5_ASSESSMENT="Structure assessment"
D6_ASSESSMENT="Constraint balance"
D7_ASSESSMENT="Trigger keywords present"
D8_ASSESSMENT="Practical examples included"

# Progressive disclosure assessment
if [[ "$REF_COUNT" -gt 0 && "$LINES" -lt 100 ]]; then
  D5_ASSESSMENT="Excellent hub + references architecture"
elif [[ "$REF_COUNT" -gt 0 ]]; then
  D5_ASSESSMENT="Has references, hub could be smaller"
elif [[ "$LINES" -gt 500 ]]; then
  D5_ASSESSMENT="Large monolithic file, needs navigation-hub refactor"
elif [[ "$LINES" -gt 300 ]]; then
  D5_ASSESSMENT="Moderate size, consider references/ partitioning"
fi

# Knowledge delta assessment based on score
if [[ "$D1" -ge 18 ]]; then
  D1_ASSESSMENT="High expert signal, minimal redundancy"
elif [[ "$D1" -ge 15 ]]; then
  D1_ASSESSMENT="Mostly expert content with some redundancy"
elif [[ "$D1" -ge 12 ]]; then
  D1_ASSESSMENT="Mixed expert/tutorial content"
else
  D1_ASSESSMENT="Significant tutorial-level redundancy"
fi

# Build critical issues section (using array for cleaner handling)
CRITICAL_ISSUES=""
ISSUE_NUM=1

# Check progressive disclosure
if [[ "$D5" -lt 10 ]]; then
  if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
  CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. Progressive disclosure needs improvement\n\n- \`skills/$SKILL_NAME/SKILL.md\` ($LINES lines)\n- **Impact**: Large single file reduces discoverability and increases maintenance cost."
  ((ISSUE_NUM++))
fi

# Check markdown issues
if [[ -n "$MD_ISSUES" ]]; then
  if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
  CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. Markdown quality-gate issues\n\n$MD_ISSUES\n\n- **Impact**: Fails lint checks, blocks clean validation."
  ((ISSUE_NUM++))
fi

# Check knowledge delta
if [[ "$D1" -lt 12 ]]; then
  if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
  CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. Low knowledge delta\n\n- \`skills/$SKILL_NAME/SKILL.md\`\n- **Impact**: Tutorial-level content dilutes expert signal."
  ((ISSUE_NUM++))
fi

if [[ -z "$CRITICAL_ISSUES" ]]; then
  CRITICAL_ISSUES="No critical issues identified."
fi

# Build recommendations (using explicit newlines for markdown)
RECOMMENDATIONS=""
REC_NUM=1

if [[ "$D5" -lt 10 ]]; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Split into navigation hub + references\n\nMove detailed sections to \`references/\` and keep SKILL.md as concise navigation."
  ((REC_NUM++))
fi

if [[ -n "$MD_ISSUES" ]]; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Fix markdown lint issues\n\nAddress MD036 and MD040 violations for clean validation."
  ((REC_NUM++))
fi

if [[ "$D1" -lt 15 ]]; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Increase knowledge delta\n\nRemove generic tutorial content and focus on expert-only patterns."
  ((REC_NUM++))
fi

if [[ -z "$RECOMMENDATIONS" ]]; then
  RECOMMENDATIONS="No critical improvements needed. Skill meets quality standards."
fi

# List skill files
FILES_INVENTORY="skills/$SKILL_NAME/
├── SKILL.md"

if [[ -d "$SKILLS_DIR/$SKILL_NAME/references" ]]; then
  REF_FILES=$(find "$SKILLS_DIR/$SKILL_NAME/references" -maxdepth 1 -name "*.md" -exec basename {} \; 2>/dev/null || true)
  for ref in $REF_FILES; do
    FILES_INVENTORY="$FILES_INVENTORY
└── references/$ref"
  done
fi

# Generate report
cat > "$REPORT_FILE" << EOF
# Skill Evaluation Report: $SKILL_NAME

**Review Date**: $DATE
**Reviewer**: automated audit
**Skill Location**: \`skills/$SKILL_NAME/SKILL.md\`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | $TOTAL/$MAX_TOTAL ($PERCENT%) |
| **Grade** | $GRADE |
| **Pattern** | $PATTERN |
| **Lines** | $LINES |
| **References** | $REF_COUNT files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | $D1 | 20 | $D1_ASSESSMENT |
| D2: Mindset + Procedures | $D2 | 15 | $D2_ASSESSMENT |
| D3: Anti-Pattern Quality | $D3 | 15 | $D3_ASSESSMENT |
| D4: Specification Compliance | $D4 | 15 | $D4_ASSESSMENT |
| D5: Progressive Disclosure | $D5 | 15 | $D5_ASSESSMENT |
| D6: Freedom Calibration | $D6 | 15 | $D6_ASSESSMENT |
| D7: Pattern Recognition | $D7 | 10 | $D7_ASSESSMENT |
| D8: Practical Usability | $D8 | 15 | $D8_ASSESSMENT |

---

## Critical Issues

$(echo -e "$CRITICAL_ISSUES")

---

## Top 3 Recommended Improvements

$(echo -e "$RECOMMENDATIONS")

---

## Files Inventory

\`\`\`text
$FILES_INVENTORY
\`\`\`

---

## Verification

\`\`\`bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts $SKILL_NAME
bunx markdownlint-cli2 "skills/$SKILL_NAME/SKILL.md"
\`\`\`

---

## Conclusion

\`$SKILL_NAME\` scores **$TOTAL/$MAX_TOTAL ($GRADE)**.
EOF

# Post-process: squeeze multiple blank lines to single (MD012)
# Use awk for portability
awk 'NF || !blank++ {print; if(NF) blank=0}' "$REPORT_FILE" > "${REPORT_FILE}.tmp" && mv "${REPORT_FILE}.tmp" "$REPORT_FILE"

echo "  Report: $REPORT_FILE"
echo "  Score: $TOTAL/$MAX_TOTAL ($GRADE)"

if [[ "$COMMIT" == true ]]; then
  cd "$PROJECT_ROOT"
  git add "$REPORT_FILE"
  git commit -m "audit: add quality audit for $SKILL_NAME skill ($GRADE, $TOTAL/$MAX_TOTAL)"
  echo "  Committed: $REPORT_FILE"
fi

echo "  Done."
