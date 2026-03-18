#!/usr/bin/env sh
# Evaluate a single skill using the skill-judge framework
# Usage: sh evaluate.sh <skill-name> [--json] [--output <path>] [--store]

set -e

SKILL_NAME=""
JSON_OUTPUT=0
OUTPUT_PATH=""
STORE_MODE=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --json) JSON_OUTPUT=1 ;;
    --store) STORE_MODE=1 ;;
    --output)
      if [ "$#" -ge 2 ]; then
        OUTPUT_PATH="$2"
        shift
      fi
      ;;
    -*) ;;
    *)
      if [ -z "$SKILL_NAME" ]; then
        SKILL_NAME="$1"
      fi
      ;;
  esac
  shift
done

if [ -z "$SKILL_NAME" ]; then
  echo "Usage: sh evaluate.sh <skill-name> [--json] [--output <path>] [--store]" >&2
  exit 1
fi

if [ "$STORE_MODE" -eq 1 ]; then
  AUDIT_DATE=$(date +%Y-%m-%d)
  AUDIT_DIR=".context/audits/${SKILL_NAME}/${AUDIT_DATE}"
  OUTPUT_PATH="${AUDIT_DIR}/audit.json"
fi

find_skill_path() {
  for path in "skills/$1/SKILL.md" "skills/$1/SKILL.md"; do
    if [ -f "$path" ]; then
      echo "$path"
      return 0
    fi
  done
  return 1
}

SKILL_PATH=$(find_skill_path "$SKILL_NAME")
if [ -z "$SKILL_PATH" ]; then
  echo "Skill not found: $SKILL_NAME" >&2
  exit 1
fi

CONTENT=$(cat "$SKILL_PATH")
LINES=$(echo "$CONTENT" | wc -l | tr -d ' ')

SKILL_DIR=$(dirname "$SKILL_PATH")
REFS_PATH="$SKILL_DIR/references"
HAS_REFS=0
REF_COUNT=0

if [ -d "$REFS_PATH" ]; then
  REF_COUNT=$(find "$REFS_PATH" -maxdepth 1 -name "*.md" ! -name ".*" 2>/dev/null | wc -l | tr -d ' ')
  if [ "$REF_COUNT" -gt 0 ]; then
    HAS_REFS=1
  fi
fi

count_matches() {
  echo "$CONTENT" | grep -oi "$1" 2>/dev/null | wc -l | tr -d ' '
}

has_pattern() {
  echo "$CONTENT" | grep -qi "$1"
}

extract_frontmatter_field() {
  echo "$CONTENT" | sed -n '/^---$/,/^---$/p' | grep "^$1:" | sed "s/^$1:[[:space:]]*//" | tr -d '"'
}

evaluate_knowledge_delta() {
  score=15
  
  for pattern in "npm install" "yarn add" "pip install" "getting started" "introduction" "basic syntax" "hello world"; do
    if has_pattern "$pattern"; then
      score=$((score - 2))
    fi
  done
  
  for pattern in "anti-pattern" "NEVER" "ALWAYS" "production" "gotcha" "pitfall"; do
    if has_pattern "$pattern"; then
      score=$((score + 1))
    fi
  done

  INSTR_FILE="$SKILL_DIR/evals/instructions.json"
  if [ -f "$INSTR_FILE" ] && command -v jq >/dev/null 2>&1; then
    total_instr=$(jq '.instructions | length' "$INSTR_FILE" 2>/dev/null || echo 0)
    if [ "$total_instr" -gt 0 ]; then
      new_know=$(jq '[.instructions[] | select(.why_given == "new knowledge")] | length' "$INSTR_FILE" 2>/dev/null || echo 0)
      pref=$(jq '[.instructions[] | select(.why_given == "preference")] | length' "$INSTR_FILE" 2>/dev/null || echo 0)
      expert_ratio=$(( (new_know + pref) * 100 / total_instr ))
      if [ "$expert_ratio" -ge 70 ]; then
        score=$((score + 2))
      elif [ "$expert_ratio" -lt 30 ]; then
        score=$((score - 2))
      fi
    fi
  fi
  
  if [ "$score" -lt 0 ]; then score=0; fi
  if [ "$score" -gt 20 ]; then score=20; fi
  echo "$score"
}

