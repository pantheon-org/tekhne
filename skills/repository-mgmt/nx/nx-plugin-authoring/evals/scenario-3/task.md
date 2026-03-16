# Write a Cache-Aware Nx Executor

## Problem Description

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
