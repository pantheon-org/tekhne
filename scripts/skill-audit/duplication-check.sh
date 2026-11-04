#!/usr/bin/env bash
#
# duplication-check.sh
#
# Cross-skill duplication scan; warns when a skill changed in the pull request is in a
# high-similarity pair. Run by .github/workflows/skill-audit.yml.
# Env:    DIRS (space-separated changed skill directories)
# Writes: duplication.json, duplication-relevant.json; relevant and summary to $GITHUB_OUTPUT

set -euo pipefail
# shellcheck disable=SC2153 # DIRS is set by the workflow env
read -r -a dirs <<<"$DIRS"
pantheon-skill-auditor duplication skills --threshold 70 --json > duplication.json || echo '[]' > duplication.json
changed=$(for dir in "${dirs[@]}"; do basename "$dir"; done | sort -u | jq -R . | jq -cs .)
jq -c --argjson names "$changed" \
  '[.[] | select((.name1 as $a | $names | index($a)) or (.name2 as $b | $names | index($b)))]' \
  duplication.json > duplication-relevant.json
n=$(jq 'length' duplication-relevant.json)
echo "relevant=$n" >> "$GITHUB_OUTPUT"
# Render a Markdown summary for the PR comment (surfaced in the
# "Cross-skill duplication" details block of the Skill Audit comment).
{
  echo "summary<<DUP_EOF"
  if [ "$n" -gt 0 ]; then
    echo "**${n} high-similarity (>=70%) pair(s) involving a skill changed in this PR** — check for accidental copy-paste:"
    echo ""
    echo "| Skill A | Skill B | Similarity | Common lines |"
    echo "| --- | --- | --- | --- |"
    jq -r '.[] | "| `\(.name1)` | `\(.name2)` | \(.similarity)% | \(.common_lines) |"' duplication-relevant.json
  else
    echo "No high-similarity (>=70%) pairs involving skills changed in this PR."
  fi
  echo "DUP_EOF"
} >> "$GITHUB_OUTPUT"
if [ "$n" -gt 0 ]; then
  echo "::warning::${n} high-similarity (>=70%) skill pair(s) involve a skill changed in this PR — check for accidental copy-paste. Advisory only; see the Skill Audit comment."
fi
