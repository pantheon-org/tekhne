---
name: nx-bun-integration
description: Integrate Bun runtime into Nx monorepos using @nx-bun/nx plugin. Use when setting up Bun projects, converting Node projects to Bun, or configuring Bun executors and generators in NX workspaces.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
  - Grep
  - Glob
---

# NX Bun Integration

Use this skill when integrating Bun into Nx monorepos. The @nx-bun/nx plugin brings Bun's fast runtime, bundler, and test runner to Nx workspaces.

## When to Use This Skill

- Setting up new Bun projects in Nx workspace
- Converting existing Node.js/npm projects to Bun
- Configuring Bun executors (run, build, test)
- Creating Bun applications and libraries
- Integrating Bun with Nx caching and affected commands
- Using bun create templates in Nx

## When NOT to Use This Skill

- General Nx workspace configuration (use nx-workspace-patterns)
- Bun-specific runtime APIs without Nx (use bun-runtime, bun-testing, etc.)
- Non-Bun projects in Nx workspace

## Installation

### 1. Install the Plugin

```bash
# Using npm
npm install -D @nx-bun/nx

# Using bun
bun add -D @nx-bun/nx
```

### 2. Initialize the Plugin

```bash
nx g @nx-bun/nx:init
```

**Options:**
- `--unitTestRunner` - Choose test runner: `bun` (default), `jest`, or `none`
- `--bunNXRuntime` - Run NX itself in Bun environment (experimental, loses Nx Cloud support)

## Generators

### Application Generator

Create a new Bun application:

```bash
# Basic API application
nx g @nx-bun/nx:application my-api

# With custom directory
nx g @nx-bun/nx:application my-api --directory=apps/backend

# Standalone application (no framework)
nx g @nx-bun/nx:application my-app --applicationType=none

# Add tags for module boundaries
nx g @nx-bun/nx:application api --tags=scope:backend,type:app
```

**Options:**
- `--name` (required) - Application name
- `--applicationType` - Type: `api` (default) or `none`
- `--directory` - Directory for the application
- `--projectNameAndRootFormat` - `as-provided` or `derived`
- `--tags` - Tags for linting and module boundaries
- `--rootProject` - Create at workspace root (hidden)

**Generated Structure:**
```
apps/my-api/
├── src/
│   ├── main.ts           # Entry point
│   └── main.test.ts      # Tests
├── project.json          # NX project config
├── tsconfig.json
├── tsconfig.app.json
└── tsconfig.spec.json
```

### Library Generator

Create a new Bun library:

```bash
# Basic library
nx g @nx-bun/nx:lib utils

# Publishable library
nx g @nx-bun/nx:lib ui-components --publishable --importPath=@myorg/ui

# With custom directory and tags
nx g @nx-bun/nx:lib data-access --directory=libs/shared --tags=type:data-access,scope:shared
```

**Options:**
- `--name` (required) - Library name
- `--directory` - Directory where lib is placed
- `--importPath` - Import path (required for publishable)
- `--publishable` - Generate publishable library (default: false)
- `--tags` - Tags for linting
- `--unitTestRunner` - Test runner: `bun`, `jest`, `none`
- `--simpleName` - Don't include directory in file name
- `--skipFormat` - Skip formatting files

**Generated Structure:**
```
libs/utils/
├── src/
│   ├── index.ts          # Barrel export
│   └── lib/
│       ├── utils.ts
│       └── utils.test.ts
├── project.json
├── tsconfig.json
├── tsconfig.lib.json
└── tsconfig.spec.json
```

### Create Generator (Bun Templates)

Use bun create templates directly in Nx:

```bash
# Use bun create template
nx g @nx-bun/nx:create my-project --template=hono --type=application

# Create library from template
nx g @nx-bun/nx:create my-lib --template=library-template --type=library
```

**Options:**
- `--name` (required) - Project name
- `--template` (required) - Bun create template name
- `--type` - `application` (default) or `library`
- `--directory` - Directory for the project
- `--importPath` - Import path for publishable library
- `--publishable` - Generate publishable library
- `--projectNameAndRootFormat` - `as-provided` or `derived`

**Common Templates:**
- `hono` - Fast web framework
- `elysia` - Fast HTTP server
- `kingworld` - Web framework
- Custom templates from GitHub

### Convert to Bun Generator

Convert existing Node.js projects to use Bun:

```bash
# Convert entire project
nx g @nx-bun/nx:convert-to-bun --project=my-app

# Convert specific targets only
nx g @nx-bun/nx:convert-to-bun --project=my-app --targets=build,test

# Use custom conversion registry
nx g @nx-bun/nx:convert-to-bun --project=my-app --customConversionRegistry=@custom/registry
```

**Options:**
- `--project` (required) - Project/Library name to convert
- `--targets` - List of specific targets to convert
- `--customConversionRegistry` - Custom conversion registries

