#!/usr/bin/env sh
# check-skill-audit-status.sh - Verify audit compliance for all skills

set -e

# Check dependencies
command -v jq >/dev/null 2>&1 || {
  echo "❌ Error: jq is required but not installed"
  echo "   Install: brew install jq (macOS) or apt-get install jq (Linux)"
  exit 1
}

# Terminal color support detection
if [ -t 1 ]; then
  RED='\033[0;31m'
  GREEN='\033[0;32m'
  YELLOW='\033[1;33m'
  NC='\033[0m'
else
  RED=''
  GREEN=''
  YELLOW=''
  NC=''
fi

echo "📊 Checking skill audit compliance..."
echo ""

total=0
compliant=0
outdated=0
missing=0

cd skills || exit 1

for skill_dir in */; do
  skill=$(basename "$skill_dir")
  
  # Skip if not a skill directory
  if [ ! -f "$skill_dir/SKILL.md" ]; then
    continue
  fi
  
  total=$((total + 1))
  
  # Check if audit exists
  if [ ! -d "../.context/audits/$skill" ]; then
    printf "${RED}✗${NC} %s - MISSING AUDIT\n" "$skill"
    missing=$((missing + 1))
    continue
  fi
  
  # Check if latest symlink exists
  if [ ! -L "../.context/audits/$skill/latest" ]; then
    printf "${YELLOW}⚠${NC} %s - Missing 'latest' symlink\n" "$skill"
    outdated=$((outdated + 1))
    continue
  fi
  
  # Check if symlink target exists
  if [ ! -e "../.context/audits/$skill/latest" ]; then
    printf "${RED}✗${NC} %s - Broken 'latest' symlink\n" "$skill"
    missing=$((missing + 1))
    continue
  fi
  
  # Check audit age (warn if > 30 days old)
  audit_file="../.context/audits/$skill/latest/audit.json"
  if [ -f "$audit_file" ]; then
    # Check file modification time
    if find "$audit_file" -mtime +30 2>/dev/null | grep -q .; then
      score=$(jq -r '.total' "$audit_file" 2>/dev/null || echo "?")
      grade=$(jq -r '.grade' "$audit_file" 2>/dev/null || echo "?")
      printf "${YELLOW}⚠${NC} %s - Audit >30 days old (score: %s/120, grade: %s)\n" "$skill" "$score" "$grade"
      outdated=$((outdated + 1))
    else
      score=$(jq -r '.total' "$audit_file" 2>/dev/null || echo "?")
      grade=$(jq -r '.grade' "$audit_file" 2>/dev/null || echo "?")
      printf "${GREEN}✓${NC} %s - Compliant (score: %s/120, grade: %s)\n" "$skill" "$score" "$grade"
      compliant=$((compliant + 1))
    fi
  else
    printf "${RED}✗${NC} %s - Missing audit.json\n" "$skill"
    missing=$((missing + 1))
  fi
done

cd ..

echo ""
echo "======================================"
printf "Summary:\n"
printf "  Total skills: %d\n" "$total"
printf "  ${GREEN}Compliant: %d${NC}\n" "$compliant"
printf "  ${YELLOW}Outdated: %d${NC}\n" "$outdated"
printf "  ${RED}Missing: %d${NC}\n" "$missing"
echo "======================================"

if [ "$missing" -gt 0 ] || [ "$outdated" -gt 0 ]; then
  echo ""
  echo "To audit missing or outdated skills:"
  echo "  1. Run Phase 1 validation (5 sample skills)"
  echo "  2. Run Phase 2 batch processing (all remaining skills)"
  echo "  See: .context/plans/phase-2-batch-processing.md"
  exit 1
else
  echo ""
  echo "✅ All skills are compliant!"
  exit 0
fi
