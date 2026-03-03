---
name: mise-complete
description: >
  Configure and operate Mise for deterministic developer environments. Use when installing runtime/tool versions,
  defining reusable tasks, managing layered environment variables, migrating from asdf/nvm/pyenv, or debugging
  mise.toml behavior in CI and local shells. Keywords: mise, mise.toml, tool versions, tasks, env, asdf migration,
  setup automation, dev environment.
allowed-tools: Read, Write, Edit, Bash
---

# Mise Complete

## When to Use

Use this skill when setup, version management, or task automation needs to be standardized with Mise.

## When Not to Use

Do not use this skill when the repository intentionally uses another tool manager as the source of truth.

## Core Principles

1. Keep one canonical `mise.toml` per project scope.
2. Pin tool versions for reproducible builds.
3. Make tasks deterministic and non-interactive by default.
4. Keep secrets out of committed task definitions.

## Deterministic Workflow

1. Detect existing tool managers and migration constraints.
2. Define or update `mise.toml` tool versions.
3. Add or normalize tasks for common workflows.
4. Configure environment variables by scope.
5. Install configured tools and validate with `mise install && mise doctor`.
6. If `mise install` fails, run `mise doctor` to diagnose path or plugin issues, then re-run `mise install`.

## Quick Commands

### Install configured tools

```bash
mise install
```

Expected result: configured runtimes are installed for the current project.

### Verify environment health after install

```bash
mise doctor
```

Expected result: no errors reported; any warnings indicate misconfigured plugins or PATH issues to resolve before proceeding.

### Verify active tool versions match pinned values

```bash
mise ls
```

Expected result: each tool shows the pinned version from `mise.toml` as the active version.

### Pin a tool version

```bash
mise use node@22
```

Expected result: `mise.toml` updated with pinned Node version.

### Run a named task

```bash
mise run test
```

Expected result: task executes with the environment defined by Mise.

### Inspect effective environment

```bash
mise env
```

Expected result: resolved environment variables and tool paths are printed.

### Evaluate this skill quality

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh mise-complete --json
```

Expected result: updated audit dimensions and grade.

## mise.toml Example

A minimal `mise.toml` showing pinned tool versions, a task definition, and a scoped environment variable:

```toml
[tools]
node = "22.4.0"
python = "3.12.3"
terraform = "1.8.5"

[tasks.test]
description = "Run test suite"
run = "npm test"

[tasks.lint]
description = "Run linter"
run = "npm run lint"

[env]
NODE_ENV = "development"
```

- All tool versions are pinned to exact releases for reproducibility.
- Tasks are self-contained; no implicit shell state is assumed.
- Environment variables are scoped to this project and not committed with secrets.

## Anti-Patterns

### NEVER keep floating tool versions in team repositories

- **WHY:** Unpinned versions cause machine-to-machine drift.
- **BAD:** `node = "latest"` for shared project tooling.
- **GOOD:** Pin exact versions required by CI and teammates.
- **Consequence:** Builds pass on one machine and fail on another.

### NEVER embed secrets in committed `mise.toml`

- **WHY:** Version control is not a secret store.
- **BAD:** Hardcode API tokens in environment blocks.
- **GOOD:** Load secrets from external secret management or local env files.
- **Consequence:** Secret leakage and mandatory credential rotation.

### NEVER define tasks that rely on implicit shell state

- **WHY:** Hidden state makes task outcomes non-deterministic.
- **BAD:** Task assumes manual `cd` or prior env exports.
- **GOOD:** Task commands are self-contained with explicit paths.
- **Consequence:** Task behavior differs across terminals and CI.

### NEVER mix multiple tool managers as active authority

- **WHY:** Overlapping managers conflict on PATH and versions.
- **BAD:** Keep both `nvm` and Mise controlling Node in CI.
- **GOOD:** Migrate ownership per tool and remove overlapping activation.
- **Consequence:** Unpredictable runtime resolution and flaky builds.

## References

- `references/tools-installation.md`
- `references/tools-versions.md`
- `references/tools-migration.md`
- `references/tasks-definition.md`
- `references/tasks-execution.md`
- `references/env-definition.md`
- `references/env-hierarchies.md`
- `references/config-structure.md`
- `references/config-management.md`
- `references/config-anti-patterns.md`
