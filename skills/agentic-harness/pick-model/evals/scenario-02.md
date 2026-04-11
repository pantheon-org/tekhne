# Scenario 02: Sonnet Task — Bug Fix in a Single File

## User Prompt

"What model should I pick to debug a flaky unit test in a single Python file?"

## Expected Behavior

1. Agent parses the task: bug fix, single file, unit test context
2. Agent classifies against the decision matrix — single-file bug fix maps to Sonnet (moderate debugging, single-system)
3. Agent checks complexity escalators — single file means no scope escalator; no production risk or ambiguity signals present
4. Agent outputs recommendation using the standard output format template

## Success Criteria

- Agent recommends Sonnet (not Haiku or Opus)
- Output follows the template: `[emoji] **[Model]** — [1-line reason]` on the first line
- Output includes `Cost: medium | Speed: medium` cost/speed line
- Rationale references "single-file", "bug fix", or "moderate debugging"
- No complexity escalator is incorrectly applied that would push to Opus

## Failure Conditions

- Agent recommends Haiku (under-estimates reasoning required for debugging)
- Agent recommends Opus without citing a valid escalator
- Agent omits the cost/speed line from the output
- Rationale does not reference the single-file or debugging classification signal
- Agent provides a recommendation without an explanation
