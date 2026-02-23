---
name: nx-generators
description: Create Nx generators for TypeScript monorepos with deterministic Tree API usage, schema-driven options, template file generation, project-graph-safe updates, and testable workflows; use when scaffolding code, enforcing conventions, or building reusable plugin automation in Nx workspaces.
license: MIT
compatibility: opencode
metadata:
  category: nx-development
  audience: nx-developers
---

# Nx Generators

Navigation hub for authoring and operating custom Nx generators.

## When to Use

- You need to scaffold files/projects with repeatable conventions.
- You are adding or modifying custom generators in an Nx plugin.
- You need deterministic project configuration updates through the Tree API.

## When Not to Use

- The task requires runtime execution logic (use executors instead).
- The task is workspace policy only, without code scaffolding.

## Principles

- Keep generators deterministic and idempotent where possible.
- Prefer small, composable generation steps over one large mutation pass.
- You may compose existing generators when it improves consistency.

## Workflow

1. Define generator schema and option constraints.
2. Implement generation logic using Tree API only.
3. Add templates and variable mapping.
4. Update project configuration safely when needed.
5. Validate with dry-run, tests, and generated output checks.

## Constraint vs Flexibility Guidelines

### Hard Constraints (MUST)

- MUST use Tree API (`tree.write`, `generateFiles`, `updateJson`) for filesystem edits.
- MUST validate generator options via schema and explicit guards.
- MUST respect Nx project graph boundaries and existing tags.
- MUST test with `--dry-run` before broad rollouts.

### Flexible Choices (CAN)

- CAN choose template style (EJS-heavy vs minimal placeholders).
- CAN choose option naming conventions if documented in schema.
- CAN compose other generators when it reduces duplication.

### Fallback Behaviors

| Missing Input | Fallback |
| --- | --- |
| `directory` omitted | Use workspace default location |
| optional flags omitted | Apply safe schema defaults |
| custom template not provided | Generate minimal boilerplate |

## Quick Commands

```bash
nx g @nx/plugin:generator my-generator --project=my-plugin
```

```bash
nx g my-plugin:my-generator sample-name --dry-run
```

```bash
nx g my-plugin:my-generator sample-name
```

```bash
nx test my-plugin
```

```bash
bun run nx g my-plugin:my-generator sample-name --dry-run
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
- **GOOD**: derive paths from schema/context helpers.

### NEVER skip schema validation and typed options

- **WHY**: invalid inputs fail late with unclear errors.
- **BAD**: `schema: any` and no guardrails.
- **GOOD**: typed schema + required fields + runtime guards.

### NEVER generate across project boundaries without explicit checks

- **WHY**: hidden boundary violations can introduce circular dependencies.
- **BAD**: generator in one scope writing imports into disallowed scopes.
- **GOOD**: verify tags and dependency direction first.

### NEVER mutate project configuration blindly

- **WHY**: naive updates remove existing targets/tags.
- **BAD**: overwrite full `project.json`.
- **GOOD**: read, merge, and update only intended keys.

## Quick Reference

| Topic | Reference |
| --- | --- |
| Tree API method patterns | [references/tree-api-reference.md](references/tree-api-reference.md) |
| Schema design patterns | [references/schema-design-patterns.md](references/schema-design-patterns.md) |
| Template engine guidance | [references/template-engine-guide.md](references/template-engine-guide.md) |
| Core concepts | [knowledge-base/concepts.md](knowledge-base/concepts.md) |
| Template system details | [knowledge-base/template-system.md](knowledge-base/template-system.md) |
| Utility helpers | [knowledge-base/utilities.md](knowledge-base/utilities.md) |

## References

- [Nx Plugin Development](https://nx.dev/extending-nx/intro)
- [Nx Devkit API](https://nx.dev/reference/devkit)
