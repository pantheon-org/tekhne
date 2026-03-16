# Add Biome to a GitHub Actions CI Pipeline

## Problem Description

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
