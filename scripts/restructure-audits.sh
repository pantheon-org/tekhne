#!/usr/bin/env sh
# Restructure audits to .context/audits/<skill>/<date>/ format

SKILLS="acceptance-criteria agents-md bdd-testing biome-complete bun-development cdk-nag cfn-behavior-validator cfn-template-compare colyseus-multiplayer commanderjs conventional-commits create-context-file extending-nx-plugins github-copilot-models implementation-plan-splitter markdown-authoring mise-complete moscow-prioritization nx-biome-integration nx-bun-integration nx-executors nx-generators nx-vite-integration nx-workspace-patterns opencode-config plain-english skill-quality-auditor software-design-principles test-driven-development typescript-advanced ui-debug-workflow"

DATES="2026-02-21 2026-02-22 2026-02-23"

for skill in $SKILLS; do
  for date in $DATES; do
    mdfile=".context/audits/${skill}-audit-${date}.md"
    out_dir=".context/audits/${skill}/${date}"
    
    if [ -f "$mdfile" ]; then
      mkdir -p "$out_dir"
      
      total=$(grep -oP '\*\*Total Score\*\*\s*\|\s*\K[0-9]+' "$mdfile" | head -1)
      grade=$(grep -oP '\*\*Grade\*\*\s*\|\s*\K[A-F][+]?' "$mdfile" | head -1)
      lines=$(grep -oP 'File size is\s*\K[0-9]+' "$mdfile" | head -1)
      refs=$(grep -oP 'references count is\s*\K[0-9]+' "$mdfile" | head -1)
      
      [ -z "$total" ] && total=0
      [ -z "$grade" ] && grade="F"
      [ -z "$lines" ] && lines=0
      [ -z "$refs" ] && refs=0
      
      cat > "$out_dir/audit.json" <<EOF
{
  "skill": "$skill",
  "auditDate": "$date",
  "totalScore": $total,
  "grade": "$grade",
  "fileSize": $lines,
  "referenceCount": $refs
}
EOF

      cp "$mdfile" "$out_dir/analysis.md"
      
      plan=".context/plans/${skill}-remediation-plan.md"
      if [ -f "$plan" ]; then
        cat > "$out_dir/remediation-plan.md" <<EOF
# Remediation Plan for $skill

**Date:** $date

$(cat "$plan")
EOF
      else
        cat > "$out_dir/remediation-plan.md" <<EOF
# Remediation Plan for $skill

**Date:** $date

No remediation plan exists yet. Run skill-judge to generate recommendations.
EOF
      fi
      
      echo "Created $out_dir/"
    fi
  done
  
  latest_dir=".context/audits/${skill}/2026-02-23"
  if [ -d "$latest_dir" ]; then
    rm -f ".context/audits/${skill}/latest"
    ln -s 2026-02-23 ".context/audits/${skill}/latest"
    echo "Linked latest for $skill"
  fi
done
