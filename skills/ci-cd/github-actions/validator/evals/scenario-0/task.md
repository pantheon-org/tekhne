# Secure a Pull Request Automation Workflow

## Problem/Feature Description

A platform team maintains a GitHub Actions workflow that automatically posts a comment on pull requests with a summary of what changed. A security researcher has flagged that the workflow contains a code injection risk but didn't provide specifics. The team needs the workflow analyzed and the security issue corrected before it goes back into production. The workflow runs on every pull request opened or updated against the main branch.

Analyze the workflow below, identify the security issue(s), explain what attack vector they expose, and produce a corrected version of the workflow file.

## Output Specification

Produce:
- A corrected `pr-comment.yml` with the security issue(s) fixed
- A `validation-report.md` documenting: each issue found, the type of issue, and what fix was applied

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/pr-comment.yml ===============
name: PR Comment

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  comment:
    runs-on: ubuntu-latest
    permissions:
      pull-requests: write
    steps:
      - name: Post PR summary
        run: |
          echo "PR Title: ${{ github.event.pull_request.title }}"
          echo "Author: ${{ github.event.pull_request.user.login }}"
          gh pr comment ${{ github.event.pull_request.number }} --body "Review requested for: ${{ github.event.pull_request.title }}"
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
