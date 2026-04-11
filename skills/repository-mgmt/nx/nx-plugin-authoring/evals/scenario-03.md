# Scenario 03: Write a Cache-Aware Nx Executor

## User Prompt

A team needs a custom executor that runs `tsc` to compile TypeScript and writes output to `dist/{projectName}`. The executor should:
1. Read the `outputPath` option from the schema
2. Run `tsc` for the project
3. Return `{ success: boolean }` based on the result
4. Be registered so Nx can cache it correctly

Produce:
1. `executors/compile/executor.ts` — the executor implementation
2. `executors/compile/schema.json` — typed schema with `outputPath` as a required string
3. `executors/compile/target-example.json` — an example project.json target entry showing how to register this executor with outputs declared

The executor package name is `@myorg/tools`.

## Expected Behavior

1. Return an object with a `success` boolean field — not `void`, a string, or a number
2. Define `outputPath` as a required string with type and description in `schema.json`
3. Use package notation (`@myorg/tools:compile`) in `target-example.json` — not a relative path
4. Include an `outputs` array (e.g. `["{options.outputPath}"]`) in `target-example.json` so Nx can cache the task
5. Return `{ success: false }` on error and `{ success: true }` on completion — not just a happy path

## Success Criteria

- **Executor returns { success: boolean }**: `executor.ts` returns an object with a `success` boolean field — not `void`, not a string, not a number
- **schema.json has typed outputPath field**: `schema.json` defines `outputPath` as a required string with type and description
- **Package notation used in target example**: `target-example.json` uses `executor: '@myorg/tools:compile'` (package notation), not a relative path
- **outputs declared in target example**: `target-example.json` includes an `outputs` array (e.g. `['{options.outputPath}']`) so Nx can cache the task correctly
- **Executor handles success/failure**: `executor.ts` returns `{ success: false }` on error and `{ success: true }` on completion — not just a happy path

## Failure Conditions

- Executor returns `void` or always returns a truthy value without a `success` boolean
- `schema.json` uses a generic type or omits `outputPath` as a required typed field
- `target-example.json` uses a relative path for the executor instead of package notation
- `target-example.json` has no `outputs` array, preventing Nx from caching the task
- Executor has no error branch and always returns success regardless of `tsc` exit code
