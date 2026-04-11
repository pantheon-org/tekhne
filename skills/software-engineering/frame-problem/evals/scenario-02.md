# Scenario 02: Known Technical Bug Routes to Troubleshoot

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

## Expected Behavior (continued)

8. Agent does NOT route to investigate (no evolving/growth signal present).

## Success Criteria

- **Auto-classification attempted**: Agent identifies the "was working, now failing" pattern and classifies at ≥80% confidence.
- **Complicated-Degraded correctly identified**: Route is troubleshoot, not investigate or brainstorm.
- **Adjacent challenge run**: Agent explicitly considers whether the failure could be Complex before confirming Complicated.
- **Handoff template used**: Agent references or fills `references/frame-problem-to-troubleshoot-llm.md`.
- **No brainstorm route**: Agent does not route to brainstorm (no divergent exploration needed for a known regression).
- **OpenSpec correctly set to No**: Agent marks OpenSpec=No per routing table for Complicated-Degraded.

## Failure Conditions

- Agent routes to brainstorm because the failure cause is unknown.
- Agent skips the Adjacent Domain Challenge after auto-classifying.
- Agent routes to investigate because the team wants to understand the system better.
- Agent classifies as Complex due to the word "errors" without checking T1/T2.
- Agent fills the brainstorm handoff template instead of the troubleshoot template.
- Agent proceeds without asking user to confirm the classification.
