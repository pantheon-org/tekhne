#!/usr/bin/env bash
#
# pr-auto-rebase.sh
#
# Rebases open, non-draft PR branches that are behind main, so the strict
# up-to-date ruleset does not leave them blocked after every push to main.
# Run by .github/workflows/pr-auto-rebase.yml under the pantheon-ai-bot App token.
# Env: GH_TOKEN (App token), GH_REPO (owner/name)

set -euo pipefail

# Candidates: open, non-draft PRs, excluding the release-please PR
# (release-please regenerates and force-pushes its own branch on every
# push to main, so rebasing it here would race with that workflow).
mapfile -t prs < <(
  gh pr list --state open --limit 100 \
    --json number,isDraft,author,headRefName \
    -q '.[]
        | select(.isDraft == false)
        | select(.author.login != "app/github-actions")
        | select((.headRefName | startswith("release-please--")) | not)
        | .number'
)

if [ "${#prs[@]}" -eq 0 ]; then
  echo "No candidate PRs to rebase."
  echo "No candidate PRs to rebase." >> "$GITHUB_STEP_SUMMARY"
  exit 0
fi

echo "### Auto-rebase results" >> "$GITHUB_STEP_SUMMARY"

for n in "${prs[@]}"; do
  head="$(gh pr view "$n" --json headRefName -q .headRefName)"

  # `mergeStateStatus` is computed lazily and returns UNKNOWN for a
  # while right after a push to main, i.e. exactly when this runs. The
  # compare API is synchronous: behind_by is how many commits `main`
  # has that the PR branch lacks, so behind_by > 0 means it needs a
  # rebase.
  behind="$(gh api "repos/$GH_REPO/compare/main...$head" -q '.behind_by' 2>/dev/null || echo "")"

  if [ -z "$behind" ]; then
    echo "PR #$n ($head): could not compare, skipping"
    echo "- PR #$n: compare failed, skipped :grey_question:" >> "$GITHUB_STEP_SUMMARY"
    continue
  fi

  if [ "$behind" -eq 0 ]; then
    echo "PR #$n ($head): up to date, skipping"
    echo "- PR #$n: up to date, skipped" >> "$GITHUB_STEP_SUMMARY"
    continue
  fi

  if gh pr update-branch "$n" --rebase; then
    echo "PR #$n ($head): rebased onto main (was behind by $behind)"
    echo "- PR #$n: rebased :white_check_mark: (was behind by $behind)" >> "$GITHUB_STEP_SUMMARY"
  else
    echo "::warning title=Auto-rebase failed::PR #$n could not be rebased (likely a merge conflict); needs a manual rebase"
    echo "- PR #$n: conflict, needs manual rebase :warning:" >> "$GITHUB_STEP_SUMMARY"
  fi
done
