#!/usr/bin/env bash
#
# changed-skills.sh
#
# Read changed file paths (one per line, repo-relative) on stdin and print the
# skills they belong to, one per line, as keys relative to skills/ (the form
# `pantheon-skill-auditor` takes). Sorted and de-duplicated.
#
# A path belongs to the nearest folder above it that holds a SKILL.md, so a
# change to evals/, references/, scripts/ or assets/ counts, not only SKILL.md.
# The auditor scores all of those. Stored audits (.audits/) are ignored so that
# storing an audit never retriggers itself, and paths whose skill no longer
# exists (a deleted skill) are skipped.
#
# The pre-push hook and CI both use this, so the two cannot disagree about which
# skills a change affects.
#
# Usage: git diff --name-only A...B | scripts/stored-audits/changed-skills.sh [repo_root]
set -euo pipefail

root="${1:-$(git rev-parse --show-toplevel)}"

while IFS= read -r path; do
  case "$path" in
    skills/*) ;;
    *) continue ;;
  esac
  case "$path" in
    */.audits/*) continue ;;
  esac
  dir="$(dirname "$path")"
  while [ "$dir" != "skills" ] && [ "$dir" != "." ]; do
    if [ -f "$root/$dir/SKILL.md" ]; then
      echo "${dir#skills/}"
      break
    fi
    dir="$(dirname "$dir")"
  done
done | sort -u
