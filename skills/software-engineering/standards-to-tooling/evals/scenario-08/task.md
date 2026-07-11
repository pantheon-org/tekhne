# Scenario 08: Add CI Pipeline for Tooling Checks

## User Prompt

"Now that we have ESLint, Prettier, and TypeScript config, can you add CI steps to run them on every push?"

## Setup

The project has a GitHub Actions workflow at `.github/workflows/ci.yml` that currently only runs tests. No lint or typecheck steps.

## Expected Behavior

1. Read the existing CI workflow.
2. Add steps for lint (`npm run lint`), format check (`npm run format:check`), and typecheck (`npm run typecheck`).
3. Add npm scripts to `package.json` if they don't exist.
4. Run lint/typecheck in parallel with the test step for speed, or sequentially if dependencies require it.
5. Ensure the lint job runs before or in parallel with tests to fail fast.
6. Explain the CI additions.

## Success Criteria

- The CI workflow is updated with lint, format, and typecheck steps.
- The steps reference package.json scripts.
- Scripts are added to package.json if missing.
- The additions are explained.

## Failure Conditions

- The CI config is written incorrectly (wrong syntax, missing steps).
- The agent adds inline commands instead of referencing package.json scripts.
- The agent overwrites existing CI steps without preserving them.
- Format check is omitted.
