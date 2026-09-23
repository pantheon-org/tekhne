# Scenario 5: Bridge a Current Project to a Long-Term Goal

## User Prompt

"/bridge CUR → LTG: the audience-building we're doing now unlocks future optionality for the platform launch if it works"

## Expected Behavior

1. Agent loads the project config and resolves aliases `CUR` and `LTG`.
2. Agent parses source=`CUR`, target=`LTG`, description from after the colon.
3. Agent auto-detects archetype: "future", "optionality", "unlocks" → `option`.
4. Agent detects `direction: one-way` (no `↔` or "bidirectional" in input).
5. Agent detects `strength: potential` from "if it works" and "would unlock" language.
6. Agent checks bridges directory, determines today's sequence number.
7. Agent writes a YAML file with `archetype: option`, `emoji: ⚡`, `direction: one-way`, `strength: potential`.
8. Agent responds with one line: `🔗 Bridge #N: ⚡ CUR → LTG (option) — audience-building unlocks future optionality for platform launch`.
9. Agent does not write anything to GTD or other directories — bridges directory only.
10. Agent resumes the prior work immediately.

## Failure Conditions

- Agent writes any file outside the bridges directory (e.g., to GTD or a project artifact).
- Agent assigns wrong archetype (e.g., `narrative` or `knowledge` instead of `option`).
- Agent assigns `strength: theoretical` when "potential" keywords are present.
- Agent reformulates the description instead of storing it verbatim.
- Agent adds commentary beyond the one-line capture response.
- Agent fails to resume the prior conversation immediately after capture.
