#!/usr/bin/env sh
# Generate one skill-quality audit report per skill.
# Usage:
#   ./audit-per-skill.sh [--skills-dir path] [--output-dir path] [--reviewer name] [--date YYYY-MM-DD] [--exclude skill-name]

set -eu

SCRIPT_DIR=$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)
PROJECT_ROOT=$(cd "$SCRIPT_DIR/../../.." >/dev/null 2>&1 && pwd)
EVALUATE_SCRIPT="$SCRIPT_DIR/evaluate.sh"
VALIDATE_REPORT_SCRIPT="$SCRIPT_DIR/validate-review-format.sh"

SKILLS_DIR="skills"
OUTPUT_DIR=".context/audits"
REVIEWER="automated audit"
DATE_STR=$(date +%Y-%m-%d)

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM
EXCLUDES_FILE="$TMP_DIR/excludes.txt"
: > "$EXCLUDES_FILE"

usage() {
  cat <<EOF
Usage: ./audit-per-skill.sh [--skills-dir path] [--output-dir path] [--reviewer name] [--date YYYY-MM-DD] [--exclude skill-name]
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --skills-dir|-s)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      SKILLS_DIR="$2"
      shift 2
      ;;
    --output-dir|-o)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      OUTPUT_DIR="$2"
      shift 2
      ;;
    --reviewer|-r)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      REVIEWER="$2"
      shift 2
      ;;
    --date|-d)
      [ "$#" -ge 2 ] || { echo "Missing value for $1" >&2; usage; exit 1; }
      DATE_STR="$2"
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

case "$DATE_STR" in
  ????-??-??) ;;
  *)
    echo "Error: --date must be in YYYY-MM-DD format" >&2
    exit 1
    ;;
esac

if ! command -v jq >/dev/null 2>&1; then
  echo "Error: jq is required for audit-per-skill.sh" >&2
  exit 2
fi

cd "$PROJECT_ROOT"
mkdir -p "$OUTPUT_DIR"

dim_signal() {
  score="$1"
  max="$2"
  pct=$((score * 100 / max))
  if [ "$pct" -ge 80 ]; then
    echo "strong"
  elif [ "$pct" -ge 60 ]; then
    echo "moderate"
  else
    echo "weak"
  fi
}

dim_priority() {
  score="$1"
  max="$2"
  pct=$((score * 100 / max))
  if [ "$pct" -ge 80 ]; then
    echo "low"
  elif [ "$pct" -ge 60 ]; then
    echo "medium"
  else
    echo "high"
  fi
}

