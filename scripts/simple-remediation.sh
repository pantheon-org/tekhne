#!/usr/bin/env sh
# Create simple remediation-plan.md without heading conflicts

for skill_dir in .context/audits/*/; do
  skill=$(basename "$skill_dir")
  
  for date_dir in "$skill_dir"*/; do
    date=$(basename "$date_dir")
    
    out="$date_dir/remediation-plan.md"
    
    {
      echo "# Remediation Plan"
      echo ""
      echo "**Skill:** $skill"
      echo "**Date:** $date"
      echo ""
      echo "See [analysis.md](analysis.md) for detailed audit results."
      echo ""
      echo "Individual remediation plans are stored in:"
      echo "\`.context/plans/${skill}-remediation-plan.md\`"
    } > "$out"
  done
done

echo "Created simple remediation plans"
