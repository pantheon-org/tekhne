# Scenario 01: Configure nx.json targetDefaults for a Full Nx Workspace

## User Prompt

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

## Expected Behavior

1. Set `targetDefaults.build.dependsOn` to `["^build"]`
2. Set `targetDefaults.build.inputs` to include `"production"` and `"^production"`
3. Set `targetDefaults.test.dependsOn` to `["build"]`
4. Set `targetDefaults.test.inputs` to include `"default"`, `"^production"`, and `"{workspaceRoot}/jest.preset.js"`
5. Set `targetDefaults.lint.inputs` to include `"default"` and `"{workspaceRoot}/.eslintrc.json"`
6. Set `cache: true` on all three targets
7. Include `$schema` and `version: 2` at the top level

## Success Criteria

- **build.dependsOn is ["^build"]**: `targetDefaults.build.dependsOn` is exactly `["^build"]`.
- **build.inputs includes production and ^production**: `targetDefaults.build.inputs` contains both `"production"` and `"^production"`.
- **test.dependsOn is ["build"]**: `targetDefaults.test.dependsOn` is exactly `["build"]`.
- **test.inputs includes default, ^production, and jest preset**: `targetDefaults.test.inputs` contains `"default"`, `"^production"`, and `"{workspaceRoot}/jest.preset.js"`.
- **lint.inputs includes default and .eslintrc.json**: `targetDefaults.lint.inputs` contains `"default"` and `"{workspaceRoot}/.eslintrc.json"`.
- **cache: true on all three targets**: `targetDefaults.build.cache`, `targetDefaults.test.cache`, and `targetDefaults.lint.cache` are all `true`.
- **Schema and version present**: `nx.json` has a `$schema` field and `version: 2`.

## Failure Conditions

- `build.dependsOn` is absent or set to something other than `["^build"]`, breaking upstream dependency ordering
- `build.inputs` omits `"production"` or `"^production"`, causing incorrect cache invalidation
- `test.dependsOn` is absent or set to something other than `["build"]`, allowing tests to run before the build completes
- `test.inputs` omits any of `"default"`, `"^production"`, or `"{workspaceRoot}/jest.preset.js"`
- `lint.inputs` omits `"default"` or `"{workspaceRoot}/.eslintrc.json"`, causing lint results to be stale after config changes
- Any of the three targets has `cache` absent or `false`
- `$schema` or `version` is missing from the output
