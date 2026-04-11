# Scenario 04: Validate a Reusable Workflow Pair

## User Prompt

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

## Expected Behavior

1. Validate both the callee (test-template.yml) and the caller (service-ci.yml) — not just one in isolation
2. Analyze the callee's workflow_call inputs, jobs, and steps
3. Analyze the caller's `uses:` reference and `with:`/`secrets:` blocks
4. Check whether the caller's `with:` inputs match the callee's declared inputs (node-version, run-coverage)
5. Acknowledge at least one limitation where static analysis cannot fully verify runtime behavior
6. Include a section or mention of cross-file or caller-callee relationship concerns

## Success Criteria

- **Both files validated**: validation-report.md contains a section covering the callee (test-template.yml) AND a section covering the caller (service-ci.yml) — not just one
- **Callee analysis present**: The report includes analysis of test-template.yml's workflow_call inputs, jobs, and steps
- **Caller analysis present**: The report includes analysis of service-ci.yml's uses: reference and with:/secrets: blocks
- **Input compatibility checked**: The report notes whether the caller's with: inputs match the callee's declared inputs (node-version, run-coverage)
- **Static analysis limits acknowledged**: The report notes at least one limitation where static analysis cannot fully verify runtime behavior (e.g., secret availability, input type coercion, or matrix context)
- **No isolation-only approach**: The agent does NOT report only on one of the two files and declare the validation complete
- **Cross-file issues section**: The report includes a section or mention of cross-file or caller-callee relationship concerns

## Failure Conditions

- The report only validates test-template.yml or only validates service-ci.yml, not both
- No analysis of the callee's workflow_call inputs, jobs, or steps is present
- No analysis of the caller's `uses:` reference or `with:`/`secrets:` blocks is present
- The report does not check whether the caller's inputs are compatible with the callee's declarations
- The report does not acknowledge any limitations of static analysis
- The report declares validation complete after reviewing only one file
- No mention of cross-file issues or the caller-callee relationship
