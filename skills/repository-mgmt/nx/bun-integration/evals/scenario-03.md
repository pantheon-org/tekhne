# Scenario 03: Migrate a project.json Test Target from Jest to the Bun Test Runner

## User Prompt

You have an Nx library `data-access` at `libs/data-access` whose current `project.json` includes a Jest test target. You are migrating this project to the Bun test runner as part of a workspace-wide Bun adoption.

Produce the updated `project.json` that replaces the Jest test target with an `@nx-bun/nx:test` target. The Jest target must be removed entirely — do not leave both targets in the output.

## Requirements

- Remove the `test` target that uses `@nx/jest:jest`.
- Add a new `test` target using `@nx-bun/nx:test`.
- The new test target must:
  - Set `cache: true`.
  - Declare outputs using a workspace-relative coverage path.
  - Include `timeout` and `bail` options (values: timeout 5000, bail true).
- Preserve all other targets (`build`, `lint`) from the starting file unchanged.

## Starting project.json

```json
{
  "name": "data-access",
  "root": "libs/data-access",
  "projectType": "library",
  "sourceRoot": "libs/data-access/src",
  "targets": {
    "build": {
      "executor": "@nx/js:tsc",
      "outputs": ["{options.outputPath}"],
      "options": {
        "outputPath": "dist/libs/data-access",
        "main": "libs/data-access/src/index.ts",
        "tsConfig": "libs/data-access/tsconfig.lib.json"
      }
    },
    "lint": {
      "executor": "@nx/linter:eslint",
      "options": {
        "lintFilePatterns": ["libs/data-access/**/*.ts"]
      }
    },
    "test": {
      "executor": "@nx/jest:jest",
      "outputs": ["{workspaceRoot}/coverage/libs/data-access"],
      "options": {
        "jestConfig": "libs/data-access/jest.config.ts",
        "passWithNoTests": true
      }
    }
  }
}
```

## Output

Produce the updated file `project.json`.

## Expected Behavior

1. Remove the `test` target that uses `@nx/jest:jest` entirely
2. Add a new `test` target using `@nx-bun/nx:test` as the executor
3. Set `cache: true` on the new test target
4. Declare outputs using a workspace-relative coverage path (containing `{workspaceRoot}/coverage`)
5. Preserve the `build` and `lint` targets exactly as they appear in the starting file

## Success Criteria

- **Jest test target removed**: The output does NOT contain any target with executor `@nx/jest:jest`.
- **New test target uses @nx-bun/nx:test**: `targets.test.executor` is exactly `"@nx-bun/nx:test"`.
- **New test target has cache: true**: `targets.test.cache` is `true`.
- **New test target declares workspace-relative coverage output**: `targets.test.outputs` contains a path with `{workspaceRoot}/coverage` (not a project-relative path).
- **Build and lint targets preserved unchanged**: `targets.build` and `targets.lint` are identical to the starting file.

## Failure Conditions

- The Jest target (`@nx/jest:jest`) remains in the output (dual test ownership anti-pattern)
- New `test` target does not use `@nx-bun/nx:test` executor
- New `test` target has `cache` absent or `false`
- New `test` target uses a project-relative coverage path instead of a `{workspaceRoot}/coverage/...` path
- `build` or `lint` targets are modified, missing, or contain different options than the starting file
