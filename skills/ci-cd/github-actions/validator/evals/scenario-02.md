# Scenario 02: Modernize a Legacy CI Workflow

## User Prompt

A company's engineering team has inherited a CI workflow written two years ago. Their security scan tooling has started flagging the workflow as using outdated dependencies, but the team isn't sure what specifically needs updating or why it matters. They need the workflow reviewed, any outdated or deprecated actions identified, and the workflow brought up to current best practices before deploying it to their new repository.

Review the workflow below, identify all actions that are outdated or deprecated, explain the risk each outdated action poses, update them to current versions, and produce a final corrected workflow.

## Output Specification

Produce:
- A corrected `ci.yml` with all actions updated
- A `modernization-report.md` with a table showing: each action found, its original version, the updated version used, and why the update matters

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/ci.yml ===============
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm ci
      - run: npm test
      - uses: actions/upload-artifact@v2
        with:
          name: test-results
          path: coverage/

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm run deploy

## Expected Behavior

1. Identify `actions/checkout@v2` as outdated and update it to a newer version (v3, v4, or later) in both jobs
2. Identify `actions/setup-node@v2` as outdated and update it to a newer version in both jobs
3. Identify `actions/upload-artifact@v2` as outdated and update it to a newer version
4. Use full SHA commit hashes for at least one updated action reference
5. Explain that deprecated versions may not receive security patches
6. Produce a report table with: action name, original version, updated version, and reason
7. Ensure no @v2 references remain in the corrected ci.yml

## Success Criteria

- **checkout updated**: actions/checkout is updated from @v2 to a newer version (v3, v4, or later)
- **setup-node updated**: actions/setup-node is updated from @v2 to a newer version
- **upload-artifact updated**: actions/upload-artifact is updated from @v2 to a newer version
- **SHA pinning used**: At least one updated action reference uses a full SHA commit hash (40-character hex) rather than a tag version
- **Deprecation risk explained**: modernization-report.md explains that deprecated versions may not receive security patches
- **Report table present**: modernization-report.md includes a table or list with: original version, updated version, and reason for each action
- **All occurrences updated**: Both jobs in ci.yml have their actions updated (checkout and setup-node appear in both jobs and both are updated)
- **No v2 references remain**: No @v2 references remain in ci.yml

## Failure Conditions

- `actions/checkout` is not updated from @v2 to a newer version
- `actions/setup-node` is not updated from @v2 to a newer version in both jobs
- `actions/upload-artifact` is not updated from @v2 to a newer version
- No updated action uses a full SHA commit hash
- Report does not explain the security risk of leaving deprecated versions in place
- Report does not include a table or list with original version, updated version, and reason
- Either the build or deploy job still has @v2 references for checkout or setup-node
- Any @v2 reference remains in the corrected ci.yml
