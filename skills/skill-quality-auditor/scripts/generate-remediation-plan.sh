#!/usr/bin/env sh
# Generate a remediation plan from audit results
# Usage: ./generate-remediation-plan.sh <skill-name> [target-score]

set -e

SKILL_NAME="${1:-}"
TARGET_SCORE="${2:-102}"

if [ -z "$SKILL_NAME" ]; then
    echo "Usage: $0 <skill-name> [target-score]"
    echo "Example: $0 typescript-advanced 108"
    exit 1
fi

AUDIT_FILE=".context/audits/${SKILL_NAME}-audit-2026-02-22.md"

if [ ! -f "$AUDIT_FILE" ]; then
    echo "Error: Audit file not found: $AUDIT_FILE"
    echo "Run: sh skills/skill-quality-auditor/scripts/evaluate.sh $SKILL_NAME"
    exit 1
fi

PLAN_FILE=".context/plans/${SKILL_NAME}-remediation-plan.md"

cat > "$PLAN_FILE" << EOF
---
plan_date: $(date +%Y-%m-%d)
skill_name: $SKILL_NAME
source_audit: $AUDIT_FILE
---

# Remediation Plan: $SKILL_NAME

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | [SCORE]/120 ([PERCENT]%) | $TARGET_SCORE/120 ([TARGET_PERCENT]%) |
| **Grade** | [GRADE] | [TARGET_GRADE] |
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
| Overall Score | >= $TARGET_SCORE/120 ([TARGET_GRADE]) |

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
