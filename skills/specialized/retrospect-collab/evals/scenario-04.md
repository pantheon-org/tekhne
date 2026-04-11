# Scenario 04: Computing Metrics Across Multiple Sessions

## User Prompt

"Give me the collaboration metrics for the last 30 days — prompts, tool calls, duration, and impact breakdown."

## Expected Behavior

1. Agent runs the session filter script with `--last 30d` to load sessions from the past 30 days
2. Reads each session file and extracts: `user_prompts` count, `tool_calls` count, `duration_seconds`, and `subagent_spawns`
3. Aggregates totals and computes averages: total prompts, avg prompts/session, total tool calls, avg tool calls/session, total duration, avg duration/session
4. Classifies each session into one of three impact categories: Automation, Low-impact augmentation, High-impact augmentation
5. Computes impact percentages and compares against targets: >60% High-impact, <20% Automation
6. Reports whether the High-impact target is being met or missed
7. Populates the Metrics Summary section of the report with the computed values
8. Writes full report to `.retro/insights/collab/{PERIOD}.md` and reports the file path

## Success Criteria

- **30-day scope applied**: Sessions from the full 30-day window are loaded and counted
- **All five metric fields present in report**: Sessions count, total/avg prompts, total/avg tool calls, total/avg duration, subagent count
- **Impact breakdown present**: Report states High-impact %, Low-impact %, and Automation % with session counts
- **Target comparison made**: Report explicitly states whether >60% High-impact target is met or missed
- **Metrics section formatted correctly**: Follows the template format `Sessions: N | Total prompts: X (avg Y) | ...`
- **Output file written**: File exists at `.retro/insights/collab/` with correct PERIOD name

## Failure Conditions

- Agent reports metrics without reading actual session data (fabricated numbers)
- Agent omits one or more of the five required metric fields
- Impact breakdown is missing or uses categories other than the three defined ones
- Agent does not compare impact breakdown against the defined targets
- Agent only prints metrics to chat without writing the output file
- Averages are incorrect (total divided by wrong session count)
