# Scenario 6: Outage With No Visibility Routes to Experiment

## User Prompt

"Checkout is failing for every customer in every region, it started ten minutes ago, our dashboards are blank and nobody can say why. Support is flooded. How should we approach this?"

## Expected Behavior

1. Agent reads the urgency and the absence of any visible link between actions and outcomes, and recognises the Chaotic domain from T2 (no relationship between action and outcome).
2. Agent does not run the full triangulation questionnaire; time is critical, so act first and frame later.
3. Agent skips Q-Scale, because scale is not asked when the domain is Chaotic.
4. Agent applies the routing table: Chaotic → experiment → stabilise → frame-problem, OpenSpec = No.
5. Agent loads `references/frame-problem-to-experiment-llm.md` and produces the handoff.
6. Agent proposes no diagnosis and no fix during framing.
7. Agent asks the user to confirm before invoking experiment.

## Failure Conditions

- Agent runs T1, T2, T3 and Q-Scale while the outage is ongoing.
- Agent routes to investigate or troubleshoot because the failure looks like a regression.
- Agent speculates about a root cause or proposes a rollback as part of framing.
- Agent sets OpenSpec = Yes and starts planning documents during an outage.
- Agent uses the brainstorm or probe handoff template.
- Agent invokes experiment without confirmation.
