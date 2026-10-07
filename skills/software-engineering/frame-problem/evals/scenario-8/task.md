# Scenario 8: Blank Question Answers and a Liminal Result

## User Prompt

"Should we adopt event sourcing for our order history? I'm not sure whether we know enough to just design it."

## Setup

The harness returns blank answers for the first `AskUserQuestion` call (no UI is shown). After the agent falls back to a numbered text list, the user replies with T1=2 (someone on the team has done it), T2=ordered (same inputs give the same result), T3=entangled (changing one part changes the whole).

## Expected Behavior

1. Agent asks the triangulation questions in a single `AskUserQuestion` call.
2. Agent checks the answers, sees they are blank, and does not proceed on assumptions.
3. Agent outputs a notice that the questions did not display (known harness bug outside planning mode) and presents the options as a numbered text list.
4. Agent waits for the user's reply before continuing.
5. With T1=2 and T2=ordered but T3=entangled, Agent recognises expertise bias, reports T3 as the dissenting liminal signal and does not force a binary Complicated verdict.
6. Agent applies the routing table: Liminal Complicated↔Complex → probe first to resolve the boundary → re-frame, OpenSpec = No.
7. Agent loads `references/frame-problem-to-probe-liminal-llm.md` and asks the user to confirm.

## Failure Conditions

- Agent treats the blank answers as "no preference" and picks a domain itself.
- Agent proceeds to route without waiting for the user's numbered reply.
- Agent presents the questions as separate calls or as free prose with no numbered options.
- Agent classifies as plain Complicated and routes to investigate.
- Agent uses the probe or investigate handoff template instead of the liminal one.
