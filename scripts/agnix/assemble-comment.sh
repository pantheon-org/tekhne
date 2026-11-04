#!/usr/bin/env bash
#
# assemble-comment.sh
#
# Builds comment.md, the body of the "## agnix" pull request comment, from the agnix
# output and the introduced-errors list.
# Run by .github/workflows/agnix.yml from the workspace root.
# Reads:  agnix.json, introduced-errors.txt, tracked-issues.md (optional)
# Env:    AGNIX_VERSION
# Writes: comment.md

set -euo pipefail
errors=$(jq '.summary.errors' agnix.json)
warnings=$(jq '.summary.warnings' agnix.json)
files=$(jq '.files_checked' agnix.json)
introduced=0
if [ -s introduced-errors.txt ]; then introduced=$(wc -l < introduced-errors.txt | tr -d ' '); fi
existing=$((errors - introduced))
if [ "$introduced" -eq 0 ]; then verdict="PASS"; else verdict="FAIL"; fi
escape() { sed -e 's/&/\&amp;/g' -e 's/</\&lt;/g' -e 's/>/\&gt;/g' -e 's/`/\&#96;/g' -e 's/@/\&#64;/g' -e 's/|/\&#124;/g'; }
{
  echo "## agnix"
  echo
  if [ "$introduced" -eq 0 ]; then
    echo "**${verdict}**: this pull request introduces no errors. agnix ${AGNIX_VERSION} checked ${files} file(s): ${existing} error(s) already on \`main\`, ${warnings} warning(s)."
  else
    echo "**${verdict}**: this pull request introduces ${introduced} error(s). agnix ${AGNIX_VERSION} checked ${files} file(s): ${existing} other error(s) already on \`main\`, ${warnings} warning(s)."
    echo
    echo "| Rule | File |"
    echo "| --- | --- |"
    head -n 50 introduced-errors.txt \
      | while IFS=$'\t' read -r rule file; do
          printf '| %s | %s |\n' "$(printf '%s' "$rule" | escape)" "$(printf '%s' "$file" | escape)"
        done
  fi
  echo
  echo "Only an error the pull request introduces fails this check. Errors already on \`main\` and warnings are tracked in GitHub issues and do not. Findings are also shown as annotations on the changed files."
  if [ -s tracked-issues.md ]; then
    echo
    echo "Tracked in:"
    echo
    cat tracked-issues.md
  fi
  if [ "$warnings" -gt 0 ]; then
    echo
    echo "<details><summary>Warnings by rule</summary>"
    echo
    jq -r '[.diagnostics[] | select(.level == "warning") | .rule] | group_by(.) | map("- \(.[0]): \(length)") | .[]' agnix.json
    echo
    echo "</details>"
  fi
} > comment.md
