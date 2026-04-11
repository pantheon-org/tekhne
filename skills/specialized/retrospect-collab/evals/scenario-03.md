# Scenario 03: Flagging Under-Utilisation of Critical Thinking

## User Prompt

"Look at my sessions this month and flag anywhere I should have challenged the AI but didn't."

## Expected Behavior

1. Agent runs the session filter script with `--month` to scope the analysis to the current month
2. Reads each session file and extracts the sequence of user prompts following AI responses
3. Identifies instances where the AI produced output that warranted follow-up challenge: complex code, architectural decisions, non-trivial recommendations
4. Flags sessions where the user accepted a non-trivial AI output with no follow-up question, no alternative request, and no verification action
5. Compares flagged sessions against sessions where the user did challenge outputs to establish a ratio
6. Populates the Critical Thinking section with both "Effective challenges" and "Missed challenges" sub-sections
7. Generates a Start recommendation targeting a specific critical-thinking practice (e.g., "always request an alternative approach before accepting a major architectural suggestion")
8. Writes insights to `.retro/insights/collab/{PERIOD}.md`

## Success Criteria

- **Monthly scope applied**: Sessions loaded span the full current month via `--month` flag
- **Missed challenges section populated**: Report's Critical Thinking section includes at least two specific examples of missed challenges with session references
- **Effective challenges also documented**: Report is not one-sided; positive examples of user challenge are also cited
- **Ratio quantified**: Report states "user challenged outputs in X of Y non-trivial exchanges"
- **AI pushback examples noted**: Any instances where the AI itself challenged the user are recorded under "AI challenged me"
- **Start recommendation is specific**: Names a concrete practice to begin, not a vague directive

## Failure Conditions

- Agent invents missed challenges not evidenced in the session data
- Agent only lists missed challenges without documenting effective challenges (unbalanced report)
- Report's Critical Thinking section is empty or contains only generic observations
- Agent does not quantify the ratio of challenged vs unchallenged exchanges
- Start recommendation is absent or generic ("think more critically")
- Agent confuses bias signals with critical thinking signals in the report
