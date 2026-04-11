# Scenario 01: Create Root Biome Configuration for an Nx Workspace

## User Prompt

You are setting up Biome in a new Nx monorepo that currently has no linting or formatting tooling.

Produce a single file: `biome.json` placed at the workspace root.

## Requirements

- Reference the Biome JSON schema URL (`https://biomejs.dev/schemas/1.9.4/schema.json`) via the `$schema` field.
- Enable the formatter with `indentStyle: "space"` and `indentWidth: 2`.
- Enable the linter and activate the recommended ruleset.
- Set `files.ignoreUnknown` to `false`.

## Output

Produce the file `biome.json` with the correct content.

## Expected Behavior

1. Include a `$schema` field pointing to a `biomejs.dev/schemas` URL
2. Set `formatter.enabled` to `true`, `formatter.indentStyle` to `"space"`, and `formatter.indentWidth` to `2`
3. Set `linter.enabled` to `true`
4. Set `linter.rules.recommended` to `true`
5. Set `files.ignoreUnknown` to `false`

## Success Criteria

- **Schema reference present**: The file contains a `$schema` field pointing to a `biomejs.dev/schemas` URL.
- **Formatter enabled with correct indent settings**: `formatter.enabled` is `true`, `indentStyle` is `"space"`, and `indentWidth` is `2`.
- **Linter enabled**: `linter.enabled` is `true`.
- **Recommended ruleset activated**: `linter.rules.recommended` is `true`.
- **files.ignoreUnknown set to false**: `files.ignoreUnknown` is explicitly set to `false`.

## Failure Conditions

- The `$schema` field is missing or does not reference `biomejs.dev/schemas`
- The formatter is missing, disabled, or uses incorrect indent settings (e.g. tabs or width 4)
- `linter.enabled` is absent or `false`
- `linter.rules.recommended` is absent or `false`
- `files.ignoreUnknown` is absent or set to `true`
