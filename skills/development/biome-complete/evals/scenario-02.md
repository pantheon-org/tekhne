# Scenario 02: Add Biome to a GitHub Actions CI Pipeline

## User Prompt

A team has Biome installed and a configured `biome.json` in their TypeScript project. They want to add Biome checks to their GitHub Actions CI pipeline so that the build fails when any lint errors or warnings are present.

The current workflow file `ci.yml` only runs tests:

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v2
      - run: bun install
      - run: bun test
```

Produce an updated `ci.yml` that adds a Biome lint step that:
1. Uses the correct CI command (not the local dev command)
2. Fails the build on both errors AND warnings
3. Runs before the test step

## Expected Behavior

1. Add a Biome step using `bunx @biomejs/biome check . --error-on-warnings` (not the bare dev command)
2. Position the Biome step before the `bun test` step in the job steps list
3. Use `bunx @biomejs/biome` (not `npx biome` or a global binary assumption)
4. Ensure `bun install` runs before the Biome step so the package is available

## Success Criteria

- **--error-on-warnings flag used**: The Biome step in `ci.yml` uses `bunx @biomejs/biome check . --error-on-warnings` (not just `biome check .` without the flag)
- **Biome step runs before test step**: The lint/check step appears before the `bun test` step in the job steps list
- **Uses bunx to invoke Biome**: The command uses `bunx @biomejs/biome` (not `npx biome` or a global biome binary assumption)
- **bun install step present before Biome**: `bun install` runs before the Biome step so the package is available

## Failure Conditions

- Agent uses `biome check .` without `--error-on-warnings`, missing the CI-fail-on-warnings requirement
- Agent places the Biome step after the `bun test` step instead of before it
- Agent uses `npx biome` or assumes a globally installed Biome binary instead of `bunx @biomejs/biome`
- Agent places the Biome step before `bun install`, causing the step to fail because the package is not installed
