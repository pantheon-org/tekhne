---
name: goal-tracker
description: "Records the goal for a working session as a .context/goals/ file with items and evidence, keeps its status current as work lands, and answers \"what's left\" with a sub-100-word summary covering what is achieved, what was parked, and what remains. Use when the user states a goal at session start, asks what's left, what's outstanding, whether the goal is met, or where the session got to. Also use to park an item, to promote an overgrown goal into a plan, and to close a goal out at session end."
license: MIT
metadata:
  version: "1.0.0"
  audience: agents
  workflow: planning, tracking, session-management
---

# Goal Tracker

A session goal is the one thing this session is for. This skill writes it down
where it survives `/clear`, keeps it honest as work lands, and answers "what's
left" without you having to reconstruct the session from memory.

## When to Use

- The user opens a session by stating a goal ("goal: rotate the token and
  unblock Phase 1").
- The user asks "what's left", "what's outstanding", "are we done", "where did
  we get to".
- An item turns out to be deferred and needs parking.
- A goal outgrows itself and should become a plan.
- The session is ending and the goal needs closing out and landing.

## When Not to Use

- Work that already has a `.context/plans/` file driving it. Track it there.
  A goal points at a plan, it does not shadow one.
- A single mechanical request with no end state worth recording ("rename this
  variable"). Recording a goal for it costs more than the work.
- Capturing deferred work that was never part of a goal. That is
  `create-context-file --type follow-ups`.

## Core Rules

These four are not style preferences. Breaking any one of them makes "what's
left" untrustworthy, which defeats the skill.

1. **One active goal per repository at a time.** A second stated goal either
   replaces the first (close it out) or is an item within it. Two active goals
   means "what's left" has no single answer.
2. **Evidence before done.** No item reaches `done` without evidence. An
   outside-reach item whose work is performed but unconfirmed sits at
   `awaiting`, never `done`. See
   [`references/evidence.md`](references/evidence.md). The rule for anything
   needing outside reach is hard and admits no exception.
3. **Parking files a follow-up in the same turn.** A parked item leaves the
   goal and becomes a `.context/follow-ups/` entry immediately, not at
   close-out. The goal shrinks to what is genuinely still in play.
4. **Never self-attest an item the user has to confirm.** If an item required
   outside reach, only the user's express confirmation marks it done. Your own
   observation, however confident, is not evidence.

## Workflow

### 1. Recording a goal

Run the clarity test in
[`references/clarity-test.md`](references/clarity-test.md) against what the
user said. It routes three ways:

| Result | Route |
| --- | --- |
| No verifiable end state, or items not enumerable | `agentic-harness--socratic-method` |
| End state clear, but items hinge on an unmade choice between known options | `agentic-harness--guided-interview` |
| Passes | Record it and start work |

Then scaffold the file:

```bash
bash scripts/goal.sh new "Rotate the exposed token and unblock Phase 1"
```

This writes `.context/goals/<YYYY-MM-DD>-<slug>.md` from
[`assets/goal-template.md.tmpl`](assets/goal-template.md.tmpl) with `goal-status: new`.
Fill in the `## Done looks like` sentence and the items table yourself. Each
item declares its `reach` (`local` or `outside`) at creation time, not when you
come to close it.

### 2. While working

- Flip `goal-status` to `in-progress` the first time any item gains evidence.
- Add the evidence to the item's row as you land it, in the same turn. An item
  marked `done` in one turn and evidenced in a later one is the drift this
  skill exists to prevent.
- Append a dated line to `## Log` on every state change.
- New work discovered mid-goal is a follow-up by default. It joins the goal
  only when the stated end state is unreachable without it. If it joins, check
  the promotion triggers.

### 3. Parking an item

```bash
bash scripts/goal.sh park <item-number> "reason"
```

This prints the `create-context-file` invocation to run. File the follow-up,
then delete the item row from the goal and log the park with a link to the
new follow-up path. The goal's item count drops.

### 4. Answering "what's left"

```bash
bash scripts/goal.sh status
```

Renders the summary. It is capped at 100 words and leads with the single next
action. Report it as-is. If the user wants the detail behind it, give them the
file path rather than expanding the summary.

### 5. Promotion

Check after every item addition. Any one trigger fires promotion:

- More than five items
- The work spans a second worktree
- The work needs wave structure or parallel subagents

On promotion, hand off to `project-mgmt--plan-create`, set the goal's
`goal-status: promoted` and `status: done`, and add the plan's path to
`related`. See [`references/promotion.md`](references/promotion.md). A promoted
goal is not a completed goal and must never be recorded as one.

### 6. Close-out

A goal reaches `goal-status: completed` and `status: done` when no `todo` or
`awaiting` items remain. An `awaiting` item blocks close-out by design: it is
the user's confirmation that is missing, and asking for it is the close-out
step. Then land it: the goal file is committed once, at session end, through
the repository's normal landing procedure. During the session it stays
uncommitted on purpose, so status churn costs nothing.

Run `bash scripts/goal.sh check` before landing. It fails the goal if any item
is marked `done` without evidence, or if an `outside` item was closed without a
recorded user confirmation.

## File Shape

```markdown
---
title: Rotate the exposed token and unblock Phase 1
type: goal
date: 2026-09-14
status: active
goal-status: in-progress
tags: [security]
---

# Rotate the exposed token and unblock Phase 1

## Done looks like

Phase 1 steps 4 to 8 complete against a valid credential, with no exposed
token in the environment.

## Items

| # | Item | State | Reach | Evidence |
| - | ---- | ----- | ----- | -------- |
| 1 | Rotate the personal access token | done | outside | confirmed by user 14-09-2026 |
| 2 | Finish Phase 1 steps 4 to 8 | todo | local | |
| 3 | Send both drafts to Andy | awaiting | outside | |

## Log

- 14-09-2026 created, goal-status new
- 14-09-2026 item 1 done, goal-status in-progress
```

`status` is the index-facing field every context file carries and stays
`active` or `done`. `goal-status` carries the lifecycle the user asked for.
They are not redundant: `status: done` covers both a completed goal and a
promoted one, and only `goal-status` distinguishes them.

The full frontmatter contract, including which `goal-status` values pair with
which `status`, is
[`assets/schemas/goal-frontmatter.schema.json`](assets/schemas/goal-frontmatter.schema.json).

## Verification

- Does `.context/goals/` hold exactly one file with `status: active`?
- Does every `done` item have a non-empty evidence cell, and does every
  performed-but-unconfirmed outside item sit at `awaiting`?
- Does every `done` item with `reach: outside` cite a user confirmation with a
  date, rather than an agent observation?
- Does the item count sit at five or fewer, or has promotion fired?
- Does `## Log` have a dated line for every state change in the table?
- Is the rendered summary under 100 words and led by a next action?

## References

- [`references/clarity-test.md`](references/clarity-test.md): the three checks and where each failure routes
- [`references/evidence.md`](references/evidence.md): local versus outside reach, and the hard confirmation rule
- [`references/promotion.md`](references/promotion.md): triggers, and how a goal becomes a plan
