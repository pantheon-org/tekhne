# Task: Migrate a project.json test target from Jest to the Bun test runner

You have an Nx library `data-access` at `libs/data-access` whose current `project.json` includes a Jest test target. You are migrating this project to the Bun test runner as part of a workspace-wide Bun adoption.

Produce the updated `project.json` that replaces the Jest test target with an @nx-bun/nx:test target. The Jest target must be removed entirely — do not leave both targets in the output.

## Requirements

- Remove the `test` target that uses `@nx/jest:jest`.
- Add a new `test` target using `@nx-bun/nx:test`.
- The new test target must:
  - Set `cache: true`.
  - Declare outputs using a workspace-relative coverage path.
  - Include `timeout` and `bail` options (values: timeout 5000, bail true).
- Preserve all other targets (build, lint) from the starting file unchanged.

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
