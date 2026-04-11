# Scenario 04: Select from Multiple Streams

## User Prompt

"Resume session"

## Expected Behavior

1. Agent runs `rtk ls -t .context/session/CONTEXT-*llm.md .context/session/done/CONTEXT-*llm.md` and finds three files: `CONTEXT-llm.md`, `CONTEXT-feature-auth-llm.md`, `CONTEXT-bugfix-payments-llm.md`
2. Because multiple streams exist and no stream was specified, agent calls `AskUserQuestion` to present stream options
3. After receiving the answer, agent checks whether the answer is empty/blank
4. If the answer is empty/blank (known Claude Code bug), agent outputs the warning message and presents options as a numbered text list, then waits
5. Once a valid selection is received, agent loads and formats the chosen stream

## Success Criteria

- Agent detects multiple streams before prompting (does not assume default when multiple exist)
- Agent calls `AskUserQuestion` with all stream options listed
- Agent checks the returned answers for empty/blank values immediately after the call
- If answers are empty, agent outputs the exact warning: "Questions didn't display (known Claude Code bug outside Plan Mode)."
- Agent presents options as a numbered text list when falling back from failed AskUserQuestion
- Agent waits for explicit user reply before loading any stream
- Done-folder files are marked with the archive indicator in the options list

## Failure Conditions

- Agent silently loads the default stream without asking when multiple streams are present
- Agent calls `AskUserQuestion` but does not check for empty answer
- Agent proceeds with assumptions after receiving an empty answer
- Agent does not present numbered text list fallback when AskUserQuestion fails
- Agent loads a stream without waiting for the user's selection reply
