#!/usr/bin/env bash
# Pass/fail table for the PR-title regex in
# .github/workflows/conventional-commits.yml.
#
# The gate runs on pull_request_target and rejects titles, so a mistake in
# the regex blocks every pull request including release-please's. The rows
# below are real titles from this repository plus release-please's default
# title forms.
set -euo pipefail

workflow=".github/workflows/conventional-commits.yml"
pattern=$(grep -o "pattern='[^']*'" "$workflow" | head -1 | sed "s/^pattern='//;s/'$//")
[ -n "$pattern" ] || { echo "could not extract the regex from $workflow" >&2; exit 1; }

fails=0
check() { # check <expected pass|fail> <title>
    local want="$1" title="$2" got
    if [[ "$title" =~ $pattern ]]; then got=pass; else got=fail; fi
    if [ "$got" != "$want" ]; then
        printf '  MISMATCH want=%-4s got=%-4s %s\n' "$want" "$got" "$title"
        fails=$((fails + 1))
    fi
}

# Real merged titles from this repository.
check pass "fix: stop the parity gates rotting on skill edits"
check pass "feat(context-index): add check-wave-evidence.sh sibling"
check pass "ci: give every job a timeout"
check pass "chore(deps): bump @astrojs/mdx from 7.0.8 to 8.0.0 in /docs"
check pass "chore(deps-dev): bump tessl from 0.92.0 to 0.99.0"
check pass "ci(deps): bump the actions-minor-patch group with 3 updates"
check pass "feat!: drop the Go validator"
check pass "refactor(skill-auditor)!: rename the scoring entry point"

# release-please default title forms. If any of these fail, every release
# pull request is blocked by this gate.
check pass "chore(main): release 1.2.3"
check pass "chore: release main"
check pass "chore(skills/agentic-harness/agents-md): release 1.0.1"

# Must be rejected.
check fail "Add blocks/blocked-by dependency links"   # real PR #284: no type
check fail "fix stop the parity gates rotting"        # no colon
check fail "fix: Stop the parity gates rotting"       # uppercase description
check fail "security: patch the thing"                # type release-please ignores
check fail "feat:"                                    # empty description
check fail "feat: "                                   # whitespace-only description
check fail ": no type at all"

if [ "$fails" -gt 0 ]; then
    echo "commit-title regex: $fails row(s) mismatched" >&2
    exit 1
fi
echo "commit-title regex OK: all rows behave as expected"
