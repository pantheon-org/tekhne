# Scenario 6: Fix Lint and Format Drift

## User Prompt

"Our main branch has formatting drift and a pile of lint errors since we added Biome. Clean it up and tell me when it's actually green."

## Repo State

A TypeScript repository with a committed `biome.json`, no ESLint or Prettier left, and sources under `src/`. Running the check currently prints a mix of fixable and non-fixable diagnostics.
