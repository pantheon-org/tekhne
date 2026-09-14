# Scenario 2: "What's Left" Is Reported Verbatim, Not Expanded

## User Prompt

"What's left?"

Repo state: one active goal file, `goal-status: in-progress`, four items. Item 1 is `done`
with a commit SHA. Item 2 is `todo`. Item 3 is `awaiting` with outside reach. Item 4 is `done`
with a path. The log records one parked item filed as a follow-up.

Running the status subcommand renders:

```text
Next: Rebuild the staging index
Goal: Unblock the nightly reconciliation job (in-progress, 2 of 4 done).
Awaiting your confirmation: Notify the data team of the cutover.
Parked to follow-ups: 1.
File: <goals-dir>/2026-09-14-unblock-the-nightly-reconciliation-job.md
```

## Expected Behavior

1. Read the active goal rather than reconstructing status from the conversation.
2. Report the rendered summary as-is (Workflow step 4). The cap and the ordering are the
   product.
3. Lead with the next action, exactly as rendered.
4. Surface the `awaiting` item as needing the user's confirmation, not as done and not as
   ordinary outstanding work.
5. Offer the file path if more detail is wanted, rather than expanding the summary inline.

## Success Criteria

- The next action appears first in the response.
- The counts reported match the file (2 of 4 done), not a recount from memory.
- The `awaiting` item is described as needing the user's confirmation.
- The parked item is reported as parked to follow-ups, not as outstanding work in the goal.
- The summary is not rewritten into paragraphs or padded with commentary about each item.
- The response stays at or below roughly 100 words of summary content.
- The goal file path is offered rather than its contents being pasted.

## Failure Conditions

- Expands the four-line summary into a prose narrative of the session.
- Answers from conversation memory without reading the goal file.
- Reports the `awaiting` item as done, or buries it among the todo items.
- Omits the parked item entirely, so the user cannot see work left the goal.
- Adds a per-item status table the user did not ask for.
