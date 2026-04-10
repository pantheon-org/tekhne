# Scenario 03: Migrate from asdf .tool-versions to mise.toml

## User Prompt

A project currently uses asdf for tool version management. The `.tool-versions` file is:

```
nodejs 20.11.0
python 3.11.7
ruby 3.3.0
terraform 1.7.4
```

The team also has these npm scripts in `package.json`:

```json
{
  "scripts": {
    "dev": "node server.js",
    "test": "jest --coverage",
    "lint": "eslint src/"
  }
}
```

The project has no existing `mise.toml`. Migrate the tooling to Mise.

Produce:
1. `mise.toml` — with all four tools pinned at the same versions, the three tasks defined as Mise tasks, and no secrets
2. `MIGRATION.md` — listing the steps taken and any instructions for team members (including what to do about the old `.tool-versions` file and any overlap with asdf)

## Expected Behavior

1. Pin all four tools in `[tools]` at the exact same versions from `.tool-versions`: `nodejs`/`node` 20.11.0, `python` 3.11.7, `ruby` 3.3.0, `terraform` 1.7.4 — no ranges or `latest`
2. Define `[tasks.dev]`, `[tasks.test]`, and `[tasks.lint]` with equivalent run commands matching the npm scripts
3. Ensure task run commands do not rely on implicit shell state; use explicit commands matching the npm script behavior
4. Note in `MIGRATION.md` that asdf and Mise should not both be active; instruct team to deactivate asdf or remove `.tool-versions` to avoid PATH conflicts
5. Include the step to run `mise install` (and optionally `mise doctor`) after migration in `MIGRATION.md`

## Success Criteria

- **All 4 tools pinned at exact same versions**: `mise.toml` `[tools]` section contains `nodejs`/`node` 20.11.0, `python` 3.11.7, `ruby` 3.3.0, and `terraform` 1.7.4 (exact versions, not ranges)
- **3 tasks defined as Mise tasks**: `mise.toml` defines `[tasks.dev]`, `[tasks.test]`, and `[tasks.lint]` with equivalent run commands to the npm scripts
- **Tasks are self-contained**: Task run commands do not rely on implicit shell state; they use explicit commands matching the npm script behavior
- **MIGRATION.md addresses asdf overlap**: `MIGRATION.md` notes that asdf and Mise should not both be active; instructs team to deactivate asdf or remove `.tool-versions` to avoid PATH conflicts
- **MIGRATION.md includes mise install step**: `MIGRATION.md` includes the step to run `mise install` (and optionally `mise doctor`) after migration

## Failure Conditions

- Uses floating version specifiers or versions that differ from `.tool-versions`
- Omits one or more of the three tasks (`dev`, `test`, `lint`)
- Wraps task commands in `npm run` instead of using the direct commands from `package.json` scripts
- Does not address the asdf/Mise overlap or PATH conflict risk in `MIGRATION.md`
- Omits the `mise install` instruction from `MIGRATION.md`
