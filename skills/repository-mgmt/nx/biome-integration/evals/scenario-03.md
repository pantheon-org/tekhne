# Scenario 03: Add Explicit Biome Targets to a project.json

## User Prompt

You are adding Biome lint and format targets to an existing Nx library project named `ui-components`, located at `libs/ui-components`.

The workspace root already contains a `biome.json`. Your job is to produce the updated `project.json` for this project that includes `biome-lint` and `biome-format` targets.

## Requirements

- The file must be valid JSON for the project at `libs/ui-components`.
- Add a `biome-lint` target that runs `npx @biomejs/biome lint` against the project root.
- Add a `biome-format` target that runs `npx @biomejs/biome format --write` against the project root (for local use).
- Both targets must reference the project root directory (use `{projectRoot}` token).
- Both targets must have `cache: true`.
- The existing project metadata (`name`, `root`, `projectType`, `sourceRoot`) should be preserved in the output.

## Starting project.json

```json
{
  "name": "ui-components",
  "root": "libs/ui-components",
  "projectType": "library",
  "sourceRoot": "libs/ui-components/src",
  "targets": {}
}
```

## Output

Produce the updated file `project.json` for `libs/ui-components`.

## Expected Behavior

1. Preserve all existing project metadata: `name`, `root`, `projectType`, and `sourceRoot`
2. Add a `biome-lint` target to `targets`
3. Add a `biome-format` target to `targets`
4. The `biome-lint` target's command must invoke `@biomejs/biome lint` and reference `{projectRoot}`
5. The `biome-format` target's command must invoke `@biomejs/biome format` and reference `{projectRoot}`
6. Both targets must have `cache: true`

## Success Criteria

- **Existing project metadata preserved**: The output `project.json` retains `name`, `root`, `projectType`, and `sourceRoot` from the starting file.
- **biome-lint target present**: `targets.biome-lint` exists in the output.
- **biome-format target present**: `targets.biome-format` exists in the output.
- **biome-lint uses correct biome lint command with {projectRoot}**: The `biome-lint` target's command or executor invokes `@biomejs/biome lint` and references `{projectRoot}`.
- **biome-format uses correct biome format command with {projectRoot}**: The `biome-format` target's command or executor invokes `@biomejs/biome format` and references `{projectRoot}`.
- **cache: true on both targets**: Both `biome-lint` and `biome-format` targets have `cache` set to `true`.

## Failure Conditions

- Project metadata (`name`, `root`, `projectType`, or `sourceRoot`) is modified or missing
- `biome-lint` target is absent from `targets`
- `biome-format` target is absent from `targets`
- `biome-lint` target does not run `@biomejs/biome lint` or does not include `{projectRoot}`
- `biome-format` target does not run `@biomejs/biome format` or does not include `{projectRoot}`
- Either target has `cache` absent or set to `false`
