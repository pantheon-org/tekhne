# Scenario 02: Plan a Baseline-vs-Changed Evidence Capture Session

## User Prompt

A team is investigating a visual regression in a checkout flow. The fix branch is `fix/checkout-button-alignment`. The app runs at `http://localhost:4200` with `npm run start`. Both runs must use the same seeded test account.

Using the UI Debug Workflow skill, produce a session plan document saved to `evidence-capture-plan.md` that documents exactly how the baseline and changed evidence will be collected and compared.

Your output must include:

1. The exact `capture-evidence.sh` command for the baseline run, with output directory.
2. The exact `capture-evidence.sh` command for the changed run, with output directory.
3. The exact `compare-evidence.sh` command referencing both output directories.
4. A checklist of conditions that MUST match between baseline and changed runs (URL, viewport, seed data, build mode).
5. Instructions for what to do if unexpected differences appear in the comparison output.

## Expected Behavior

1. Include a `capture-evidence.sh` invocation with a `baseline` label and a named output directory (e.g. `./baseline`) in `evidence-capture-plan.md`
2. Include a `capture-evidence.sh` invocation with a `changed` label referencing `http://localhost:4200` and a separate output directory
3. Include a `compare-evidence.sh` invocation referencing both output directories and producing a comparison output file
4. List at least 4 conditions that must match between runs: URL, viewport size, seed data, and build mode
5. Describe the 3-step response to unexpected comparison differences: verify conditions, check for flaky elements, re-capture both runs

## Success Criteria

- **Correct baseline capture command**: `evidence-capture-plan.md` contains a `capture-evidence.sh` invocation with `baseline` label and a named output directory (e.g. `./baseline`)
- **Correct changed capture command**: `evidence-capture-plan.md` contains a `capture-evidence.sh` invocation with `changed` label referencing `http://localhost:4200` and a separate output directory
- **Correct compare command**: `evidence-capture-plan.md` contains a `compare-evidence.sh` invocation referencing both output directories and producing a comparison output file
- **Identical conditions checklist**: The document lists at least 4 conditions that must match between runs: URL, viewport size, seed data, and build mode
- **Unexpected differences handling**: The document describes the 3-step response to unexpected comparison differences: verify conditions, check for flaky elements, re-capture both runs

## Failure Conditions

- `evidence-capture-plan.md` does not include a `capture-evidence.sh` command with a `baseline` label and output directory
- The `changed` run command does not reference `http://localhost:4200` or uses the same output directory as the baseline
- No `compare-evidence.sh` command is included referencing both output directories
- Fewer than 4 matching conditions are listed (URL, viewport, seed data, build mode)
- The response to unexpected differences does not cover all three steps: verify conditions, check for flaky elements, re-capture both runs
