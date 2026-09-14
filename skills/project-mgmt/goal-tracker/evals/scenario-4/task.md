# Scenario 4: An Overgrown Goal Is Promoted, Not Declared Complete

## User Prompt

"Add the cache invalidation work to the goal as well."

Repo state: one active goal, `goal-status: in-progress`, already holding five items. Items 1
and 2 are `done` with commit SHAs. Items 3, 4 and 5 are `todo`. Adding the requested work
would make six. The goal's `## Done looks like` sentence does not mention caching, and the
stated end state is reachable without it.

## Expected Behavior

1. Apply the discovered-work test first (Workflow step 2). The end state is reachable without
   the cache invalidation work, so by default it is a follow-up, not a goal item.
2. Say so, and state the test that produced the answer, rather than silently appending the row.
3. If the user's instruction is taken as overriding that default, adding the item makes six,
   which crosses the promotion threshold of five (Workflow step 5).
4. On promotion, hand the end state and the items to `plan-create`, carrying items 1 and 2
   across as already-completed tasks with their existing commit SHAs intact.
5. Set the goal's `goal-status` to `promoted` and `status` to `done`. Never `completed`
   (Core Rule 4).
6. Link the plan from the goal's `related` frontmatter and log the promotion with the trigger
   that fired.

## Success Criteria

- The discovered-work test is applied and its result stated before the item is added.
- The response notes that adding the item crosses the five-item promotion threshold.
- If promotion happens, `goal-status` is set to `promoted`, not `completed`.
- Items 1 and 2 carry across with their commit SHAs rather than being re-opened as fresh tasks.
- The promotion is logged with the trigger that fired.
- The plan path is recorded in the goal's `related` frontmatter.

## Failure Conditions

- Appends the sixth row silently, leaving a goal above the threshold with no promotion.
- Marks the goal `completed` on promotion, claiming work is finished when it has grown.
- Drops the existing evidence, so completed work is re-planned as outstanding.
- Promotes without saying which trigger fired.
- Parks or deletes existing items purely to stay under the threshold without telling the user.
