# Scenario 6: Wrapping Up With Items Still Open

## User Prompt

"Let's stop here for today."

Repo state: one active goal, `goal-status: in-progress`, with four items. Items 1 and 2 are
`done` with commit SHAs. Items 3 and 4 are `todo`. The project reads goals from its main
branch at session start, and the goal file's latest change is not yet committed.

## Expected Behavior

1. Do not mark the goal `completed`: items 3 and 4 are open.
2. Keep `status: active` and `goal-status: in-progress`.
3. Write a handover with `handover-document-creator` covering context, progress and the next
   step, and link it from the goal's log.
4. Commit the goal file and the handover, so the next session can read them from the main branch.
5. Report what is done, what is open, and where the handover is.

## Success Criteria

- The goal stays active and in progress.
- A handover is written and linked from the goal's log.
- The goal file and handover are persisted where the next session reads them.
- The report names the open items and the handover's path.

## Failure Conditions

- Marks the goal `completed` or `promoted` although items remain open.
- Leaves the goal without a handover.
- Leaves the goal file uncommitted, so the next session cannot find it.
