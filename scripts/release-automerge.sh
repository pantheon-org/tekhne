#!/usr/bin/env bash
#
# release-automerge.sh
#
# Squash-merges the open release-please PR under the pantheon-ai-bot App token, so
# downstream workflows on the merge and the tag fire.
# Run by .github/workflows/release-automerge.yml.
# Env: GH_TOKEN (App token), GH_REPO (owner/name)

set -euo pipefail

# Select by the release-please branch (stable) rather than by title:
# this is a multi-package manifest release, so the aggregate PR title
# varies with which components changed. The head branch name is fixed.
pr=$(gh pr list --repo "$GH_REPO" \
  --state open \
  --head 'release-please--branches--main' \
  --json number,title,mergeStateStatus,isDraft \
  --jq '.[0] // empty')

if [ -z "$pr" ]; then
  echo "No open release PR found. Nothing to merge."
  echo "No open release PR found." >> "$GITHUB_STEP_SUMMARY"
  exit 0
fi

number=$(jq -r '.number' <<<"$pr")
title=$(jq -r '.title' <<<"$pr")
state=$(jq -r '.mergeStateStatus' <<<"$pr")
draft=$(jq -r '.isDraft' <<<"$pr")
echo "Found release PR #$number (\"$title\", mergeStateStatus=$state, draft=$draft)"

if [ "$draft" = "true" ]; then
  echo "PR #$number is a draft; skipping." | tee -a "$GITHUB_STEP_SUMMARY"
  exit 0
fi

# DIRTY/BLOCKED/BEHIND mean the PR cannot merge cleanly. pr-auto-rebase.yml
# deliberately skips the release-please branch, so a BEHIND release PR is
# expected here: refresh it in place so the next scheduled run can merge
# the now up-to-date branch, rather than force-merging a stale one.
case "$state" in
  BEHIND)
    echo "PR #$number is BEHIND main; updating its branch and deferring the merge to the next run."
    gh pr update-branch "$number" --repo "$GH_REPO" --rebase || \
      echo "::warning::Could not rebase PR #$number; it needs a manual rebase."
    echo "- PR #$number: was BEHIND, rebased; merge deferred to next run." >> "$GITHUB_STEP_SUMMARY"
    exit 0
    ;;
  DIRTY|BLOCKED)
    echo "::warning::PR #$number is not in a mergeable state ($state); skipping."
    echo "- PR #$number: not mergeable ($state); skipped." >> "$GITHUB_STEP_SUMMARY"
    exit 0
    ;;
esac

gh pr merge "$number" --repo "$GH_REPO" --squash --delete-branch
echo "Merged release PR #$number. Downstream release workflows will now run."
echo "- PR #$number: merged :white_check_mark: — downstream workflows triggered." >> "$GITHUB_STEP_SUMMARY"
