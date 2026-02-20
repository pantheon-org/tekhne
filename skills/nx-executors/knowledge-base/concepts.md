---
title: Nx Executors - Core Concepts
category: nx-development
last_updated: January 2026
nx_version: 22
---

# Nx Executors - Core Concepts

## What Are Executors?

Executors are Nx's mechanism for defining reusable build, test, and development tasks. They encapsulate task logic that can be shared across multiple projects in a workspace.

## Executor Anatomy

### File Structure

```
tools/executors/
├── my-executor/
│   ├── executor.ts      # Implementation
│   ├── schema.json      # JSON Schema for options
│   └── schema.d.ts      # TypeScript type definitions
├── executors.json       # Registry of all executors
└── package.json         # Package definition
```

### Schema Definition

The schema defines options that users can pass to the executor:

```json
{
  "$schema": "http://json-schema.org/schema",
  "cli": "nx",
  "title": "My Executor",
  "description": "What this executor does",
  "type": "object",
  "properties": {
    "option1": {
      "type": "string",
      "description": "Description of option1"
    },
    "option2": {
      "type": "number",
      "description": "Description of option2",
      "default": 42
    }
  },
  "required": ["option1"]
}
```

### TypeScript Schema

```typescript
export interface MyExecutorSchema {
  option1: string;
  option2?: number;
}
```

### Executor Implementation

```typescript
import { ExecutorContext } from '@nx/devkit';
import { MyExecutorSchema } from './schema';

export default async function myExecutor(
  options: MyExecutorSchema,
  context: ExecutorContext
): Promise<{ success: boolean }> {
  console.log('Executing my-executor');
  console.log('Project:', context.projectName);

  try {
    // Executor logic
    return { success: true };
  } catch (error) {
    console.error('Executor failed:', error);
    return { success: false };
  }
}
```

## ExecutorContext API

The `ExecutorContext` provides access to project and workspace information:

```typescript
interface ExecutorContext {
  // Project being executed
  projectName: string;
  
  // Target being executed
  targetName: string;
  
  // Configuration name (e.g., "production", "development")
  configurationName?: string;
  
  // Workspace root directory
  root: string;
  
  // Current working directory
  cwd: string;
  
  // Whether this is a dry run
  isVerbose: boolean;
  
  // Project graph with all projects and dependencies
  projectGraph: ProjectGraph;
  
  // Workspace configuration
  workspace: WorkspaceConfiguration;
}
```

### Common Context Usage Patterns

```typescript
export default async function myExecutor(
  options: MyExecutorSchema,
  context: ExecutorContext
) {
  // Get current project name
  const projectName = context.projectName;
  
  // Get project root directory
  const projectRoot = context.projectGraph.nodes[projectName].data.root;
  
  // Get project configuration
  const projectConfig = context.projectGraph.nodes[projectName].data;
  
  // Get all project dependencies
  const dependencies = context.projectGraph.dependencies[projectName];
  
  // Get workspace root
  const workspaceRoot = context.root;
  
  // Check if verbose mode
  if (context.isVerbose) {
    console.log('Verbose output enabled');
  }
}
```

## Executor Registry

The `executors.json` file registers all available executors:

```json
{
  "executors": {
    "my-executor": {
      "implementation": "./my-executor/executor",
      "schema": "./my-executor/schema.json",
      "description": "What this executor does"
    },
    "another-executor": {
      "implementation": "./another-executor/executor",
      "schema": "./another-executor/schema.json",
      "description": "Another executor"
    }
  }
}
```

## Package Configuration

The `package.json` defines the executor package:

```json
{
  "name": "@scope/tools",
  "version": "0.1.0",
  "main": "../src/index.ts",
  "executors": "./executors.json"
}
```

**Key fields:**
- `name`: Package name used in references
- `main`: Entry point (required for package resolution)
- `executors`: Path to executor registry

## Executor Resolution

Nx resolves executors in this order:

1. **Import the package** → Uses workspace inclusion + path mapping
2. **Read executors.json** → Defined in package.json via `"executors"` field
3. **Load executor implementation** → Reads path from executors.json
4. **Execute with schema validation** → Validates options against schema.json

## Package vs. Path References

### Package Reference (Recommended)

```json
{
  "executor": "@scope/tools:my-executor"
}
```

**Benefits:**
- Clean, semantic reference
- Refactoring-safe (directory structure can change)
- Consistent with external packages
- Self-documenting

### Relative Path Reference (Avoid)

```json
{
  "executor": "../../tools/executors:my-executor"
}
```

**Problems:**
- Breaks if directory structure changes
- Unclear ownership
- Inconsistent with external packages
- Harder to maintain

## Return Value Contract

Executors must return a Promise with a success status:

```typescript
type ExecutorResult = Promise<{ success: boolean }>;
```

This allows Nx to:
- Track task success/failure
- Stop execution on failure (with `--stop-on-error`)
- Report task results in the UI
- Cache successful executions

## Best Practices

### 1. Single Responsibility
Each executor should do one thing well.

### 2. Clear Schema Documentation
Provide descriptions for all options:

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

### 3. Graceful Error Handling
Always catch errors and return proper status:

```typescript
try {
  // Logic
  return { success: true };
} catch (error) {
  console.error('Error:', error.message);
  return { success: false };
}
```

### 4. Leverage ExecutorContext
Use context for project information instead of hardcoding paths.

### 5. Support Standard Options
Consider supporting:
- `--verbose` for detailed output
- `--dry-run` for preview mode
- Environment-specific behavior via configuration

## References

- [Nx Executors Documentation](https://nx.dev/concepts/executors-and-configurations)
- [Creating Custom Executors](https://nx.dev/extending-nx/recipes/local-executors)
- [Nx Devkit API](https://nx.dev/reference/devkit)
