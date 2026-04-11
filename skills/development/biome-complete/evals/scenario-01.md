# Scenario 01: Migrate ESLint + Prettier to Biome

## User Prompt

A monorepo has been using ESLint for linting and Prettier for formatting since 2022. The configuration has grown unwieldy — there's an `.eslintrc.json`, `.prettierrc`, separate lint and format scripts, and intermittent CI failures when the two tools disagree on certain patterns. The team wants to migrate to Biome as the single tool for both concerns.

Perform the migration. Produce:
1. `package.json` — updated with Biome scripts replacing ESLint and Prettier scripts
2. `biome.json` — configured Biome file that replaces the existing ESLint and Prettier setup
3. `MIGRATION.md` — brief notes on what was removed and what replaces it

## Input Files

The following files are provided. Extract them before beginning.

=============== FILE: inputs/package.json ===============
{
  "name": "my-monorepo",
  "scripts": {
    "lint": "eslint src --ext .ts,.tsx",
    "lint:fix": "eslint src --ext .ts,.tsx --fix",
    "format": "prettier --write src",
    "format:check": "prettier --check src",
    "ci:lint": "eslint src --ext .ts,.tsx --max-warnings 0"
  },
  "devDependencies": {
    "eslint": "^8.50.0",
    "@typescript-eslint/parser": "^6.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "prettier": "^3.0.0"
  }
}

=============== FILE: inputs/.eslintrc.json ===============
{
  "parser": "@typescript-eslint/parser",
  "plugins": ["@typescript-eslint"],
  "rules": {
    "no-unused-vars": "error",
    "no-console": "warn"
  }
}

## Expected Behavior

1. Remove all `eslint` commands from `package.json` scripts and replace them with Biome equivalents
2. Remove all `prettier` commands from `package.json` scripts and replace them with Biome equivalents
3. Add `biome check` and `biome check --write` (or equivalent) as lint and format scripts
4. Update the CI script to use `bunx @biomejs/biome check . --error-on-warnings`
5. Remove ESLint and Prettier packages from `devDependencies`
6. Configure `biome.json` with a `linter` section containing at least one explicit rule enabled
7. Document in `MIGRATION.md` that ESLint and Prettier have been removed and replaced with Biome

## Success Criteria

- **ESLint removed from scripts**: `package.json` scripts no longer contain eslint commands — Biome commands replace them
- **Prettier removed from scripts**: `package.json` scripts no longer contain prettier commands — Biome commands replace them
- **Biome scripts added**: `package.json` includes biome check and biome check --write (or equivalent) as lint and format scripts
- **CI script uses --error-on-warnings**: The CI-targeted script in `package.json` uses `bunx @biomejs/biome check . --error-on-warnings`
- **ESLint/Prettier devDeps removed**: `devDependencies` in `package.json` no longer include eslint, prettier, or their associated plugins
- **biome.json linter configured**: `biome.json` has a linter section with at least one explicit rule enabled (not empty default)
- **MIGRATION.md notes removal**: `MIGRATION.md` explicitly states that ESLint and Prettier have been removed and replaced with Biome

## Failure Conditions

- Agent leaves eslint commands in `package.json` scripts alongside Biome commands
- Agent leaves prettier commands in `package.json` scripts alongside Biome commands
- Agent does not add Biome check and format scripts to `package.json`
- Agent uses the local dev check command instead of `--error-on-warnings` for the CI script
- Agent leaves ESLint or Prettier packages in `devDependencies`
- Agent produces an empty or default `biome.json` linter section with no explicit rules
- Agent omits `MIGRATION.md` or does not explicitly mention ESLint and Prettier removal
