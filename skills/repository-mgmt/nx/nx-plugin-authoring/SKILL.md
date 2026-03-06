---
name: nx-plugin-authoring
description: Create Nx plugins with custom generators and executors for TypeScript monorepos. Covers plugin scaffolding, Tree API usage, schema-driven options, ExecutorContext API, template generation, project-graph-safe updates, cache-aware outputs, and testable workflows. Use when creating Nx plugins, building custom generators, implementing executors, enforcing conventions, or extending Nx workspace automation.
license: MIT
compatibility: opencode
metadata:
  category: nx-development
  audience: nx-developers
  version: 1.0.0
---

# Nx Plugin Authoring

Navigation hub for creating custom Nx plugins, generators, and executors.

## When to Use

- You need to scaffold files/projects with repeatable conventions (generators).
- You need custom task execution logic (executors).
- You are building reusable plugin automation for Nx workspaces.
- You want to enforce organization-specific standards and patterns.

## When Not to Use

- The task is generic Nx workspace setup unrelated to custom plugin development.
- You only need to use existing plugins, not create new ones.

## Principles

- Keep plugins as orchestration tools, not business-logic containers.
- Prefer small, composable generation steps over large mutation passes.
- Keep executors deterministic and cache-aware.
- Use Tree API exclusively for filesystem operations in generators.

## Workflow

### 1. Create Plugin Structure

**Start new workspace with plugin:**
```bash
npx create-nx-plugin my-plugin
```

**Add plugin to existing workspace:**
```bash
npx nx add @nx/plugin
npx nx g plugin tools/my-plugin
```

### 2. Build Generators

**Create a generator:**
```bash
npx nx g generator my-plugin/src/generators/library-with-readme
```

**Implement with Tree API:**
- Use `tree.write`, `generateFiles`, `updateJson` for filesystem edits
- Define typed schema with required fields
- Add templates with EJS syntax
- Validate with `--dry-run` before execution

**Validate**: Run `npx nx g my-plugin:library-with-readme mylib --dry-run`

**Stop if**: Unexpected files appear or required options fail validation

### 3. Build Executors

**Create executor structure:**
```bash
mkdir -p tools/executors/my-executor
```

**Implement with ExecutorContext:**
- Define schema.json with required fields and defaults
- Return `{ success: boolean }` from executor
- Use package notation for registration
- Declare outputs for cache awareness

**Validate**: Run `npx nx run my-project:my-target --help`

**Stop if**: Executor fails to resolve or schema is invalid

### 4. Test and Publish

**Run tests:**
```bash
npx nx test my-plugin
```

**Validate in real workspace:**
```bash
npx nx g my-plugin:my-generator sample --dry-run
npx nx run my-project:my-target
```

## Quick Commands

```bash
npx create-nx-plugin my-plugin
```

```bash
npx nx add @nx/plugin
```

```bash
npx nx g generator my-plugin/src/generators/my-gen
```

```bash
npx nx g my-plugin:my-gen sample --dry-run
```

```bash
npx nx run my-project:my-target --skip-nx-cache
```

```bash
npx nx reset
```

```bash
rg -n "generateFiles|updateJson|readProjectConfiguration" plugins tools
```

## Anti-Patterns

### NEVER modify files outside the Tree API

- **WHY**: direct filesystem writes bypass dry-run and change tracking.
- **BAD**: `writeFileSync("libs/my-lib/src/index.ts", content)`.
- **GOOD**: `tree.write("libs/my-lib/src/index.ts", content)`.

### NEVER hardcode project paths in generator logic

- **WHY**: brittle paths fail when workspace layout evolves.
- **BAD**: fixed `libs/my-lib/...` writes.
- **GOOD**: derive paths from `readProjectConfiguration(tree, name).root`.

### NEVER skip schema validation and typed options

- **WHY**: invalid inputs fail late with unclear errors.
- **BAD**: `schema: any` and no guardrails.
- **GOOD**: typed schema.json with required fields, types, defaults, descriptions.

### NEVER generate across project boundaries without explicit checks

- **WHY**: hidden boundary violations introduce circular dependencies.
- **BAD**: generator writes imports into disallowed scopes without verification.
- **GOOD**: verify tags and dependency direction before cross-scope imports.

### NEVER mutate project configuration blindly

- **WHY**: naive updates remove existing targets/tags.
- **BAD**: overwrite full `project.json`.
- **GOOD**: `const config = readProjectConfiguration(tree, name); config.targets.build = {...}; updateProjectConfiguration(tree, name, config);`.

### NEVER put business logic directly inside executor entrypoints

