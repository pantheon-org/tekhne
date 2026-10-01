# Scenario 6: Write a generator that adds a build target to an existing library

## User Prompt

Write `add-build-target/generator.ts` and its `schema.json`. The generator takes a project name and an optional `directory`, adds a `build` target to that project's configuration, and creates a small `BUILD.md` note beside it. The workspace uses `scope:` tags and a library tagged `scope:ui` must not be given a dependency on `scope:data` libraries. The generator is meant to be rolled out across the whole workspace.

## Repo state

- Nx plugin at `tools/my-plugin` with `generators.json` already present.
- Target projects already have `project.json` files with `lint` and `test` targets and existing tags.
