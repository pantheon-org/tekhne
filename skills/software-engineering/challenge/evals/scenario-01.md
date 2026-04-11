# Scenario 01: User Says "Are You Sure?" — Anchor Dispatch

## User Prompt

"are you sure about that approach?"

## Expected Behavior

1. Agent detects no explicit subcommand in the prompt.
2. Agent invokes `AskUserQuestion` presenting options: A) Anchoring bias, B) Factual accuracy, C) Wrong framing, D) High stakes.
3. Agent checks whether answers returned are empty/blank (AskUserQuestion guard).
4. If answers are empty, agent outputs the warning message and presents the same options as a numbered text list.
5. User selects option A (Anchoring bias).
6. Agent reads `references/protocols/anchor.md` and executes all 4 anchor patterns: Gatekeeper, Reset, Alternative Approaches, Pre-mortem.
7. Agent outputs a Challenge Report following the format in `references/reference.md`.

## Success Criteria

- **AskUserQuestion guard**: Agent checks for empty/blank answers after every `AskUserQuestion` call.
- **Anchor dispatch**: After user selects A, agent dispatches to the anchor protocol.
- **Protocol file read**: Agent reads `references/protocols/anchor.md` before executing.
- **All 4 patterns applied**: Gatekeeper, Reset, Alternative Approaches, and Pre-mortem each appear in the output.
- **Report format**: Output follows the Challenge Report structure with Target, Error type, Technique Selection, Findings, Verdict, and Recommended Action sections.
- **Thinking transparency**: Each finding includes Observation, Technique family, Reasoning, and Confidence fields.

## Failure Conditions

- Agent proceeds with assumptions when AskUserQuestion returns empty answers.
- Agent dispatches to wrong subcommand (e.g., verify or framing).
- Agent skips reading `references/protocols/anchor.md` and improvises pattern execution.
- Agent applies fewer than 4 anchor patterns.
- Agent produces prose narrative instead of structured Challenge Report.
- Agent omits Confidence rating from any finding.
