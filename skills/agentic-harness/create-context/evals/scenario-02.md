# Scenario 02: Priority-Based Token Threshold Decisions

## User Prompt

"Bootstrap context. I have a 600-word spec (HIGH), a 4000-word design doc (MEDIUM), and a 30000-word legacy reference (HIGH)."

## Expected Behavior

1. Agent accepts the three files with their stated priorities
2. Agent estimates tokens: spec ≈ 800 tokens (600/0.75), design doc ≈ 5333 tokens (4000/0.75), legacy ref ≈ 40000 tokens (30000/0.75)
3. For the 800-token HIGH file: copies inline to baseline (≤1500 threshold)
4. For the 5333-token MEDIUM file: copies to `.context/session/ctx/`, summarizes directly (>2500 but ≤25K)
5. For the 40000-token HIGH file: copies to `.context/session/ctx/`, dispatches `summarize-for-context` sub-agent (>25K)
6. Manifest records correct `action` field for each file: `inline`, `summarized`, `summarized`
7. Agent writes baseline referencing summary file paths for the two larger files

## Success Criteria

- **Token estimation**: Agent uses `tokens ≈ words / 0.75` formula
- **Inline threshold respected**: 800-token HIGH file appears inline in baseline; not summarized
- **Direct summarize path**: 5333-token MEDIUM file is summarized directly (not sub-agent)
- **Sub-agent dispatched**: 40000-token HIGH file triggers `summarize-for-context` Task call
- **Manifest actions**: Each source entry has the correct `action` value matching the applied logic
- **Baseline under budget**: Final baseline is ≤2000 tokens (summaries referenced, not included verbatim)

## Failure Conditions

- Agent ignores token thresholds and inlines all files regardless of size
- Agent dispatches sub-agent for a file that is within the direct summarize range (≤25K)
- Agent fails to use the `summarize-for-context` sub-agent for the >25K file
- Manifest `action` fields do not match what the agent actually did
- Agent counts words instead of estimating tokens (word count ≠ token estimate)
