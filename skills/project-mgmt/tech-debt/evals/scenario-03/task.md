# Scenario 03: Resolving a Row That Revealed a Bigger Problem

## User Prompt

"Row #1 in docs/TECH_DEBT.md is the eslint `no-unused-vars` warning-not-error
gap. I just fixed the underlying lint config so it's now an error. Mark #1
fixed."

Suppose that in fixing the gate, it becomes clear the reason `no-unused-vars`
was only a warning is that a previous engineer downgraded it specifically to
stop a flaky test suite from failing on generated fixture files -- and that
flaky test suite still isn't covered by anything else.

## Expected Behavior

1. Before deleting row #1, apply Rule 3's resolving-direction check: did
   fixing this reveal a real cost or risk? Yes -- the flaky, uncovered test
   suite the downgrade was originally covering for.
2. Do NOT just delete row #1 silently.
3. Add a new row to `docs/RISK_REGISTER.md` describing the flaky/uncovered
   test suite, referencing tech-debt item #1 by name so the discovery isn't
   lost once #1's row disappears.
4. Only then delete row #1 from `docs/TECH_DEBT.md` -- deletion is the
   resolution mechanism here, not a `Resolved` status flip.
5. Explain to the user why a second doc got a new row, not just delete #1
   as asked.

## Success Criteria

- Agent does not delete row #1 without first checking for a revealed risk.
- A new `docs/RISK_REGISTER.md` row is added (or the agent explicitly
  recommends the `risk-register` skill do so) referencing tech-debt item #1
  by name.
- Row #1 is deleted from `docs/TECH_DEBT.md` (not marked `Resolved` -- that
  status doesn't exist here).
- Agent explains the two-step reasoning to the user rather than silently
  doing more than asked.

## Failure Conditions

- Row #1 deleted immediately with no check for a revealed risk.
- The flaky/uncovered test suite discovery is mentioned but not captured
  anywhere durable.
- Row #1 marked `Resolved` instead of deleted.
- Agent invents an append-only mechanism for tech-debt to "be safe."
