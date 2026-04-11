# Scenario 01: Extracting Learnings from a Debugging Session

## User Prompt

"Review yesterday's debugging session and extract what I learned about the codebase."

## Expected Behavior

1. Agent runs the session filter script with `--last 1d` to load sessions from the previous day
2. Checks whether a `technical` domain framework file exists at the project-configurable domain frameworks location
3. Reads the session file and extracts conversation turns, tool calls used, and git context
4. Applies the Technical Domain framework: new patterns discovered, decisions made, what worked, what failed, edge cases encountered
5. Produces structured domain learnings under the four Technical Domain headings: What I Learned, What Worked Well, What Didn't Work, Decisions Made
6. Generates Start/Stop/Continue recommendations grounded in the session evidence
7. Writes insights to `.retro/insights/domain/{PERIOD}.md`
8. Reports the output file path and suggests running `/retrospect collab` or `/retrospect report` for broader context

## Success Criteria

- **Technical domain framework applied**: Report uses Technical Domain headings (patterns, decisions, edge cases), not the generic framework
- **Session evidence cited**: At least two findings reference specific tools used or conversation turns from the session
- **Four required sections populated**: What I Learned, What Worked Well, What Didn't Work, Decisions Made are all non-empty
- **Start/Stop/Continue present**: At least one item per category in the SSC section
- **Output file written**: File exists at `.retro/insights/domain/` with the correct PERIOD in the filename
- **Follow-up suggestion made**: Agent suggests the next retrospect command in the chat response

## Failure Conditions

- Agent uses the generic framework instead of the Technical Domain framework for a debugging session
- Agent invents technical decisions not evidenced in the session
- Report sections are empty or contain only placeholder text
- Agent does not write the output file
- Follow-up suggestion to `/retrospect collab` or `/retrospect report` is omitted
- Agent uses a hardcoded output path instead of the project-configurable `.retro/insights/` location
