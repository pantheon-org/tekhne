#!/usr/bin/env bash
#
# audit-changed.sh
#
# Grades the changed skills in one batch call and classifies each against the base
# commit. Run by .github/workflows/skill-audit.yml.
# Env:    DIRS (space-separated changed skill directories)
# Reads:  base.json (optional)
# Writes: batch.json, verdict.json, details.md, unscored.txt; table, fail and debt to $GITHUB_OUTPUT

set -euo pipefail
# shellcheck disable=SC2153 # DIRS is set by the workflow env
read -r -a dirs <<<"$DIRS"
rels=()
for dir in "${dirs[@]}"; do
  rels+=("${dir#skills/}")
done
pantheon-skill-auditor batch "${rels[@]}" --json > batch.json || echo '[]' > batch.json
[ -f base.json ] || echo '[]' > base.json
bash scripts/skill-audit/classify.sh batch.json base.json > verdict.json
# The per-skill detail blocks are cosmetic. They come from a second
# run so that nothing in that path can change batch.json, the table or
# `fail` above. A failure here leaves an empty file, never a red job.
pantheon-skill-auditor batch "${rels[@]}" --json --details-markdown details.md > /dev/null 2>&1 || : > details.md
[ -f details.md ] || : > details.md
rows=$(jq -r '
  .[]
  | (if .tier == "pass" then "✅ pass"
     elif .tier == "warn" then "⚠️ warning"
     elif .tier == "debt" then "⚠️ below B, not lower than base (tracked)"
     else "❌ fail (" + .reason + ")" end) as $tier
  | "| `\(.skill)` | \(.grade) | \(.total)/\(.maxTotal) | \(if .baseTotal == null then "new" else (.baseTotal | tostring) end) | \($tier) |"
' verdict.json)
fail=$(jq '[.[] | select(.tier == "fail")] | length' verdict.json)
debt=$(jq '[.[] | select(.tier == "debt")] | length' verdict.json)
got=$(jq 'length' batch.json)
want=${#rels[@]}
if [ "$got" -lt "$want" ]; then
  fail=$((fail + want - got))
fi
# Skills that were asked for but did not come back, one per line, for
# the comment. Written to a file, not GITHUB_OUTPUT, because the names
# come from the PR; this does not feed `fail`, and a failure here
# leaves an empty list rather than failing the step.
{
  jq -r '.[] | (.skill | sub(".*/skills/"; "") | sub("/SKILL.md$"; ""))' batch.json > scored.txt
  for rel in "${rels[@]}"; do
    grep -qxF -- "$rel" scored.txt || printf '%s\n' "$rel"
  done > unscored.txt
} || : > unscored.txt
{
  echo "table<<AUDIT_EOF"
  printf '| Skill | Grade | Score | Base | Tier |\n'
  printf '| --- | --- | --- | --- | --- |\n'
  printf '%s\n' "$rows"
  echo "AUDIT_EOF"
  echo "fail=$fail"
  echo "debt=$debt"
} >> "$GITHUB_OUTPUT"
