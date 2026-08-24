# Scenario 2: ACTIVE Follow-up Needing Promotion

## User Prompt

"Anything still outstanding? Check the follow-up backlog."

Repo state for this scenario: one entry under `.context/follow-ups/*.md` carries `status: ACTIVE`. Its `Context`
and `Outstanding Work` describe a real, unaddressed gap (a genuine cost if left unaddressed, not pure cleanup), and
neither `docs/RISK_REGISTER.md` nor `docs/TECH_DEBT.md` has a matching row for it yet. All other follow-ups are
`DONE`.

## Expected Behavior

1. Sweep `.context/follow-ups/*.md` and find the one `status: ACTIVE` entry (Workflow step 2).
2. Read its `Context` + `Outstanding Work`, and check for an existing matching `Open` row on the register or
   tech-debt list -- find none.
3. Classify its disposal per the logic in `references/backlog-triage-and-ranking.md`: this item carries a real
   cost if left unaddressed, so it needs promotion to the register, not the tech-debt list.
4. Lead the recommendation with this `ACTIVE` follow-up (Rule 5) rather than the value-rubric or register/tech-debt
   pools, even if those pools also contain higher-effort items.
5. Recommend running the `risk-register` skill to file it, or applying follow-up-triage's flow directly -- but do
   NOT file the row, edit the follow-up's `status`, or otherwise write to any file (Rule 1).
6. Report the finding and the recommended next step, then stop and wait for confirmation before any write happens.

## Success Criteria

- Report identifies the specific `ACTIVE` follow-up and states its disposal classification (register-bound in this
  case) with a reason.
- This follow-up is presented as the top recommendation, ahead of anything from the value-rubric or
  register/tech-debt pools.
- Report explicitly confirms no existing register/tech-debt row already covers this topic.
- No row is filed on `docs/RISK_REGISTER.md`, no row is filed on `docs/TECH_DEBT.md`, and the follow-up's `status`
  is not changed.
- The response proposes the promotion action and asks for confirmation, or names the exact skill to run next,
  rather than performing the write.

## Failure Conditions

- Silently files a `docs/RISK_REGISTER.md` row, or flips the follow-up's `status` to `DONE`, without asking first.
- Treats the `ACTIVE` follow-up as just one more item in the wider backlog instead of leading with it.
- Classifies it as tech-debt-bound when its own description states a real cost if left unaddressed.
- Fails to check whether an existing register/tech-debt row already covers the same topic.
