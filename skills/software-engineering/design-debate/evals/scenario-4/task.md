# Scenario 4: Proceed verdict, but Migration/Risk surfaces a separate unfixed gap

## User Prompt

"Should we consolidate the six per-cadence EventBridge rules into a single rule with a
parameterized schedule? Give me a real debate on this one."

## Setup

The Advocate argues consolidation reduces Terraform duplication. The Skeptic argues the
current six-rule setup is simple and battle-tested. The Migration/Risk role's investigation
turns up a separate, concrete finding: the retry-sweep rule's cron expression currently
overlaps with the daily cadence rule by a few minutes, which is unrelated to whether
consolidation happens but is a real gap in the current setup regardless of the verdict.

## Expected Behavior

1. Ground the debate in the current rule count, Terraform duplication, and cron expressions.
2. Spawn Advocate, Skeptic, and Migration/Risk in parallel with an identical brief.
3. Synthesize an overall verdict of `proceed` (or `proceed_with_modification`) on the
   consolidation question itself.
4. Recognize that the Migration/Risk role's cron-overlap observation is a separate, concrete
   gap that won't be fixed by this session's work and is not resolved by the consolidation
   verdict either way.
5. Record the overall verdict as a finding (since it is `proceed`), and -- independently --
   record the cron-overlap gap as its own `KNOWN_ISSUE`, not folded into the main finding and
   not dropped because the main question got resolved.
6. Ask the user explicitly before persisting each artifact.

## Success Criteria

- Main verdict is `proceed` or `proceed_with_modification` on the consolidation question.
- The cron-overlap gap is identified as a distinct, concrete issue separate from the
  consolidation decision.
- Two separate artifacts are produced: a finding for the main verdict and a known-issue for
  the cron-overlap gap.
- The known-issue is not silently merged into the finding's body as a footnote.
- The user is asked explicitly before either artifact is persisted.

## Failure Conditions

- Treating the cron-overlap observation as resolved simply because the main verdict was
  `proceed`.
- Merging the cron-overlap gap into the main finding instead of giving it its own
  known-issue.
- Mentioning the cron-overlap gap only in chat ("worth flagging separately") without ever
  creating the known-issue artifact.
- Skipping the explicit persistence question for either artifact.
