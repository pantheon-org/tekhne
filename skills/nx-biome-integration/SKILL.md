---
name: nx-biome-integration
description: Integrate Biome into Nx monorepos with deterministic setup, caching, migration from ESLint and Prettier, and plugin-based inferred tasks; use when adding Biome, replacing ESLint/Prettier, tuning cache inputs, or scaling lint and format workflows across projects.
---

# Nx Biome Integration

Navigation hub for integrating Biome in Nx workspaces with predictable setup and migration.

## When to Use

- You need to add Biome lint and format targets in an Nx workspace.
- You are migrating from ESLint or Prettier to Biome.
- You need Nx cache-friendly Biome task configuration.
- You are creating plugin-based inferred tasks for workspace scale.

## Principles

- Prefer one root config unless migration constraints require nested overrides.
- Keep lint and format authority singular for each file path.
- Optimize for deterministic CI (read-only checks) and reproducible local fixes.

## When Not to Use

- The repository standard is intentionally ESLint/Prettier-only.
- The task is not Nx-related and does not require workspace task integration.

## Workflow

1. Confirm prerequisites: Nx workspace, Node.js 18+, package manager selected.
2. Install Biome and initialize root configuration.
3. Add project targets (or plugin inferred tasks) and enable Nx caching inputs.
4. Validate lint and format execution on one project, then run across workspace.
5. Apply migration strategy (all-at-once or selective nested `biome.json`).
6. Enforce anti-pattern checks before rollout.

## Quick Commands

### Install and Initialize

```bash
npm install --save-dev @biomejs/biome
npx @biomejs/biome init
```

Expected: root `biome.json` exists.

### Run on One Project

```bash
nx biome-lint my-project
nx biome-format my-project
```

Expected: lint and format targets execute for the selected project.

### Run Across Workspace

```bash
nx run-many -t biome-lint
nx run-many -t biome-format
```

Expected: all configured projects run with consistent config and cache behavior.

### Run Through Package Script

```bash
bun run nx run-many -t biome-lint
```

Expected: Biome lint runs through package-managed Nx entrypoint.

### Reset Nx State After Plugin Changes

```bash
nx reset
```

Expected: daemon cache is cleared and targets refresh.

## Anti-Patterns

### NEVER use nested `biome.json` for simple workspace-wide setups

**WHY**: unnecessary config layering increases maintenance and rule drift.

**BAD**: create `biome.json` in every project by default.

**GOOD**: use one root config and add nested configs only for migration or real overrides.
BAD: nested configs everywhere. GOOD: root-first config with explicit exceptions.

### NEVER run ESLint and Biome on the same files without explicit boundary

**WHY**: dual lint pipelines produce conflicting diagnostics and wasted CI time.

**BAD**: keep ESLint glob coverage unchanged after adding Biome.

**GOOD**: define migration boundaries so each file set has one active lint authority.
BAD: same files linted twice. GOOD: single lint owner per file path.

### NEVER omit `biome.json` and Biome dependency from Nx cache inputs

**WHY**: cache hits become stale after config/tool version changes.

**BAD**: cache only `default` and `^default` inputs.

**GOOD**: include `"{workspaceRoot}/biome.json"` and `externalDependencies: ["@biomejs/biome"]`.
BAD: omit config/dependency inputs. GOOD: include config and tool-version inputs.

### NEVER auto-enable formatter writes in CI verification jobs

**WHY**: write-mode formatting in CI mutates files and obscures validation failures.

**BAD**: run `biome format --write` in required CI checks.

**GOOD**: use check/lint mode in CI; reserve write-mode for local or dedicated fix jobs.
BAD: mutating CI verification. GOOD: read-only CI checks.

### NEVER downgrade critical Biome rules to suppress migration noise

**WHY**: suppressing high-signal rules hides real issues and delays stabilization.

**BAD**: globally reduce important rules to `off` during migration.

**GOOD**: scope temporary exceptions narrowly with documented expiration.
BAD: blanket rule disablement. GOOD: time-boxed scoped exceptions.

## Reference Map

- Deep configuration and cache tuning: `references/biome-configuration-deep-dive.md`
- Migration from ESLint and Prettier: `references/migration-guide.md`
- Plugin-based inferred tasks: `references/plugin-patterns.md`

## References

- [Biome Docs](https://biomejs.dev/)
- [Nx Plugin Docs](https://nx.dev/extending-nx)
- [Nx Inferred Tasks](https://nx.dev/concepts/inferred-tasks)
- [Biome Big Projects Guide](https://biomejs.dev/guides/big-projects/)
