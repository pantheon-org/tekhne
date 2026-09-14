---
category: principles
priority: CRITICAL
load_when: Run before any item is closed
---

# Evidence

No item reaches `done` without evidence. What counts depends on the item's
`reach`, which is declared when the item is written, not when it is closed.

## Reach: local

The item can be proved by something an agent can re-check without the user
present. The evidence cell holds the pointer, not a description of it.

| Item kind | Evidence |
| --- | --- |
| Code change | Commit SHA, plus the test result if tests cover it |
| File produced | Repository-relative path |
| Check passed | The command and its decisive output (`0 failed`, not `done`) |
| Decision recorded | Path to the decision file, filed under the decisions typology |

A pointer that cannot be re-checked is not a pointer. "Tests pass" fails.
"`bun test`: 20 pass, 0 fail" passes. Copy the number from the actual output at
the moment you record it, never from memory.

## Reach: outside

**Hard rule. This admits no exception.**

Any item requiring outside reach or out-of-band work needs the user's
validation, confirmation, or express agreement before it can be marked done.
Anything less is unacceptable.

Outside reach means the work left the agent's own verifiable surface:

- A message sent to a person (Slack, email, a review comment)
- A ticket raised, transitioned, or commented on
- A credential rotated, an account changed, a setting altered in a console
- A judgement the user owns (adopt or reject, ship or hold)
- Anything in a system the agent cannot read back

The evidence cell records the confirmation and its date:

```text
confirmed by user 14-09-2026
```

If the user also supplies a pointer (a ticket key, a message link), record both.
The pointer strengthens the record but does not replace the confirmation, and a
pointer alone never closes an outside item.

### What you may never do

- Mark an outside item done because you watched a tool call succeed.
- Mark an outside item done because the user's earlier message implied they
  would do it.
- Infer confirmation from the user moving on to the next topic.
- Ask a leading question ("so that's done, yes?") and treat a non-answer as
  agreement.

When an outside item looks finished but has no confirmation, set it to `awaiting`
and it surfaces in "what's left" under **awaiting your confirmation**. That line
is the prompt for the user to confirm. Treating it as done instead is the exact
failure this rule exists to stop.

## Why the asymmetry

A local item closed wrongly costs a re-run. An outside item closed wrongly
means the agent has told the user something happened in the world that did
not: a ticket nobody raised, a message nobody received, a token still live.
That error is invisible until it causes damage, so the bar sits higher.
