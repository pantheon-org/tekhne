# Scenario 03: Diagnose and Fix a Multi-Error Workflow

## User Prompt

A new hire at a startup has written their first GitHub Actions workflow for a release pipeline. The workflow has multiple problems and won't run correctly on GitHub. A senior engineer needs to audit it, document every issue found, categorize each by type, and deliver a corrected version along with clear explanations the new hire can learn from.

Review the workflow below, identify and explain every error, and produce a corrected version. Make sure the report clearly maps each error to its category (scheduling, runner, syntax, security, action version, job dependency, etc.) and includes the corrected code for each fix.

## Output Specification

Produce:
- A corrected `release.yml` with all issues fixed
- A `audit-report.md` with a table that lists each error found, its category/type, the explanation, and the corrected code snippet

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/release.yml ===============
name: Release Pipeline

on:
  schedule:
    - cron: '0 0 * * 8'
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-lastest
    steps:
      - uses: actions/checkout@v3
      - run: |
          echo "Testing PR: ${{ github.event.pull_request.title }}"
          npm test

  publish:
    needs: tset
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm publish
        env:
          NPM_TOKEN: ${{ secrets.NPM_TOKEN }}

## Expected Behavior

1. Identify the invalid cron value `0 0 * * 8` (weekday field out of range 0-6) and correct it to a valid cron expression
2. Identify `ubuntu-lastest` as a typo for `ubuntu-latest` and correct it
3. Identify `needs: tset` as referencing a non-existent job (typo for `test`) and correct it
4. Identify the `${{ github.event.pull_request.title }}` interpolation in a `run:` step as a script injection risk and fix it
5. Categorize each error by type in the audit report
6. Note that `actions/checkout@v3` could be updated to a newer version
7. Include corrected code snippets in the report for each error found

## Success Criteria

- **CRON error identified**: audit-report.md identifies the invalid cron value '0 0 * * 8' (weekday out of range 0-6) as an error
- **CRON error fixed**: In release.yml, the cron expression is corrected to a valid value (weekday field is 0-6)
- **Runner typo identified**: audit-report.md identifies 'ubuntu-lastest' as an invalid runner label
- **Runner typo fixed**: In release.yml, 'ubuntu-lastest' is corrected to 'ubuntu-latest'
- **Job dependency typo identified**: audit-report.md identifies 'needs: tset' as referencing a non-existent job
- **Job dependency typo fixed**: In release.yml, 'needs: tset' is corrected to 'needs: test'
- **Script injection identified**: audit-report.md identifies the ${{ github.event.pull_request.title }} interpolation in run: as a script injection risk
- **Script injection fixed**: In release.yml, the untrusted value is routed through an env: variable instead of direct interpolation
- **Error categories present**: audit-report.md categorizes each error by type (e.g., CRON/Schedule, Runner, Job Dependency, Security/Injection, or similar labels)
- **Outdated action noted**: audit-report.md notes that actions/checkout@v3 could be updated to a newer version
- **Fix code quoted**: audit-report.md includes at least one corrected code snippet per error (not just a description)

## Failure Conditions

- The invalid cron weekday value (8) is not identified as an error
- Corrected workflow still has an invalid cron expression
- `ubuntu-lastest` typo is not flagged
- Corrected workflow still has `ubuntu-lastest` instead of `ubuntu-latest`
- `needs: tset` is not identified as referencing a non-existent job
- Corrected workflow still has `needs: tset`
- The direct ${{ }} interpolation of PR title in a run: step is not flagged as a security risk
- Corrected workflow still uses direct ${{ }} interpolation of the PR title in a run: step
- Report does not categorize errors by type
- Report provides no corrected code snippets
