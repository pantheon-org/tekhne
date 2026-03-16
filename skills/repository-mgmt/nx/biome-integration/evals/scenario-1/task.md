# Task: Create the root Biome configuration for an Nx workspace

You are setting up Biome in a new Nx monorepo that currently has no linting or formatting tooling.

Produce a single file: `biome.json` placed at the workspace root.

## Requirements

- Reference the Biome JSON schema URL (`https://biomejs.dev/schemas/1.9.4/schema.json`) via the `$schema` field.
- Enable the formatter with `indentStyle: "space"` and `indentWidth: 2`.
- Enable the linter and activate the recommended ruleset.
- Set `files.ignoreUnknown` to `false`.

## Output

Produce the file `biome.json` with the correct content.
