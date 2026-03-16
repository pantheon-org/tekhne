# Scenario 2: Plan a Baseline-vs-Changed Evidence Capture Session

## Context

A team is investigating a visual regression in a checkout flow. The fix branch is
`fix/checkout-button-alignment`. The app runs at `http://localhost:4200` with
`npm run start`. Both runs must use the same seeded test account.

## Your Task

Using the UI Debug Workflow skill, produce a session plan document saved to
`evidence-capture-plan.md` that documents exactly how the baseline and changed
evidence will be collected and compared.

## Requirements

Your output must include:

1. The exact `capture-evidence.sh` command for the baseline run, with output directory.
2. The exact `capture-evidence.sh` command for the changed run, with output directory.
3. The exact `compare-evidence.sh` command referencing both output directories.
4. A checklist of conditions that MUST match between baseline and changed runs
   (URL, viewport, seed data, build mode).
5. Instructions for what to do if unexpected differences appear in the comparison output.

## Output Spec

File: `evidence-capture-plan.md`
