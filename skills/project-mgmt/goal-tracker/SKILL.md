---
name: goal-tracker
description: "Record a session goal as a dated file with items plus evidence, keep its status current, then answer \"what's left\" in under 100 words: achieved, parked, still open. Use when the user states a session goal, asks what's left, what's outstanding, what's still open, whether the goal is met, are we done, where the session got to. Also use to park an item, to promote an overgrown goal into a plan, to close a goal out."
license: MIT
metadata:
  version: "1.3.0"
  audience: agents
  workflow: planning, tracking, session-management
---

# Goal Tracker

Write down the one thing this session is for, keep it honest as work lands, and answer
"what's left" without reconstructing it from memory.

## Mindset

**An unevidenced status is worse than no status.** A goal claiming an item is done, with
nothing anyone can point at, turns an open question into a false answer and the reader stops
checking. Trade convenience for provability everywhere.

Keep trust asymmetric. Close work done inside the repository yourself, since anyone can
re-check it. Leave work that left the repository for its owner to close.

## When to Use

- The user states a goal at session start.
- The project requires a goal for every conversation.
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

Run the clarity test first. Use `socratic-method` for a goal with no verifiable end state,
and `guided-interview` when its items hinge on an unmade choice. Otherwise draft it:

```bash
./scripts/goal.sh new "Rotate the exposed token and unblock Phase 1"
```

Check the draft with the user. Start work only once they confirm or correct it.

Set `goal-status: in-progress` on the first item to gain evidence. Write that evidence in the
same turn the item closes. Add a dated log line on every change, and save the file in that
turn where the next session reads it, such as the main branch. Keep discovered work as a
follow-up unless the end state is unreachable without it.

Run `park`, then file what it prints:

```bash
./scripts/goal.sh park 3 "Andy is on leave until Monday"
```

Run `status` and report its summary verbatim. Offer the path; never expand it:

```console
$ ./scripts/goal.sh status
Next: Finish Phase 1 steps 4 to 8
Goal: Rotate the exposed token (in-progress, 1 of 3 done).
Awaiting your confirmation: Send both drafts to Andy.
File: goals/2026-09-14-rotate-the-exposed-token.md
```

Promote on any trigger: more than five items, a second worktree, or wave structure. Pass the
end state and items to `plan-create`, carrying existing evidence across as completed tasks.

Close out only when the check exits zero:

```bash
./scripts/goal.sh check
```

Keep the goal active on a wrap-up with items open. Write a handover with
`handover-document-creator` and link it from the log.

## Anti-Patterns

**NEVER start work on a goal the user has not confirmed.** WHY: the goal is then the agent's
reading of the request, and every later "what's left" answer inherits the misreading.

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

Keep the two status fields paired:

```yaml
status: active        # done once goal-status is completed or promoted
goal-status: new      # new | in-progress | completed | promoted
```

Then confirm each of these:

- Exactly one goal file carries `status: active`.
- Every `done` item has evidence; every outside one cites a dated user confirmation.
- Item count is five or fewer, or promotion fired.
- The log has a dated line per state change.
- `./scripts/goal.sh check` exits zero.

## References

- [Clarity Test](references/clarity-test.md): the three clarity checks. Load when a new goal does not yet name a checkable end state.
- [Evidence](references/evidence.md): local versus outside reach. Load when closing any item.
- [Promotion](references/promotion.md): triggers and the goal-to-plan handover. Load when an item was just added, or the goal holds five.
- [Goal Schema](assets/templates/goal.yaml): the goal file contract. Load when hand-authoring or repairing a goal.
- [File Shape](references/file-shape.md): worked example, status pairing and `check` failures. Load when diagnosing a failing check.
