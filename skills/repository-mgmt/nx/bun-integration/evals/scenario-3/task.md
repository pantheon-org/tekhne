# Scenario 3: Migrate a project.json Test Target from Jest to the Bun Test Runner

## User Prompt

You have an Nx library `data-access` at `libs/data-access` whose current `project.json` includes a Jest test target. You are migrating this project to the Bun test runner as part of a workspace-wide Bun adoption.

Produce the updated `project.json` that replaces the Jest test target with an `@nx-bun/nx:test` target. The Jest target must be removed entirely — do not leave both targets in the output.
