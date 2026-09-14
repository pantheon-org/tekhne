---
category: procedures
priority: HIGH
load_when: Run when an item is added
---

# Promotion: when a goal becomes a plan

Every piece of work starts as a goal. Nothing is classified as a plan up
front, because that judgement has to be made before the work is understood,
which is when it is least reliable. Instead, a goal promotes itself once it
demonstrates plan-shaped behaviour.

## Triggers

Check after every item addition. Any single trigger fires promotion.

| Trigger | Threshold | Why it is the line |
| --- | --- | --- |
| Item count | More than five items | Past five, "what's left" cannot stay under 100 words without suppressing something material. |
| Worktree span | The work touches a second worktree | A goal file lives in one working copy. Two copies means two truths. |
| Wave structure | The work needs waves or parallel subagents | The repository's wave rules already govern this shape, including proof-of-work lines and wave boundary re-verification. |

## What promotion is not

Promotion is not failure and it is not completion. A promoted goal reached a
size where a different instrument fits better. Recording it as `completed`
would claim the work is finished when it has barely started, which corrupts
every later question about what got done.

## The promotion procedure

1. Hand the goal's `## Done looks like` sentence and its items to
   `project-mgmt--plan-create` as the plan's scope and initial tasks.
2. Carry the evidence across. Items already `done` become completed tasks with
   their existing proof-of-work, not fresh tasks. Losing evidence in the
   handover means redoing work that was already proved.
3. Set the goal file's `goal-status: promoted` and `status: done`.
4. Add the new plan's path to the goal's `related` frontmatter, and add the
   goal's path to the plan's.
5. Log the promotion with the date and the trigger that fired.
6. State a fresh goal for the remainder of the session if one is needed. A
   session working through a plan can hold a goal that points at a subset of
   its phases.

## Item count is a symptom, not the disease

A goal hitting six items usually means the end state was drawn too wide, not
that the work is genuinely large. Before promoting on item count alone, check
whether two or three of the items are follow-ups that joined the goal because
nobody applied the test: work joins a goal only when the stated end state is
unreachable without it. Parking them is often the right move and leaves a goal
that never needed a plan.

Promotion on worktree span or wave structure has no such escape hatch. Those
are structural and promote immediately.
