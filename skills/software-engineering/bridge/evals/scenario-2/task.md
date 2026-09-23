# Scenario 2: Capture a People-Bridge

## User Prompt

"/bridge HP ↔ SL: Matthieu carries context both ways between these two projects"

## Expected Behavior

1. Agent loads project config and resolves aliases `HP` and `SL`.
2. Agent parses the command: source=`HP`, target=`SL`, description from after the colon.
3. Agent detects `↔` in the command → sets `direction` to `bidirectional`.
4. Agent checks config stakeholders: "Matthieu" appears in a stakeholder with `also_in` → archetype auto-detected as `people`.
5. Agent detects description contains "carries context" as a `people` keyword signal.
6. Agent determines `strength: active` because "carries context" implies ongoing activity.
7. Agent writes a YAML file with `direction: bidirectional`, `archetype: people`, `emoji: 👤`.
8. Agent responds with one line: `🔗 Bridge #N: 👤 HP ↔ SL (people) — Matthieu carries context both ways`.
9. Agent resumes prior work.

## Failure Conditions

- Agent stores `direction: one-way` despite `↔` in the command.
- Agent assigns archetype other than `people` when person name and `also_in` config are present.
- Agent reformulates the description instead of storing it verbatim.
- Agent creates the file outside the configured bridges directory.
- Agent emits more than one line of response before resuming.
