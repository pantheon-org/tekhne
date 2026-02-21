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

REPORT_FILE="$AUDITS_DIR/${SKILL_NAME}-${DATE}.md"

echo "Auditing: $SKILL_NAME"

# Run evaluation to get scores (JSON output for parsing)
EVAL_OUTPUT=""
if [[ -f "$PROJECT_ROOT/skills/skill-quality-auditor/scripts/evaluate.ts" ]] && command -v bun >/dev/null 2>&1; then
  EVAL_OUTPUT=$(cd "$PROJECT_ROOT" && bun run skills/skill-quality-auditor/scripts/evaluate.ts "$SKILL_NAME" --json 2>/dev/null || true)
fi
if [[ -z "$EVAL_OUTPUT" ]]; then
  EVAL_OUTPUT=$(cd "$PROJECT_ROOT" && sh skills/skill-quality-auditor/scripts/evaluate.sh "$SKILL_NAME" --json 2>/dev/null || true)
fi

if [[ -z "$EVAL_OUTPUT" ]]; then
  echo "Error: Evaluation failed for $SKILL_NAME"
  exit 1
fi

json_get_number() {
  local key="$1"
  echo "$EVAL_OUTPUT" | sed -n "s/.*\"$key\"[[:space:]]*:[[:space:]]*\\([0-9][0-9]*\\).*/\\1/p" | head -1
}

json_get_string() {
  local key="$1"
  echo "$EVAL_OUTPUT" | sed -n "s/.*\"$key\"[[:space:]]*:[[:space:]]*\"\\([^\"]*\\)\".*/\\1/p" | head -1
}

# Parse JSON scores using portable sed (no jq/grep -P dependency)
TOTAL=$(json_get_number "total")
MAX_TOTAL=$(json_get_number "maxTotal")
GRADE=$(json_get_string "grade")
LINES=$(json_get_number "lines")
REF_COUNT=$(json_get_number "referenceCount")

D1=$(json_get_number "knowledgeDelta")
D2=$(json_get_number "mindsetProcedures")
D3=$(json_get_number "antiPatternQuality")
D4=$(json_get_number "specificationCompliance")
D5=$(json_get_number "progressiveDisclosure")
D6=$(json_get_number "freedomCalibration")
D7=$(json_get_number "patternRecognition")
D8=$(json_get_number "practicalUsability")

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
# Detect unlabeled fenced-code openings only (not closing fences)
if awk '
  BEGIN { in_fence = 0; has_unlabeled_open = 0 }
  /^```/ {
    if (in_fence == 0) {
      if ($0 ~ /^```[[:space:]]*$/) has_unlabeled_open = 1
      in_fence = 1
    } else {
      in_fence = 0
    }
  }
  END { exit(has_unlabeled_open ? 0 : 1) }
' "$SKILL_PATH"; then
  MD_ISSUES="${MD_ISSUES}"$'\n'"- MD040: Fenced code block without language"
fi

# Count code blocks
CODE_BLOCKS=$(grep -c '```' "$SKILL_PATH" 2>/dev/null || echo "0")
CODE_BLOCKS=$((CODE_BLOCKS / 2))

# Generate assessments for each dimension
D1_ASSESSMENT="Contains skill-specific guidance"
D2_ASSESSMENT="Workflow guidance present"
D3_ASSESSMENT="Anti-pattern coverage"
D4_ASSESSMENT="Format and metadata compliance"
D5_ASSESSMENT="Structure assessment"
D6_ASSESSMENT="Constraint balance"
D7_ASSESSMENT="Trigger and pattern clarity"
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

# Mindset + procedures assessment based on score
if [[ "$D2" -ge 13 ]]; then
  D2_ASSESSMENT="Clear and reusable workflow with validation cadence"
elif [[ "$D2" -ge 10 ]]; then
  D2_ASSESSMENT="Procedural guidance present but not fully deterministic"
