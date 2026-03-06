---
title: Nx Executors Guide
category: nx-development
last_updated: January 2026
nx_version: 22
---

# Nx Executors Guide

## What Are Executors?

Executors define reusable build, test, and development tasks. They encapsulate task logic that can be shared across multiple projects in a workspace with cache-aware execution.

## Executor Implementation

**Basic structure:**
```typescript
import { ExecutorContext } from '@nx/devkit';
import { ExecutorSchema } from './schema';

export default async function runExecutor(
  options: ExecutorSchema,
  context: ExecutorContext
): Promise<{ success: boolean }> {
  console.log('Executing task with options:', options);
  console.log('Project:', context.projectName);
  
  try {
    // Implementation logic here
    return { success: true };
  } catch (error) {
    console.error('Execution failed:', error);
    return { success: false };
  }
}
```

## ExecutorContext API

**Available properties:**
```typescript
interface ExecutorContext {
  root: string;                    // Workspace root
  cwd: string;                     // Current working directory
  projectName: string;             // Current project name
  targetName: string;              // Current target name
  configurationName?: string;      // Active configuration
  workspace: WorkspaceJsonConfiguration;
  projectsConfigurations: ProjectsConfigurations;
  nxJsonConfiguration: NxJsonConfiguration;
  isVerbose: boolean;
}
```

**Common usage:**
```typescript
export default async function runExecutor(
  options: ExecutorSchema,
  context: ExecutorContext
): Promise<{ success: boolean }> {
  const project = context.projectsConfigurations.projects[context.projectName];
  const projectRoot = project.root;
  const workspaceRoot = context.root;
  
  console.log(`Running ${context.targetName} for ${context.projectName}`);
  console.log(`Project root: ${projectRoot}`);
  console.log(`Workspace root: ${workspaceRoot}`);
  
  // Implementation
  return { success: true };
}
```

## Schema Definition

**schema.json:**
```json
{
  "$schema": "http://json-schema.org/schema",
  "cli": "nx",
  "title": "My Executor",
  "description": "Execute something useful",
  "type": "object",
  "properties": {
    "outputPath": {
      "type": "string",
      "description": "Output directory"
    },
    "watch": {
      "type": "boolean",
      "description": "Watch for changes",
      "default": false
    },
    "port": {
      "type": "number",
      "description": "Port number",
      "default": 3000
    }
  },
  "required": ["outputPath"]
}
```

**schema.d.ts:**
```typescript
export interface ExecutorSchema {
  outputPath: string;
  watch?: boolean;
  port?: number;
}
```

## Executor Registration

**executors.json:**
```json
{
  "executors": {
    "build": {
      "implementation": "./src/executors/build/executor",
      "schema": "./src/executors/build/schema.json",
      "description": "Build the project"
    },
    "serve": {
      "implementation": "./src/executors/serve/executor",
      "schema": "./src/executors/serve/schema.json",
      "description": "Serve the project"
    }
  }
}
```

## Target Configuration

**project.json:**
```json
{
  "name": "my-app",
  "targets": {
    "build": {
      "executor": "@my-org/my-plugin:build",
      "outputs": ["{options.outputPath}"],
      "options": {
        "outputPath": "dist/apps/my-app"
      },
      "configurations": {
        "production": {
          "optimization": true,
          "sourceMap": false
        }
      }
    }
  }
}
```

## Cache Configuration

**Declare outputs for caching:**
```json
{
  "targets": {
    "build": {
      "executor": "@my-org/my-plugin:build",
      "outputs": [
        "{options.outputPath}",
        "{workspaceRoot}/dist/{projectRoot}"
      ],
      "options": {
        "outputPath": "dist/apps/my-app"
      }
    }
  }
}
```

**Declare inputs for cache invalidation:**
```json
{
  "targets": {
    "build": {
      "executor": "@my-org/my-plugin:build",
      "inputs": [
        "default",
        "{workspaceRoot}/tsconfig.base.json"
      ],
      "outputs": ["{options.outputPath}"]
    }
  }
}
```

## Async File Operations

**Use async I/O:**
```typescript
import { promises as fs } from 'fs';
import { join } from 'path';

export default async function runExecutor(
  options: ExecutorSchema,
  context: ExecutorContext
): Promise<{ success: boolean }> {
  const project = context.projectsConfigurations.projects[context.projectName];
  const filePath = join(context.root, project.root, 'package.json');
  
  try {
    const content = await fs.readFile(filePath, 'utf-8');
    const pkg = JSON.parse(content);
    
    // Process package.json
    
    await fs.writeFile(
      join(options.outputPath, 'manifest.json'),
      JSON.stringify(pkg, null, 2)
    );
    
    return { success: true };
  } catch (error) {
    console.error('File operation failed:', error);
    return { success: false };
  }
}
```

## Watch Mode Support

**Implement watch mode:**
```typescript
import { watch } from 'fs';

export default async function runExecutor(
  options: ExecutorSchema,
  context: ExecutorContext
): Promise<{ success: boolean }> {
  const project = context.projectsConfigurations.projects[context.projectName];
  const watchPath = join(context.root, project.root, 'src');
  
  if (options.watch) {
    console.log(`Watching ${watchPath} for changes...`);
    
    watch(watchPath, { recursive: true }, async (eventType, filename) => {
      console.log(`File ${filename} changed, rebuilding...`);
      await build(options, context);
    });
    
    return { success: true };
  }
  
  return await build(options, context);
}

async function build(options: ExecutorSchema, context: ExecutorContext) {
  // Build logic
  return { success: true };
}
```

## Workspace Setup

**Enable package references:**

1. Add to root `package.json`:
```json
{
  "workspaces": [
    "packages/*",
    "tools/*"
  ]
}
```

1. Add to `tsconfig.base.json`:
```json
{
  "compilerOptions": {
    "paths": {
      "@my-org/tools": ["tools/src/index.ts"]
    }
  }
}
```

1. Create entry point `tools/src/index.ts`:
```typescript
export * from './executors/build/executor';
export * from './executors/serve/executor';
```

1. Set main field in `tools/package.json`:
```json
{
  "name": "@my-org/tools",
  "main": "./src/index.ts",
  "executors": "./executors.json"
}
```

## Testing

**Unit test example:**
```typescript
import { ExecutorContext } from '@nx/devkit';
import executor from './executor';
import { ExecutorSchema } from './schema';

const options: ExecutorSchema = {
  outputPath: 'dist/test'
};

const context: ExecutorContext = {
  root: '/workspace',
  cwd: '/workspace',
  projectName: 'test-project',
  targetName: 'build',
  workspace: { version: 2, projects: {} },
  projectsConfigurations: {
    version: 2,
    projects: {
      'test-project': {
        root: 'apps/test-project'
      }
    }
  },
  isVerbose: false
};

describe('Build Executor', () => {
  it('should execute successfully', async () => {
    const result = await executor(options, context);
    expect(result.success).toBe(true);
  });
});
```

## Best Practices

1. **Return success boolean** - always return `{ success: boolean }`
2. **Use package notation** - use `@scope/package:executor` not relative paths
3. **Declare outputs** - enable cache for deterministic tasks
4. **Use async I/O** - avoid blocking operations
5. **Delegate logic** - keep executors thin, delegate to library functions
6. **Validate options** - check required fields early
7. **Log clearly** - provide actionable output for debugging

## References

- [Nx Executors Documentation](https://nx.dev/extending-nx/recipes/local-executors)
- [ExecutorContext API](executor-context-api.md)
- [Schema Design](executor-schema-design.md)
