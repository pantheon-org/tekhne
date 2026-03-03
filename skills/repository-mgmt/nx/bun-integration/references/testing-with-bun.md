# Testing with Bun in Nx Workspaces

Use this reference for Bun test runner integration details.

## Test Target Basics

`@nx-bun/nx:test` runs tests with Bun's built-in runner and supports standard Nx invocation.

```bash
nx test my-lib
nx test my-lib --watch
nx test my-lib --timeout=10000
```

## Suggested Options

- `bail`: fail fast to shorten feedback loops.
- `timeout`: tune per-project for integration-heavy suites.
- `watch`: local development only.

## Coverage and Outputs

- Configure coverage output paths for predictable artifact collection.
- Keep coverage directories under workspace conventions.

## Avoiding Jest Conflicts

- Do not maintain duplicate test targets for the same files.
- During migration, keep clear project/path ownership for each runner.

## CI Guidance

- Prefer non-watch execution.
- Run `nx affected -t test` to reduce CI load.
- Record flaky test behavior before and after runner migration.
