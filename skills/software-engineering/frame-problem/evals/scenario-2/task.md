# Scenario 2: Known Technical Bug Routes to Troubleshoot

## User Prompt

"Our API endpoint /v2/payments has been returning 500 errors since the deploy at 14:30. It was working fine this morning."

## Expected Behavior

1. Agent reads the explicit time-bound regression signal ("was working, now failing") and attempts auto-classification at confidence ≥80%.
2. Agent proposes Complicated (Degraded) with T1=2 (team has expertise), T2=ordered (same input should give same result), T3=disassemblable.
3. Agent runs Adjacent Domain Challenge: considers whether this could be Complex (unpredictable failure mode) and argues why Complicated still holds (known system, regression after deploy).
4. Agent presents classification with liminal signal note if T3 raises any doubt.
5. Agent applies routing table: Complicated + Degraded → troubleshoot → stabilize → re-frame.
6. Agent loads `references/frame-problem-to-troubleshoot-llm.md` and produces handoff.
7. Agent asks user to confirm before invoking troubleshoot.

## Failure Conditions

- Agent routes to brainstorm because the failure cause is unknown.
- Agent skips the Adjacent Domain Challenge after auto-classifying.
- Agent routes to investigate because the team wants to understand the system better.
- Agent classifies as Complex due to the word "errors" without checking T1/T2.
- Agent fills the brainstorm handoff template instead of the troubleshoot template.
- Agent proceeds without asking user to confirm the classification.