evaluate_mindset_procedures() {
  score=8
  
  if echo "$CONTENT" | grep -qi "##[[:space:]]*\(mindset\|philosophy\|principles\)"; then
    score=$((score + 2))
  fi
  if echo "$CONTENT" | grep -qE "^[[:space:]]*[0-9]+\."; then
    score=$((score + 2))
  fi
  if has_pattern "when to \(use\|apply\)"; then
    score=$((score + 2))
  fi
  if has_pattern "when not to"; then
    score=$((score + 1))
  fi
  
  if [ "$score" -gt 15 ]; then score=15; fi
  echo "$score"
}

evaluate_anti_pattern_quality() {
  score=8
  
  never_count=$(count_matches "NEVER")
  if [ "$never_count" -gt 3 ]; then
    score=$((score + 3))
  elif [ "$never_count" -gt 0 ]; then
    score=$((score + never_count))
  fi
  
  if echo "$CONTENT" | grep -qi "BAD.*GOOD"; then
    score=$((score + 2))
  fi
  if has_pattern "WHY:"; then
    score=$((score + 2))
  fi

  INSTR_FILE="$SKILL_DIR/evals/instructions.json"
  if [ -f "$INSTR_FILE" ] && command -v jq >/dev/null 2>&1; then
    anti_instr=$(jq '[.instructions[] | select((.original_snippets | if type == "array" then join(" ") else . end) | test("NEVER|ALWAYS|anti-pattern|avoid|do not"; "i"))] | length' "$INSTR_FILE" 2>/dev/null || echo 0)
    if [ "$anti_instr" -ge 5 ]; then
      score=$((score + 2))
    elif [ "$anti_instr" -ge 3 ]; then
      score=$((score + 1))
    fi
  fi
  
  if [ "$score" -gt 15 ]; then score=15; fi
  if [ "$score" -lt 0 ]; then score=0; fi
  echo "$score"
}

