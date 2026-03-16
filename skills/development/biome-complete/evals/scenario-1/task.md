# Migrate ESLint + Prettier to Biome

## Problem Description

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
