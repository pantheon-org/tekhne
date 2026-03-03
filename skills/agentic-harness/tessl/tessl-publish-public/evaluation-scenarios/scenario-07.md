# Scenario 07: Tessl Optimization Impact

## User Prompt

"The typescript-advanced skill scored 86% on Tessl review. Optimize it for public publishing."

## Expected Behavior

1. Agent recognizes 86% is below 90% threshold
2. Agent runs `tessl skill review skills/development/typescript-advanced --optimize`
3. Agent waits for optimization to complete
4. Agent runs `tessl skill review skills/development/typescript-advanced` again to check improvements
5. Agent compares before (86%) and after scores
6. Agent explains what optimization changed/improved
7. If now ≥90%, agent confirms ready for next publication steps
8. If still <90%, agent suggests manual improvements needed

## Success Criteria

- Agent identifies 86% is below 90% threshold
- Agent uses `--optimize` flag correctly
- Agent runs review twice (before and after optimization)
- Agent reports score improvement (e.g., 86% → 94%)
- Agent explains optimization impact clearly
- Agent confirms next steps based on final score
- Agent documents the optimization results
- Agent doesn't proceed to publish if still <90%

## Failure Conditions

- Agent skips optimization step
- Agent publishes at 86% without optimization attempt
- Agent runs optimization but doesn't re-review
- Agent doesn't compare before/after scores
- Agent proceeds to publish if still <90% after optimization
- Agent misinterprets optimization flag usage
- Agent suggests lowering threshold instead of optimizing
