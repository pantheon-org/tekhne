# Scenario 1: Full-flow debate on a multi-file decision

## User Prompt

"We're thinking about replacing our hand-rolled retry logic in `packages/lambdas/src/delivery/`
with a library dependency. I want pushback before we commit to this -- should we do it?"

## Expected Behavior

1. Restate the decision precisely as an A-vs-B choice ("adopt retry library X vs. keep the
   hand-rolled retry sweep") rather than debating the vague topic of "retry logic."
2. Before assigning positions, investigate real facts: how many call sites use the current
   retry logic, how large it is, and what the library would actually replace.
3. Compose one identical brief containing the decision and the grounding facts for every
   reviewer.
4. Assign genuinely opposing roles -- at minimum Advocate and Skeptic -- and add a
   Migration/Risk role because the change touches multiple files and a downstream consumer
   (the delivery retry sweep).
5. Spawn all reviewers in a single message so they run in parallel, not sequentially.
6. Synthesize a verdict: state which argument held up under the others' scrutiny and render
   an explicit recommendation (`proceed`, `do_not_proceed_for_now`, or
   `proceed_with_modification`) rather than "it depends."
7. Run the verdict through the schema validation script before treating it as final.
8. Ask the user explicitly whether to persist the outcome, then record it (a finding if
   `proceed`/`proceed_with_modification`, a `KNOWN_ISSUE` if `do_not_proceed_for_now`).
9. If the verdict is `proceed`, hand off to `plan-create` with the debate's grounding facts
   and chosen design rather than re-deriving them.

## Success Criteria

- Decision restated as a precise A-vs-B choice before any role is assigned.
- Concrete investigation step performed (call-site count, size, blast radius) before the
  debate starts, not asserted from assumption.
- All reviewers receive the identical brief (same decision, same facts).
- At least Advocate and Skeptic assigned; Migration/Risk included because multiple files
  and a downstream consumer are affected.
- Reviewers spawned in one message/parallel batch, not one-by-one.
- A synthesized verdict names the winning argument and picks one of the three explicit
  recommendation values.
- Verdict validated against the debate-verdict schema before being treated as final.
- User is asked explicitly before anything is persisted.
- Outcome is recorded as a finding or known-issue depending on the verdict value.
- A `proceed` verdict is handed off to `plan-create` with the debate's facts carried
  forward, not re-derived.

## Failure Conditions

- Debating "retry logic" in the abstract without ever pinning down a precise A-vs-B choice.
- Assigning roles before investigating any real facts.
- Giving reviewers different or unequal information.
- Assigning a "balanced/neutral" framing to any reviewer, or omitting Migration/Risk despite
  a multi-file, downstream-consumer change.
- Spawning reviewers sequentially (advocate, then wait, then skeptic).
- Presenting the three reviewers' raw opinions as the final answer with no synthesized
  verdict.
- Silently persisting or silently discarding the outcome without asking the user.
- Recording a `do_not_proceed_for_now` verdict without a revisit trigger.