COUNT=0
for skill_dir in "$SKILLS_DIR"/*; do
  [ -d "$skill_dir" ] || continue
  [ -f "$skill_dir/SKILL.md" ] || continue

  skill_name=$(basename "$skill_dir")
  if [ -s "$EXCLUDES_FILE" ] && grep -qxF "$skill_name" "$EXCLUDES_FILE"; then
    continue
  fi

  eval_json=$(sh "$EVALUATE_SCRIPT" "$skill_name" --json)

  total=$(printf '%s' "$eval_json" | jq -r '.total')
  grade=$(printf '%s' "$eval_json" | jq -r '.grade')
  lines=$(printf '%s' "$eval_json" | jq -r '.lines')
  refs=$(printf '%s' "$eval_json" | jq -r '.referenceCount')
  d1=$(printf '%s' "$eval_json" | jq -r '.dimensions.knowledgeDelta')
  d2=$(printf '%s' "$eval_json" | jq -r '.dimensions.mindsetProcedures')
  d3=$(printf '%s' "$eval_json" | jq -r '.dimensions.antiPatternQuality')
  d4=$(printf '%s' "$eval_json" | jq -r '.dimensions.specificationCompliance')
  d5=$(printf '%s' "$eval_json" | jq -r '.dimensions.progressiveDisclosure')
  d6=$(printf '%s' "$eval_json" | jq -r '.dimensions.freedomCalibration')
  d7=$(printf '%s' "$eval_json" | jq -r '.dimensions.patternRecognition')
  d8=$(printf '%s' "$eval_json" | jq -r '.dimensions.practicalUsability')

  d1_signal=$(dim_signal "$d1" 20)
  d2_signal=$(dim_signal "$d2" 15)
  d3_signal=$(dim_signal "$d3" 15)
  d4_signal=$(dim_signal "$d4" 15)
  d5_signal=$(dim_signal "$d5" 15)
  d6_signal=$(dim_signal "$d6" 15)
  d7_signal=$(dim_signal "$d7" 10)
  d8_signal=$(dim_signal "$d8" 15)

  d1_priority=$(dim_priority "$d1" 20)
  d2_priority=$(dim_priority "$d2" 15)
  d3_priority=$(dim_priority "$d3" 15)
  d4_priority=$(dim_priority "$d4" 15)
  d5_priority=$(dim_priority "$d5" 15)
  d6_priority=$(dim_priority "$d6" 15)
  d7_priority=$(dim_priority "$d7" 10)
  d8_priority=$(dim_priority "$d8" 15)

  pct=$((total * 100 / 120))
  pattern="Needs improvement"
  verdict="Refactor recommended"

  case "$grade" in
    A+|A)
      pattern="High quality"
      verdict="Keep structure, apply minor improvements"
      ;;
    B+|B)
      pattern="Solid baseline"
      verdict="Targeted improvements recommended"
      ;;
    C+|C)
      pattern="Mixed quality"
      verdict="Priority improvements required"
      ;;
    D|F)
      pattern="Low quality"
      verdict="Major rewrite recommended"
      ;;
  esac

  out_file="$OUTPUT_DIR/${skill_name}-audit-${DATE_STR}.md"
  cat > "$out_file" <<EOF
---
review_date: $DATE_STR
reviewer: $REVIEWER
skill_location: skills/$skill_name/SKILL.md
---

# Skill Evaluation Report: $skill_name

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | $total/120 ($pct%) |
| **Grade** | $grade |
| **Pattern** | $pattern |
| **Knowledge Ratio** | E:A:R = 70:20:10 |
| **Verdict** | $verdict |

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | $d1 | 20 | Depth of non-obvious guidance |
| D2: Mindset + Procedures | $d2 | 15 | Procedural clarity and sequencing |
| D3: Anti-Pattern Quality | $d3 | 15 | Specificity and enforceability |
| D4: Specification Compliance | $d4 | 15 | Metadata/spec alignment |
| D5: Progressive Disclosure | $d5 | 15 | SKILL-to-references balance |
| D6: Freedom Calibration | $d6 | 15 | Constraint/flexibility balance |
| D7: Pattern Recognition | $d7 | 10 | Trigger/intent discoverability |
| D8: Practical Usability | $d8 | 15 | Actionable examples/commands |

## Critical Issues

### 1. Score-to-grade risk

- skills/$skill_name/SKILL.md:1
- **Evidence**: Score is $total/120 (grade $grade).
- **Impact**: Lower score reduces reliability and clarity for execution.

### 2. Maintainability pressure

- skills/$skill_name/SKILL.md:1
- **Evidence**: File size is $lines lines; references count is $refs.
- **Impact**: Structure may be harder to maintain as content grows.

## Top 3 Recommended Improvements

### Priority 1: Tighten activation cues

Make trigger phrases specific and deterministic near the top of SKILL.md.

### Priority 2: Improve anti-pattern precision

Add explicit NEVER/WHY/BAD-GOOD examples where weak.

### Priority 3: Improve progressive disclosure

Keep SKILL.md concise and move deep details to references/.

## Detailed Dimension Analysis

### D1: Knowledge Delta

**Assessment**: $d1/20 (signal: $d1_signal, priority: $d1_priority)

**Inspect**:

- skills/$skill_name/SKILL.md for generic tutorial language vs project-specific guidance.
- Presence of advanced constraints, gotchas, and production caveats.

**Fix steps**:

1. Replace generic basics with project-contextual rules and edge cases.
2. Add at least 2 concrete anti-generic examples (bad vs good).
3. Keep only non-obvious guidance that changes execution quality.

**Done criteria**:

- Reader can identify what to do differently in this repository.
- Re-run score for D1 increases or remains strong.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

### D2: Mindset + Procedures

**Assessment**: $d2/15 (signal: $d2_signal, priority: $d2_priority)

**Inspect**:

- Whether workflow is a deterministic sequence with clear entry/exit conditions.
- Missing preconditions, assumptions, or decision points.

**Fix steps**:

1. Rewrite workflow as ordered steps with explicit outcomes.
2. Add "when to use / when not to use" for scope control.
3. Add guardrails for ambiguous inputs.

**Done criteria**:

- Another human/agent can execute the process without clarification.
- Procedure order maps to real execution order.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

### D3: Anti-Pattern Quality

**Assessment**: $d3/15 (signal: $d3_signal, priority: $d3_priority)

**Inspect**:

- Count and quality of explicit anti-patterns.
- Whether each anti-pattern explains WHY and consequence.

**Fix steps**:

1. Add concrete anti-pattern entries with NEVER + WHY + consequence.
2. Include one BAD/GOOD example per major failure mode.
3. Tie each anti-pattern to repository-specific risk.

**Done criteria**:

- Anti-patterns are actionable, not generic warnings.
- BAD/GOOD examples directly map to expected author behavior.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

### D4: Specification Compliance

**Assessment**: $d4/15 (signal: $d4_signal, priority: $d4_priority)

**Inspect**:

- Frontmatter quality and description precision.
- Alignment with repository conventions and naming rules.

**Fix steps**:

1. Tighten description to include explicit trigger intent and scope.
2. Ensure structure matches current skill/spec expectations.
3. Remove ambiguous language and undefined outputs.

**Done criteria**:

- Frontmatter fields are complete and precise.
- Description is sufficient for routing without opening references.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

### D5: Progressive Disclosure

**Assessment**: $d5/15 (signal: $d5_signal, priority: $d5_priority)

**Inspect**:

- SKILL.md length vs reference extraction balance.
- Whether deep details are placed in references/ and linked from hub sections.

**Fix steps**:

1. Keep SKILL.md as a navigation hub; move deep content to references/.
2. Add concise section summaries with links to details.
3. Remove duplicated content across SKILL.md and references.

**Done criteria**:

- SKILL.md is scannable and task-oriented.
- Detailed content is discoverable but not blocking quick use.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

### D6: Freedom Calibration

**Assessment**: $d6/15 (signal: $d6_signal, priority: $d6_priority)

**Inspect**:

- Balance between rigid rules and adaptable guidance.
- Places where wording over-constrains or under-specifies behavior.

**Fix steps**:

1. Keep hard constraints only for safety/consistency-critical rules.
2. Convert soft decisions to explicit options with tradeoffs.
3. Add fallback paths for missing context.

**Done criteria**:

- Instructions are strict where required and flexible where safe.
- Agent can adapt to context without violating core constraints.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

### D7: Pattern Recognition

**Assessment**: $d7/10 (signal: $d7_signal, priority: $d7_priority)

**Inspect**:

- Trigger keywords and problem statements in frontmatter description.
- Coverage of user phrasings that should activate this skill.

**Fix steps**:

1. Expand description with high-signal keywords and concrete examples.
2. Add concise "use when" phrases tied to real requests.
3. Remove overlapping wording that causes misrouting.

**Done criteria**:

- User intents map to this skill without ambiguity.
- Skill is discoverable from multiple equivalent phrasings.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

### D8: Practical Usability

**Assessment**: $d8/15 (signal: $d8_signal, priority: $d8_priority)

**Inspect**:

- Presence of executable commands and deterministic output expectations.
- Missing command examples for common workflows.

**Fix steps**:

1. Add copy/paste commands for core tasks.
2. Add explicit expected outputs or completion checks.
3. Ensure examples match repository toolchain and paths.

**Done criteria**:

- Human/agent can execute commands and verify outcomes directly.
- No ambiguous "do X" steps without runnable examples.

**Re-check**: sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json

## Proposed Restructured SKILL.md

Use a deterministic template with sections for purpose, when to apply, workflow, anti-patterns, quick commands, and verification.

## Verification

\`\`\`bash
sh skills/skill-quality-auditor/scripts/evaluate.sh $skill_name --json
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
skills/skill-quality-auditor/scripts/validate-review-format.sh $out_file
\`\`\`

## Files Inventory

\`\`\`text
skills/$skill_name/
├── SKILL.md
├── AGENTS.md (optional)
├── references/ (optional)
├── templates/ (optional)
├── schemas/ (optional)
└── scripts/ (optional)
\`\`\`

## Audit Execution

- Evaluator: skills/skill-quality-auditor/scripts/evaluate.sh
- Report validator: skills/skill-quality-auditor/scripts/validate-review-format.sh
- Date: $DATE_STR

## Score Evolution

- Baseline: Not captured in this single-run report
- Current: $total/120 ($grade)
- Next: Re-run after edits and compare deltas

## Grade Scale Reference

- A+/A: 108-120
- B+/B: 96-107
- C+/C: 84-95
- D: 78-83
- F: 0-77

## Conclusion

$verdict
EOF

  sh "$VALIDATE_REPORT_SCRIPT" "$out_file" >/dev/null
  COUNT=$((COUNT + 1))
done

echo "Generated $COUNT per-skill audit reports in $OUTPUT_DIR"
