# Scenario 03: Surfacing a Domain Anti-Pattern

## User Prompt

"Review this month's sessions and highlight any technical anti-patterns I keep falling into."

## Expected Behavior

1. Agent runs the session filter script with `--month` to scope analysis to the current month
2. Reads each session file and extracts implementation approaches, tool usage sequences, and any explicit failures or reversals noted in the conversation
3. Identifies approaches that recur and are associated with problems: failures, reversions, multiple fix attempts, or explicit "this didn't work" statements
4. Classifies identified patterns as anti-patterns using the Technical Domain framework (failed approaches, edge cases encountered, technical debt uncovered)
5. For each anti-pattern, documents: a descriptive name, the sessions in which it appeared, specific examples of the problem behavior, and its consequence
6. Generates a Stop recommendation for each identified anti-pattern
7. Writes insights to `.retro/insights/domain/{PERIOD}.md`
8. Reports the file path

## Success Criteria

- **Monthly scope applied**: Sessions from the current month are loaded via `--month`
- **Anti-pattern evidence-based**: Each anti-pattern is tied to specific session evidence (not invented)
- **Descriptive name assigned**: Each anti-pattern has a short descriptive label (e.g., "skipping type validation before API calls")
- **Consequence documented**: Report explains what problem the anti-pattern caused
- **Stop recommendation present**: At least one Stop item in SSC section targeting a specific anti-pattern
- **What Didn't Work section populated**: Report's "What Didn't Work" section includes the identified anti-patterns
- **Output file written**: File exists at `.retro/insights/domain/` with correct PERIOD

## Failure Conditions

- Agent invents anti-patterns not evidenced in session data
- Anti-patterns are described without session references
- Report omits consequences — just names a pattern without stating what went wrong
- Stop recommendation is absent or too generic ("avoid mistakes")
- Agent flags one-off failures as anti-patterns (single occurrence without recurrence)
- "What Didn't Work" section is empty despite identified anti-patterns
