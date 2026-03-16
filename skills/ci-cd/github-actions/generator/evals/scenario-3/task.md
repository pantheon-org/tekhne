# PR Labeler and Coverage Uploader for Open-Source Repository

## Problem/Feature Description

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