**What It Does:**
- Updates executors to use @nx-bun/nx executors
- Converts npm/yarn commands to bun commands
- Updates test configuration for Bun test runner
- Preserves Nx caching and affected command compatibility

## Executors

### Run Executor

Run JavaScript/TypeScript programs with Bun runtime:

```json
// project.json
{
  "targets": {
    "serve": {
      "executor": "@nx-bun/nx:run",
      "options": {
        "buildTarget": "my-api:build",
        "hot": true,
        "watch": true
      }
    }
  }
}
```

**Options:**
- `--buildTarget` - Target to build the app
- `--hot` - Enable auto reload (default: true)
- `--watch` - Run in watch mode (default: false)
- `--bun` - Force Bun.js instead of Node.js (default: false)
- `--smol` - Reduce memory usage (default: false)
- `--tsConfig` - Custom tsconfig path
- `--config` - Bun config file (e.g., bunfig.toml)

**Usage:**
```bash
# Run with hot reload
nx serve my-api

# Run in watch mode
nx serve my-api --watch

# Run with custom config
nx serve my-api --config=bunfig.production.toml
```

### Build Executor

Bundle programs using bun.build:

```json
// project.json
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build",
      "outputs": ["{options.outputPath}"],
      "options": {
        "entrypoints": ["src/main.ts"],
        "outputPath": "dist/apps/my-api",
        "target": "bun",
        "format": "esm",
        "minify": false,
        "sourcemap": "external"
      },
      "configurations": {
        "production": {
          "minify": true,
          "sourcemap": "none"
        }
      }
    }
  }
}
```

**Options:**
- `--entrypoints` (required) - Entry points for bundler
- `--outputPath` (required) - Output directory
- `--target` - Target environment: `bun`, `node`, `browser`
- `--format` - Bundle format: `esm` (default), `cjs`
- `--minify` - Minify output (default: false)
- `--sourcemap` - Source maps: `none`, `inline`, `external`
- `--splitting` - Enable code splitting (default: false)
- `--external` - External dependencies (not bundled)
- `--compile` - Create standalone executable (default: false)
- `--publicPath` - Public path for assets
- `--config` - Bun config file
- `--smol` - Reduce memory usage

**Usage:**
```bash
# Development build
nx build my-api

# Production build
nx build my-api --configuration=production

# Create executable
nx build my-api --compile

# With external dependencies
nx build my-api --external=react,react-dom
```

### Test Executor

Run tests with Bun's built-in test runner:

```json
// project.json
{
  "targets": {
    "test": {
      "executor": "@nx-bun/nx:test",
      "outputs": ["{workspaceRoot}/coverage/{projectRoot}"],
      "options": {
        "bail": true,
        "timeout": 5000
      }
    }
  }
}
```

**Options:**
- `--bail` - Abort after N failures (default: true)
- `--timeout` - Per-test timeout in ms (default: 5000)
- `--watch` - Watch mode
- `--rerun-each` - Run each test N times (for flaky tests)
- `--preload` - Lifecycle hooks file
- `--config` - Bun config file
- `--tsConfig` - Custom tsconfig
- `--bun` - Force Bun.js (default: false)
- `--smol` - Reduce memory usage

**Usage:**
```bash
# Run all tests
nx test my-lib

# Watch mode
nx test my-lib --watch

# With coverage
nx test my-lib --coverage

# Rerun flaky tests
nx test my-lib --rerun-each=3

# Custom timeout
nx test my-lib --timeout=10000
```

## Integration Patterns

### Pattern 1: Nx Workspace with Bun Projects

```
workspace/
├── apps/
│   ├── api/              # Bun API server
│   │   ├── src/
│   │   │   └── main.ts
│   │   └── project.json  # Uses @nx-bun/nx executors
│   └── web/              # Node.js web app (can coexist)
│       └── project.json
├── libs/
│   ├── shared-utils/     # Bun library
│   │   ├── src/
│   │   └── project.json
│   └── ui-components/    # React library (Node)
│       └── project.json
├── nx.json
└── package.json
```

### Pattern 2: Project Configuration with Bun

