# Scenario 01: Vague Problem Routes to Brainstorm

## User Prompt

"Our users keep churning after onboarding. Something feels off but we have no idea what it is or where to start."

## Expected Behavior

1. Agent detects vague problem with no stated hypothesis and no clear cause-effect; confidence in auto-classification is below 80%.
2. Agent runs triangulation: asks T1 (who has done this before), T2 (same inputs, same result), T3 (can you take it apart), Q-Scale.
3. User answers indicate T1=3 (outside org knowledge), T2=unpredictable (path-dependent), T3=entangled — all three converge on Complex.
4. Agent presents classification: Complex domain with enabling constraints, no hypothesis yet.
5. Agent applies routing table: Complex + no hypothesis → brainstorm → probe → openspec-plan.
6. Agent loads `references/frame-problem-to-brainstorm-llm.md` and produces a handoff with thinking trail, decisions, and accumulated context.
7. Agent asks user to confirm before invoking brainstorm.

## Success Criteria

- **Triangulation runs**: All three tests (T1, T2, T3) and Q-Scale are asked before classification.
- **Domain correctly identified**: Agent classifies as Complex (not Complicated).
- **Route matches table**: Agent selects brainstorm → probe chain, not investigate or troubleshoot.
- **Handoff template used**: Agent references or fills `references/frame-problem-to-brainstorm-llm.md`.
- **Confirmation sought**: Agent uses AskUserQuestion to confirm before invoking the next skill.
- **No premature solution**: Agent does not propose implementation steps or diagnoses during framing.

## Failure Conditions

- Agent classifies the problem as Complicated and routes to investigate without checking T3 (entanglement).
- Agent skips triangulation and immediately suggests a solution to the churn problem.
- Agent routes to troubleshoot because the word "problem" appeared in the prompt.
- Agent does not use the handoff template and passes bare context to brainstorm.
- Agent proceeds to invoke brainstorm without AskUserQuestion confirmation.
- Agent asks the user "what domain does this belong to?" instead of asking the three triangulation questions.
