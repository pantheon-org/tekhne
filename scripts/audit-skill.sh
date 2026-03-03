#!/usr/bin/env sh
# audit-skill.sh - Run skill-quality-auditor evaluation on a single skill
# Usage: ./scripts/audit-skill.sh <skill-name>

set -e

SKILL="$1"

# Dependency checks
command -v jq >/dev/null 2>&1 || {
  echo "❌ Error: jq is required but not installed"
  echo "   Install: brew install jq (macOS) or apt-get install jq (Linux)"
  exit 1
}

if [ -z "$SKILL" ]; then
  echo "Usage: $0 <skill-name>"
  echo "Example: $0 acceptance-criteria"
  exit 1
fi

# Verify skill exists
if [ ! -d "skills/$SKILL" ]; then
  echo "❌ Error: Skill not found: skills/$SKILL"
  exit 1
fi

echo "🔍 Auditing: $SKILL"

# Run evaluation with skill-quality-auditor
echo "  → Running evaluation..."
if ! sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh "$SKILL" --json --store; then
  echo "  ❌ Evaluation failed for $SKILL"
  exit 1
fi

# Verify symlink was created (evaluate.sh creates this automatically)
if [ ! -L ".context/audits/$SKILL/latest" ]; then
  echo "  ❌ Latest symlink not created"
  exit 1
fi

# Extract score and grade
AUDIT_JSON=".context/audits/$SKILL/latest/audit.json"
if [ ! -f "$AUDIT_JSON" ]; then
  echo "  ❌ audit.json not found"
  exit 1
fi

SCORE=$(jq -r '.total' "$AUDIT_JSON" 2>/dev/null || echo "unknown")
GRADE=$(jq -r '.grade' "$AUDIT_JSON" 2>/dev/null || echo "unknown")

# Verify required files exist
REQUIRED_FILES="analysis.md audit.json"
for file in $REQUIRED_FILES; do
  if [ ! -f ".context/audits/$SKILL/latest/$file" ]; then
    echo "  ❌ Missing required file: $file"
    exit 1
  fi
done

# Check if remediation plan should exist (score < 108 = A grade threshold)
if [ "$SCORE" != "unknown" ] && [ "$SCORE" -lt 108 ]; then
  if [ ! -f ".context/audits/$SKILL/latest/remediation-plan.md" ]; then
    echo "  ⚠️  Warning: Score < 108 but remediation-plan.md missing"
  fi
fi

echo "  ✅ Audit complete: $SKILL (score: $SCORE/120, grade: $GRADE)"
