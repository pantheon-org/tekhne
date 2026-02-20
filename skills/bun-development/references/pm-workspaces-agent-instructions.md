# Bun Workspaces: Agent Instructions

## Purpose

Use this guide when an agent needs to design, migrate, or maintain a Bun workspace monorepo.
These instructions are generic and reusable across projects.

## When To Use

Apply this workflow if the user asks to:

- split a codebase into packages/apps
- migrate from single-package to monorepo
- improve dependency ownership
- speed up targeted test/build/lint execution
- standardize versions across multiple packages

## Core Principles

1. Keep root package orchestration-only (`private: true`).
2. Put runtime dependencies in the workspace that actually uses them.
3. Use `"workspace:*"` for internal package dependencies.
4. Prefer package imports over deep relative cross-package imports.
5. Migrate in phases and verify after each phase.

## Minimum Workspace Setup

Root `package.json`:

```json
{
  "private": true,
  "workspaces": ["apps/*", "packages/*"]
}
```

Workspace package example:

```json
{
  "name": "@org/example-package",
  "version": "0.1.0",
  "type": "module",
  "dependencies": {
    "@org/shared": "workspace:*"
  }
}
```

## Agent Workflow

1. **Inventory**
   - Identify domains/boundaries (app entrypoints, shared/core libs, feature modules).
   - Map current dependency usage (who imports what, who owns runtime deps).

2. **Propose target topology**
   - `apps/*` for executables/services.
   - `packages/*` for reusable libraries.
   - Define package names and ownership boundaries.

3. **Create scaffolding**
   - Add root `workspaces`.
   - Add workspace `package.json` files.
   - Set internal deps via `"workspace:*"`.

4. **Migrate incrementally**
   - Move one boundary at a time.
   - Update imports to package names.
   - Keep API surfaces explicit (avoid wildcard barrels across package boundaries).

5. **Verify each phase**
   - Run tests, typecheck, lint.
   - Fix regressions before next move.

6. **Finalize scripts/CI**
   - Replace repo scripts with workspace-aware commands.
   - Add filtered workflows for targeted iteration.

## Command Patterns (Bun)

```bash
# Install all workspaces
bun install

# Run in all workspaces that define the script
bun run --workspaces --if-present build
bun run --workspaces --if-present lint
bun run --parallel --workspaces --if-present test

# Run in selected workspace(s)
bun run --filter '@org/package-a' test
bun run --filter '@org/*' build

# Install for selected workspace(s)
bun install --filter '@org/*'
```

## Catalog Guidance

Use `catalog`/`catalogs` in root `package.json` when many workspaces share the same dependency versions.

Root:

```json
{
  "catalog": {
    "typescript": "^5.9.0",
    "@types/node": "^25.0.0"
  }
}
```

Workspace:

```json
{
  "devDependencies": {
    "typescript": "catalog:typescript",
    "@types/node": "catalog:@types/node"
  }
}
```

## Migration Safety Checklist

- [ ] Root is `private: true`
- [ ] All internal package dependencies use `"workspace:*"`
- [ ] No cross-workspace deep relative imports remain
- [ ] Runtime deps moved out of root to owning workspaces
- [ ] Workspace scripts run successfully with `--workspaces`
- [ ] Filtered runs work with `--filter`
- [ ] Tests/typecheck/lint pass after migration

## Common Failures To Avoid

- Keeping dependencies centralized in root after split
- Moving all modules at once (hard to debug)
- Not defining clear package ownership boundaries
- Exporting overly broad wildcard APIs across package boundaries
- Skipping per-phase verification