evaluate_specification_compliance() {
  score=10
  
  description=$(extract_frontmatter_field "description")
  desc_len=${#description}
  
  if [ "$desc_len" -gt 100 ]; then
    score=$((score + 2))
  fi
  if [ "$desc_len" -gt 200 ]; then
    score=$((score + 1))
  fi
  
  # Task focus check: penalize multi-purpose descriptions (kitchen-sink skills)
  and_or_count=$(echo "$description" | grep -oi ' and \| or ' | wc -l | tr -d ' ')
  if [ "$and_or_count" -gt 3 ]; then
    score=$((score - 2))
  elif [ "$and_or_count" -gt 1 ]; then
    score=$((score - 1))
  fi
  
  # Cross-harness portability checks (3 points)
  # Check for harness-specific paths
  if ! echo "$CONTENT" | grep -qE '\.(opencode|claude|cursor|aider|continue)/'; then
    score=$((score + 1))
  fi
  
  # Check for agent-specific references in instructions
  if ! echo "$CONTENT" | grep -qiE 'claude code|cursor agent|github copilot|aider|continue\.dev'; then
    score=$((score + 1))
  fi
  
  # Check for relative paths from skill directory (scripts/, references/, assets/)
  if echo "$CONTENT" | grep -qE '(scripts|references|assets)/[a-zA-Z0-9_-]+'; then
    score=$((score + 1))
  fi

  # Self-containment check: penalize references to files outside the skill directory
  # Strip fenced code blocks before checking (content inside ``` is examples, not deps)
  non_code_content=$(echo "$CONTENT" | awk '/^```/{skip=!skip; next} !skip{print}')

  # Penalty: ../ path references escape the skill directory (-2)
  if echo "$non_code_content" | grep -qE '\.\./'; then
    score=$((score - 2))
  fi

  # Penalty: absolute skills/X/Y paths assume repo structure (-1)
  if echo "$non_code_content" | grep -qE 'skills/[a-z][a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+'; then
    score=$((score - 1))
  fi

  # Penalty: references to repo-root dirs like .context/, .agents/ (-1)
  if echo "$non_code_content" | grep -qE '\.(context|agents)/'; then
    score=$((score - 1))
  fi

  # Penalty: absolute repo paths in scripts/ files (-1 per file, cap -2)
  if [ -d "$SKILL_DIR/scripts" ]; then
    scripts_abs_penalty=0
    for sf in "$SKILL_DIR/scripts"/*; do
      [ -f "$sf" ] || continue
      if grep -qE 'skills/[a-z][a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+' "$sf" 2>/dev/null; then
        scripts_abs_penalty=$((scripts_abs_penalty + 1))
        [ "$scripts_abs_penalty" -ge 2 ] && break
      fi
    done
    score=$((score - scripts_abs_penalty))
  fi

  # Penalty: repo-root dir references in scripts/ files (-1 per file, cap -2)
  if [ -d "$SKILL_DIR/scripts" ]; then
    scripts_root_penalty=0
    for sf in "$SKILL_DIR/scripts"/*; do
      [ -f "$sf" ] || continue
      if grep -qE '\.(context|agents)/' "$sf" 2>/dev/null; then
        scripts_root_penalty=$((scripts_root_penalty + 1))
        [ "$scripts_root_penalty" -ge 2 ] && break
      fi
    done
    score=$((score - scripts_root_penalty))
  fi

  # Penalty: absolute repo paths in references/ files (-1 per file, cap -2)
  if [ -d "$SKILL_DIR/references" ]; then
    refs_abs_penalty=0
    for rf in "$SKILL_DIR/references"/*; do
      [ -f "$rf" ] || continue
      if grep -qE 'skills/[a-z][a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+' "$rf" 2>/dev/null; then
        refs_abs_penalty=$((refs_abs_penalty + 1))
        [ "$refs_abs_penalty" -ge 2 ] && break
      fi
    done
    score=$((score - refs_abs_penalty))
  fi

  # Penalty: repo-root dir references in references/ files (-1 per file, cap -2)
  if [ -d "$SKILL_DIR/references" ]; then
    refs_root_penalty=0
    for rf in "$SKILL_DIR/references"/*; do
      [ -f "$rf" ] || continue
      if grep -qE '\.(context|agents)/' "$rf" 2>/dev/null; then
        refs_root_penalty=$((refs_root_penalty + 1))
        [ "$refs_root_penalty" -ge 2 ] && break
      fi
    done
    score=$((score - refs_root_penalty))
  fi

  # Cap base score at 15 before applying bonuses
  if [ "$score" -gt 15 ]; then score=15; fi
  if [ "$score" -lt 0 ]; then score=0; fi

  # --- Bonuses (each +1, applied after base cap, max combined +2) ---
  bonus=0

  # Script portability bonus: reward Python/TS/JS scripts for complex logic (+1)
  SCRIPTS_DIR="$SKILL_DIR/scripts"
  if [ -d "$SCRIPTS_DIR" ]; then
    for sf in "$SCRIPTS_DIR"/*.py "$SCRIPTS_DIR"/*.ts "$SCRIPTS_DIR"/*.js; do
      if [ -f "$sf" ]; then
        bonus=$((bonus + 1))
        break
      fi
    done
  fi

  # References Section Format bonus: ## References must be last H2, bullet links with labels (+1)
  last_h2=$(echo "$CONTENT" | grep "^## " | tail -1 | sed 's/^## //')
  if [ "$last_h2" = "References" ]; then
    if echo "$CONTENT" | grep -qE "^\- \[.+\]\(.+\)"; then
      bonus=$((bonus + 1))
    fi
  fi

  score=$((score + bonus))
  if [ "$score" -gt 17 ]; then score=17; fi
  echo "$score"
}

evaluate_progressive_disclosure() {
  if [ "$HAS_REFS" -eq 1 ] && [ "$REF_COUNT" -gt 0 ]; then
    if [ "$LINES" -lt 100 ]; then echo "15"; return; fi
    if [ "$LINES" -lt 150 ]; then echo "13"; return; fi
    if [ "$LINES" -lt 200 ]; then echo "11"; return; fi
    echo "10"; return
  fi
  
  if [ "$LINES" -lt 200 ]; then echo "12"; return; fi
  if [ "$LINES" -lt 300 ]; then echo "10"; return; fi
  if [ "$LINES" -lt 500 ]; then echo "7"; return; fi
  echo "5"
}

evaluate_freedom_calibration() {
  score=10
  
  never_always=$(count_matches "NEVER\|ALWAYS")
  if [ "$never_always" -gt 5 ]; then
    score=$((score + 3))
  elif [ "$never_always" -gt 2 ]; then
    score=$((score + 2))
  fi
  
  if has_pattern "consider\|optionally\|may"; then
    score=$((score + 2))
  fi
  
  if [ "$score" -gt 15 ]; then score=15; fi
  if [ "$score" -lt 0 ]; then score=0; fi
  echo "$score"
}

evaluate_pattern_recognition() {
  description=$(extract_frontmatter_field "description")
  keyword_count=$(echo "$description" | tr -cs '[:alnum:]' '\n' | awk 'length > 3' | wc -l | tr -d ' ')
  
  if [ "$keyword_count" -gt 15 ]; then echo "10"; return; fi
  if [ "$keyword_count" -gt 10 ]; then echo "9"; return; fi
  if [ "$keyword_count" -gt 5 ]; then echo "8"; return; fi
  echo "6"
}

evaluate_practical_usability() {
  score=8
  
  code_blocks=$(echo "$CONTENT" | grep -c '```' 2>/dev/null || echo 0)
  code_blocks=$((code_blocks / 2))
  
  if [ "$code_blocks" -gt 5 ]; then
    score=$((score + 4))
  elif [ "$code_blocks" -gt 2 ]; then
    score=$((score + 2))
  fi
  
  if has_pattern "\./\|bun run"; then
    score=$((score + 2))
  fi
  if echo "$CONTENT" | grep -q '```\(bash\|shell\|typescript\|javascript\)'; then
    score=$((score + 1))
  fi
  
  if [ "$score" -gt 15 ]; then score=15; fi
  if [ "$score" -lt 0 ]; then score=0; fi
  echo "$score"
}

evaluate_eval_validation() {
  score=0
  EVALS_DIR="$SKILL_DIR/evals"

  if [ ! -d "$EVALS_DIR" ]; then
    echo "$score"
    return
  fi
  score=$((score + 4))

  INSTRUCTIONS_FILE="$EVALS_DIR/instructions.json"
  if [ -f "$INSTRUCTIONS_FILE" ]; then
    if command -v jq >/dev/null 2>&1; then
      instr_count=$(jq '.instructions | length' "$INSTRUCTIONS_FILE" 2>/dev/null || echo 0)
      if [ "$instr_count" -gt 0 ]; then
        score=$((score + 3))
      fi
    else
      if [ -s "$INSTRUCTIONS_FILE" ]; then
        score=$((score + 3))
      fi
    fi
  fi

  SUMMARY_FILE="$EVALS_DIR/summary.json"
  if [ -f "$SUMMARY_FILE" ]; then
    if command -v jq >/dev/null 2>&1; then
      coverage=$(jq -r '.instructions_coverage.coverage_percentage // "0"' "$SUMMARY_FILE" 2>/dev/null | tr -d '%"')
      coverage=$(echo "$coverage" | cut -d. -f1)
      if [ -n "$coverage" ] && [ "$coverage" -ge 0 ] 2>/dev/null; then
        score=$((score + 3))
        if [ "$coverage" -ge 80 ]; then
          score=$((score + 3))
        fi
      fi
    else
      if [ -s "$SUMMARY_FILE" ]; then
        score=$((score + 3))
      fi
    fi
  fi

  scenario_count=0
  valid_scenarios=0
  for scenario_dir in "$EVALS_DIR"/scenario-*/; do
    [ -d "$scenario_dir" ] || continue
    scenario_count=$((scenario_count + 1))
    has_task=0; has_criteria=0; has_capability=0
    [ -f "${scenario_dir}task.md" ] && has_task=1
    [ -f "${scenario_dir}criteria.json" ] && has_criteria=1
    [ -f "${scenario_dir}capability.txt" ] && has_capability=1
    if [ "$has_task" -eq 1 ] && [ "$has_criteria" -eq 1 ] && [ "$has_capability" -eq 1 ]; then
      if command -v jq >/dev/null 2>&1; then
        criteria_sum=$(jq '[.checklist[].max_score] | add // 0' "${scenario_dir}criteria.json" 2>/dev/null || echo 0)
        if [ "$criteria_sum" -eq 100 ] 2>/dev/null; then
          valid_scenarios=$((valid_scenarios + 1))
        fi
      else
        valid_scenarios=$((valid_scenarios + 1))
      fi
    fi
  done

  if [ "$valid_scenarios" -ge 3 ]; then
    score=$((score + 4))
  elif [ "$valid_scenarios" -ge 1 ]; then
    score=$((score + 2))
  fi

  if [ "$score" -gt 20 ]; then score=20; fi
  echo "$score"
}

calculate_grade() {
  if [ "$1" -ge 133 ]; then echo "A+"; return; fi
  if [ "$1" -ge 126 ]; then echo "A"; return; fi
  if [ "$1" -ge 119 ]; then echo "B+"; return; fi
  if [ "$1" -ge 112 ]; then echo "B"; return; fi
  if [ "$1" -ge 105 ]; then echo "C+"; return; fi
  if [ "$1" -ge 98 ]; then echo "C"; return; fi
  if [ "$1" -ge 91 ]; then echo "D"; return; fi
  echo "F"
}

generate_analysis_md() {
  output_file="$1"
  cat > "$output_file" <<EOF
# Skill Quality Analysis: $SKILL_NAME

**Date**: $(date +%Y-%m-%d)  
**Overall Score**: $TOTAL/140 ($((TOTAL * 100 / 140))%)  
**Grade**: $GRADE

---

## Dimension Breakdown

| Dimension | Score | Max | Percentage | Status |
|-----------|-------|-----|------------|--------|
| D1: Knowledge Delta | $D1 | 20 | $((D1 * 100 / 20))% | $([ "$D1" -ge 16 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D2: Mindset + Procedures | $D2 | 15 | $((D2 * 100 / 15))% | $([ "$D2" -ge 12 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D3: Anti-Pattern Quality | $D3 | 15 | $((D3 * 100 / 15))% | $([ "$D3" -ge 12 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D4: Specification Compliance | $D4 | 15 | $((D4 * 100 / 15))% | $([ "$D4" -ge 12 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D5: Progressive Disclosure | $D5 | 15 | $((D5 * 100 / 15))% | $([ "$D5" -ge 12 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D6: Freedom Calibration | $D6 | 15 | $((D6 * 100 / 15))% | $([ "$D6" -ge 12 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D7: Pattern Recognition | $D7 | 10 | $((D7 * 100 / 10))% | $([ "$D7" -ge 8 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D8: Practical Usability | $D8 | 15 | $((D8 * 100 / 15))% | $([ "$D8" -ge 12 ] && echo "✓ Good" || echo "⚠ Needs work") |
| D9: Eval Validation | $D9 | 20 | $((D9 * 100 / 20))% | $([ "$D9" -ge 16 ] && echo "✓ Good" || echo "⚠ Needs work") |

---

## Metadata

- **Skill Path**: $SKILL_PATH
- **Total Lines**: $LINES
- **Has References**: $([ "$HAS_REFS" -eq 1 ] && echo "Yes" || echo "No")
- **Reference Count**: $REF_COUNT files
- **References Section Compliant**: $([ "$REFS_SECTION_COMPLIANT" = "true" ] && echo "Yes (D4 +1 bonus awarded)" || echo "No (D4 bonus not awarded)")

---

## Interpretation

$(if [ "$TOTAL" -ge 126 ]; then
  echo "**Grade A**: This skill meets high quality standards. Minor improvements may be possible."
elif [ "$TOTAL" -ge 112 ]; then
  echo "**Grade B**: This skill is good but has room for improvement in specific dimensions."
elif [ "$TOTAL" -ge 98 ]; then
  echo "**Grade C**: This skill needs moderate improvements across multiple dimensions."
else
  echo "**Grade D/F**: This skill requires significant remediation work."
fi)

---

## Next Steps

$(if [ "$TOTAL" -lt 126 ]; then
  echo "See \`remediation-plan.md\` in this directory for detailed improvement steps."
else
  echo "No remediation plan needed. Consider minor refinements based on dimension scores."
fi)
EOF
}

generate_remediation_plan() {
  output_file="$1"
  
  target_score=126
  if [ "$TOTAL" -ge 112 ]; then
    target_score=126
  elif [ "$TOTAL" -ge 98 ]; then
    target_score=119
  else
    target_score=112
  fi
  
  target_grade=$(calculate_grade "$target_score")
  score_delta=$((target_score - TOTAL))
  
  # Determine priority
  priority="High"
  [ "$TOTAL" -lt 98 ] && priority="Critical"
  [ "$TOTAL" -ge 112 ] && priority="Medium"
  
  # Determine effort
  effort="M"
  [ "$score_delta" -gt 20 ] && effort="L"
  [ "$score_delta" -lt 10 ] && effort="S"
  
  cat > "$output_file" <<EOF
# Remediation Plan: $SKILL_NAME

---
plan_date: "$(date +%Y-%m-%d)"
skill_name: "$SKILL_NAME"
source_audit: ".context/audits/$SKILL_NAME/$(date +%Y-%m-%d)/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | $TOTAL/140 ($((TOTAL * 100 / 140))%) | $target_score/140 ($((target_score * 100 / 140))%) |
| **Grade** | $GRADE | $target_grade |
| **Priority** | $priority | |
| **Effort** | $effort | |

**Focus Areas**:
$(
  [ "$((D1 * 100 / 20))" -lt 80 ] && echo "- D1: Knowledge Delta ($D1/20)"
  [ "$((D2 * 100 / 15))" -lt 80 ] && echo "- D2: Mindset + Procedures ($D2/15)"
  [ "$((D3 * 100 / 15))" -lt 80 ] && echo "- D3: Anti-Pattern Quality ($D3/15)"
  [ "$((D4 * 100 / 15))" -lt 80 ] && echo "- D4: Specification Compliance ($D4/15)"
  [ "$((D5 * 100 / 15))" -lt 80 ] && echo "- D5: Progressive Disclosure ($D5/15)"
  [ "$((D6 * 100 / 15))" -lt 80 ] && echo "- D6: Freedom Calibration ($D6/15)"
  [ "$((D7 * 100 / 10))" -lt 80 ] && echo "- D7: Pattern Recognition ($D7/10)"
  [ "$((D8 * 100 / 15))" -lt 80 ] && echo "- D8: Practical Usability ($D8/15)"
  [ "$((D9 * 100 / 20))" -lt 80 ] && echo "- D9: Eval Validation ($D9/20)"
)

**Verdict**: Targeted improvements needed to reach grade $target_grade (+$score_delta points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
$(
  [ "$((D1 * 100 / 20))" -lt 80 ] && echo "| Low knowledge delta signals | D1 ($D1/20) | High | Skill may duplicate basic docs |"
  [ "$((D2 * 100 / 15))" -lt 80 ] && echo "| Missing mindset/procedures | D2 ($D2/15) | High | Agents lack decision frameworks |"
  [ "$((D3 * 100 / 15))" -lt 80 ] && echo "| Insufficient anti-patterns | D3 ($D3/15) | High | Agents repeat common mistakes |"
  [ "$((D4 * 100 / 15))" -lt 80 ] && echo "| Weak description field | D4 ($D4/15) | Medium | Skill discovery suffers |"
  [ "$((D5 * 100 / 15))" -lt 80 ] && echo "| Poor progressive disclosure | D5 ($D5/15) | High | Skill is too long or lacks refs |"
  [ "$((D6 * 100 / 15))" -lt 80 ] && echo "| Imbalanced constraint language | D6 ($D6/15) | Medium | Over/under-prescriptive |"
  [ "$((D7 * 100 / 10))" -lt 80 ] && echo "| Weak pattern recognition | D7 ($D7/10) | Medium | Skill trigger rate is low |"
  [ "$((D8 * 100 / 15))" -lt 80 ] && echo "| Limited practical examples | D8 ($D8/15) | High | Agents struggle to apply skill |"
  [ "$((D9 * 100 / 20))" -lt 80 ] && echo "| Missing or incomplete evals | D9 ($D9/20) | High | Skill not validated at runtime |"
)

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.

$(
  phase=1
  [ "$((D3 * 100 / 15))" -lt 80 ] && cat <<PHASE3

### Phase $phase: Anti-Pattern Quality - Priority: High

**Target**: Increase D3 from $D3/15 to 13/15 (+$((13 - D3)) points)

#### Step $phase.1: Add NEVER/ALWAYS Constraints

Add explicit anti-pattern warnings to prevent common mistakes.

**File**: \`$SKILL_PATH\`

**Action**: Add section with BAD vs GOOD examples.

PHASE3
  [ "$((D3 * 100 / 15))" -lt 80 ] && phase=$((phase + 1))
  
  [ "$((D5 * 100 / 15))" -lt 80 ] && cat <<PHASE5

### Phase $phase: Progressive Disclosure - Priority: High

**Target**: Increase D5 from $D5/15 to 13/15 (+$((13 - D5)) points)

#### Step $phase.1: Create Reference Files

Move detailed content to \`references/\` directory.

**Action**: Extract deep-dive content into separate files, keep SKILL.md as navigation hub.

PHASE5
  [ "$((D5 * 100 / 15))" -lt 80 ] && phase=$((phase + 1))
  
  [ "$((D8 * 100 / 15))" -lt 80 ] && cat <<PHASE8

### Phase $phase: Practical Usability - Priority: High

**Target**: Increase D8 from $D8/15 to 13/15 (+$((13 - D8)) points)

#### Step $phase.1: Add Code Examples

Add executable code blocks with language tags.

**File**: \`$SKILL_PATH\`

**Action**: Include bash/typescript examples with clear syntax highlighting.

PHASE8
  [ "$((D8 * 100 / 15))" -lt 80 ] && phase=$((phase + 1))

  [ "$((D9 * 100 / 20))" -lt 80 ] && cat <<PHASE9

### Phase $phase: Eval Validation - Priority: High

**Target**: Increase D9 from $D9/20 to 17/20 (+$((17 - D9)) points)

#### Step $phase.1: Create Eval Scenarios

Use the \`creating-eval-scenarios\` skill to generate evaluation scenarios.

**Action**: Run the eval scenario creation workflow to produce:
- \`evals/instructions.json\` - Extract all instructions from SKILL.md
- \`evals/summary.json\` - Coverage statistics (target >= 80%)
- \`evals/scenario-N/\` - 5 scenarios with task.md, criteria.json, capability.txt

#### Step $phase.2: Run Evals

\`\`\`bash
tessl eval run <tile-path>
tessl eval view-status <status_id> --json
\`\`\`

#### Step $phase.3: Validate Coverage

Verify \`summary.json\` shows \`coverage_percentage >= 80\` and all criteria.json files sum to 100.

PHASE9
)

---

## Verification Commands

\`\`\`bash
# Re-run evaluation
./scripts/evaluate.sh $SKILL_NAME --json --store

# Check target score achieved
./scripts/evaluate.sh $SKILL_NAME --json | jq ".total >= $target_score"
\`\`\`

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= $target_score/140 |
| Grade | >= $target_grade |
$(
  [ "$((D3 * 100 / 15))" -lt 80 ] && echo "| D3: Anti-Pattern Quality | >= 13/15 |"
  [ "$((D5 * 100 / 15))" -lt 80 ] && echo "| D5: Progressive Disclosure | >= 13/15 |"
  [ "$((D8 * 100 / 15))" -lt 80 ] && echo "| D8: Practical Usability | >= 13/15 |"
  [ "$((D9 * 100 / 20))" -lt 80 ] && echo "| D9: Eval Validation | >= 17/20 |"
)

---

## Effort Estimate

| Phase | Effort | Time |
|-------|--------|------|
| Total | $effort | $(if [ "$effort" = "S" ]; then echo "1-2 hours"; elif [ "$effort" = "M" ]; then echo "2-4 hours"; else echo "4-8 hours"; fi) |

---

## Dependencies

- None (standalone skill)

---

## Rollback Plan

\`\`\`bash
git checkout HEAD~1 -- $SKILL_PATH
\`\`\`

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
EOF
}

D1=$(evaluate_knowledge_delta)
D2=$(evaluate_mindset_procedures)
D3=$(evaluate_anti_pattern_quality)
D4=$(evaluate_specification_compliance)
D5=$(evaluate_progressive_disclosure)
D6=$(evaluate_freedom_calibration)
D7=$(evaluate_pattern_recognition)
D8=$(evaluate_practical_usability)
D9=$(evaluate_eval_validation)

TOTAL=$((D1 + D2 + D3 + D4 + D5 + D6 + D7 + D8 + D9))

# Compute references section compliance for reporting (independent of D4 scoring path)
REFS_SECTION_COMPLIANT="false"
last_h2=$(echo "$CONTENT" | grep "^## " | tail -1 | sed 's/^## //')
if [ "$last_h2" = "References" ] && echo "$CONTENT" | grep -qE "^\- \[.+\]\(.+\)"; then
  REFS_SECTION_COMPLIANT="true"
fi
GRADE=$(calculate_grade "$TOTAL")

JSON_OUTPUT_STRING=$(cat <<EOF
{
  "skill": "$SKILL_NAME",
  "dimensions": {
    "knowledgeDelta": $D1,
    "mindsetProcedures": $D2,
    "antiPatternQuality": $D3,
    "specificationCompliance": $D4,
    "progressiveDisclosure": $D5,
    "freedomCalibration": $D6,
    "patternRecognition": $D7,
    "practicalUsability": $D8,
    "evalValidation": $D9
  },
  "total": $TOTAL,
  "maxTotal": 140,
  "grade": "$GRADE",
  "lines": $LINES,
  "hasReferences": $([ "$HAS_REFS" -eq 1 ] && echo "true" || echo "false"),
  "referenceCount": $REF_COUNT,
  "referenceSectionCompliant": $REFS_SECTION_COMPLIANT
}
EOF
)

if [ -n "$OUTPUT_PATH" ]; then
  mkdir -p "$(dirname "$OUTPUT_PATH")"
  printf '%s\n' "$JSON_OUTPUT_STRING" > "$OUTPUT_PATH"
  
  # Create symlink to latest audit if in store mode
  if [ "$STORE_MODE" -eq 1 ]; then
    SKILL_AUDIT_DIR=".context/audits/${SKILL_NAME}"
    LATEST_LINK="${SKILL_AUDIT_DIR}/latest"
    
    # Remove old symlink if it exists
    [ -L "$LATEST_LINK" ] && rm "$LATEST_LINK"
    
    # Create relative symlink to current audit directory
    (cd "$SKILL_AUDIT_DIR" && ln -s "$AUDIT_DATE" latest)
    
    # Generate analysis.md
    generate_analysis_md "${AUDIT_DIR}/analysis.md"
    
    if [ "$TOTAL" -lt 126 ]; then
      generate_remediation_plan "${AUDIT_DIR}/remediation-plan.md"
    fi
  fi
  
  if [ "$JSON_OUTPUT" -eq 0 ]; then
    echo "Stored: $OUTPUT_PATH"
    if [ "$STORE_MODE" -eq 1 ]; then
      echo "Latest symlink: ${SKILL_AUDIT_DIR}/latest -> ${AUDIT_DATE}"
    fi
  fi
fi

if [ "$JSON_OUTPUT" -eq 1 ]; then
  printf '%s\n' "$JSON_OUTPUT_STRING"
else
  cat <<EOF
Skill: $SKILL_NAME
================================

Dimensions:
  D1: Knowledge Delta         $D1/20
  D2: Mindset + Procedures    $D2/15
  D3: Anti-Pattern Quality    $D3/15
  D4: Specification           $D4/15
  D5: Progressive Disclosure  $D5/15
  D6: Freedom Calibration     $D6/15
  D7: Pattern Recognition     $D7/10
  D8: Practical Usability     $D8/15
  D9: Eval Validation         $D9/20

Total: $TOTAL/140
Grade: $GRADE

Metadata:
  Lines: $LINES
  References: $REF_COUNT files
  References Section: $([ "$REFS_SECTION_COMPLIANT" = "true" ] && echo "compliant (+1 D4 bonus)" || echo "non-compliant (0 D4 bonus)")
EOF
fi
