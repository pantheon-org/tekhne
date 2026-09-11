# Scenario 03: Scholar Evaluation — Publication Readiness Check

## User Prompt

A researcher has a draft paper on "Graph Neural Networks for Protein Structure Prediction" and wants to know whether it is ready for submission to a top-tier conference.

Run a full ScholarEval assessment. Produce an overall readiness verdict (ready / needs minor revisions / needs major revisions) with a score and prioritised revision list.

## Expected Behavior

1. Evaluate all 7+ ScholarEval core dimensions before rendering a readiness verdict
2. Adjust expectations to top-tier conference standards, not generic publication standards
3. Explicitly state `ready`, `needs minor revisions`, or `needs major revisions` with a clear rationale
4. Provide a numeric overall score or summary score alongside the qualitative verdict
5. Rank recommended revisions by impact/effort, ensuring they are specific enough to act on immediately

## Success Criteria

- **All ScholarEval dimensions assessed**: The agent evaluates all 7+ core dimensions before rendering a readiness verdict.
- **Conference-level benchmark applied**: The agent adjusts expectations to top-tier conference standards, not generic publication standards.
- **Clear readiness verdict provided**: The agent explicitly states ready / needs minor revisions / needs major revisions with a rationale.
- **Aggregate score calculated**: The agent provides a numeric overall score or summary score alongside the qualitative verdict.
- **Prioritised revision list produced**: Recommended revisions are ranked by impact/effort and are specific enough to act on immediately.

## Failure Conditions

- Agent renders a readiness verdict without evaluating all 7+ ScholarEval dimensions
- Agent applies generic publication standards instead of top-tier conference benchmarks
- Agent omits or provides an ambiguous readiness verdict with no rationale
- Agent provides a qualitative verdict but no numeric score
- Agent lists revisions without ranking them by impact/effort, or revisions are too vague to act on
