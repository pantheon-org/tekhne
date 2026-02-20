---
name: nx-biome-integration
description: |-
  Integrate Biome (Rust-based linter/formatter) into NX monorepos with custom executors, inferred tasks, and caching. Use proactively when setting up Biome in NX workspaces, creating custom tool executors, or migrating from ESLint/Prettier to Biome.
  
  Examples:
  - user: "Add Biome to this NX workspace" → install, configure, create executor with caching
  - user: "Create NX executor for Biome" → scaffold plugin with inferred tasks
  - user: "Migrate from ESLint to Biome gradually" → use nested biome.json for selective adoption
  - user: "Make Biome work with NX caching" → configure inputs and targetDefaults
  - user: "Add linting to my NX monorepo" → consider Biome as fast alternative to ESLint
---

# NX Biome Integration

Integrate Biome (fast Rust-based linter and formatter) into NX monorepos with custom executors, inferred tasks, and caching support.

## Why Biome with NX

- **Performance**: Biome is written in Rust, significantly faster than ESLint/Prettier
- **NX Caching**: Properly configured Biome tasks cache results across the workspace
- **Selective Adoption**: Use nested configs for gradual migration from ESLint
- **Scale**: Custom plugins create targets automatically for hundreds of projects

## Quick Start (No Plugin)

### 1. Install Biome

```bash
npm install --save-dev @biomejs/biome
```

### 2. Initialize Configuration

```bash
npx @biomejs/biome init
```

This creates `biome.json` at workspace root.

### 3. Add npm Scripts

For **npm workspaces** projects, add to each project's `package.json`:

```json
{
  "scripts": {
    "biome-lint": "biome lint",
    "biome-format": "biome format --write"
  }
}
```

For **project.json** projects:

```json
{
  "targets": {
    "biome-lint": {
      "command": "npx @biomejs/biome lint {projectRoot}"
    },
    "biome-format": {
      "command": "npx @biomejs/biome format --write {projectRoot}"
    }
  }
}
```

**Note**: `{projectRoot}` is a special NX token replaced with the project's directory path.

### 4. Add Caching in nx.json

```json
{
  "targetDefaults": {
    "biome-lint": {
      "cache": true,
      "inputs": [
        "default",
        "^default",
        "{workspaceRoot}/biome.json",
        {
          "externalDependencies": ["@biomejs/biome"]
        }
      ]
    },
    "biome-format": {
      "cache": true,
      "inputs": [
        "default",
        "^default",
        "{workspaceRoot}/biome.json",
        {
          "externalDependencies": ["@biomejs/biome"]
        }
      ]
    }
  }
}
```

**Tip**: Copy inputs from existing ESLint configuration and modify for Biome.

### 5. Run Tasks

```bash
# Lint specific project
nx biome-lint my-project

# Lint all projects
nx run-many -t biome-lint

# Format all projects
nx run-many -t biome-format
```

## Advanced: Custom NX Plugin (Recommended for Scale)

For workspaces with many projects, create a custom NX plugin with inferred tasks. This automatically adds Biome targets to projects without manual configuration.

### 1. Install Plugin Tooling

```bash
npx nx add @nx/plugin
```

### 2. Generate Plugin

```bash
npx nx g @nx/plugin:plugin --name=biome-plugin --directory=tools/biome-plugin
```

### 3. Implement Plugin (tools/biome-plugin/src/index.ts)

```typescript
import {
  CreateNodesContextV2,
  createNodesFromFiles,
  CreateNodesV2,
} from '@nx/devkit';
import { dirname } from 'path';

export interface BiomePluginOptions {
  // Add configuration options if needed
}

export const createNodesV2: CreateNodesV2<BiomePluginOptions> = [
  // For npm workspaces: look for package.json
  // For project.json: use '**/project.json'
  '**/package.json',
  async (configFiles, options, context) => {
    return await createNodesFromFiles(
      (configFile, options, context) =>
        createNodesInternal(configFile, options, context),
      configFiles,
      options,
      context
    );
  },
];

async function createNodesInternal(
  configFilePath: string,
  options: BiomePluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);

  return {
    projects: {
      [root]: {
        targets: {
          'biome-lint': {
            command: 'npx @biomejs/biome lint {projectRoot}',
            cache: true,
            inputs: [
              'default',
              '^default',
              '{workspaceRoot}/biome.json',
              {
                externalDependencies: ['@biomejs/biome'],
              },
            ],
          },
          'biome-format': {
            command: 'npx @biomejs/biome format --write {projectRoot}',
            cache: true,
            inputs: [
              'default',
              '^default',
              '{workspaceRoot}/biome.json',
              {
                externalDependencies: ['@biomejs/biome'],
              },
            ],
          },
        },
      },
    },
  };
}
```

