# Scenario 01: Simple Haiku Task — Typo Fix

## User Prompt

"Which model should I use to fix a typo in the README?"

## Expected Behavior

1. Agent parses the task description from the prompt
2. Agent classifies the task against the decision matrix — typo fix maps to Haiku (simple transform, no reasoning)
3. Agent checks complexity escalators — none apply (no ambiguity, no scope, no stakes, no novelty)
4. Agent outputs recommendation using the standard output format template

## Success Criteria

- Agent recommends Haiku (not Sonnet or Opus)
- Output follows the template: `[emoji] **[Model]** — [1-line reason]` on the first line
- Output includes `Cost: lowest | Speed: fastest` cost/speed line
- Rationale references "typo fix" or "no reasoning" as the classification signal
- No complexity escalator is applied

## Failure Conditions

- Agent recommends Sonnet or Opus for a typo fix
- Agent omits the cost/speed line from the output
- Agent applies a complexity escalator that doesn't apply to this task
- Agent outputs a recommendation without any rationale
- Agent asks clarifying questions before providing a recommendation
