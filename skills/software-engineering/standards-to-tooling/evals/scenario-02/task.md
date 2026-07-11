# Scenario 02: Map Formatting Convention to Prettier

## User Prompt

"The team decided we should use single quotes for all strings. Can you configure Prettier for this?"

## Setup

The project has a `.prettierrc` file but no `singleQuote` setting yet. The existing config sets `printWidth: 100` and `trailingComma: "all"`.

## Expected Behavior

1. Recognize this as a standards-to-tooling mapping for Prettier.
2. Add `singleQuote: true` to `.prettierrc`.
3. Do NOT change existing settings (`printWidth`, `trailingComma`).
4. Validate that the change does not conflict with ESLint rules.
5. Show the updated config.

## Success Criteria

- `singleQuote: true` is added to `.prettierrc`.
- Existing settings are preserved.
- No conflicting ESLint rule (e.g., `quotes`) exists without appropriate config.

## Failure Conditions

- Existing config entries are removed.
- The setting is added to ESLint instead of Prettier.
- A conflicting ESLint rule is not checked or flagged.
