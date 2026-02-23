---
name: nx-workspace-patterns
description: Configure and optimize Nx monorepo workspaces with deterministic project-graph structure, boundary enforcement, cache-aware pipelines, and affected-command CI workflows; use when designing workspace architecture, tightening dependency rules, or reducing CI cost through Nx task orchestration.
---

# Nx Workspace Patterns

Navigation hub for Nx workspace architecture and operations.

## When to Use

- You are structuring or refactoring an Nx monorepo.
- You need dependency boundaries enforced by tags and lint rules.
- You need reliable affected-command CI and caching performance.

## When Not to Use

- The repository is single-project with no monorepo coordination needs.
- The task is framework-specific build setup unrelated to Nx graph and task orchestration.

## Workflow

1. Structure workspace domains (`apps/`, `libs/`, `tools/`).
2. Define tags and project graph conventions.
3. Configure target pipelines and caching defaults.
4. Enforce module boundaries and verify violations fail.
5. Integrate affected commands in CI with explicit base/head strategy.

## Constraint Guidelines

### Hard Constraints

- MUST tag projects consistently for scope/type-based constraints.
- MUST configure `targetDefaults` for build/test/lint dependency flow.
- MUST use affected commands in CI for scalable execution.

### Flexible Choices

- CAN choose tag vocabulary (`scope:*`, `type:*`, `platform:*`) if consistent.
- CAN choose cache backend (Nx Cloud or self-hosted) by org constraints.
- CAN choose library granularity based on team ownership and release cadence.

### Fallback Behaviors

| Missing Config | Fallback |
| --- | --- |
| no explicit tags | allow broad deps temporarily with warning and migration plan |
| no target defaults | tasks run independently with reduced optimization |
| no CI base/head | default to main branch and document tradeoff |

## Quick Commands

```bash
nx graph
```

```bash
nx affected -t lint,test,build --base=origin/main --parallel=3
```

```bash
nx show project my-app --web
```

```bash
nx reset
```

```bash
rg -n "@nx/enforce-module-boundaries|depConstraints" .
```

## Anti-Patterns

### NEVER create circular dependencies between projects

- **WHY**: cycles degrade graph clarity and destabilize build ordering.
- **BAD**: `libs/ui` depends on `libs/data` while `libs/data` depends on `libs/ui`.
- **GOOD**: extract shared contracts to a lower-level library.

### NEVER run `run-many --all` in CI as the default verification path

- **WHY**: full-workspace execution wastes CI budget and slows feedback loops.
- **BAD**: always build/test every project on each PR.
- **GOOD**: use `nx affected` with explicit base/head.

### NEVER tag projects inconsistently

- **WHY**: boundary rules are only as strong as tag consistency.
- **BAD**: mixed ad hoc tags with no vocabulary.
- **GOOD**: defined tag taxonomy with lint enforcement.

### NEVER skip target pipeline dependencies in `targetDefaults`

- **WHY**: missing `dependsOn` prevents optimal scheduling and can hide ordering bugs.
- **BAD**: implicit task order assumptions.
- **GOOD**: explicit `dependsOn` for build/test/lint behavior.

### NEVER ignore cache inputs and outputs for critical targets

- **WHY**: incomplete cache metadata causes stale hits or unnecessary misses.
- **BAD**: cache enabled but no stable inputs/outputs.
- **GOOD**: declare outputs and relevant named inputs per target.

## Quick Reference

| Topic | Reference |
| --- | --- |
| Project graph and nx.json patterns | [references/project-graph-configuration.md](references/project-graph-configuration.md) |
| Caching and optimization | [references/caching-strategies.md](references/caching-strategies.md) |
| Boundaries and tags | [references/project-boundaries.md](references/project-boundaries.md) |
| Affected commands and CI | [references/affected-commands.md](references/affected-commands.md) |

## References

- [Nx Documentation](https://nx.dev/getting-started/intro)
- [Module Boundaries](https://nx.dev/core-features/enforce-module-boundaries)
- [Nx Cloud](https://nx.app/)
