# NX Plugin Creation Patterns

Advanced patterns for creating custom NX plugins with inferred tasks.

## Plugin Anatomy

### Basic Structure

```typescript
import {
  CreateNodesContextV2,
  createNodesFromFiles,
  CreateNodesV2,
} from '@nx/devkit';
import { dirname } from 'path';

export interface MyPluginOptions {
  // Configuration options
}

export const createNodesV2: CreateNodesV2<MyPluginOptions> = [
  // File pattern to match
  '**/config-file-pattern',
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
  options: MyPluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);
  
  // Return project configuration
  return {
    projects: {
      [root]: {
        targets: {
          // Target definitions
        },
      },
    },
  };
}
```

## File Pattern Selection

Choose the right file pattern for your use case:

### 1. Configuration-Based Discovery

**Pattern**: `**/tool-config.json`

Use when the tool requires explicit configuration files.

```typescript
export const createNodesV2: CreateNodesV2<Options> = [
  '**/biome.json',  // Only projects with biome.json
  async (configFiles, options, context) => {
    // ...
  },
];
```

**Benefits**:
- Explicit opt-in (teams control which projects use the tool)
- Supports nested configs for selective adoption
- Clear project boundaries

**Example**: Biome, Prettier, Webpack config-based tools

### 2. Package.json Discovery

**Pattern**: `**/package.json`

Use for tools that work with any JavaScript/TypeScript project.

```typescript
export const createNodesV2: CreateNodesV2<Options> = [
  '**/package.json',
  async (configFiles, options, context) => {
    // Filter based on package.json content if needed
    const pkg = readJsonFile(configFilePath);
    if (!pkg.scripts?.['my-script']) {
      return {}; // Skip this project
    }
    // ...
  },
];
```

**Benefits**:
- Works with npm workspaces
- Automatically covers all projects
- Can filter based on dependencies or scripts

**Example**: Jest, ESLint, generic tooling

### 3. Project.json Discovery

**Pattern**: `**/project.json`

Use when not using npm workspaces (legacy NX projects).

```typescript
export const createNodesV2: CreateNodesV2<Options> = [
  '**/project.json',
  async (configFiles, options, context) => {
    // Read existing config and merge
    const projectConfig = readJsonFile(configFilePath);
    return {
      projects: {
        [root]: {
          ...projectConfig,
          targets: {
            ...projectConfig.targets,
            'my-target': {
              // Add your target
            },
          },
        },
      },
    };
  },
];
```

## Target Definition Patterns

### 1. Simple Command Target

```typescript
{
  targets: {
    'my-lint': {
      command: 'npx my-tool lint {projectRoot}',
      cache: true,
      inputs: ['default', '{workspaceRoot}/my-tool.config.js'],
    },
  },
}
```

### 2. Executor-Based Target

```typescript
{
  targets: {
    'my-build': {
      executor: 'tools/my-plugin:build',
      options: {
        outputPath: 'dist/{projectRoot}',
      },
      cache: true,
      outputs: ['{options.outputPath}'],
    },
  },
}
```

### 3. Conditional Targets

```typescript
async function createNodesInternal(
  configFilePath: string,
  options: MyPluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);
  const config = readJsonFile(configFilePath);
  
  const targets: Record<string, any> = {};
  
  // Only add build if needed
  if (config.build) {
    targets['build'] = {
      command: 'npx my-tool build {projectRoot}',
      cache: true,
    };
  }
  
  // Always add lint
  targets['lint'] = {
    command: 'npx my-tool lint {projectRoot}',
    cache: true,
  };
  
  return {
    projects: {
      [root]: { targets },
    },
  };
}
```

## Caching Configuration

### Input Patterns

```typescript
{
  cache: true,
  inputs: [
    // Project source files
    'default',
    
    // Dependency source files
    '^default',
    
    // Specific config files
    '{workspaceRoot}/my-tool.config.js',
    '{projectRoot}/my-tool.json',
    
    // External dependencies
    {
      externalDependencies: ['@my-tool/cli', '@my-tool/core'],
    },
    
    // Environment variables
    {
      env: 'MY_TOOL_ENV',
    },
  ],
}
```

### Output Patterns

```typescript
{
  cache: true,
  outputs: [
    // Static paths
    '{workspaceRoot}/dist/{projectName}',
    
    // Dynamic paths from options
    '{options.outputPath}',
    
    // Multiple outputs
    '{projectRoot}/dist',
    '{projectRoot}/build',
  ],
}
```

## Advanced Patterns

### 1. Plugin Options

```typescript
export interface MyPluginOptions {
  exclude?: string[];
  targetName?: string;
}

async function createNodesInternal(
  configFilePath: string,
  options: MyPluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);
  
  // Use options to customize behavior
  const exclude = options?.exclude || [];
  if (exclude.some(pattern => root.includes(pattern))) {
    return {}; // Skip excluded projects
  }
  
  const targetName = options?.targetName || 'my-default-target';
  
  return {
    projects: {
      [root]: {
        targets: {
          [targetName]: {
            // Target definition
          },
        },
      },
    },
  };
}
```

Configure in `nx.json`:

```json
{
  "plugins": [
    {
      "plugin": "tools/my-plugin",
      "options": {
        "exclude": ["e2e", "integration"],
        "targetName": "custom-lint"
      }
    }
  ]
}
```

