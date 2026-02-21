#!/usr/bin/env bash
# Audit all skills using opencode CLI with skill-quality-auditor
# Usage: ./scripts/audit-all-skills.sh [--skill skill-name] [--force]
#
# By default, skips skills with a report from today.
# Stale reports (not from today) will trigger a re-audit.
# Use --force to re-audit even if today's report exists.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
AUDITS_DIR="$PROJECT_ROOT/.context/audits"
SKILLS_DIR="$PROJECT_ROOT/skills"
DATE=$(date +%Y-%m-%d)
FORCE=false
TARGET_SKILL=""

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --skill|-s)
      TARGET_SKILL="$2"
      shift 2
      ;;
    --force|-f)
      FORCE=true
      shift
      ;;
    --help|-h)
      echo "Usage: ./scripts/audit-all-skills.sh [options]"
      echo ""
      echo "Options:"
      echo "  --skill, -s <name>    Audit a specific skill only"
      echo "  --force, -f           Re-audit even if today's report exists"
      echo "  --help, -h            Show this help message"
      echo ""
      echo "By default:"
      echo "  - Skips skills with a report from today"
      echo "  - Re-audits skills with stale reports (not from today)"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

cd "$PROJECT_ROOT"

mkdir -p "$AUDITS_DIR"

# Get list of skills to audit
get_skills() {
  if [[ -n "$TARGET_SKILL" ]]; then
    echo "$SKILLS_DIR/$TARGET_SKILL"
  else
    find "$SKILLS_DIR" -maxdepth 1 -mindepth 1 -type d | sort
  fi
}

# Extract rating from audit report
extract_rating() {
  local report_file="$1"
  if [[ -f "$report_file" ]]; then
    # Try to extract Total Score line (e.g., "| **Total Score** | 83/120 (69.2%) |")
    grep -oP '\*\*Total Score\*\*\s*\|\s*\K[0-9]+/[0-9]+' "$report_file" | head -1
  fi
}

# Update README.md with rating
update_readme_rating() {
  local skill_name="$1"
  local rating="$2"
  local readme="$PROJECT_ROOT/README.md"
  
  if [[ -z "$rating" ]]; then
    return
  fi
  
  # Check if Rating column exists, if not add it
  if ! grep -q "| \`$skill_name\`.*|" "$readme"; then
    return
  fi
  
  # This is a simple update - for complex table updates, consider using a proper tool
  # The README update logic is intentionally simple here
  # Full implementation would parse the table properly
  echo "  Rating for $skill_name: $rating"
}

echo "=== Skill Quality Audit Runner ==="
echo "Date: $DATE"
echo "Audits directory: $AUDITS_DIR"
echo ""

SKILLS=$(get_skills)
SKILL_COUNT=$(echo "$SKILLS" | grep -c . || echo "0")

echo "Skills to audit: $SKILL_COUNT"
echo ""

AUDITED_COUNT=0
SKIPPED_COUNT=0
declare -A RATINGS

for skill_path in $SKILLS; do
  skill_name=$(basename "$skill_path")
  report_file="$AUDITS_DIR/${skill_name}-${DATE}.md"
  
  # Skip if skill-quality-auditor itself
  if [[ "$skill_name" == "skill-quality-auditor" ]]; then
    echo "Skipping self-audit of skill-quality-auditor"
    ((SKIPPED_COUNT++)) || true
    continue
  fi
  
  # Skip if report from today exists (unless --force)
  if [[ "$FORCE" == false && -f "$report_file" ]]; then
    echo "Skipping $skill_name (today's report exists)"
    ((SKIPPED_COUNT++)) || true
    continue
  fi
  
  # Note if we're re-auditing a stale report
  if [[ "$FORCE" == false ]]; then
    EXISTING_REPORT=$(find "$AUDITS_DIR" -name "${skill_name}-*.md" 2>/dev/null | sort -r | head -1)
    if [[ -n "$EXISTING_REPORT" && -f "$EXISTING_REPORT" ]]; then
      REPORT_DATE=$(basename "$EXISTING_REPORT" | grep -oP '\d{4}-\d{2}-\d{2}' | head -1)
      if [[ "$REPORT_DATE" != "$DATE" ]]; then
        echo "Re-auditing $skill_name (stale report from $REPORT_DATE)"
      fi
    fi
  fi
  
  echo "Auditing: $skill_name"
  
  # Run opencode with skill-quality-auditor prompt
  # The prompt instructs opencode to use the skill-quality-auditor skill
  # and save the report to the appropriate location
  AUDIT_PROMPT="Use the skill-quality-auditor skill to perform a quality audit of the skill at skills/$skill_name/. 

Follow the skill-quality-auditor workflow:
1. Read the skill-quality-auditor SKILL.md first
2. Load framework-skill-judge-dimensions.md for evaluation criteria
3. Evaluate skills/$skill_name/SKILL.md against the 8-dimension framework
4. Calculate the total score and grade

Save the audit report to: .context/audits/${skill_name}-${DATE}.md

Use the canonical template at skills/skill-quality-auditor/templates/review-report-template.yaml (field: report_template_markdown). Include:
- YAML frontmatter at top:
  - review_date: ${DATE}
  - reviewer: automated audit
  - skill_location: \`skills/$skill_name/SKILL.md\`
- Executive summary with score/grade
- Dimension scores table
- Critical issues with file references
- Top 3 recommended improvements
- Detailed dimension analysis
- Files inventory
- Verification commands

After writing the report, run:
\`./skills/skill-quality-auditor/scripts/validate-review-format.sh .context/audits/${skill_name}-${DATE}.md\`

After saving the report, output ONLY the final rating in format: RATING: <score>/<max> (<grade>)"

  # Run opencode with the audit prompt
  # Using --quiet to minimize output noise
  if command -v opencode &> /dev/null; then
    OUTPUT=$(opencode run "$AUDIT_PROMPT" 2>&1) || OUTPUT="Error: opencode run failed"
  else
    echo "  Error: opencode CLI not found"
    OUTPUT=""
  fi
  
  # Extract rating from output or report
  RATING=$(echo "$OUTPUT" | grep -oP 'RATING:\s*\K[0-9]+/[0-9]+\s*\([A-F+]+\)' | head -1)
  
  if [[ -z "$RATING" && -f "$report_file" ]]; then
    RATING=$(extract_rating "$report_file")
    if [[ -n "$RATING" ]]; then
      RATING="$RATING (from report)"
    fi
  fi
  
  if [[ -n "$RATING" ]]; then
    RATINGS["$skill_name"]="$RATING"
    echo "  Rating: $RATING"
  else
    echo "  Warning: Could not extract rating"
  fi
  
  ((AUDITED_COUNT++)) || true
  echo ""
done

echo ""
echo "=== Audit Summary ==="
echo "Audited: $AUDITED_COUNT"
echo "Skipped: $SKIPPED_COUNT"
echo ""

# Output ratings for README update
if [[ ${#RATINGS[@]} -gt 0 ]]; then
  echo "Ratings collected:"
  for skill in "${!RATINGS[@]}"; do
    echo "  $skill: ${RATINGS[$skill]}"
  done
  echo ""
  echo "To update README.md, run:"
  echo "  bun run scripts/update-readme-ratings.ts"
fi

echo ""
echo "Audit reports saved to: $AUDITS_DIR"
