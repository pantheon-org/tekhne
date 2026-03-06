---
title: Testing and Troubleshooting Nx Plugins
category: nx-development
last_updated: January 2026
---

# Testing and Troubleshooting Nx Plugins

## Testing Generators

### Unit Tests

**Basic generator test:**
```typescript
import { createTreeWithEmptyWorkspace } from '@nx/devkit/testing';
import { Tree, readProjectConfiguration } from '@nx/devkit';
import { libraryGenerator } from './generator';

describe('library generator', () => {
  let tree: Tree;

  beforeEach(() => {
    tree = createTreeWithEmptyWorkspace();
  });

  it('should create library', async () => {
    await libraryGenerator(tree, { name: 'test' });
    
    const config = readProjectConfiguration(tree, 'test');
    expect(config.root).toBe('libs/test');
    expect(tree.exists('libs/test/src/index.ts')).toBeTruthy();
  });

  it('should handle directory option', async () => {
    await libraryGenerator(tree, { 
      name: 'test',
      directory: 'shared'
    });
    
    const config = readProjectConfiguration(tree, 'shared-test');
    expect(config.root).toBe('libs/shared/test');
  });
});
```

### Dry-Run Validation

**Test before applying:**
```bash
npx nx g @my-org/my-plugin:library test --dry-run
```

**Validate output structure:**
```bash
npx nx g @my-org/my-plugin:library test --dry-run | grep -E "CREATE|UPDATE"
```

## Testing Executors

### Unit Tests

**Basic executor test:**
```typescript
import { ExecutorContext } from '@nx/devkit';
import executor from './executor';
import { ExecutorSchema } from './schema';

const mockContext: ExecutorContext = {
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
    const options: ExecutorSchema = {
      outputPath: 'dist/test'
    };
    
    const result = await executor(options, mockContext);
    expect(result.success).toBe(true);
  });

  it('should fail with invalid options', async () => {
    const options: ExecutorSchema = {
      outputPath: ''
    };
    
    const result = await executor(options, mockContext);
    expect(result.success).toBe(false);
  });
});
```

### Integration Tests

**Test executor in workspace:**
```bash
npx nx run my-project:my-target
```

**Skip cache for testing:**
```bash
npx nx run my-project:my-target --skip-nx-cache
```

**Verify outputs:**
```bash
npx nx run my-project:build
ls -la dist/apps/my-project
```

## Common Issues

### "Unable to resolve executor package reference"

**Error:**
```
NX Cannot find executor '@my-org/tools:build'
Unable to resolve @my-org/tools:build
```

**Causes:**
1. Package not in workspaces
2. Missing tsconfig path mapping
3. No entry point file
4. Missing main field

**Solution:**

1. Check workspace inclusion in root `package.json`:
```json
{
  "workspaces": [
    "packages/*",
    "tools/*"
  ]
}
```

1. Verify path mapping in `tsconfig.base.json`:
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
```

1. Set main field in `tools/package.json`:
```json
{
  "main": "./src/index.ts"
}
```

1. Reset Nx daemon:
```bash
npx nx reset
npx nx run my-project:build
```

### "Generator not found"

**Error:**
```
Cannot find generator '@my-org/my-plugin:library'
```

**Solution:**

1. Check `generators.json` registration:
```json
{
  "generators": {
    "library": {
      "factory": "./src/generators/library/generator",
      "schema": "./src/generators/library/schema.json"
    }
  }
}
```

1. Verify `package.json` points to registry:
```json
{
  "generators": "./generators.json"
}
```

1. Check generator export:
```typescript
export default async function libraryGenerator(tree: Tree, options: Schema) {
  // ...
}
```

### Schema validation errors

**Error:**
```
Required property 'name' is missing
```

**Solution:**

1. Check schema.json required fields:
```json
{
  "required": ["name"]
}
```

1. Provide default values:
```json
{
  "properties": {
    "name": {
      "type": "string",
      "$default": {
        "$source": "argv",
        "index": 0
      }
    }
  }
}
```

1. Add prompts for interactive mode:
```json
{
  "properties": {
    "name": {
      "type": "string",
      "x-prompt": "What name would you like to use?"
    }
  }
}
```

### Cache not working

**Symptoms:**
- Executor runs every time
- No cache hits reported

**Solution:**

1. Declare outputs in target config:
```json
{
  "targets": {
    "build": {
      "outputs": ["{options.outputPath}"]
    }
  }
}
```

1. Ensure outputs are deterministic
2. Check inputs configuration
3. Verify executor returns `{ success: true }`

### Tree API violations

**Error:**
```
Cannot write file outside of workspace
```

**Solution:**
- Use Tree API exclusively: `tree.write()` not `fs.writeFileSync()`
- Keep all paths relative to workspace root
- Never bypass Tree with direct filesystem writes

## Debugging Tips

### Enable verbose output

```bash
npx nx run my-project:build --verbose
```

### Reset Nx cache

```bash
npx nx reset
```

### Check project configuration

```bash
npx nx show project my-project
```

### Validate workspace

```bash
npx nx workspace-lint
```

### Inspect task graph

```bash
npx nx graph
```

### Search for implementation patterns

```bash
rg -n "generateFiles|tree.write|ExecutorContext" tools plugins
```

## E2E Testing

**Create test workspace:**
```typescript
import { execSync } from 'child_process';
import { rmSync } from 'fs';

describe('plugin e2e', () => {
  const testWorkspace = 'test-workspace';

  beforeAll(() => {
    execSync(`npx create-nx-workspace ${testWorkspace} --preset=empty --no-interactive`);
    execSync(`npm install @my-org/my-plugin`, { cwd: testWorkspace });
  });

  afterAll(() => {
    rmSync(testWorkspace, { recursive: true, force: true });
  });

  it('should generate library', () => {
    execSync(`npx nx g @my-org/my-plugin:library test`, {
      cwd: testWorkspace,
      stdio: 'inherit'
    });
    
    // Verify generated files
  });
});
```

## Performance Testing

**Measure generator performance:**
```bash
time npx nx g @my-org/my-plugin:library test --dry-run
```

**Measure executor performance:**
```bash
time npx nx run my-project:build --skip-nx-cache
```

## References

- [Nx Testing Utilities](https://nx.dev/packages/nx/documents/nx-devkit#testing-utilities)
- [Debugging Nx](https://nx.dev/recipes/troubleshooting/debug-nx)
