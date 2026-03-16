# Task: Configure nx.json targetDefaults for a full Nx workspace

You are establishing baseline task orchestration and caching for an Nx monorepo with React applications and TypeScript libraries. The workspace uses ESLint and Jest.

Produce an `nx.json` file that sets up targetDefaults for the `build`, `test`, and `lint` targets following best practices for dependency ordering and cache stability.

## Requirements

- **build** targetDefault:
  - dependsOn: `["^build"]` (requires upstream builds first)
  - inputs: `"production"` and `"^production"` named inputs
  - cache: true

- **test** targetDefault:
  - dependsOn: `["build"]` (requires this project's build first)
  - inputs: `"default"`, `"^production"`, and the workspace-root jest preset file `"{workspaceRoot}/jest.preset.js"`
  - cache: true

- **lint** targetDefault:
  - No dependsOn
  - inputs: `"default"` and the workspace-root ESLint config `"{workspaceRoot}/.eslintrc.json"`
  - cache: true

- Include `$schema` and `version: 2` in the output.

## Output

Produce the file `nx.json`.