### 4. Register Plugin in nx.json

```json
{
  "plugins": ["tools/biome-plugin"]
}
```

### 5. Verify Plugin

```bash
# Check that targets are created
nx show project my-project --web
```

The `biome-lint` and `biome-format` targets should appear with "provided by tools/biome-plugin".

## Selective Adoption with Nested Configs

Use nested `biome.json` files for gradual migration from ESLint to Biome.

### 1. Modify Plugin to Look for biome.json

Update the file pattern in your plugin:

```typescript
export const createNodesV2: CreateNodesV2<BiomePluginOptions> = [
  '**/biome.json',  // Only create targets where biome.json exists
  async (configFiles, options, context) => {
    // ... rest of implementation
  },
];

async function createNodesInternal(
  configFilePath: string,
  options: BiomePluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);

  // Skip root biome.json (don't create a root project)
  if (root === '.') {
    return {};
  }

  return {
    projects: {
      [root]: {
        targets: {
          // ... targets
        },
      },
    },
  };
}
```

### 2. Create Nested biome.json Files

For each project you want to migrate:

```bash
# apps/my-app/biome.json
{
  "$schema": "https://biomejs.dev/schemas/1.9.4/schema.json",
  "root": false,
  "extends": ["../../biome.json"],
  "linter": {
    "rules": {
      // Override rules specific to this project
    }
  }
}
```

### 3. Progressive Migration

- Projects with `biome.json` → Use Biome (via plugin)
- Projects without `biome.json` → Continue using ESLint
- Gradually add `biome.json` files as you migrate

## Key Concepts

### Command Interpolation

- `{projectRoot}`: Replaced with project directory path
- `{workspaceRoot}`: Replaced with workspace root path
- Commands run from workspace root, so use `{projectRoot}` to target specific projects

### Caching Inputs

Inputs tell NX when to invalidate cache:

- `"default"`: Source files in the project
- `"^default"`: Source files in dependencies
- `"{workspaceRoot}/biome.json"`: Root config file
- `"externalDependencies"`: Package versions that affect output

### Plugin Development

During development, bypass NX daemon cache:

```bash
NX_DAEMON=false nx run-many -t biome-lint
```

In production, reset cache after plugin changes:

```bash
nx reset
```

## Common Patterns

### Add Format Script to package.json

```json
{
  "scripts": {
    "lint": "nx run-many -t biome-lint",
    "format": "nx run-many -t biome-format"
  }
}
```

### Integrate with Git Hooks (Lefthook)

```yaml
pre-commit:
  parallel: true
  commands:
    biome-format:
      glob: "*.{ts,tsx,js,jsx,json}"
      run: nx affected -t biome-format
      stage_fixed: true
```

### Add Generators

Extend your plugin with generators to create `biome.json` files:

```bash
npx nx g @nx/plugin:generator --name=init-biome --directory=tools/biome-plugin/src/generators
```

## Troubleshooting

### Targets Not Appearing

1. Check plugin is registered in `nx.json` plugins array
2. Verify file pattern matches your config files
3. Run `nx reset` to clear cache
4. Use `NX_DAEMON=false` during development

### Caching Not Working

1. Verify `cache: true` in target definition
2. Check inputs include all relevant files
3. Ensure external dependencies are listed correctly
4. Run with `--skip-nx-cache` to test without cache

### Plugin Performance

- Use `createNodesFromFiles` for parallel processing
- Keep plugin logic minimal (no heavy I/O)
- Cache plugin results if computing expensive values

## Benefits of Custom Plugins

- **No manual configuration**: Targets created automatically
- **Consistency**: All projects follow same pattern
- **Maintainability**: Update once, applies everywhere
- **Simplicity**: Your plugin only needs to handle your use case (not every edge case)

## References

- [Biome Official Docs](https://biomejs.dev/)
- [NX Plugin Documentation](https://nx.dev/extending-nx)
- [NX Inferred Tasks](https://nx.dev/concepts/inferred-tasks)
- [Biome Big Projects Guide](https://biomejs.dev/guides/big-projects/)
