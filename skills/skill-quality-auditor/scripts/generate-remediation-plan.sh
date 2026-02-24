#!/usr/bin/env sh
# Generate a remediation plan from audit results
# Usage: ./generate-remediation-plan.sh [--audit-dir <path>] [skill-name] [target-score]

set -e

AUDIT_DIR=""
SKILL_NAME=""
TARGET_SCORE="102"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --audit-dir)
      if [ "$#" -ge 2 ]; then
        AUDIT_DIR="$2"
        shift 2
      else
        echo "Error: --audit-dir requires a path" >&2
        exit 1
      fi
      ;;
    --help|-h)
      echo "Usage: $0 [--audit-dir <path>] [skill-name] [target-score]"
      echo "Example: $0 --audit-dir .context/audits/skill-audit/latest/ typescript-advanced 108"
      exit 0
      ;;
    -*)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
    *)
      if [ -z "$SKILL_NAME" ]; then
        SKILL_NAME="$1"
        shift
      elif [ -z "$TARGET_SCORE" ] || [ "$TARGET_SCORE" = "102" ]; then
        TARGET_SCORE="$1"
        shift
      else
        shift
      fi
      ;;
  esac
done

if [ -z "$AUDIT_DIR" ]; then
  if [ -L ".context/audits/skill-audit/latest" ]; then
    AUDIT_DIR=".context/audits/skill-audit/latest"
  elif [ -d ".context/audits/skill-audit/latest" ]; then
    AUDIT_DIR=".context/audits/skill-audit/latest"
  else
    AUDIT_DIR=".context/audits/skill-audit/$(find .context/audits/skill-audit/ -mindepth 1 -maxdepth 1 -type d -not -name 'latest' -printf '%f\n' 2>/dev/null | sort -r | head -1)"
  fi
fi

if [ ! -d "$AUDIT_DIR" ]; then
  echo "Error: Audit directory not found: $AUDIT_DIR"
  echo "Run: sh skills/skill-quality-auditor/scripts/audit-skills.sh"
  exit 1
fi

AUDIT_JSON="$AUDIT_DIR/audit.json"

if [ ! -f "$AUDIT_JSON" ]; then
  echo "Error: Audit JSON not found: $AUDIT_JSON"
  echo "Run: sh skills/skill-quality-auditor/scripts/audit-skills.sh"
  exit 1
fi

if [ -n "$SKILL_NAME" ]; then
  PLAN_FILE=".context/plans/${SKILL_NAME}-remediation-plan.md"
else
  PLAN_FILE="$AUDIT_DIR/remediation-plan.md"
fi

SKILL_DATA=$(grep -o "\"skill\": *\"$SKILL_NAME\"[^}]*}" "$AUDIT_JSON" | head -1)

if [ -z "$SKILL_DATA" ]; then
  echo "Error: Skill '$SKILL_NAME' not found in audit"
  exit 1
fi

SCORE=$(printf '%s' "$SKILL_DATA" | sed -n 's/.*"total": *\([0-9]*\).*/\1/p')
GRADE=$(printf '%s' "$SKILL_DATA" | sed -n 's/.*"grade": *"\([^"]*\)".*/\1/p')

PERCENT=$((SCORE * 100 / 120))
TARGET_PERCENT=$((TARGET_SCORE * 100 / 120))

case "$TARGET_SCORE" in
  114) TARGET_GRADE="A+" ;;
  108) TARGET_GRADE="A" ;;
  102) TARGET_GRADE="B+" ;;
  96) TARGET_GRADE="B" ;;
  90) TARGET_GRADE="C+" ;;
  84) TARGET_GRADE="C" ;;
  *) TARGET_GRADE="B" ;;
esac

cat > "$PLAN_FILE" << EOF
---
plan_date: $(date +%Y-%m-%d)
skill_name: $SKILL_NAME
source_audit: $AUDIT_JSON
---

# Remediation Plan: $SKILL_NAME

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | $SCORE/120 ($PERCENT%) | $TARGET_SCORE/120 ($TARGET_PERCENT%) |
| **Grade** | $GRADE | $TARGET_GRADE |
| **Priority** | [PRIORITY] | - |
| **Effort** | [EFFORT] | - |

**Focus Areas**: [Dimension 1], [Dimension 2], [Dimension 3]

**Verdict**: [Assessment of current state]

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| [Issue 1] | [D#]([score]/[max]) | [Critical/High/Medium/Low] | [Impact] |
| [Issue 2] | [D#]([score]/[max]) | [Critical/High/Medium/Low] | [Impact] |
| [Issue 3] | [D#]([score]/[max]) | [Critical/High/Medium/Low] | [Impact] |

## Detailed Remediation Steps

### Phase 1: [Dimension] - Priority: [Priority]

**Target**: Increase from [current]/[max] to [target]/[max] (+[delta] points)

#### Step 1.1: [Step title]

**[File to modify]**: \`skills/$SKILL_NAME/SKILL.md\`

[Action description]

---

### Phase 2: [Dimension] - Priority: [Priority]

**Target**: Increase from [current]/[max] to [target]/[max] (+[delta] points)

#### Step 2.1: [Step title]

[Action description]

---

## Verification Commands

\`\`\`bash
sh skills/skill-quality-auditor/scripts/evaluate.sh $SKILL_NAME --json
bunx markdownlint-cli2 "skills/$SKILL_NAME/**/*.md"
\`\`\`

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| [Dimension] Score | Score >= [target]/[max] |
| Overall Score | >= $TARGET_SCORE/120 ($TARGET_GRADE) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1 | [S/M/L] | [time] |
| Phase 2 | [S/M/L] | [time] |
| **Total** | **[S/M/L]** | **[total]** |

## Dependencies

- None (standalone skill)

## Rollback Plan

\`\`\`bash
git checkout HEAD~1 -- skills/$SKILL_NAME/SKILL.md
\`\`\`

## Notes

- Rating: **X/10** - [Assessment]
EOF

echo "Created: $PLAN_FILE"
echo "Edit the plan to fill in the [bracketed] values from the audit."
echo "Run: sh skills/skill-quality-auditor/scripts/evaluate.sh $SKILL_NAME --json"
