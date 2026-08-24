# Scenario 3: Verdict is do_not_proceed_for_now

## User Prompt

"Should we migrate our DynamoDB delivery-status table to a relational database? I want a
real debate, not a rubber stamp."

## Setup

The grounding investigation (which the agent must perform) shows: the current table has a
single-digit number of GSIs, low write volume, no query patterns that DynamoDB can't serve,
and no team has relational-database operational experience. The Advocate's case rests mainly
on "it would be easier to query," the Skeptic's case is that the operational cost and
migration risk outweigh a query convenience that isn't currently a bottleneck.

## Expected Behavior

1. Ground the debate in the facts above before assigning roles.
2. Assign Advocate, Skeptic, and (since a migration is the subject) Migration/Risk roles,
   spawned in parallel.
3. Synthesize a verdict of `do_not_proceed_for_now` given the facts (no current bottleneck,
   real migration risk, no team experience).
4. Name a concrete, checkable revisit trigger (for example: "revisit if delivery-status
   query patterns require a join DynamoDB can't express, or if write volume crosses a stated
   threshold") -- not "revisit later."
5. Validate the verdict against the debate-verdict schema and confirm the revisit trigger is
   present before treating the verdict as final.
6. Ask the user explicitly before persisting, then record the outcome as a `KNOWN_ISSUE`
   (not a finding) using `context-file`, with the revisit trigger as its fix condition.

## Success Criteria

- Verdict is `do_not_proceed_for_now`, not "it depends" or a forced `proceed`.
- A concrete, checkable revisit trigger is stated, not a vague "later."
- Schema validation is run and confirms the revisit trigger is present.
- The outcome is recorded as a `KNOWN_ISSUE` context-file entry, not a finding.
- The user is asked explicitly before the known-issue is written.

## Failure Conditions

- Rendering a `do_not_proceed_for_now` verdict with no revisit trigger, or with "revisit
  later" as the only trigger.
- Skipping schema validation before treating the verdict as final.
- Recording the outcome as a finding instead of a known-issue.
- Persisting the outcome without asking the user first.
- Forcing a `proceed` verdict despite the grounding facts pointing the other way.
