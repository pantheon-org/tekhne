# Scenario 00: Onboard Biome to a TypeScript Project

## User Prompt

A team is starting a new TypeScript API project and wants consistent code quality from day one. They've decided to use Biome as their single tool for both linting and formatting. They need a biome.json tailored to their conventions: 2-space indentation, double quotes, semicolons enabled, Node.js globals available to the linter, and the `dist/` and `node_modules/` directories excluded from checks.

Set up Biome for the project. Produce:
1. `biome.json` — the configured Biome config file
2. `SETUP.md` — a short guide with the commands to run checks and apply fixes

Include the commands for local development and for CI.

## Expected Behavior

1. Reference `bunx @biomejs/biome init` as the initialization step in `SETUP.md`
2. Configure `biome.json` with an explicit `formatter` section setting `indentWidth` to 2 and `indentStyle` to space
3. Configure an explicit quote style (e.g., `javascript.formatter.quoteStyle: "double"`)
4. Add a `files.ignore` or `vcs` section that excludes `dist/` and/or `node_modules/`
5. Document the local dev check command (`bunx @biomejs/biome check .`) in `SETUP.md`
6. Document the CI check command with `--error-on-warnings` in `SETUP.md`

## Success Criteria

- **Init command referenced**: `SETUP.md` mentions `bunx @biomejs/biome init` as the initialization step
- **Formatter indent configured**: `biome.json` contains an explicit formatter section with `indentWidth` or `indentStyle` set to 2-space
- **Quote style configured**: `biome.json` has an explicit quote style setting (e.g., `javascript.formatter.quoteStyle` or `formatter.quoteStyle`)
- **Ignore patterns configured**: `biome.json` includes a `files.ignore` or `vcs` section that excludes `dist/` and/or `node_modules/`
- **Check command documented**: `SETUP.md` includes `bunx @biomejs/biome check .` for running diagnostics locally
- **CI command with error-on-warnings**: `SETUP.md` includes `bunx @biomejs/biome check . --error-on-warnings` for CI usage

## Failure Conditions

- Agent does not mention `bunx @biomejs/biome init` in `SETUP.md`
- Agent omits the `formatter` section or does not set indent width to 2-space
- Agent does not configure an explicit quote style in `biome.json`
- Agent does not configure ignore patterns for `dist/` or `node_modules/`
- Agent omits the local `biome check .` command from `SETUP.md`
- Agent omits or uses the wrong CI command (missing `--error-on-warnings` flag)