else
  D2_ASSESSMENT="Workflow is underspecified and hard to execute consistently"
fi

# Anti-pattern assessment based on score
if [[ "$D3" -ge 13 ]]; then
  D3_ASSESSMENT="Strong anti-pattern guidance with practical examples"
elif [[ "$D3" -ge 10 ]]; then
  D3_ASSESSMENT="Anti-patterns are present but coverage is partial"
else
  D3_ASSESSMENT="Insufficient anti-pattern coverage for safe usage"
fi

# Specification compliance assessment based on score
if [[ "$D4" -ge 13 ]]; then
  D4_ASSESSMENT="Spec metadata and structural conventions are mostly compliant"
elif [[ "$D4" -ge 10 ]]; then
  D4_ASSESSMENT="Core spec elements present with notable gaps"
else
  D4_ASSESSMENT="Spec compliance gaps likely break validation workflows"
fi

# Freedom calibration assessment based on score
if [[ "$D6" -ge 13 ]]; then
  D6_ASSESSMENT="Good balance of constraints and implementation freedom"
elif [[ "$D6" -ge 10 ]]; then
  D6_ASSESSMENT="Usable constraints but ambiguity can leak into outputs"
else
  D6_ASSESSMENT="Too loose or too rigid for reliable execution"
fi

# Pattern recognition assessment based on score
if [[ "$D7" -ge 8 ]]; then
  D7_ASSESSMENT="Pattern and trigger language are clear and actionable"
elif [[ "$D7" -ge 6 ]]; then
  D7_ASSESSMENT="Pattern intent exists but trigger language is shallow"
else
  D7_ASSESSMENT="Pattern/trigger ambiguity causes inconsistent activation"
fi

# Practical usability assessment based on score
if [[ "$D8" -ge 13 ]]; then
  D8_ASSESSMENT="Directly usable with strong examples and verification steps"
elif [[ "$D8" -ge 10 ]]; then
  D8_ASSESSMENT="Usable in practice but examples or checks need expansion"
else
  D8_ASSESSMENT="Limited real-world usability without extra interpretation"
fi

# Skill-specific deep issue detection helpers
find_line_exact() {
  local text="$1"
  awk -v target="$text" 'index($0, target) { print NR; exit }' "$SKILL_PATH"
}

find_line_regex() {
  local pattern="$1"
  awk -v pat="$pattern" '$0 ~ pat { print NR; exit }' "$SKILL_PATH"
}

HAS_HYBRID_OUTPUT=0
HAS_INPUT_FALLBACK=0
HAS_AMBIGUOUS_STEPS=0

LINE_MIXED_QUESTION=$(find_line_exact "4. Mixed behavior and constraints?")
LINE_MIXED_USAGE=$(find_line_exact "- Use both formats in one story")
LINE_OUTPUT_FORMAT=$(find_line_regex "^## Output Format")
LINE_HYBRID_OUTPUT=$(find_line_regex "Acceptance Criteria [(]Hybrid[)]")

if [[ -n "$LINE_MIXED_QUESTION" && -n "$LINE_MIXED_USAGE" && -n "$LINE_OUTPUT_FORMAT" && -z "$LINE_HYBRID_OUTPUT" ]]; then
  HAS_HYBRID_OUTPUT=1
fi

LINE_INPUTS_REQUIRED=$(find_line_regex "^## Inputs Required")
LINE_USER_STORY_REQUIRED=$(find_line_regex "User story in the format")
LINE_INPUT_FALLBACK=$(find_line_regex "If a full user story is unavailable|fallback|Draft pending stakeholder confirmation")

if [[ -n "$LINE_INPUTS_REQUIRED" && -n "$LINE_USER_STORY_REQUIRED" && -z "$LINE_INPUT_FALLBACK" ]]; then
  HAS_INPUT_FALLBACK=1
fi

LINE_AMBIGUOUS_STEPS=$(find_line_regex "<= 3 steps")
if [[ -n "$LINE_AMBIGUOUS_STEPS" ]]; then
  HAS_AMBIGUOUS_STEPS=1
