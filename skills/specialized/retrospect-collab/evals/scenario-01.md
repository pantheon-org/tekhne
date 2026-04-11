# Scenario 01: Review a Single Session for Collaboration Quality

## User Prompt

"Analyze my last session for collaboration quality — how well did I use context, guidance, and critical thinking?"

## Expected Behavior

1. Agent runs the session filter script with `--last 1d` or equivalent single-session scope
2. Reads the session file and extracts user prompts, tool calls, and duration metadata
3. Analyzes Technical Effectiveness across all four dimensions: Context Management, Guidance, Critical Thinking, and Bias
4. Analyzes Cognitive Posture across Intentionality, Agency, Impact, and Progression dimensions
5. Generates Start/Stop/Continue recommendations backed by specific session examples
6. Writes the insights report to `.retro/insights/collab/{PERIOD}.md`
7. Reports the output file path to the user

## Success Criteria

- **Session filter executed**: Agent runs `retrospect-load-sessions.sh` and parses the PERIOD header and session paths from output
- **All four technical dimensions covered**: Report contains sections for Context Management, Guidance, Critical Thinking, and Bias
- **Evidence-based analysis**: Each finding references actual prompts or tool calls from the session, not generic observations
- **Impact categorized**: Session is classified as Automation, Low-impact, or High-impact augmentation
- **Start/Stop/Continue present**: Report includes at least one item per category in the SSC section
- **Output file written**: `.retro/insights/collab/` file created with the correct PERIOD in the filename
- **File path reported**: Agent outputs the path to the written file

## Failure Conditions

- Agent invents issues not evidenced in the session data
- Agent skips one or more of the four technical effectiveness dimensions
- Agent writes vague findings like "prompts could be clearer" without citing specific examples
- Agent does not classify the session's impact category
- Agent fails to write the output file and only prints findings to chat
- Agent uses a hardcoded path instead of the project-configurable `.retro/insights/` location
