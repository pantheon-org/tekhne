# Task: Configure @nx-bun/nx executor targets in a project.json

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
