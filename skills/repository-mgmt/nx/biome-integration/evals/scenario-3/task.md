# Task: Add explicit Biome targets to a project.json

You are adding Biome lint and format targets to an existing Nx library project named `ui-components`, located at `libs/ui-components`.

The workspace root already contains a `biome.json`. Your job is to produce the updated `project.json` for this project that includes `biome-lint` and `biome-format` targets.

## Requirements

- The file must be valid JSON for the project at `libs/ui-components`.
- Add a `biome-lint` target that runs `npx @biomejs/biome lint` against the project root.
- Add a `biome-format` target that runs `npx @biomejs/biome format --write` against the project root (for local use).
- Both targets must reference the project root directory (use `{projectRoot}` token).
- Both targets must have `cache: true`.
- The existing project metadata (name, root, projectType, sourceRoot) should be preserved in the output.

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
