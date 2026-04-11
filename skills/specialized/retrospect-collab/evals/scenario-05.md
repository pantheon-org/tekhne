# Scenario 05: Producing a Collaboration Improvement Recommendation

## User Prompt

"Based on my sessions over the last month, give me one concrete recommendation to improve how I collaborate with AI."

## Expected Behavior

1. Agent runs the session filter script with `--month` to load the current month's sessions
2. Reads each session file and performs full Technical Effectiveness and Cognitive Posture analysis
3. Identifies the single most significant collaboration weakness across all analyzed dimensions (Context Management, Guidance, Critical Thinking, Bias, Intentionality, Agency)
4. Selects the recommendation that would have the highest leverage: the change that would most improve impact categorization or session effectiveness
5. Presents the recommendation as a concrete, actionable Start item with a specific behavioral description
6. Backs the recommendation with a minimum of two session examples that demonstrate the pattern
7. Writes the full insights report to `.retro/insights/collab/{PERIOD}.md`
8. Reports the output file path and highlights the top recommendation in the chat response

## Success Criteria

- **Single top recommendation identified**: Agent does not deliver a laundry list; one primary recommendation is elevated
- **Recommendation is actionable**: Described in terms of a specific behavior change (e.g., "before sending a complex prompt, write one sentence stating the goal"), not a concept
- **Two session examples cited**: Recommendation is backed by at least two concrete examples from the analyzed sessions
- **Dimension attributed**: Report states which collaboration dimension the recommendation addresses
- **Full report written**: Complete report with all sections is written to `.retro/insights/collab/{PERIOD}.md`, not just the recommendation
- **File path reported**: Agent outputs the path to the written report

## Failure Conditions

- Agent delivers multiple unranked recommendations without identifying a primary one
- Recommendation is abstract ("improve critical thinking") without a concrete behavioral description
- Recommendation is not backed by session evidence
- Agent skips writing the full report and only outputs the recommendation to chat
- Agent fabricates session patterns not present in the actual session data
- Recommendation contradicts the evidence (e.g., recommends stopping a pattern that was actually effective)
