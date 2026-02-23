---
name: nx-bun-integration
description: Integrate Bun runtime into Nx monorepos with deterministic plugin setup, executor configuration, migration from Node.js toolchains, and cache-aware build/test workflows; use when adding @nx-bun/nx, converting projects, or standardizing Bun targets across Nx workspaces.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
  - Grep
  - Glob
---

# Nx Bun Integration

Navigation hub for integrating Bun into Nx workspaces.

## When to Use

- You are adding Bun projects to an existing Nx workspace.
- You are converting Node.js projects to Bun executors.
- You need cache-aware build, run, and test targets with `@nx-bun/nx`.
- You need predictable migration guidance for package-manager and runtime changes.

## When Not to Use

- The task is Bun runtime API usage without Nx integration concerns.
- The workspace is intentionally standardized on Node.js executors only.

## Workflow

1. Install `@nx-bun/nx` and initialize the plugin.
2. Generate or convert projects to Bun executors.
3. Configure cache inputs and outputs for build/test targets.
4. Validate serve, build, and test on one project.
5. Roll out to affected projects and enforce anti-pattern checks.

## Quick Commands

### Install and Initialize Plugin

```bash
bun add -D @nx-bun/nx
nx g @nx-bun/nx:init --unitTestRunner=bun
```

Expected: plugin is installed and generators/executors are available.

### Generate Bun Application

```bash
nx g @nx-bun/nx:application api --directory=apps/backend --tags=runtime:bun,type:app
```

Expected: Bun-ready app with Nx project configuration.

### Convert Existing Project

```bash
nx g @nx-bun/nx:convert-to-bun --project=my-api
```

Expected: project targets move to Bun executors while preserving Nx target structure.

### Verify Build/Test/Run

```bash
nx build my-api
nx test my-api
nx serve my-api
```

Expected: build outputs generated, tests run under Bun test runner, serve target starts.

### Run Affected in CI-style

```bash
bun nx affected -t lint,test,build --base=main --parallel=3
```

Expected: only changed projects execute with Nx scheduling.

## Anti-Patterns

### NEVER mix npm/yarn installs with Bun in the same migration scope

**WHY**: mixed lockfiles and resolver behavior cause drift and non-reproducible installs.

**BAD**: `npm install` after `bun add` in a Bun-migrated workspace.

**GOOD**: use Bun consistently for migrated projects and clean old lockfiles per migration policy.

### NEVER run Jest and `bun test` on the same suite without boundary

**WHY**: duplicate test ownership creates inconsistent results and CI noise.

**BAD**: keep Jest target unchanged while adding `@nx-bun/nx:test` to same files.

**GOOD**: split test ownership by project/path and decommission old target deliberately.

### NEVER skip cache metadata on Bun build/test targets

**WHY**: missing outputs/inputs reduces cache hit reliability and slows CI.

**BAD**: executor configured with no `outputs` and no tuned inputs.

**GOOD**: declare outputs and reference production/default named inputs as appropriate.

### NEVER assume SQLite behavior is identical to Node DB wrappers

**WHY**: transaction handling and connection patterns differ across libraries.

**BAD**: copy Node pooling assumptions directly into Bun SQLite implementation.

**GOOD**: use Bun SQLite patterns validated in reference examples.

### NEVER rely on hot reload defaults for production validation

**WHY**: hot/watch modes hide startup and lifecycle differences seen in production runs.

**BAD**: validate only with `hot: true` local serve.

**GOOD**: run non-hot production-oriented build and run validation before release.

## Gotchas

- `Bun.spawn` is async and returns a handle; choose stdout/stderr piping explicitly.
- Migration can leave stale lockfiles; remove obsolete lockfiles as part of a controlled cutover.
- Nx daemon/cache can mask executor changes; use `nx reset` after plugin updates.

## Quick Reference

| Topic | Reference |
| --- | --- |
| Bun runtime APIs, SQLite, and WebSockets | [references/bun-runtime-api.md](references/bun-runtime-api.md) |
| Nx executor and caching configuration | [references/nx-executor-configuration.md](references/nx-executor-configuration.md) |
| Node.js to Bun migration checklist | [references/migration-from-node.md](references/migration-from-node.md) |
| Bun testing in Nx | [references/testing-with-bun.md](references/testing-with-bun.md) |

## References

- [nx-bun Plugin Docs](https://jordan-hall.github.io/nx-bun/)
- [nx-bun GitHub](https://github.com/jordan-hall/nx-bun)
- [Nx Docs](https://nx.dev/)
- [Bun Docs](https://bun.sh/docs)
