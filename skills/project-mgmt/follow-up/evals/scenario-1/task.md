# Scenario 1: Empty Follow-up List Falls Through to the Wider Backlog

## User Prompt

"/follow-up"

Repo state for this scenario: all 15 entries under `.context/follow-ups/*.md` currently carry `status: DONE`.
`.context/index.yaml`'s header entry count matches the actual file count. `docs/RISK_REGISTER.md` has 38 `Open`
rows (of 43 total). `docs/TECH_DEBT.md` has 7 `Open` rows.

## Expected Behavior

1. Run the freshness check: compare the index's header entry count against the actual file count (Workflow step
   1) -- confirm it's current before proceeding.
2. Sweep `.context/follow-ups/*.md`, filter to `status: ACTIVE`, and find zero matches.
3. State the zero-`ACTIVE` result explicitly and continue to the wider backlog anyway (Rule 2) -- do not stop here
   or report "nothing to report."
4. Apply the value-rubric read protocol over the index's `PLAN`/`FINDING`/`KNOWN_ISSUE` entries to find the tier-1
   top pick (Workflow step 3).
5. Collect `Open` rows from `docs/RISK_REGISTER.md` (38) and `docs/TECH_DEBT.md` (7).
6. Synthesize exactly one recommendation (Workflow step 4). If it's sourced from the register/tech-debt pool
   rather than the value-rubric pool, label it explicitly as a qualitative judgment call, not rubric output (Rule
   3).
7. Report using the Follow-up status / Wider backlog / Recommended next item structure, then stop -- no file is
   edited.

## Success Criteria

- Report explicitly states the follow-up count is zero `ACTIVE` (out of 15 total, all `DONE`).
- Report does not stop at the zero-follow-up result -- it proceeds to report on the register and tech-debt pools.
- The 38 and 7 open-row counts (or an equivalent accurate count) appear in the report.
- A single recommended next item is named, with a stated reason it beats the runner-up.
- If the recommendation is sourced from the register or tech-debt list, the report explicitly says this is a
  judgment call, not a value-rubric-sorted pick.
- No file is edited, no register/tech-debt row is filed or modified, and no follow-up's `status` is changed.
- The response offers a next-step handoff (e.g. running `risk-register`, `tech-debt`, or `plan-create`) rather
  than performing the action itself.

## Failure Conditions

- Reports "no open follow-ups, nothing to report" and stops there.
- Skips the register/tech-debt pools entirely because the follow-up list was empty.
- Presents a register or tech-debt row's rank as if it carried a `value` grade.
- Silently edits `docs/RISK_REGISTER.md`, `docs/TECH_DEBT.md`, or a follow-up file's `status`.
- Recommends an item without stating why it beats the alternative.
