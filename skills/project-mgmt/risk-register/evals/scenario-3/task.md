# Scenario 3: Mid-Task Discovery, Not a Side Note

## User Prompt

The agent is in the middle of an unrelated task (fixing a Terraform variable). While working, it notices the deploy
job has no rollback procedure documented anywhere, and there is no existing rule that already covers this. The agent
is about to reply to the user.

## Expected Behavior

1. Recognize that "no rollback procedure documented" is a genuine, previously-unseen gap -- not something already
   settled by an existing convention -- and treat this as the trigger to file a register row immediately, in the
   same turn, rather than only mentioning it in the reply.
2. Do NOT describe it as "a side note, not part of this task, worth flagging separately" and then move on without
   filing -- that exact hedge language is the anti-pattern this step exists to catch.
3. File the row with the next unused `#`, `Type: Open Decision` or `Accepted Risk` as appropriate, a concrete
   `Description` and `Risk if unaddressed` (not "rollback stuff might be missing"), `Added` as today's date, and
   `Status: Open` with `Date`/`Decision` both `--`.
4. Confirm the row was filed (not merely described) before concluding the reply, and confirm the validation scripts
   pass.
5. Continue with and complete the original Terraform task in the same reply.

## Success Criteria

- The agent files an actual register row in `docs/RISK_REGISTER.md`, not just a chat mention.
- The row is filed in the same turn the gap was noticed, not deferred to "later" or "a follow-up pass."
- No hedge phrasing ("worth flagging separately", "not part of this task") appears without an accompanying filed row.
- The row's `Description` and `Risk if unaddressed` are concrete, not vague restatements.
- The original Terraform task is still completed in the same response.

## Failure Conditions

- The agent only mentions the gap in prose ("worth noting for later...") without adding a row.
- The agent defers filing to "a future session" or promises to "come back to it."
- The row, if filed, has a vague Description or Risk if unaddressed.
- The agent drops the original Terraform task to only handle the register entry, or vice versa.
