# Scenario 04: PR Labeler and Coverage Uploader for Open-Source Repository

## User Prompt

A popular open-source project on GitHub receives many pull requests from external contributors (forks). The maintainers want two automated behaviours:

1. **Auto-label PRs** based on the PR title: if the title starts with `fix:` apply the `bug` label; if it starts with `feat:` apply the `enhancement` label. This requires write access to the repository to add labels.

2. **Upload code coverage** to an external service after CI passes. The upload requires a `CODECOV_TOKEN` secret that must not be exposed to untrusted fork code.

A previous engineer drafted a single workflow that uses `pull_request_target` so it can access secrets, and it checks out the PR's head commit directly. Before shipping this, the team wants it reviewed and rewritten following safe patterns. The draft is provided below.

## Output Specification

Produce a corrected and production-ready YAML file (or files) that implement both requirements safely. Save your output as `.github/workflows/pr-automation.yml` (and optionally a second workflow file if splitting is the safer approach).

## Input Files

The following draft workflow is provided. Extract it before beginning.

=============== FILE: draft-pr-automation.yml ===============
name: PR Automation

on:
  pull_request_target:
    types: [opened, synchronize]

jobs:
  label-and-coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          ref: ${{ github.event.pull_request.head.sha }}

      - name: Run tests and coverage
        run: npm test -- --coverage

      - name: Upload coverage
        run: |
          curl -s https://codecov.io/bash -o codecov.sh
          bash codecov.sh -t ${{ secrets.CODECOV_TOKEN }}

      - name: Label PR based on title
        run: |
          TITLE="${{ github.event.pull_request.title }}"
          if echo "$TITLE" | grep -q "^fix:"; then
            gh pr edit ${{ github.event.number }} --add-label bug
          elif echo "$TITLE" | grep -q "^feat:"; then
            gh pr edit ${{ github.event.number }} --add-label enhancement
          fi
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}

## Expected Behavior

1. Remove the dangerous combination of `pull_request_target` with a checkout of the PR head SHA — this exposes secrets to untrusted fork code
2. Pass PR title (and other user-supplied event fields) through an `env:` block variable rather than interpolating directly with `${{ }}` inside shell scripts
3. Ensure the coverage upload step (which uses CODECOV_TOKEN) does not run in a context where untrusted fork code has been checked out and executed
4. Pin all action references with full 40-character SHA hashes
5. Include a `permissions:` block and scope write permissions (e.g., `pull-requests: write`) only at the job level that needs them
6. Add a `concurrency:` block with `cancel-in-progress: true`

## Success Criteria

- **No pull_request_target + head checkout**: The output does NOT combine `pull_request_target` with a checkout step that uses `ref: ${{ github.event.pull_request.head.sha }}` or any PR head ref
- **PR title via env var**: When the PR title (or other user-supplied event field) is used in a `run:` step, it is passed through an environment variable rather than interpolated directly with `${{ }}` inside the shell script
- **Secrets not exposed to fork code**: The coverage upload step (or equivalent secret-using step) runs in a context where untrusted fork code from the PR has NOT been checked out and executed
- **SHA-pinned actions**: All `uses:` steps in the output reference actions via full 40-character SHA
- **Top-level permissions block**: The workflow(s) produced include a `permissions:` block
- **Minimal write permissions scoped**: Write permissions (e.g., `pull-requests: write`) are granted only at the job level that needs them, not as a global `write-all`
- **Concurrency block present**: At least one workflow file contains a `concurrency:` block with `cancel-in-progress: true`

## Failure Conditions

- `pull_request_target` is retained and combined with a checkout of the PR head SHA
- PR title or other user-supplied event data is directly interpolated with `${{ }}` inside a `run:` shell script
- The CODECOV_TOKEN secret is used in a context where untrusted fork code has been checked out and executed
- Any action uses a tag reference (e.g., `@v4`) instead of a full 40-character SHA
- No `permissions:` block is present
- Write permissions are granted globally (write-all) instead of scoped to the specific job that needs them
- No `concurrency:` block with `cancel-in-progress: true` is present
