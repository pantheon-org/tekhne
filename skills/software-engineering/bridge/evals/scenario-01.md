# Scenario 01: Capture a Shared Pattern Between Two Projects

## User Prompt

"/bridge HP → DS: the observability framework we built for the dashboard is reusable as atelier content"

## Expected Behavior

1. Agent loads the project config to resolve aliases `HP` and `DS`.
2. Agent parses the command: source=`HP`, target=`DS`, description from after the colon.
3. Agent auto-detects archetype from description keywords ("framework", "reusable") → `knowledge`.
4. Agent detects direction as `one-way` (no bidirectional indicator) and strength as `potential` (no active/theoretical keywords).
5. Agent checks the configured workspace root for the bridges directory.
6. Agent determines today's sequence number by counting existing `{date}-*` files.
7. Agent writes a YAML file named `{date}-{seq}-hp-to-ds.yaml` with the correct schema fields.
8. Agent responds with one line: `🔗 Bridge #N: 🧠 HP → DS (knowledge) — observability framework reusable as atelier content`.
9. Agent resumes prior work immediately.

## Success Criteria

- YAML file is created in the configured bridges directory with the correct filename pattern.
- File contains all required fields: `source`, `target`, `archetype`, `emoji`, `description`, `direction`, `strength`, `date`.
- `archetype` is `knowledge` and `emoji` is `🧠`.
- `direction` is `one-way` and `strength` is `potential`.
- Response is a single line matching the `🔗 Bridge #N:` format.
- Agent does not add commentary beyond the one-line response.

## Failure Conditions

- Agent stores a reformulated description instead of the user's verbatim text.
- Agent assigns the wrong archetype (e.g., `narrative` instead of `knowledge`).
- Agent creates the file in a location other than the configured bridges directory.
- YAML file is missing one or more required schema fields.
- Agent adds explanatory paragraphs after the capture response.
- Agent does not resume the prior conversation after pinning.
