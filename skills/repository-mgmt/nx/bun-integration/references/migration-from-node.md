# Migration from Node.js Tooling to Bun in Nx

Use this checklist to migrate safely and keep CI deterministic.

## Migration Checklist

1. Install and initialize `@nx-bun/nx`.
2. Choose migration scope (single project, bounded set, or full workspace).
3. Convert target executors using `convert-to-bun`.
4. Standardize package-manager usage for migrated scope.
5. Validate run/build/test targets and cache behavior.
6. Remove obsolete scripts and lockfiles per policy.

## Compatibility Notes

- Some Node-centric dependencies may assume Node runtime behavior.
- Validate native modules and startup scripts before broad rollout.
- Keep mixed-runtime boundaries explicit through project tags.

## Package Manager Transition

- Move from `npm`/`yarn` commands to `bun` commands for migrated projects.
- Avoid parallel lockfile ownership in the same migration boundary.

## Rollout Strategy

- Start with one low-risk application.
- Capture CI timing and failure-mode differences.
- Expand migration only after baseline stability is verified.
