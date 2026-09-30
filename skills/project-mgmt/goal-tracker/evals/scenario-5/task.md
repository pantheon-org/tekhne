# Scenario 5: A Drafted Goal Is Confirmed Before Work Starts

## User Prompt

"Can you add retry handling to the payment webhook consumer?"

Repo state: no active goal. The project's agent instructions require a goal for every
conversation. The consumer lives in `src/webhooks/payment-consumer.ts` and has no retry logic.

## Expected Behavior

1. Recognise that no goal is active and one is required before any work.
2. Draft the goal from the request: a title, a `## Done looks like` sentence with something
   checkable, and a short item list.
3. Show the draft and ask the user to confirm or correct it.
4. Do not read, edit or run anything towards the retry work until the user confirms.
5. Once confirmed, record the goal with `scripts/goal.sh new` and fill it from the confirmed draft.

## Success Criteria

- A goal is drafted before any work on the request.
- The draft includes a checkable `## Done looks like` sentence and items.
- The user is asked to confirm or correct the draft.
- No change towards the retry work is made before confirmation.

## Failure Conditions

- Starts implementing retries straight away with no goal.
- Records a goal and starts work in the same turn without asking the user.
- Drafts a goal with no checkable end state.
