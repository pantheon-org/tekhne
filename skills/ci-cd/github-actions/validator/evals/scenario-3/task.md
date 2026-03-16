# Validate a Reusable Workflow Pair

## Problem/Feature Description

A platform team has introduced a reusable workflow pattern to standardize how microservices run their tests. They have a shared `test-template.yml` workflow (the callee) and a `service-ci.yml` (the caller) that invokes it. Before enabling this pattern across 20 services, they want the pair validated to catch any issues that static analysis might surface, including expression type errors and input/output mismatches that only appear when both files are considered together.

Validate both workflow files, document any issues found in either file, and note any limitations where runtime-only behavior cannot be verified by static analysis alone.

## Output Specification

Produce:
- A `validation-report.md` documenting: (1) results for the callee workflow, (2) results for the caller workflow, (3) any cross-file issues, (4) limitations of static analysis for this scenario

## Input Files

The following files are provided as input. Extract them before beginning.

=============== FILE: inputs/test-template.yml ===============
name: Test Template

on:
  workflow_call:
    inputs:
      node-version:
        required: true
        type: string
      run-coverage:
        required: false
        type: boolean
        default: false
    secrets:
      NPM_TOKEN:
        required: false

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: ${{ inputs.node-version }}
      - run: npm ci
        env:
          NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
      - run: npm test
      - name: Upload coverage
        if: ${{ inputs.run-coverage }}
        uses: actions/upload-artifact@v4
        with:
          name: coverage
          path: coverage/

=============== FILE: inputs/service-ci.yml ===============
name: Service CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  run-tests:
    uses: ./.github/workflows/test-template.yml
    with:
      node-version: '20'
      run-coverage: true
    secrets:
      NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
