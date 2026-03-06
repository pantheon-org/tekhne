---
title: Nx Plugin Scaffolding
category: nx-development
last_updated: January 2026
nx_version: 22
---

# Nx Plugin Scaffolding

## Creating a New Plugin Workspace

**Start from scratch:**
```bash
npx create-nx-plugin my-plugin
cd my-plugin
```

This creates:
- Nx workspace with plugin package pre-configured
- Sample generator and executor
- Testing infrastructure with Jest
- E2E testing setup
- TypeScript configuration

## Adding Plugin to Existing Workspace

**Install plugin package:**
```bash
npx nx add @nx/plugin
```

**Generate plugin:**
```bash
npx nx g @nx/plugin:plugin tools/my-plugin
```

**Generated structure:**
```
tools/my-plugin/
├── src/
│   ├── generators/
│   │   └── my-plugin/
│   │       ├── generator.ts
│   │       ├── schema.json
│   │       └── schema.d.ts
│   ├── executors/
│   └── index.ts
├── package.json
├── project.json
├── tsconfig.json
└── tsconfig.lib.json
```

## Plugin Directory Structure

**Minimal plugin:**
```
my-plugin/
├── src/
│   ├── generators/
│   │   ├── generator-name/
│   │   │   ├── generator.ts
│   │   │   ├── schema.json
│   │   │   ├── schema.d.ts
│   │   │   └── files/              # Optional templates
│   │   │       └── __name__.txt.template
│   │   └── generators.json
│   ├── executors/
│   │   ├── executor-name/
│   │   │   ├── executor.ts
│   │   │   ├── schema.json
│   │   │   └── schema.d.ts
│   │   └── executors.json
│   └── index.ts
├── package.json
└── project.json
```

## Package Configuration

**tools/my-plugin/package.json:**
```json
{
  "name": "@my-org/my-plugin",
  "version": "0.0.1",
  "main": "./src/index.ts",
  "generators": "./generators.json",
  "executors": "./executors.json",
  "dependencies": {
    "@nx/devkit": "^22.0.0",
    "tslib": "^2.3.0"
  }
}
```

## Registry Files

**generators.json:**
```json
{
  "generators": {
    "my-generator": {
      "factory": "./src/generators/my-generator/generator",
      "schema": "./src/generators/my-generator/schema.json",
      "description": "Generate something useful"
    }
  }
}
```

**executors.json:**
```json
{
  "executors": {
    "my-executor": {
      "implementation": "./src/executors/my-executor/executor",
      "schema": "./src/executors/my-executor/schema.json",
      "description": "Execute something useful"
    }
  }
}
```

## Workspace Integration

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
      "@my-org/my-plugin": ["tools/my-plugin/src/index.ts"]
    }
  }
}
```

1. Create entry point `tools/my-plugin/src/index.ts`:
```typescript
export * from './generators/my-generator/generator';
export * from './executors/my-executor/executor';
```

## Testing Setup

**Unit tests:**
```typescript
import { createTreeWithEmptyWorkspace } from '@nx/devkit/testing';
import { Tree, readProjectConfiguration } from '@nx/devkit';
import { myGenerator } from './generator';

describe('my-generator', () => {
  let tree: Tree;

  beforeEach(() => {
    tree = createTreeWithEmptyWorkspace();
  });

  it('should generate files', async () => {
    await myGenerator(tree, { name: 'test' });
    expect(tree.exists('libs/test/README.md')).toBeTruthy();
  });
});
```

**E2E tests:**
```typescript
import { execSync } from 'child_process';
import { rmSync } from 'fs';

describe('my-plugin e2e', () => {
  beforeAll(() => {
    execSync('npx create-nx-workspace test-workspace --preset=empty --no-interactive');
  });

  afterAll(() => {
    rmSync('test-workspace', { recursive: true, force: true });
  });

  it('should generate library', () => {
    execSync('npx nx g @my-org/my-plugin:library test', {
      cwd: 'test-workspace',
      stdio: 'inherit'
    });
  });
});
```

## Publishing

**Local testing:**
```bash
npm link
cd ../test-workspace
npm link @my-org/my-plugin
```

**Registry publishing:**
```bash
npm publish --access public
```

**Nx plugin registry:**
Submit to [nx.dev/plugin-registry](https://nx.dev/plugin-registry) after publishing to npm.

## References

- [Nx Plugin Development](https://nx.dev/extending-nx/intro)
- [Create Nx Plugin](https://nx.dev/nx-api/plugin/generators/create-package)
- [Plugin Registry Submission](https://nx.dev/community/plugin-registry-submission)
