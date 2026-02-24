#!/usr/bin/env sh
# Generate remediation-plan.md referencing existing plans in .context/plans/

for AUDIT_DIR in .context/audits/skill-audit/2026-02-21 .context/audits/skill-audit/2026-02-22 .context/audits/skill-audit/2026-02-23; do
  AUDIT_DATE=$(basename "$AUDIT_DIR")
  AUDIT_JSON="$AUDIT_DIR/audit.json"
  
  [ -f "$AUDIT_JSON" ] || continue
  
  {
    echo "# Remediation Plan"
    echo ""
    echo "**Date:** $AUDIT_DATE"
    echo ""
    echo "This document references individual skill remediation plans."
    echo ""
    echo "---"
    echo ""
    echo "## Skills with Remediation Plans"
    echo ""
    echo "| Skill | Remediation Plan |"
    echo "|-------|------------------|"
    
    for skill in $(grep -oP '"skill":\s*"\K[^"]+' "$AUDIT_JSON"); do
      grade=$(grep -A4 "\"skill\": \"$skill\"" "$AUDIT_JSON" | grep -oP '"grade":\s*"\K[^"]+' | head -1)
      
      plan_file=".context/plans/${skill}-remediation-plan.md"
      if [ -f "$plan_file" ]; then
        echo "| $skill | [View Plan](../plans/${skill}-remediation-plan.md) |"
      else
        echo "| $skill (Grade: $grade) | *No plan yet* |"
      fi
    done
    
    echo ""
    echo "---"
    echo ""
    echo "## How to Use"
    echo ""
    echo "1. Review the audit results in [analysis.md](analysis.md)"
    echo "2. Select skills needing remediation"
        echo "3. Follow the steps in each skill's remediation plan"
    echo "4. Re-run the audit after improvements"
    echo ""
    echo "*Auto-generated from audit data*"
    
  } > "$AUDIT_DIR/remediation-plan.md"
  
  echo "Created $AUDIT_DIR/remediation-plan.md"
done
