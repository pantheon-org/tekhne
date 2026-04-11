# Scenario 01: Configure @nx-bun/nx Executor Targets in a project.json

## User Prompt

You are setting up a new Bun-based API application in an Nx workspace. The project is named `api` and lives at `apps/backend`.

Produce a complete `project.json` for this project with Bun executor targets for build, serve, and test.

## Requirements

- Project name: `api`, root: `apps/backend`, projectType: `application`.
- **build** target:
  - executor: `@nx-bun/nx:build`
  - options: entrypoints pointing to `apps/backend/src/main.ts`, outputPath of `dist/apps/backend`, target `bun`, format `esm`
  - outputs must reference `{options.outputPath}`
  - cache: true
- **serve** target:
  - executor: `@nx-bun/nx:run`
  - options: buildTarget `api:build`, hot: true, watch: true
- **test** target:
  - executor: `@nx-bun/nx:test`
  - outputs must reference a workspace-relative coverage path (e.g. `{workspaceRoot}/coverage/apps/backend`)
  - options: timeout: 5000, bail: true
  - cache: true

## Output

Produce the file `project.json`.

## Expected Behavior

1. Set project `name` to `"api"`, `root` to `"apps/backend"`, and `projectType` to `"application"`
2. Configure the `build` target with executor `@nx-bun/nx:build` and declare `outputs` referencing `{options.outputPath}`
3. Set `cache: true` on the `build` target
4. Configure the `serve` target with executor `@nx-bun/nx:run`
5. Configure the `test` target with executor `@nx-bun/nx:test` and declare `outputs` referencing a workspace-relative coverage path containing `{workspaceRoot}/coverage`
6. Set `cache: true` on the `test` target

## Success Criteria

- **Correct project metadata**: `project.json` has `name` `"api"`, `root` `"apps/backend"`, and `projectType` `"application"`.
- **build target uses @nx-bun/nx:build**: `targets.build.executor` is exactly `"@nx-bun/nx:build"`.
- **build target has outputs referencing {options.outputPath}**: `targets.build.outputs` contains `"{options.outputPath}"`.
- **build target has cache: true**: `targets.build.cache` is `true`.
- **serve target uses @nx-bun/nx:run**: `targets.serve.executor` is exactly `"@nx-bun/nx:run"`.
- **test target uses @nx-bun/nx:test**: `targets.test.executor` is exactly `"@nx-bun/nx:test"`.
- **test target has workspace-relative coverage output**: `targets.test.outputs` contains a path with `{workspaceRoot}/coverage` (not a project-relative `./coverage` path).
- **test target has cache: true**: `targets.test.cache` is `true`.

## Failure Conditions

- Project metadata (`name`, `root`, or `projectType`) is incorrect
- `build` target does not use `@nx-bun/nx:build` executor
- `build` target `outputs` omits `{options.outputPath}`, preventing Nx cache from tracking the output
- `build` target has `cache` absent or `false`
- `serve` target does not use `@nx-bun/nx:run` executor
- `test` target does not use `@nx-bun/nx:test` executor
- `test` target `outputs` uses a project-relative path (`./coverage`) instead of a workspace-relative path (`{workspaceRoot}/coverage/...`)
- `test` target has `cache` absent or `false`
