# Scenario 03: Pin a Scope Constraint

## User Prompt

"One constraint before we start: do not touch anything in production — staging only for this sprint."

## Expected Behavior

1. Agent acknowledges the constraint in its response.
2. Agent immediately invokes `/pin 📌 staging only — do not touch production this sprint` without being prompted.
3. Agent derives `PINS_FILE` path from CWD via the git slug mechanism.
4. Agent reads or initialises `pins.json`.
5. Agent checks for a duplicate scope item with the same content; finds none.
6. Agent checks the 5-per-type and 20-total limits before appending.
7. Agent appends the item with `type: "scope"` and `emoji: "📌"`.
8. Agent writes the updated file.
9. Agent responds with a single line: `📌 Pinned #N: 📌 staging only — do not touch production this sprint`.
10. Agent proceeds with the sprint planning discussion respecting the constraint.

## Success Criteria

- Item written to `pins.json` has `type: "scope"` and `emoji: "📌"`.
- Pin response is a single line; no explanation added.
- Agent's subsequent responses respect the scope constraint (no production references).
- Duplicate check runs before append.
- Agent does not ask the user to confirm the pin.

## Failure Conditions

- Agent acknowledges the constraint verbally but does not pin it.
- Agent uses `approved` or `correction` type instead of `scope`.
- Agent adds a second sentence explaining what the scope constraint means.
- Agent asks "Do you want me to pin this constraint?" before pinning.
- Agent later suggests production changes despite the pinned scope constraint.