```json
// apps/api/project.json
{
  "name": "api",
  "$schema": "../../node_modules/nx/schemas/project-schema.json",
  "sourceRoot": "apps/api/src",
  "projectType": "application",
  "tags": ["type:app", "scope:backend", "runtime:bun"],
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build",
      "outputs": ["{options.outputPath}"],
      "options": {
        "entrypoints": ["apps/api/src/main.ts"],
        "outputPath": "dist/apps/api",
        "target": "bun",
        "format": "esm",
        "sourcemap": "external"
      },
      "configurations": {
        "production": {
          "minify": true,
          "sourcemap": "none"
        }
      }
    },
    "serve": {
      "executor": "@nx-bun/nx:run",
      "options": {
        "buildTarget": "api:build",
        "hot": true
      },
      "configurations": {
        "production": {
          "buildTarget": "api:build:production",
          "hot": false
        }
      }
    },
    "test": {
      "executor": "@nx-bun/nx:test",
      "outputs": ["{workspaceRoot}/coverage/apps/api"],
      "options": {
        "bail": true,
        "timeout": 5000
      }
    },
    "lint": {
      "executor": "@nx/eslint:lint",
      "outputs": ["{options.outputFile}"],
      "options": {
        "lintFilePatterns": ["apps/api/**/*.{ts,tsx}"]
      }
    }
  }
}
```

### Pattern 3: Nx Caching with Bun Builds

```json
// nx.json
{
  "targetDefaults": {
    "build": {
      "dependsOn": ["^build"],
      "inputs": ["production", "^production"],
      "cache": true
    },
    "test": {
      "inputs": ["default", "^production"],
      "cache": true
    }
  },
  "namedInputs": {
    "default": [
      "{projectRoot}/**/*",
      "sharedGlobals"
    ],
    "production": [
      "default",
      "!{projectRoot}/**/?(*.)+(spec|test).[jt]s?(x)?(.snap)",
      "!{projectRoot}/tsconfig.spec.json"
    ],
    "sharedGlobals": [
      "{workspaceRoot}/bunfig.toml",
      "{workspaceRoot}/tsconfig.base.json"
    ]
  }
}
```

### Pattern 4: CI Configuration with Bun Projects

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  main:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: oven-sh/setup-bun@v1
        with:
          bun-version: latest

      - name: Install dependencies
        run: bun install

      - name: Derive SHAs for affected
        uses: nrwl/nx-set-shas@v4

      - name: Run affected lint
        run: bun nx affected -t lint --parallel=3

      - name: Run affected test
        run: bun nx affected -t test --parallel=3

      - name: Run affected build
        run: bun nx affected -t build --parallel=3
```

### Pattern 5: Mixed Runtime Workspace

```json
// nx.json - Supporting both Bun and Node projects
{
  "targetDefaults": {
    "build": {
      "dependsOn": ["^build"],
      "cache": true
    }
  },
  "tasksRunnerOptions": {
    "default": {
      "runner": "nx/tasks-runners/default",
      "options": {
        "cacheableOperations": ["build", "lint", "test"]
      }
    }
  }
}
```

```json
// apps/bun-api/project.json
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build"
    }
  }
}
```

```json
// apps/node-api/project.json
{
  "targets": {
    "build": {
      "executor": "@nx/webpack:webpack"
    }
  }
}
```

### Pattern 6: Module Boundaries for Bun Projects

```json
// .eslintrc.json
{
  "overrides": [
    {
      "files": ["*.ts", "*.tsx"],
      "rules": {
        "@nx/enforce-module-boundaries": [
          "error",
          {
            "depConstraints": [
              {
                "sourceTag": "runtime:bun",
                "onlyDependOnLibsWithTags": [
                  "runtime:bun",
                  "runtime:universal"
                ]
              },
              {
                "sourceTag": "runtime:node",
                "onlyDependOnLibsWithTags": [
                  "runtime:node",
                  "runtime:universal"
                ]
              }
            ]
          }
        ]
      }
    }
  ]
}
```

## Common Workflows

### Create New Bun API Application

```bash
# 1. Generate application
nx g @nx-bun/nx:application api --directory=apps/backend --tags=type:app,scope:backend,runtime:bun

# 2. Generate supporting libraries
nx g @nx-bun/nx:lib data-access --directory=libs/backend --tags=type:data-access,scope:backend,runtime:bun
nx g @nx-bun/nx:lib utils --directory=libs/shared --tags=type:util,scope:shared,runtime:universal

# 3. Run development server
nx serve api

# 4. Run tests
nx test api

# 5. Build for production
nx build api --configuration=production
```

### Convert Existing Node Project to Bun

```bash
# 1. Install plugin
bun add -D @nx-bun/nx

# 2. Initialize
nx g @nx-bun/nx:init --unitTestRunner=bun

# 3. Convert specific project
nx g @nx-bun/nx:convert-to-bun --project=my-api

# 4. Update dependencies (if needed)
bun install

# 5. Verify build works
nx build my-api

# 6. Verify tests work
nx test my-api
```

### Create Publishable Bun Library

```bash
# 1. Generate library
nx g @nx-bun/nx:lib ui-kit \
  --publishable \
  --importPath=@myorg/ui-kit \
  --directory=libs/ui

# 2. Add build target for publishing
# Edit libs/ui/ui-kit/project.json
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build",
      "options": {
        "entrypoints": ["libs/ui/ui-kit/src/index.ts"],
        "outputPath": "dist/libs/ui/ui-kit",
        "format": "esm",
        "external": ["react", "react-dom"]
      }
    }
  }
}

