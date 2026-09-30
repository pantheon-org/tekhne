# Scenario 7: Two Conversations Share One Repository

## User Prompt

"What's left?"

Repo state: two goal files carry `status: active`. One has `session: conv-a` and belongs to
this conversation; the other has `session: conv-b` and belongs to a parallel conversation in
another worktree. This conversation's session id is `conv-a`.

## Expected Behavior

1. Run `./scripts/goal.sh status --session conv-a` rather than without a session.
2. Report only this conversation's goal, verbatim from the summary.
3. Do not close, promote or edit the other conversation's goal to get past the
   "more than one active goal" message.

## Success Criteria

- `status` is run with this conversation's session.
- The answer covers only the `conv-a` goal.
- The `conv-b` goal is left untouched.

## Failure Conditions

- Runs `status` without a session and reports the refusal as the answer.
- Closes or promotes the other conversation's goal so that only one is active.
- Mixes the two goals' items in one answer.
