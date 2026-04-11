# Scenario 02: Identifying a Recurring Decision Pattern Across Sessions

## User Prompt

"Look at my sessions from the last two weeks and tell me if there are any repeated decisions I keep making."

## Expected Behavior

1. Agent runs the session filter script with `--last 14d` to load sessions from the past two weeks
2. Reads each session file and extracts decision points: architectural choices, library selections, approach trade-offs, configuration decisions
3. Groups similar decisions across sessions and identifies any that recur two or more times
4. For each recurring decision, documents: the decision itself, the sessions in which it appeared, and the rationale given (if any)
5. Evaluates whether recurring decisions represent a consistent strategy or an unexamined default
6. Populates the Patterns Emerged section with the recurring decision pattern and cross-session evidence
7. Generates a Start or Stop recommendation if the pattern is suboptimal
8. Writes insights to `.retro/insights/domain/{PERIOD}.md`

## Success Criteria

- **Multi-session scope processed**: Agent loads sessions spanning 14 days, not just the most recent
- **Decision grouping performed**: Similar decisions from different sessions are grouped, not listed independently
- **Recurrence count stated**: Report states "this decision appeared in X of Y sessions"
- **Rationale assessment made**: Report notes whether consistent rationale was provided or the decision appears to be an unexamined default
- **Patterns Emerged section populated**: At least one cross-session pattern is documented with session references
- **Recommendation present if warranted**: If a recurring decision is identified as suboptimal, a Start or Stop item is included
- **Output file written**: `.retro/insights/domain/` file created with the correct PERIOD

## Failure Conditions

- Agent treats each session's decisions in isolation without cross-session grouping
- Agent invents decision patterns not evidenced in the session data
- Report omits the Patterns Emerged section
- Recurrence count is missing (no quantification of how often the pattern appears)
- Agent flags every decision as a pattern regardless of recurrence frequency
- Output file is not written; agent only prints findings to chat
