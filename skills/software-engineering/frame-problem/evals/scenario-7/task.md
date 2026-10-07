# Scenario 7: Solution-Shaped Request With Expertise Bias

## User Prompt

"We need a microservice for notifications. We have built three of these before, so the team is confident."

## Expected Behavior

1. Agent notices the statement is solution-shaped ("a microservice") and does not accept it at face value; it asks what need or pain sits behind it and restates it as a need (for example, notifications are coupled to the order flow and block releases).
2. Agent asks the three triangulation questions (T1, T2, T3) and Q-Scale in one call instead of asking the user which domain this is.
3. User answers: T1=2 (the team has built these), T2=unordered (delivery behaviour varies heavily by customer segment), T3=entangled (preferences, channels and rate limits depend on each other).
4. Agent applies the majority rule: two of three tests say Complex, so T1 alone does not make it Complicated. T1 is noted as a liminal signal.
5. Agent applies the routing table: Complex, no hypothesis → brainstorm → probe → openspec-plan, OpenSpec = Yes.
6. Agent loads `references/frame-problem-to-brainstorm-llm.md`, produces the handoff and asks the user to confirm.

## Failure Conditions

- Agent accepts "a microservice" as the problem and routes straight to design.
- Agent routes to investigate because the team has done this before.
- Agent asks the user "is this Complicated or Complex?".
- Agent drops the T1 signal entirely instead of noting it as liminal.
- Agent uses the investigate handoff template.
- Agent invokes the next skill without confirmation.
