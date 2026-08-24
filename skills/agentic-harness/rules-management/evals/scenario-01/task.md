# Scenario 01: Read-First, Schema-Validated Append

## User Prompt

"New rule: never merge a pull request that touches the delivery Lambda without running the retry-sweep acceptance test locally first."

## Expected Behavior

1. Read the existing rules file first and check every `### Rule:` heading for an existing rule covering the same directive.
2. If no duplicate exists, append a new rule entry with all three required parts: `### Rule: <short imperative title>`, `**Directive:**` (ALWAYS/NEVER phrasing), `**Rationale:**` (why it exists).
3. PREFER generating the entry via the validation script's `generate` mode over hand-typing the three-line block.
4. Run the validation script's `validate` mode after appending, and only report success once it passes.
5. Inform the user the rule was added.

## Failure Conditions

- Appending without first reading the existing rules file.
- A new entry missing the Directive or Rationale part.
- No validation run after appending, or success reported without validation actually passing.
- The directive copied so loosely that it isn't actionable (e.g. missing the ALWAYS/NEVER framing).
