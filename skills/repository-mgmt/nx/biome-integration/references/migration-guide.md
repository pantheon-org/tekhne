# ESLint/Prettier to Biome Migration Guide (Nx)

Use this guide to migrate safely without lint conflicts or CI instability.

## Migration Modes

### Mode A: Big-Bang Migration

Use when workspace is small or consistency is urgent.

1. Add Biome and initialize root config.
2. Replace ESLint/Prettier targets with Biome targets.
3. Remove old lint/format scripts from CI pipelines.
4. Run workspace-wide lint and format checks.

### Mode B: Selective Migration

Use when workspace is large or has legacy areas.

1. Keep root `biome.json` as baseline.
2. Add nested `biome.json` only to opted-in projects.
3. Configure plugin inference pattern to detect project configs.
4. Keep ESLint for non-migrated projects until completion.

## ESLint to Biome Steps

1. Inventory existing ESLint targets and file globs.
2. Define Biome coverage boundaries (which projects/files move first).
3. Ensure no file path is linted by both tools simultaneously.
4. Port essential policy rules first; defer stylistic edge cases.
5. Track temporary rule exceptions with clear removal criteria.

## Prettier to Biome Steps

1. Disable Prettier format target for migrated projects.
2. Use `biome format` for local write workflows.
3. In CI, use non-write checks to validate formatting.
4. Remove duplicate formatting hooks once Biome is authoritative.

## Common Pitfalls and Fixes

### Pitfall: Duplicate lint errors in CI

Cause: ESLint and Biome both run on same files.

Fix: split coverage by project or glob and decommission old target incrementally.

### Pitfall: Cache serving stale results

Cause: missing `biome.json` or package dependency in cache inputs.

Fix: include root config and `externalDependencies` in `targetDefaults`.

### Pitfall: Formatter rewrites in validation jobs

Cause: using `--write` in CI.

Fix: separate check vs write workflows; keep CI read-only.

## Exit Criteria

- Biome is single lint/format authority for migrated scope.
- CI is deterministic and non-mutating in verification stages.
- No temporary migration exceptions remain undocumented.
