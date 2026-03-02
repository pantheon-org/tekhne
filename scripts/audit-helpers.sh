#!/usr/bin/env sh
# audit-helpers.sh - Helper functions for batch audit processing

# Check audit quality for a single skill
check_audit() {
  skill="$1"
  if [ -L ".context/audits/$skill/latest" ]; then
    score=$(jq -r '.total' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERR")
    grade=$(jq -r '.grade' ".context/audits/$skill/latest/audit.json" 2>/dev/null || echo "ERR")
    if [ "$score" = "ERR" ]; then
      echo "✗ $skill: INVALID AUDIT"
    else
      echo "✓ $skill: $score/120 ($grade)"
    fi
  else
    echo "✗ $skill: MISSING AUDIT"
  fi
}

# Note: Source this file with `. scripts/audit-helpers.sh` to use check_audit function