# 3. Build library
nx build ui-kit

# 4. Publish
cd dist/libs/ui/ui-kit
npm publish
```

### Run Affected Commands with Bun

```bash
# Test only affected projects
bun nx affected -t test --base=main

# Build only affected projects
bun nx affected -t build --base=main --configuration=production

# Run multiple targets on affected
bun nx affected -t lint,test,build --base=main --parallel=3

# View affected graph
bun nx affected:graph --base=main
```

## Best Practices

### Use Bun for Appropriate Workloads

Bun excels at:
- API servers and backend services
- CLI tools and scripts
- Libraries with heavy computation
- Projects requiring fast startup

Consider Node.js for:
- Projects with deep native module dependencies
- Production systems requiring maximum stability
- Teams without Bun experience

### Leverage Nx Caching

```json
// Ensure all Bun executors cache properly
{
  "targetDefaults": {
    "build": {
      "cache": true,
      "inputs": ["production", "^production"]
    },
    "test": {
      "cache": true,
      "inputs": ["default", "^production"]
    }
  }
}
```

### Tag Projects by Runtime

```bash
# Tag Bun projects
nx g @nx-bun/nx:application api --tags=runtime:bun

# Tag universal libraries
nx g @nx-bun/nx:lib utils --tags=runtime:universal

# Enforce boundaries
# .eslintrc.json enforces that runtime:bun only imports runtime:bun or runtime:universal
```

### Configure Bun-Specific Settings

```toml
# bunfig.toml at workspace root
[install]
cache = ".bun/cache"
lockfile = true

[test]
coverage = true
coverageDir = "coverage"

[run]
bun = true
```

### Use Nx Affected in CI

```yaml
# Only test/build what changed
- run: bun nx affected -t test --base=${{ github.event.pull_request.base.sha }}
- run: bun nx affected -t build --base=${{ github.event.pull_request.base.sha }}
```

## Anti-Patterns

### Don't Mix Package Managers Inconsistently

```bash
# Bad - Mixed package managers
npm install
bun add some-package
yarn add another-package

# Good - Choose one (preferably bun in bun workspace)
bun install
bun add some-package
```

### Don't Skip Nx Caching Configuration

```json
// Bad - Not leveraging cache
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build"
      // Missing: "cache": true
    }
  }
}

// Good - Cache enabled
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build",
      "cache": true,
      "inputs": ["production", "^production"]
    }
  }
}
```

### Don't Ignore Module Boundaries

```typescript
// Bad - Direct import from another scope
// apps/web/src/main.ts
import { ApiClient } from '../../../apps/api/src/client';

// Good - Import from shared library
// apps/web/src/main.ts
import { ApiClient } from '@myorg/api-client';
```

### Don't Forget to Configure Outputs

```json
// Bad - Missing outputs (breaks caching)
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build"
    }
  }
}

// Good - Outputs configured
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build",
      "outputs": ["{options.outputPath}"]
    }
  }
}
```

### Don't Use --bunNXRuntime in Production

```bash
# Bad - Experimental feature, loses Nx Cloud
nx g @nx-bun/nx:init --bunNXRuntime=true

# Good - Use default Node runtime for Nx
nx g @nx-bun/nx:init
```

## Troubleshooting

### Build Fails with External Dependencies

```json
// Add external dependencies that should not be bundled
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build",
      "options": {
        "external": ["react", "react-dom", "express"]
      }
    }
  }
}
```

### Tests Not Finding Modules

```json
// Ensure tsConfig points to correct path
{
  "targets": {
    "test": {
      "executor": "@nx-bun/nx:test",
      "options": {
        "tsConfig": "apps/api/tsconfig.spec.json"
      }
    }
  }
}
```

### Cache Not Working

```bash
# Clear Nx cache
nx reset

# Verify cache configuration
bun nx show project my-api --web

# Check targetDefaults in nx.json
```

### Hot Reload Not Working

```json
// Ensure hot is enabled
{
  "targets": {
    "serve": {
      "executor": "@nx-bun/nx:run",
      "options": {
        "hot": true,
        "watch": true
      }
    }
  }
}
```

## Related Skills

- **nx-workspace-patterns** - General Nx workspace configuration and patterns
- **bun-runtime** - Bun runtime APIs (file I/O, servers)
- **bun-testing** - Bun test runner details and patterns
- **bun-bundler** - Bun bundling capabilities
- **bun-package-manager** - Bun package management

## References

- [NX Bun Documentation](https://jordan-hall.github.io/nx-bun/)
- [NX Bun GitHub](https://github.com/jordan-hall/nx-bun)
- [Nx Documentation](https://nx.dev/)
- [Bun Documentation](https://bun.sh/docs)
