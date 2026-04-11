# Scenario 03: Resume Save After a Long Session (Stream Reuse)

## User Prompt

"Save context — we've been working on the payments module for three hours."

## Expected Behavior

1. Agent receives no explicit stream name in arguments
2. Agent checks for a prior `/load-context` stream in the conversation to reuse
3. If a prior stream name exists (e.g., `payments`), agent uses it without prompting
4. If no prior stream is found, agent calls `AskUserQuestion` to ask for a stream name
5. After obtaining the stream name, agent checks if responses are non-empty (AskUserQuestion guard)
6. Agent synthesizes a richer Session section reflecting the long session: aggregated progression steps, key decisions, insights
7. Agent keeps Session section ≤780 tokens despite the long history by aggregating, not listing every message
8. Agent writes the CONTEXT file and runs the INDEX upsert

## Success Criteria

- **Stream resolution**: Agent reuses prior `/load-context` stream name OR asks user when none exists
- **AskUserQuestion guard applied**: After any AskUserQuestion call, agent checks for empty responses and falls back to numbered text list
- **Session aggregation**: Session section aggregates a long session into ≤780 tokens; it does not include verbatim message excerpts
- **Hot Files accurate**: Up to 10 files that were actually discussed or edited are listed (not all files in the repo)
- **INDEX updated**: `scripts/upsert-index.sh` runs successfully with all 6 args

## Failure Conditions

- Agent defaults to stream `default` without asking when no prior stream exists in conversation
- Agent exceeds 780 tokens in the Session section by listing every event
- Agent calls `AskUserQuestion` and proceeds with empty answers without showing a fallback numbered list
- Hot Files section includes files never mentioned in the session
- Agent omits the `upsert-index.sh` call for a long-running session