fi

# Build critical issues section (using array for cleaner handling)
CRITICAL_ISSUES=""
ISSUE_NUM=1

# Skill-specific critical issues first (higher value than score-only generic checks)
if [[ "$HAS_HYBRID_OUTPUT" -eq 1 ]]; then
  if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
  CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. Hybrid output ambiguity\n\nThe decision tree allows mixed usage of Given/When/Then and rule-oriented criteria, but the output section defines only one deterministic structure.\n\n- \`skills/$SKILL_NAME/SKILL.md:$LINE_MIXED_QUESTION\`\n- \`skills/$SKILL_NAME/SKILL.md:$LINE_OUTPUT_FORMAT\`\n- **Impact**: Inconsistent output formatting across agents when handling mixed stories."
  ((ISSUE_NUM++))
fi

if [[ "$HAS_INPUT_FALLBACK" -eq 1 ]]; then
  if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
  CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. No fallback when required inputs are incomplete\n\nInputs are listed as required (including full user story format), but no fallback is defined for partial briefs or bug reports.\n\n- \`skills/$SKILL_NAME/SKILL.md:$LINE_INPUTS_REQUIRED\`\n- \`skills/$SKILL_NAME/SKILL.md:$LINE_USER_STORY_REQUIRED\`\n- **Impact**: Workflow can stall when upstream artifacts are incomplete."
  ((ISSUE_NUM++))
fi

if [[ "$HAS_AMBIGUOUS_STEPS" -eq 1 ]]; then
  if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
  CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. One testability example is interpretation-sensitive\n\nThe phrase \"<= 3 steps\" can be interpreted differently unless step boundaries are explicitly defined.\n\n- \`skills/$SKILL_NAME/SKILL.md:$LINE_AMBIGUOUS_STEPS\`\n- **Impact**: QA and product can disagree on pass/fail for the same criterion."
  ((ISSUE_NUM++))
fi

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

# Check knowledge delta (only if no skill-specific critical issues were found)
if [[ "$D1" -lt 12 ]]; then
  if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
  CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. Low knowledge delta\n\n- \`skills/$SKILL_NAME/SKILL.md\`\n- **Impact**: Tutorial-level content dilutes expert signal."
  ((ISSUE_NUM++))
fi

# Check pattern recognition (avoid duplicating skill-specific findings)
if [[ "$D7" -lt 8 ]]; then
  if [[ "$HAS_HYBRID_OUTPUT" -eq 0 ]]; then
    if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
    CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. Pattern trigger clarity is below target\n\n- \`skills/$SKILL_NAME/SKILL.md\`\n- **Evidence**: D7 score is $D7/10\n- **Impact**: Agents may activate late or choose inconsistent output shapes."
    ((ISSUE_NUM++))
  fi
fi

# Check practical usability (avoid duplicating skill-specific findings)
if [[ "$D8" -lt 12 ]]; then
  if [[ "$HAS_INPUT_FALLBACK" -eq 0 && "$HAS_AMBIGUOUS_STEPS" -eq 0 ]]; then
    if [[ -n "$CRITICAL_ISSUES" ]]; then CRITICAL_ISSUES="$CRITICAL_ISSUES\n\n"; fi
    CRITICAL_ISSUES="${CRITICAL_ISSUES}### $ISSUE_NUM. Practical usability needs stronger execution support\n\n- \`skills/$SKILL_NAME/SKILL.md\`\n- **Evidence**: D8 score is $D8/15\n- **Impact**: Users and agents require extra interpretation to apply the skill reliably."
    ((ISSUE_NUM++))
  fi
fi

if [[ -z "$CRITICAL_ISSUES" ]]; then
  CRITICAL_ISSUES="No critical issues identified."
fi

# Build recommendations (using explicit newlines for markdown)
RECOMMENDATIONS=""
REC_NUM=1
can_add_recommendation() {
  [[ "$REC_NUM" -le 3 ]]
}

if [[ "$HAS_HYBRID_OUTPUT" -eq 1 ]] && can_add_recommendation; then
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Add explicit hybrid output template\n\nAdd a section under Output Format for mixed stories that combines scenario flow and independent rules.\n\n\`\`\`markdown\nAcceptance Criteria (Hybrid):\n1. Scenario-Based Criteria (Given/When/Then)\n2. Rule-Based Criteria (independent constraints)\n3. Negative/Edge Scenarios\n4. Out of Scope (Won't Have)\n\`\`\`"
  ((REC_NUM++))
fi

if [[ "$HAS_INPUT_FALLBACK" -eq 1 ]] && can_add_recommendation; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Define fallback behavior for missing inputs\n\nAdd guidance for when only partial context is available.\n\n\`\`\`markdown\nIf a full user story is unavailable:\n- Draft: As a [role], I want [action], so that [benefit]\n- List assumptions explicitly\n- Flag AC set as \"Draft pending stakeholder confirmation\"\n\`\`\`"
  ((REC_NUM++))
fi

if [[ "$HAS_AMBIGUOUS_STEPS" -eq 1 ]] && can_add_recommendation; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Tighten ambiguous testability example\n\nReplace \"<= 3 steps\" with an explicit step boundary definition.\n\n\`\`\`markdown\n\"Checkout is completed in <= 3 user-visible screen transitions from cart to confirmation.\"\n\`\`\`"
  ((REC_NUM++))
fi

if [[ "$D5" -lt 10 ]] && can_add_recommendation; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Split into navigation hub + references\n\nMove detailed sections to \`references/\` and keep SKILL.md as concise navigation."
  ((REC_NUM++))
fi

if [[ -n "$MD_ISSUES" ]] && can_add_recommendation; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Fix markdown lint issues\n\nAddress markdownlint violations for clean validation."
  ((REC_NUM++))
fi

if [[ "$D1" -lt 15 ]] && can_add_recommendation; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Increase knowledge delta\n\nRemove generic tutorial content and focus on expert-only patterns."
  ((REC_NUM++))
fi

if [[ "$D7" -lt 8 ]] && can_add_recommendation; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Tighten trigger language and pattern cues\n\nAdd explicit trigger phrases and deterministic output structure for ambiguous request types."
  ((REC_NUM++))
fi

if [[ "$D8" -lt 12 ]] && can_add_recommendation; then
  if [[ -n "$RECOMMENDATIONS" ]]; then RECOMMENDATIONS="$RECOMMENDATIONS\n\n"; fi
  RECOMMENDATIONS="${RECOMMENDATIONS}### Priority $REC_NUM: Expand applied examples and verification checklist\n\nAdd at least one concrete end-to-end example and explicit pass/fail checks tied to expected outputs."
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
---
review_date: $DATE
reviewer: automated audit
skill_location: \`skills/$SKILL_NAME/SKILL.md\`
---

# Skill Evaluation Report: $SKILL_NAME

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

## Detailed Dimension Analysis

### D1: Knowledge Delta ($D1/20)

**Assessment**: $D1_ASSESSMENT

**Strengths**:

- Skill has domain-specific direction beyond generic writing guidance.

**Weaknesses**:

- Scores below 18 usually indicate overlap that can be collapsed into references.

### D2: Mindset + Procedures ($D2/15)

**Assessment**: $D2_ASSESSMENT

**Strengths**:

- Workflow sequence is documented and executable.

**Weaknesses**:

- Lower scores indicate missing fallback behavior or weak decision points.

### D3: Anti-Pattern Quality ($D3/15)

**Assessment**: $D3_ASSESSMENT

**Strengths**:

- Anti-pattern section gives guardrails for common failure modes.

**Weaknesses**:

- Lower scores indicate missing "why this fails" and corrected examples.

### D4: Specification Compliance ($D4/15)

**Assessment**: $D4_ASSESSMENT

| Requirement | Status | Notes |
| --- | --- | --- |
| Valid frontmatter | PASS | Metadata keys detected |
| Trigger-oriented description | PASS | Description is present in skill frontmatter |
| Structural consistency | PASS | Headings and sections are parseable |

### D5: Progressive Disclosure ($D5/15)

**Assessment**: $D5_ASSESSMENT

**Strengths**:

- Reference-based structure exists when references/ is present.

**Weaknesses**:

- Lower scores indicate hub is too large or references are underused.

### D6: Freedom Calibration ($D6/15)

**Assessment**: $D6_ASSESSMENT

**Strengths**:

- Constraints are present to shape output quality.

**Weaknesses**:

- Lower scores suggest over-permissive guidance or over-constraining rules.

### D7: Pattern Recognition ($D7/10)

**Assessment**: $D7_ASSESSMENT

**Strengths**:

- Trigger keywords exist and pattern intent is identifiable.

**Weaknesses**:

- Lower scores imply ambiguous activation cues and mixed-output risk.

### D8: Practical Usability ($D8/15)

**Assessment**: $D8_ASSESSMENT

**Strengths**:

- Includes actionable examples or verification commands.

**Weaknesses**:

- Lower scores indicate insufficient end-to-end examples and testable checks.

---

## Proposed Restructured SKILL.md

\`\`\`\`markdown
## Output Format

Produce acceptance criteria in this structure:

\`\`\`markdown
User Story:
As a [role], I want [action], so that [benefit].

Acceptance Criteria (Must Have):
1. ...
2. ...

Negative/Edge Scenarios:
1. ...
2. ...

Out of Scope (Won't Have):
- ...
\`\`\`

For mixed behavior and constraints, use Hybrid:

\`\`\`markdown
Acceptance Criteria (Hybrid):
1. Scenario-Based Criteria (Given/When/Then)
2. Rule-Based Criteria (independent measurable constraints)

Negative/Edge Scenarios:
1. ...
2. ...

Out of Scope (Won't Have):
- ...
\`\`\`

If inputs are incomplete, produce a Draft section with explicit assumptions.
\`\`\`\`

---

## Action Items

| Priority | Action | Effort |
| --- | --- | --- |
| 1 | Address all critical issues listed above | 15-30 min |
| 2 | Apply top 3 recommended improvements in order | 20-45 min |
| 3 | Re-run evaluation and markdown validation | 5-10 min |

---

## Files Inventory

\`\`\`text
$FILES_INVENTORY
\`\`\`

---

## Verification

\`\`\`bash
sh skills/skill-quality-auditor/scripts/evaluate.sh $SKILL_NAME --json
bunx markdownlint-cli2 "skills/$SKILL_NAME/SKILL.md"
\`\`\`

---

## Conclusion

\`$SKILL_NAME\` scores **$TOTAL/$MAX_TOTAL ($GRADE)**.
EOF

# Post-process: squeeze multiple blank lines to single (MD012)
# Use awk for portability
awk 'NF || !blank++ {print; if(NF) blank=0}' "$REPORT_FILE" > "${REPORT_FILE}.tmp" && mv "${REPORT_FILE}.tmp" "$REPORT_FILE"

# Validate generated report format
(
  cd "$PROJECT_ROOT"
  ./skills/skill-quality-auditor/scripts/validate-review-format.sh "$REPORT_FILE"
)

echo "  Report: $REPORT_FILE"
echo "  Score: $TOTAL/$MAX_TOTAL ($GRADE)"

if [[ "$COMMIT" == true ]]; then
  cd "$PROJECT_ROOT"
  git add "$REPORT_FILE"
  git commit -m "audit: add quality audit for $SKILL_NAME skill ($GRADE, $TOTAL/$MAX_TOTAL)"
  echo "  Committed: $REPORT_FILE"
fi

echo "  Done."
