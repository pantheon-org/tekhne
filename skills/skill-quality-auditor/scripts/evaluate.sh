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
  OUTPUT_PATH=".context/audits/skill-audit/latest/audit.json"
fi

find_skill_path() {
  for path in "skills/$1/SKILL.md" ".agents/skills/$1/SKILL.md"; do
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
  
  if [ "$score" -gt 15 ]; then score=15; fi
  if [ "$score" -lt 0 ]; then score=0; fi
  echo "$score"
}

evaluate_specification_compliance() {
  score=10
  
  description=$(extract_frontmatter_field "description")
  desc_len=${#description}
  
  if [ "$desc_len" -gt 100 ]; then
    score=$((score + 3))
  fi
  if [ "$desc_len" -gt 200 ]; then
    score=$((score + 2))
  fi
  
  if [ "$score" -gt 15 ]; then score=15; fi
  if [ "$score" -lt 0 ]; then score=0; fi
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

calculate_grade() {
  if [ "$1" -ge 114 ]; then echo "A+"; return; fi
  if [ "$1" -ge 108 ]; then echo "A"; return; fi
  if [ "$1" -ge 102 ]; then echo "B+"; return; fi
  if [ "$1" -ge 96 ]; then echo "B"; return; fi
  if [ "$1" -ge 90 ]; then echo "C+"; return; fi
  if [ "$1" -ge 84 ]; then echo "C"; return; fi
  if [ "$1" -ge 78 ]; then echo "D"; return; fi
  echo "F"
}

D1=$(evaluate_knowledge_delta)
D2=$(evaluate_mindset_procedures)
D3=$(evaluate_anti_pattern_quality)
D4=$(evaluate_specification_compliance)
D5=$(evaluate_progressive_disclosure)
D6=$(evaluate_freedom_calibration)
D7=$(evaluate_pattern_recognition)
D8=$(evaluate_practical_usability)

TOTAL=$((D1 + D2 + D3 + D4 + D5 + D6 + D7 + D8))
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
    "practicalUsability": $D8
  },
  "total": $TOTAL,
  "maxTotal": 120,
  "grade": "$GRADE",
  "lines": $LINES,
  "hasReferences": $([ "$HAS_REFS" -eq 1 ] && echo "true" || echo "false"),
  "referenceCount": $REF_COUNT
}
EOF
)

if [ -n "$OUTPUT_PATH" ]; then
  mkdir -p "$(dirname "$OUTPUT_PATH")"
  printf '%s\n' "$JSON_OUTPUT_STRING" > "$OUTPUT_PATH"
  if [ "$JSON_OUTPUT" -eq 0 ]; then
    echo "Stored: $OUTPUT_PATH"
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

Total: $TOTAL/120
Grade: $GRADE

Metadata:
  Lines: $LINES
  References: $REF_COUNT files
EOF
fi
