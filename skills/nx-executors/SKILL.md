---
name: nx-executors
description: Create and use custom Nx executors in TypeScript monorepos. Covers executor structure, package references (@scope:executor), schema definitions, ExecutorContext API, and best practices for reusable build tasks.
license: MIT
compatibility: opencode
metadata:
  category: nx-development
  audience: nx-developers
---

# Custom Nx Executors

Create reusable build, test, and development tasks for Nx workspaces.

## When to Use This Skill

- User requests creating a new Nx executor
- User needs to use or modify existing executors in the workspace
- User encounters executor resolution errors
- User asks about executor configuration or usage

## Knowledge Base References

This skill references domain knowledge from:
- [Core Concepts](./knowledge-base/concepts.md) - Nx executor fundamentals and API
- [Workspace Setup](./knowledge-base/workspace-setup.md) - Project-specific configuration
- [Troubleshooting](./knowledge-base/troubleshooting.md) - Common issues and solutions

## Quick Reference

### This Workspace Uses Package References

```json
{
  "executor": "@pantheon-org/tools:executor-name"
}
```

**Never use relative paths:**
```json
{
  "executor": "../../tools/executors:executor-name"  // ❌ WRONG
}
```

### Existing Executors

Available in `@pantheon-org/tools`:
- `dev-proxy` - Development server with hot reload
- `check-mirror-exists` - Validate GitHub mirror repository
- `typecheck` - TypeScript type checking

See [Workspace Setup](./knowledge-base/workspace-setup.md) for details.

## Agent Workflow

### Creating a New Executor

**Steps:**

1. **Create structure:** `mkdir tools/executors/<executor-name>`
2. **Create files:** `executor.ts`, `schema.json`, `schema.d.ts`
3. **Define schema** - See [Core Concepts](./knowledge-base/concepts.md#schema-definition)
4. **Implement executor** - Use arrow function pattern (see `AGENTS.md`)
5. **Register** in `tools/executors/executors.json`
6. **Test:** `bunx nx run <project>:<target> --help`

**Implementation template:**

```typescript
import type { ExecutorContext } from '@nx/devkit';
import type { ExecutorNameSchema } from './schema';

export default async (
  options: ExecutorNameSchema,
  context: ExecutorContext
): Promise<{ success: boolean }> => {
  try {
    // Executor logic
    return { success: true };
  } catch (error) {
    console.error('Executor failed:', error);
    return { success: false };
  }
};
```

**Key requirements:**
- Return `{ success: boolean }`
- Use arrow function (not named function)
- Handle errors gracefully
- Use `ExecutorContext` for project info

### Using Executors in Projects

**Add to `project.json`:**

```json
{
  "targets": {
    "my-task": {
      "executor": "@pantheon-org/tools:executor-name",
      "options": { "option1": "value" }
    }
  }
}
```

**Test:** `bunx nx run <project>:my-task --help`

### Modifying Existing Executors

1. Locate: `tools/executors/<executor-name>/`
2. Edit using arrow function pattern
3. Test: `bunx nx run <project>:<target> --skip-nx-cache`
4. Clear cache if needed: `bunx nx reset`

### Debugging Executors

**Resolution errors:** See [Troubleshooting Guide](./knowledge-base/troubleshooting.md#unable-to-resolve-scopetoolsexecutor-name)

**Quick diagnostics:**
```bash
bun pm ls | grep @pantheon-org/tools
bunx nx run <project>:<target> --verbose
bunx nx show project <project> --web
```

### Best Practices for Agents

**1. Reference Knowledge Base**

When users ask about:
- Executor concepts → [Core Concepts](./knowledge-base/concepts.md)
- Configuration → [Workspace Setup](./knowledge-base/workspace-setup.md)
- Errors → [Troubleshooting](./knowledge-base/troubleshooting.md)

**2. Always Use Package References**

Never generate relative path references. Always use:
```json
"executor": "@pantheon-org/tools:executor-name"
```

**3. Follow Arrow Function Pattern**

This workspace uses arrow functions exclusively:

```typescript
// ✅ Correct
export default async (options, context) => {
  // implementation
};

// ❌ Wrong
export default async function executor(options, context) {
  // implementation
}
```

See `AGENTS.md` in workspace root.

**4. Test Before Declaring Success**

Always verify:
```bash
bunx nx run <project>:<target> --help  # Resolution works
bunx nx run <project>:<target>         # Execution works
```

**5. Handle Errors Gracefully**

Every executor should:
- Return `{ success: boolean }`
- Catch and log errors
- Provide helpful error messages

**6. Document Options Clearly**

Schema descriptions help users:

```json
{
  "properties": {
    "port": {
      "type": "number",
      "description": "Port number for the development server",
      "default": 3000
    }
  }
}
```

## Common Patterns

See [Core Concepts](./knowledge-base/concepts.md#common-context-usage-patterns) for:
- Accessing project information via `ExecutorContext`
- Supporting dry run and verbose modes
- Error handling patterns

## Resources

- [Nx Executors Documentation](https://nx.dev/concepts/executors-and-configurations)
- [Creating Custom Executors](https://nx.dev/extending-nx/recipes/local-executors)
- [Nx Devkit API](https://nx.dev/reference/devkit)
