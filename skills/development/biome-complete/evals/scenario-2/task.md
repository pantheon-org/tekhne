# Scenario 2: Migrate ESLint + Prettier to Biome

## User Prompt

A monorepo has been using ESLint for linting and Prettier for formatting since 2022. The configuration has grown unwieldy — there's an `.eslintrc.json`, `.prettierrc`, separate lint and format scripts, and intermittent CI failures when the two tools disagree on certain patterns. The team wants to migrate to Biome as the single tool for both concerns.

Perform the migration. Produce:
1. `package.json` — updated with Biome scripts replacing ESLint and Prettier scripts
2. `biome.json` — configured Biome file that replaces the existing ESLint and Prettier setup
3. `MIGRATION.md` — brief notes on what was removed and what replaces it
