#!/usr/bin/env bash
#
# assemble-comment.sh
#
# Builds comment.md, the Skill Audit pull request comment, and drops the per-skill detail
# when it would exceed MAX_COMMENT_BYTES. Run by .github/workflows/skill-audit.yml.
# Env:    TABLE, DUP_SUMMARY, MAX_COMMENT_BYTES
# Reads:  unscored.txt, tracked-issues.md, drift.md, details.md (each optional)
# Writes: comment.md

set -euo pipefail
escape() {
  sed -e 's/&/\&amp;/g' -e 's/</\&lt;/g' -e 's/>/\&gt;/g' -e 's/`/\&#96;/g' -e 's/@/\&#64;/g'
}
build() {
  printf '## Skill Audit\n\n%s\n\n' "$TABLE"
  printf '**Gate:**\n- ✅ pass = A / A+ (≥126)\n- ⚠️ warning = B – B+ (112–125, non-blocking)\n- ⚠️ below B, not lower than base = known debt (non-blocking, tracked by an issue)\n- ❌ fail = below B (<112) and either a new skill or scoring lower than on the base branch (blocks merge)\n\n'
  if [ -s unscored.txt ]; then
    printf 'Not scored (the audit could not read them): '
    escape < unscored.txt | awk '{ printf "%s<code>%s</code>", (NR > 1 ? ", " : ""), $0 } END { print "" }'
    printf '\n'
  fi
  if [ -s tracked-issues.md ]; then
    printf '**Tracking issues** (one open issue per skill below B that this pull request did not lower, filed automatically):\n\n'
    cat tracked-issues.md
    printf '\n'
  fi
  if [ -s drift.md ]; then
    printf '**Dropped trigger phrases** (advisory; confirm the description still names each intent, or restore the phrase):\n\n'
    cat drift.md
    printf '\n'
  fi
  if [ "$1" = 1 ] && [ -s details.md ]; then
    printf '**Where the points were lost** (the three biggest losses for each skill graded below A):\n\n'
    cat details.md
    printf '\n'
  fi
  printf '<details>\n<summary>Cross-skill duplication (advisory)</summary>\n\n%s\n\n</details>\n' "$DUP_SUMMARY"
}
build 1 > comment.md
if [ "$(wc -c < comment.md)" -gt "$MAX_COMMENT_BYTES" ]; then
  echo "::warning::Skill Audit comment is over ${MAX_COMMENT_BYTES} bytes; posting without the per-skill detail."
  build 0 > comment.md
fi