- **WHY**: bloated executors are hard to test and reuse.
- **BAD**: 200 lines of transformation logic in `executor.ts`.
- **GOOD**: delegate to composable library functions.

### NEVER use relative executor references in targets

- **WHY**: relative paths are fragile across workspace changes.
- **BAD**: `"executor": "../../tools/executors:task"`.
- **GOOD**: `"executor": "@scope/tools:task"`.

### NEVER omit outputs and dependencies semantics for cacheable work

- **WHY**: Nx cache quality depends on deterministic inputs and outputs.
- **BAD**: no outputs declared, implicit file writes.
- **GOOD**: declare outputs in target config: `"outputs": ["{options.outputPath}"]`.

### NEVER block executor runs with synchronous file I/O in hot paths

- **WHY**: sync I/O hurts parallel throughput.
- **BAD**: `fs.readFileSync` in main execution flow.
- **GOOD**: `await fs.promises.readFile` with structured error handling.

### NEVER skip formatFiles after generating code

- **WHY**: inconsistent formatting causes noisy diffs and CI failures.
- **BAD**: `generateFiles()` without calling `formatFiles(tree)`.
- **GOOD**: `await generateFiles(...); await formatFiles(tree);`.

### NEVER ignore existing project structure when generating files

- **WHY**: overwriting existing files destroys user work.
- **BAD**: `tree.write(path, content)` without checking `tree.exists(path)`.
- **GOOD**: `if (tree.exists(path)) { throw new Error('File already exists'); }`.

### NEVER forget to export generator/executor from plugin index

- **WHY**: unregistered functions cannot be invoked by Nx CLI.
- **BAD**: generator implemented but missing from `src/index.ts` and `generators.json`.
- **GOOD**: export in `src/index.ts` and register in `generators.json` with factory path.

## Constraint vs Flexibility Guidelines

### Hard Constraints (MUST)

**Generators:**
- MUST use Tree API for filesystem edits
- MUST validate generator options via schema and explicit guards
- MUST respect Nx project graph boundaries and existing tags
- MUST test with `--dry-run` before broad rollouts

**Executors:**
- MUST return `{ success: boolean }` from executor function
- MUST use package notation for executor references in targets
- MUST declare outputs for cacheable tasks
- MUST implement schema validation with required fields

### Flexible Choices (CAN)

- CAN choose template style (EJS-heavy vs minimal placeholders)
- CAN choose option naming conventions if documented in schema
- CAN compose other generators to reduce duplication
- CAN delegate executor work to external libraries

### Fallback Behaviors

| Missing Input | Fallback |
| --- | --- |
| `directory` omitted in generator | Use workspace default location |
| optional flags omitted | Apply safe schema defaults |
| custom template not provided | Generate minimal boilerplate |
| executor outputs not declared | No cache, always re-run |

## Quick Reference

### Getting Started

| Topic | Reference |
| --- | --- |
| Plugin scaffolding and structure | [references/plugin-scaffolding.md](references/plugin-scaffolding.md) |
| Core concepts | [knowledge-base/concepts.md](knowledge-base/concepts.md) |

### Generators

| Topic | Reference |
| --- | --- |
| Generator implementation guide | [references/generators-guide.md](references/generators-guide.md) |
| Tree API patterns | [references/tree-api-reference.md](references/tree-api-reference.md) |
| Template engine guidance | [references/template-engine-guide.md](references/template-engine-guide.md) |
| Template system details | [knowledge-base/template-system.md](knowledge-base/template-system.md) |

### Executors

| Topic | Reference |
| --- | --- |
| Executor implementation guide | [references/executors-guide.md](references/executors-guide.md) |
| ExecutorContext API usage | [references/executor-context-api.md](references/executor-context-api.md) |
| Executor schema design | [references/executor-schema-design.md](references/executor-schema-design.md) |

### Shared Resources

| Topic | Reference |
| --- | --- |
| Schema design patterns | [references/schema-design-patterns.md](references/schema-design-patterns.md) |
| Utility helpers | [knowledge-base/utilities.md](knowledge-base/utilities.md) |
| Testing and troubleshooting | [references/testing-and-troubleshooting.md](references/testing-and-troubleshooting.md) |

## References

- [Nx Plugin Development](https://nx.dev/extending-nx/intro)
- [Nx Devkit API](https://nx.dev/reference/devkit)
- [Enforce Best Practices Tutorial](https://nx.dev/extending-nx/organization-specific-plugin)
- [Tool Integration Tutorial](https://nx.dev/extending-nx/tooling-plugin)
- [Local Executors Recipe](https://nx.dev/extending-nx/recipes/local-executors)
- [Plugin Registry](https://nx.dev/plugin-registry)