### 2. Filtering Projects

```typescript
async function createNodesInternal(
  configFilePath: string,
  options: MyPluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);
  
  // Skip root config
  if (root === '.') {
    return {};
  }
  
  // Only process specific project types
  const pkg = readJsonFile(configFilePath);
  if (pkg.projectType !== 'application') {
    return {};
  }
  
  // Check for specific dependencies
  const hasDependency = 
    pkg.dependencies?.['@my-tool/core'] ||
    pkg.devDependencies?.['@my-tool/core'];
  
  if (!hasDependency) {
    return {};
  }
  
  // Process project
  return {
    projects: {
      [root]: {
        // ...
      },
    },
  };
}
```

### 3. Reading Configuration

```typescript
import { readJsonFile } from '@nx/devkit';
import { existsSync, readFileSync } from 'fs';
import { join } from 'path';

async function createNodesInternal(
  configFilePath: string,
  options: MyPluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);
  
  // Read JSON config
  const config = readJsonFile(join(context.workspaceRoot, root, 'my-config.json'));
  
  // Read text config
  const textConfigPath = join(context.workspaceRoot, root, '.myrc');
  const textConfig = existsSync(textConfigPath)
    ? readFileSync(textConfigPath, 'utf-8')
    : null;
  
  // Use config to customize targets
  return {
    projects: {
      [root]: {
        targets: {
          'my-target': {
            command: `npx my-tool ${config.subcommand} {projectRoot}`,
          },
        },
      },
    },
  };
}
```

## Testing Plugins

### Development Workflow

1. **Bypass daemon cache during development**:

```bash
NX_DAEMON=false nx run-many -t my-target
```

2. **Reset cache between changes**:

```bash
nx reset
```

3. **View generated targets**:

```bash
nx show project my-project --web
```

### Debugging

Add console logs in plugin (temporary):

```typescript
async function createNodesInternal(
  configFilePath: string,
  options: MyPluginOptions | undefined,
  context: CreateNodesContextV2
) {
  console.log('Processing:', configFilePath);
  console.log('Options:', options);
  
  const root = dirname(configFilePath);
  console.log('Project root:', root);
  
  // ... rest of logic
}
```

Run with verbose output:

```bash
NX_VERBOSE_LOGGING=true nx run-many -t my-target
```

## Performance Tips

1. **Use createNodesFromFiles**: Processes files in parallel
2. **Minimize I/O**: Only read files you actually need
3. **Cache expensive computations**: Store results in plugin state
4. **Filter early**: Return `{}` as soon as you know a project should be skipped

## Complete Example: Multi-Tool Plugin

```typescript
import {
  CreateNodesContextV2,
  createNodesFromFiles,
  CreateNodesV2,
  readJsonFile,
} from '@nx/devkit';
import { dirname, join } from 'path';
import { existsSync } from 'fs';

export interface DevToolsPluginOptions {
  lintTarget?: string;
  formatTarget?: string;
  testTarget?: string;
}

export const createNodesV2: CreateNodesV2<DevToolsPluginOptions> = [
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
  options: DevToolsPluginOptions | undefined,
  context: CreateNodesContextV2
) {
  const root = dirname(configFilePath);
  
  // Skip root
  if (root === '.') {
    return {};
  }
  
  const pkg = readJsonFile(configFilePath);
  const targets: Record<string, any> = {};
  
  // Add lint target if biome.json exists
  const biomeConfigPath = join(context.workspaceRoot, root, 'biome.json');
  if (existsSync(biomeConfigPath)) {
    const targetName = options?.lintTarget || 'lint';
    targets[targetName] = {
      command: 'npx @biomejs/biome lint {projectRoot}',
      cache: true,
      inputs: [
        'default',
        '{workspaceRoot}/biome.json',
        '{projectRoot}/biome.json',
        { externalDependencies: ['@biomejs/biome'] },
      ],
    };
  }
  
  // Add format target if prettier config exists
  const prettierConfigPath = join(context.workspaceRoot, root, '.prettierrc');
  if (existsSync(prettierConfigPath)) {
    const targetName = options?.formatTarget || 'format';
    targets[targetName] = {
      command: 'npx prettier --write {projectRoot}',
      cache: true,
      inputs: [
        'default',
        '{workspaceRoot}/.prettierrc',
        '{projectRoot}/.prettierrc',
        { externalDependencies: ['prettier'] },
      ],
    };
  }
  
  // Add test target if jest config exists
  if (pkg.scripts?.test || pkg.jest) {
    const targetName = options?.testTarget || 'test';
    targets[targetName] = {
      command: 'jest',
      cache: true,
      inputs: [
        'default',
        '^default',
        '{workspaceRoot}/jest.config.js',
        { externalDependencies: ['jest'] },
      ],
      outputs: ['{projectRoot}/coverage'],
    };
  }
  
  // Only return project config if we found any targets
  if (Object.keys(targets).length === 0) {
    return {};
  }
  
  return {
    projects: {
      [root]: { targets },
    },
  };
}
```

## References

- [NX Plugin API](https://nx.dev/extending-nx/recipes/project-graph-plugins)
- [NX Devkit API](https://nx.dev/nx-api/devkit)
- [Inferred Tasks Documentation](https://nx.dev/concepts/inferred-tasks)
