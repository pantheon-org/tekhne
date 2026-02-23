---
name: nx-executors
description: Create and operate custom Nx executors in TypeScript monorepos with deterministic schema design, ExecutorContext usage, package-reference registration, cache-aware outputs, and migration-safe testing workflows; use when implementing a new executor, debugging target resolution, or standardizing reusable task orchestration in Nx plugins.
license: MIT
compatibility: opencode
metadata:
  category: nx-development
  audience: nx-developers
---

# Nx Executors

Navigation hub for custom Nx executor authoring and operations.

## When to Use

- You are creating a new custom executor in an Nx plugin or tools package.
- You need to register or fix executor references in `project.json` targets.
- You are debugging executor resolution, schema, or runtime behavior.

## When Not to Use

- The task is generator authoring rather than executor authoring.
- The task is generic Nx workspace setup unrelated to custom executors.

## Principles

- ALWAYS keep executors as orchestration units, not business-logic containers.
- ALWAYS return deterministic outputs when targets produce artifacts.
- You may start simple, then add schema constraints as usage hardens.

## Workflow

1. Step 1: Define schema.
Preconditions: executor intent and options are known.
Actions: create `schema.json` with required fields, defaults, and descriptions.
Exit: schema validates and CLI help renders expected options.
2. Step 2: Implement executor.
Preconditions: schema exists.
Actions: implement `executor.ts` default export with `ExecutorContext` and error handling.
Exit: TypeScript compiles and returns `{ success: boolean }`.
3. Step 3: Register executor.
Preconditions: implementation path is final.
Actions: register in plugin manifest and reference via package notation.
Exit: `nx run <project>:<target> --help` resolves successfully.
4. Step 4: Verify execution.
Preconditions: target exists in `project.json`.
Actions: run with and without cache; confirm output paths and behavior.
Exit: target executes predictably in local and CI contexts.

## Quick Commands

```bash
mkdir -p tools/executors/my-executor
```

```bash
bunx nx run my-project:my-target --help
```

```bash
bunx nx run my-project:my-target --skip-nx-cache
```

```bash
bunx nx reset
```

```bash
bun run tsc -p tsconfig.base.json --noEmit
```

```bash
rg -n '"executor"\s*:' tools apps libs
```

## Anti-Patterns

### NEVER put business logic directly inside executor entrypoints

- **WHY**: bloated executors are hard to test and reuse.
- **BAD**: 200 lines of transformation logic in `executor.ts`.
- **GOOD**: delegate to composable library functions.

### NEVER use relative executor references in targets

- **WHY**: relative paths are fragile across workspace changes.
- **BAD**: `"executor": "../../tools/executors:task"`.
- **GOOD**: `"executor": "@scope/tools:task"`.

### NEVER skip schema validation details

- **WHY**: weak schemas produce invalid runtime invocations.
- **BAD**: untyped options with no required fields.
- **GOOD**: explicit `type`, `required`, defaults, and descriptions.

### NEVER omit outputs and dependencies semantics for cacheable work

- **WHY**: Nx cache quality depends on deterministic inputs and outputs.
- **BAD**: no outputs and implicit file writes.
- **GOOD**: declare outputs and stable execution paths.

### NEVER block executor runs with synchronous file I/O in hot paths

- **WHY**: sync I/O hurts parallel throughput.
- **BAD**: `fs.readFileSync` in main execution flow.
- **GOOD**: `await fs.promises.readFile` with structured error handling.

### NEVER register executors without runnable target verification

- **WHY**: unresolved registrations fail only at invocation time.
- **BAD**: update manifest without executing target.
- **GOOD**: verify `nx run <project>:<target> --help` and actual run.

## Gotchas

- Executor resolution issues often come from manifest path drift after refactors.
- Cache confusion after plugin changes is common; run `nx reset` before re-testing.
- Production task behavior can differ from local dry runs; validate both modes.

## Quick Reference

| Topic | Reference |
| --- | --- |
| Schema design and validation patterns | [references/executor-schema-design.md](references/executor-schema-design.md) |
| `ExecutorContext` API usage | [references/context-api-reference.md](references/context-api-reference.md) |
| Core concepts | [knowledge-base/concepts.md](knowledge-base/concepts.md) |
| Workspace specifics | [knowledge-base/workspace-setup.md](knowledge-base/workspace-setup.md) |
| Troubleshooting | [knowledge-base/troubleshooting.md](knowledge-base/troubleshooting.md) |

## References

- [Nx Executors and Configurations](https://nx.dev/concepts/executors-and-configurations)
- [Nx Local Executors Recipe](https://nx.dev/extending-nx/recipes/local-executors)
- [Nx Devkit API](https://nx.dev/reference/devkit)
