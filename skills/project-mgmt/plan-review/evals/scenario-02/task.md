# Scenario 02: Classifying and Resolving Findings, Not Just Presenting Them

## User Prompt

"Here's the consolidated report from the 3 reviewers on
`.context/plans/retry-sweep-hardening-2026-08-01.md`:

- **Critical (Risk):** The plan doesn't say whether the retry sweep should
  hard-fail the whole batch on one delivery's error, or log and continue.
  Both are defensible; the plan is silent.
- **Critical (Technical):** Phase 2's step says 'wire the DLQ redrive alarm'
  but never states which SNS topic it publishes to -- the existing alerts
  topic already exists and is the obvious answer.
- **Moderate (Strategic):** The plan's Open Questions section already lists
  the hard-fail-vs-continue question, but nobody has answered it.

Present this to me."

## Expected Behavior

1. Do NOT stop at presenting the report -- Step 10 requires classifying and
   resolving every item, not treating presentation as the finish line.
2. Classify the SNS topic ambiguity as **Editorial** -- there's one clearly
   correct answer (the existing alerts topic) and no real tradeoff. Apply
   this fix directly to the plan file.
3. Classify the hard-fail-vs-continue question as **Decision** -- it's a
   genuine tradeoff with no single correct answer.
4. For the Decision-classified item, run a one-question-at-a-time interview
   (matching `guided-interview`'s pattern) rather than leaving it sitting in
   Open Questions for the user to notice later.
5. After the user answers, record the outcome in a `## Decisions` section in
   the plan, including why the alternative didn't win and a concrete revisit
   trigger if the decision is time-boxed.
6. Note that adding a `## Decisions` heading triggers `adr-capture` as a
   required next step, not optional.

## Success Criteria

- The SNS topic finding is applied directly as an editorial fix, not left
  for the user to decide.
- The hard-fail-vs-continue finding triggers an explicit one-question
  interview, not a summary statement asking the user to "let me know."
- A `## Decisions` section is proposed/created recording the interview's
  outcome, with a stated reason the alternative was rejected.
- `adr-capture` is mentioned as the required next step once `## Decisions`
  exists.
- The Strategic reviewer's "already in Open Questions but unanswered" note
  is treated as reinforcing evidence for the Decision classification, not a
  separate third bucket.

## Failure Conditions

- The report is presented and the conversation ends there, with both
  findings left as prose in the reply.
- The hard-fail-vs-continue tradeoff is "resolved" unilaterally by the agent
  without asking the user.
- The SNS topic ambiguity is escalated to a full interview when it has one
  clearly correct answer.
- `adr-capture` is never mentioned despite a `## Decisions` section being
  created.
