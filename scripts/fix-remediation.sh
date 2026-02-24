#!/usr/bin/env sh
# Regenerate remediation-plan.md with proper markdown format

for skill_dir in .context/audits/*/; do
  skill=$(basename "$skill_dir")
  
  for date_dir in "$skill_dir"*/; do
    date=$(basename "$date_dir")
    
    plan=".context/plans/${skill}-remediation-plan.md"
    out="$date_dir/remediation-plan.md"
    
    {
      echo "# Remediation Plan: $skill"
      echo ""
      echo "**Date:** $date"
      echo ""
      
      if [ -f "$plan" ]; then
        tail -n +9 "$plan" 2>/dev/null || cat "$plan"
      else
        echo "No remediation plan exists yet. Run skill-judge to generate recommendations."
      fi
    } > "$out"
  done
done

echo "Regenerated remediation plans"
