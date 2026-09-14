---
category: procedures
priority: CRITICAL
load_when: Run before a goal is recorded
---

# The clarity test

A stated goal is recordable when it passes three checks. The test exists to
pick the right questioning mode, not to gate the user. Run it silently and
route; do not narrate the test itself.

## Check 1: verifiable end state

Can you write `## Done looks like` as one sentence containing something
checkable by someone who was not in the room?

- Passes: "Phase 1 steps 4 to 8 complete against a valid credential."
- Fails: "Sort out the GitLab situation."

A sentence passes even when the check is a human confirmation rather than a
command. "Andy has replied to both drafts" is verifiable. "Improve our
credential hygiene" is not, because nothing distinguishes done from not done.

## Check 2: enumerable items

Can the work be listed as discrete items, each of which could be finished on
its own?

- Passes: rotate the token, finish steps 4 to 8, raise three tickets.
- Fails: "figure out what is wrong and fix it", where item two cannot be
  named until item one is finished.

An unenumerable goal is not a small goal, it is an investigation. Investigations
are legitimate work but they are not goals in this sense, because "what's left"
cannot answer honestly about a list that does not exist yet. Record the
investigation as item one of a goal whose later items you will add once it
reports.

## Check 3: evidence type known per item

For each item, is it already clear what would prove it, and whether proving it
needs outside reach? See [`evidence.md`](evidence.md).

An item failing this check is usually underspecified rather than unclear.
"Tidy up the hooks" has no evidence type because it has no boundary. "Remove
the hardcoded path from the index merge rule" has one.

## Routing

| Condition | Route | Why |
| --- | --- | --- |
| Check 1 fails | `agentic-harness--socratic-method` | The problem itself is undefined. Options would be guesses. |
| Check 2 fails and check 1 passed | `agentic-harness--socratic-method` | The end state is known but the path is not. Open questions surface it. |
| Checks 1 and 2 pass, check 3 fails on a choice between known approaches | `agentic-harness--guided-interview` | There is a real decision space. Concrete options resolve it faster than open questions. |
| Checks 1 and 2 pass, check 3 fails because an item is underspecified | Neither. Ask one direct question about that item. | A single vague item does not justify an interview. |
| All pass | Record and start | |

## The failure mode this prevents

Routing everything to an interview turns every session start into ceremony,
and the user stops stating goals. Routing nothing to an interview produces
goals whose items cannot be evidenced, which surfaces two hours later as an
item that can never be marked done. The test is cheap; run it every time and
skip the questioning when it passes.
