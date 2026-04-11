# Scenario 05: "Poke Holes in This Plan" — General Challenge with No-Subcommand Fallback

## User Prompt

"poke holes in this plan: we'll fix our deployment flakiness by adding a 30-second sleep before the health check"

## Expected Behavior

1. Agent detects no recognized subcommand in the prompt ("poke holes" is not anchor/verify/framing/deep).
2. Agent invokes `AskUserQuestion` presenting the four options: A) Anchoring bias, B) Factual accuracy, C) Wrong framing, D) High stakes.
3. Agent checks whether answers are empty/blank (AskUserQuestion guard).
4. If answers are empty, agent outputs the warning and presents options as a numbered text list, then waits.
5. User provides an answer selecting their concern (e.g., B or C).
6. Agent dispatches to the matching subcommand.
7. Agent reads the matching protocol file and executes it.
8. Agent produces a structured Challenge Report.

## Success Criteria

- **No-subcommand fallback triggered**: Agent does not attempt to map "poke holes" to a subcommand directly.
- **AskUserQuestion called**: Agent presents the four-option clarification question.
- **AskUserQuestion guard**: Agent checks for empty answers and falls back to numbered list if needed.
- **Correct dispatch on answer**: Agent dispatches to the subcommand matching the user's selection.
- **Protocol executed**: Agent reads and follows the matching protocol file.
- **Report produced**: Structured Challenge Report output, not free-form commentary.

## Failure Conditions

- Agent arbitrarily maps "poke holes" to a specific subcommand without asking the user.
- Agent skips `AskUserQuestion` and launches a protocol immediately.
- Agent does not check for empty answers after `AskUserQuestion`.
- Agent proceeds with assumptions when answers are empty.
- Agent produces a bulleted list of concerns without using the structured Challenge Report format.
- Agent omits the Verdict and Recommended Action sections from the report.
