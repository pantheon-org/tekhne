---
category: reference
priority: MEDIUM
load_when: Read when hand-editing a goal file
---

# Goal file shape

A goal is a markdown file under the context root's `goals/` directory, named
`<YYYY-MM-DD>-<slug>.md` like every other context file. The script writes it; this reference
is for hand-editing and for diagnosing a failing `check`.

## Worked example

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

Phase 1 steps 4 to 8 complete against a valid credential, with no exposed token in the
environment.

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

## Frontmatter

| Field | Values | Notes |
| --- | --- | --- |
| `title` | string | Matches the H1 exactly. |
| `type` | `goal` | Singular form of the `goals` typology. |
| `date` | `YYYY-MM-DD` | Set once at creation. Never updated. |
| `status` | `active`, `done` | The index-facing field shared with every context file. |
| `goal-status` | `new`, `in-progress`, `completed`, `promoted` | The goal lifecycle. |
| `tags` | list | Always present, empty list when none. |
| `related` | list | On promotion, carries the plan's path. Omit when empty. |

### Why two status fields

`status` is what the context index and any session-start tooling already understand, so a goal
sorts alongside findings and plans with no special-casing. It cannot carry four values without
breaking those consumers.

`goal-status` carries the lifecycle. The pairing is: `new` and `in-progress` are both
`status: active`; `completed` and `promoted` are both `status: done`. Only `goal-status`
distinguishes a goal that finished from one that grew into a plan, which is exactly the
distinction a later reader needs and the one most often lost.

The machine-readable contract is
[`goal-frontmatter.schema.json`](../assets/schemas/goal-frontmatter.schema.json).

## The items table

Five columns, always in this order. A row whose item text is empty is treated as an unfilled
template row and ignored by both `status` and `check`.

| Column | Values | Notes |
| --- | --- | --- |
| `#` | integer | Renumber the rows below when one is parked out. |
| `Item` | string | One finishable thing. If it needs "and", it is probably two items. |
| `State` | `todo`, `awaiting`, `done` | `awaiting` is outside work performed but unconfirmed. |
| `Reach` | `local`, `outside` | Declared when the item is written, not when it closes. |
| `Evidence` | string | Required for `done`. A pointer for `local`, a dated confirmation for `outside`. |

Declaring reach up front is deliberate. Deciding it at close time invites deciding it in
whichever direction lets the item close, which is how the confirmation rule gets quietly
dropped.

## The log

One dated line per state change, appended, never rewritten. It carries what the table cannot:
the order things happened, why an item was parked, and which trigger fired a promotion.

```markdown
## Log

- 14-09-2026 created, goal-status new
- 14-09-2026 item 1 done, goal-status in-progress
- 14-09-2026 item 4 parked to follow-ups/2026-09-14-andy-on-leave.md: blocked until Monday
- 15-09-2026 promoted on item count: see plans/2026-09-15-phase-1-completion.md
```

## What `check` enforces

| Failure | Message |
| --- | --- |
| `done` with an empty evidence cell | `item N is done with no evidence` |
| `done` + `outside` with no confirmation wording | `item N has outside reach and was closed without a recorded user confirmation` |
| State outside the three values | `item N has unknown state '...'` |
| Reach outside the two values | `item N has unknown reach '...'` |
| More than five items | `N items exceeds the promotion threshold of 5` |
| `completed` while items remain open | `goal-status is completed but items remain open` |
| No open items but not terminal | `no items remain open but goal-status is '...'` |
| Empty table | `the items table is empty` |

The outside-reach check looks for `confirm` or `agree` in the evidence text. That is a
deliberately shallow test: it cannot prove a confirmation happened, only that someone wrote
one down. It catches the common accident, an agent pasting a tool result into the cell, and
not deliberate circumvention.
