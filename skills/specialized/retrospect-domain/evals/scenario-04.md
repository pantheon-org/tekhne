# Scenario 04: Generating a Domain Insight Summary

## User Prompt

"Summarize everything I've learned about our API integration layer from sessions this month."

## Expected Behavior

1. Agent runs the session filter script with `--month` to load this month's sessions
2. Reads each session file and filters for sessions that include content relevant to the API integration layer (tool calls to API-related files, prompts mentioning API, relevant git context)
3. Applies the Technical Domain framework to extract: new API patterns discovered, decisions about integration approaches, successful strategies, failures, and edge cases
4. Synthesizes findings into a coherent domain insight summary covering: patterns, decisions, trade-offs, and gaps in understanding
5. Identifies what is now well-understood vs what remains uncertain or unresolved
6. Generates Start/Stop/Continue recommendations specific to the API integration domain
7. Writes insights to `.retro/insights/domain/{PERIOD}.md`
8. Reports the file path

## Success Criteria

- **Domain filtering applied**: Agent focuses on API integration content and notes how many sessions were relevant out of total loaded
- **Technical Domain framework used**: All six Technical Domain questions are addressed (new patterns, decisions, what worked, what failed, edge cases, technical debt)
- **Synthesis present**: Report provides a coherent narrative, not just a list of session summaries
- **Uncertainty documented**: Report distinguishes between what is known and what remains unresolved
- **SSC recommendations domain-specific**: Start/Stop/Continue items reference API integration practices, not generic development advice
- **Metrics summary present**: Report includes session count and period as per the report template
- **Output file written**: File exists at `.retro/insights/domain/` with correct PERIOD

## Failure Conditions

- Agent includes all sessions without filtering for API-integration relevance
- Report is a chronological list of session summaries rather than a synthesized domain view
- Technical Domain framework questions are not addressed
- Agent does not distinguish between known patterns and unresolved uncertainties
- SSC recommendations are generic and could apply to any technical domain
- Output file is not written
