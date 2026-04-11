# Scenario 05: Hypothesis Invalidated — Routes to Brainstorm

## User Prompt

"We probed whether switching to a graph database would improve our recommendation query performance by 10x. The probe results showed no meaningful improvement — graph traversal latency matched our existing SQL joins within 5% margin."

## Expected Behavior

1. Agent receives probe results and classifies them: hypothesis predicted 10x improvement; observed result is within 5% of baseline.
2. Agent classifies result as `refuted`: observable evidence directly contradicts the hypothesis.
3. Agent runs Phase 2 exit gate:
   a. Classifies result: `refuted`
   b. Selects transition: refuted → Complex (brainstorm) → `references/probe-to-brainstorm-llm.md`
4. Agent fills the handoff with:
   - Hypothesis tested (graph DB → 10x recommendation performance improvement)
   - What was refuted (graph traversal offers no latency advantage over SQL joins for this query pattern)
   - Eliminated causes: graph DB structure is NOT the bottleneck — do not re-explore this path
   - Sensed patterns: bottleneck likely elsewhere (e.g., network, data volume, query planner)
   - New brainstorm goal: surface alternative hypotheses about where the actual performance bottleneck lies
5. Agent writes thinking artifact with eliminated causes documented.
6. Agent presents transition to brainstorm and does NOT suggest re-probing the graph DB hypothesis.

## Success Criteria

- **Result correctly classified as `refuted`**: Agent does not reclassify as `partial` because the margin was small.
- **Correct transition selected**: refuted → probe-to-brainstorm-llm.md, not probe-to-probe or probe-to-investigate.
- **Eliminated causes documented**: Handoff explicitly states that graph DB is not the bottleneck — brainstorm must not revisit it.
- **Sensed pattern articulated**: Agent offers an interpretation of what the refuted result implies about the actual bottleneck.
- **Thinking artifact written**: Result persisted with all required sections.
- **No re-probe suggested**: Agent does not propose another graph DB experiment with different parameters.

## Failure Conditions

- Agent classifies result as `partial` because "5% could be measurement noise" and routes to re-probe.
- Agent routes to investigate because the performance bottleneck is now a Complicated question.
- Agent omits eliminated causes from the handoff, allowing brainstorm to re-explore the graph DB hypothesis.
- Agent suggests re-running the probe with a larger dataset before concluding.
- Agent does not articulate a sensed pattern — just states "hypothesis was wrong."
- Agent skips the thinking artifact because the probe was unsuccessful.
