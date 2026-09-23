# Scenario 3: Novel Architecture Question Routes to Investigate

## User Prompt

"We're planning to migrate our monolith to a microservices architecture. We have senior engineers with distributed systems experience. What's the right approach?"

## Expected Behavior

1. Agent reads the problem: team has expertise (T1=2), architectural patterns are known and analyzable (T2=ordered at the pattern level), system can be decomposed into services (T3=disassemblable).
2. Agent attempts auto-classification: Complicated (Evolving — system improving, growing capacity).
3. Agent runs Adjacent Domain Challenge: considers whether this is Complex (novel combination, has this specific org + monolith been migrated before?) and concludes Complicated holds given senior expertise and established patterns.
4. Agent checks Q-scale: Boulder (multi-step, ambiguous, architectural).
5. Agent applies routing table: Complicated + Evolving + Boulder → investigate → openspec-plan. OpenSpec=Yes.
6. Agent loads `references/frame-problem-to-investigate-llm.md` and produces handoff.
7. Agent asks user to confirm.

## Failure Conditions

- Agent routes to brainstorm because "migration" feels uncertain.
- Agent skips the Q-Scale check and fails to mark OpenSpec=Yes.
- Agent routes to troubleshoot because there are "risks" in the migration.
- Agent ignores the LLM bias warning and defaults to Complicated without checking for genuine novelty.
- Agent uses the brainstorm or troubleshoot handoff template instead of investigate.
- Agent proceeds without asking user to confirm the classification.
