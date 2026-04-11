# Scenario 03: Opus Task — Multi-File Refactor

## User Prompt

"I need to refactor the authentication module across 12 files to use the new OAuth library. Which model should I use?"

## Expected Behavior

1. Agent parses task: refactoring, 12 files explicitly mentioned, authentication domain
2. Agent classifies against the decision matrix — multi-file refactor (3+ files) maps directly to Opus
3. Agent also notes the "scope" complexity escalator is confirmed (12 files >> 3-file threshold)
4. Agent notes architectural decision signals (OAuth library migration, cross-file consistency)
5. Agent outputs Opus recommendation using the standard output format template

## Success Criteria

- Agent recommends Opus (not Haiku or Sonnet)
- Output follows the template: `[emoji] **[Model]** — [1-line reason]` on the first line
- Output includes `Cost: highest | Speed: slowest` cost/speed line
- Rationale explicitly references the multi-file scope (12 files or "3+ files" threshold)
- Agent does not cap at a lower tier despite multiple escalators (cap-at-Opus rule respected)

## Failure Conditions

- Agent recommends Sonnet or Haiku for a 12-file refactor
- Agent fails to cite the scope escalator (3+ files)
- Agent applies double-upgrade beyond Opus (impossible but would indicate misunderstanding)
- Output omits the cost/speed line
- Agent suggests a lower model for cost savings without explicitly noting the quality risk
