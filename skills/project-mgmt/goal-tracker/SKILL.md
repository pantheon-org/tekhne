---
name: goal-tracker
description: "Record a session goal as a dated file with items plus evidence, keep its status current, then answer \"what's left\" in under 100 words: achieved, parked, still open. Use when the user states a session goal, asks what's left, what's outstanding, what's still open, whether the goal is met, are we done, where the session got to. Also use to park an item, to promote an overgrown goal into a plan, to close a goal out. Keywords: goal, session goal, what's left, outstanding, parked, evidence, close-out, promotion."
license: MIT
metadata:
  version: "1.1.0"
  audience: agents
  workflow: planning, tracking, session-management
---

# Goal Tracker

Write down the one thing this session is for, keep it honest as work lands, and answer
"what's left" without reconstructing the session from memory.

## Mindset

**An unevidenced status is worse than no status.** A goal claiming an item is done, with
nothing anyone can point at, turns an open question into a false answer. The reader stops
checking. Trade convenience for provability everywhere.

Trust asymmetrically. Anyone can re-check work done inside the repository, so close it
yourself. Nobody can re-check work that left it, so let its owner close that.

## When to Use

- The user states a goal at session start.
- The user asks "what's left", "what's outstanding", "are we done".
- Park an item, promote an overgrown goal, or close one out.

## When Not to Use

- Work a plan file already drives. Track it there.
- A mechanical request with no end state worth recording.
- Deferred work that was never part of a goal. Use `create-context-file` instead.

## Core Rules

1. Keep exactly one goal active. Replace the first or fold it in; never run two.
2. Demand evidence before `done`. Park unconfirmed outside work at `awaiting`.
3. File a follow-up the same turn you park an item, then delete its row.
4. Record a promoted goal as `promoted`. Never as `completed`.

## Workflow

Run the clarity test first. Route a goal with no verifiable end state to `socratic-method`.
Route one whose items hinge on an unmade choice to `guided-interview`. Otherwise record it:

```bash
./scripts/goal.sh new "Rotate the exposed token and unblock Phase 1"
```

Set `goal-status: in-progress` on the first item to gain evidence. Write that evidence in the
same turn the item closes. Append a dated log line on every change. Treat discovered work as
a follow-up unless the end state is unreachable without it.

Park an item, then file what it prints:

```bash
./scripts/goal.sh park 3 "Andy is on leave until Monday"
```

Render the summary and report it verbatim. Offer the path; never expand it:

```console
$ ./scripts/goal.sh status
Next: Finish Phase 1 steps 4 to 8
Goal: Rotate the exposed token (in-progress, 1 of 3 done).
Awaiting your confirmation: Send both drafts to Andy.
File: goals/2026-09-14-rotate-the-exposed-token.md
```

Promote on any trigger: more than five items, a second worktree, or wave structure. Hand the
end state and items to `plan-create` and carry existing evidence across as completed tasks.

Close out only when `check` exits zero:

```console
$ ./scripts/goal.sh check
FAILED: goals/2026-09-14-rotate-the-exposed-token.md
- item 1 has outside reach and was closed without a recorded user confirmation
$ echo $?
1
```

## Anti-Patterns

**NEVER close an outside-reach item on your own observation.**
WHY: a tool call can succeed while the world disagrees, and nothing re-checks it later.

```text
BAD   | 1 | Raise the P1 ticket | done     | outside | ticket created successfully |
GOOD  | 1 | Raise the P1 ticket | awaiting | outside | PROJ-4821, needs your confirm |
```

**NEVER defer evidence to a later turn.** WHY: the intention does not survive a context
reset, and the row then reads as deliberately evidenced.

**NEVER admit discovered work by default.** WHY: a goal that absorbs everything never closes,
and "what's left" decays into a second backlog.

**NEVER paraphrase the rendered summary.** WHY: the cap and the ordering are the product.

**NEVER mark a promoted goal completed.** WHY: promotion means too large, not finished.

## Verification

Keep both status fields consistent. `status` feeds the index; `goal-status` carries the
lifecycle:

```yaml
status: active        # active while new or in-progress, done once completed or promoted
goal-status: new      # new | in-progress | completed | promoted
```

Then confirm each of these:

- Exactly one goal file carries `status: active`.
- Every `done` item has evidence; every outside one cites a dated user confirmation.
- Item count is five or fewer, or promotion fired.
- The log has a dated line per state change.
- `./scripts/goal.sh check` exits zero.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| The three clarity checks and where each failure routes | [Clarity Test](references/clarity-test.md) | Recording a new goal. Skip when it already names a checkable end state and a short item list. |
| Local versus outside reach, and the hard confirmation rule | [Evidence](references/evidence.md) | Closing any item. Skip when only rendering a summary. |
| Promotion triggers and the goal-to-plan handover | [Promotion](references/promotion.md) | An item was just added, or the goal holds five. Skip during ordinary status updates. |
| Frontmatter fields, the items table, and a worked example | [File Shape](references/file-shape.md) | Hand-editing a goal, or diagnosing a failing check. Skip when the script is writing. |
