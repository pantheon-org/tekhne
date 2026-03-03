---
title: Nx Executors - Workspace Setup
category: nx-development
project: opencode-plugins
last_updated: January 2026
---

# Nx Executors - Workspace Setup

> **Project-specific configuration for the opencode-plugins monorepo**

## Package Configuration

Custom executors in this workspace are published as `@pantheon-org/tools`.

### Directory Structure

```
tools/
├── executors/
│   ├── check-mirror-exists/      # Executor: validate mirror repo
│   │   ├── executor.ts
│   │   ├── schema.json
│   │   └── schema.d.ts
│   ├── dev-proxy/                # Executor: dev server with hot reload
│   │   ├── executor.ts
│   │   ├── schema.json
│   │   └── schema.d.ts
│   ├── typecheck/                # Executor: TypeScript type checking
│   │   ├── executor.ts
│   │   ├── schema.json
│   │   └── schema.d.ts
│   ├── executors.json            # Executor registry
│   └── package.json              # Package definition
└── src/
    └── index.ts                  # Package entry point (required)
```

## Enabling Package References

To use `@pantheon-org/tools:executor-name` syntax, three configurations are required:

### 1. Workspace Inclusion

**Root `package.json`:**

```json
{
  "workspaces": [
    "packages/*",
    "apps/*",
    "tools/executors"
  ]
}
```

This allows Bun to recognize `@pantheon-org/tools` as a workspace package.

### 2. TypeScript Path Mapping

**`tsconfig.base.json`:**

```json
{
  "compilerOptions": {
    "paths": {
      "@pantheon-org/tools": ["tools/src/index.ts"]
    }
  }
}
```

This allows TypeScript and Node.js to resolve the import path.

### 3. Package Entry Point

**a) Create `tools/src/index.ts`:**

```typescript
/**
 * @pantheon-org/tools package entry point
 * 
 * This file exists to allow Nx to resolve the @pantheon-org/tools package
 * before accessing executors defined in tools/executors/executors.json.
 * 
 * The actual executors are defined in:
 * - tools/executors/executors.json (executor registry)
 * - tools/executors/<executor-name>/executor.ts (implementation)
 * 
 * See tools/executors/package.json for the executor definitions.
 */
```

**b) Reference in `tools/executors/package.json`:**

```json
{
  "name": "@pantheon-org/tools",
  "version": "0.1.0",
  "main": "../src/index.ts",
  "executors": "./executors.json"
}
```

### Why All Three Are Required

Nx resolves executor references in this order:

1. **Import the package** → Requires workspace inclusion + path mapping + entry point
2. **Read executors.json** → Defined in package.json via `"executors"` field
3. **Load executor implementation** → Nx reads from executors.json

If step 1 fails, you get: `Unable to resolve @pantheon-org/tools:executor-name`

## Existing Executors

### dev-proxy

Development proxy server with hot-reload for testing plugins.

**Usage:**

```json
{
  "targets": {
    "dev": {
      "executor": "@pantheon-org/tools:dev-proxy",
      "options": {
        "port": 3000,
        "outputPath": "dist"
      }
    }
  }
}
```

**Schema Options:**
- `port` (number): Port for development server (default: 3000)
- `outputPath` (string): Build output directory (default: "dist")

**Location:** `tools/executors/dev-proxy/`

### check-mirror-exists

Validates that a GitHub mirror repository exists for the plugin.

**Usage:**

```json
{
  "targets": {
    "check-mirror": {
      "executor": "@pantheon-org/tools:check-mirror-exists"
    }
  }
}
```

**Schema Options:** None (reads from package.json)

**Location:** `tools/executors/check-mirror-exists/`

### typecheck

TypeScript type checking executor.

**Usage:**

```json
{
  "targets": {
    "typecheck": {
      "executor": "@pantheon-org/tools:typecheck",
      "options": {
        "tsConfig": "tsconfig.json"
      }
    }
  }
}
```

**Schema Options:**
- `tsConfig` (string): Path to tsconfig.json (default: "tsconfig.json")

**Location:** `tools/executors/typecheck/`

## Using Executors in Projects

### In project.json

```json
{
  "name": "my-plugin",
  "targets": {
    "dev": {
      "executor": "@pantheon-org/tools:dev-proxy",
      "options": {
        "port": 3000
      }
    },
    "check-mirror": {
      "executor": "@pantheon-org/tools:check-mirror-exists"
    },
    "typecheck": {
      "executor": "@pantheon-org/tools:typecheck"
    }
  }
}
```

### Running Executors

```bash
# Show help
bunx nx run my-plugin:dev --help

# Execute
bunx nx run my-plugin:dev

# Execute with options
bunx nx run my-plugin:dev --port=4000

# Verbose mode
bunx nx run my-plugin:dev --verbose
```

## Migration from Relative Paths

### Before (Relative Path)

```json
{
  "targets": {
    "dev": {
      "executor": "../../tools/executors:dev-proxy"
    }
  }
}
```

### After (Package Reference)

```json
{
  "targets": {
    "dev": {
      "executor": "@pantheon-org/tools:dev-proxy"
    }
  }
}
```

### Migration Steps

1. Ensure workspace configuration is complete (see "Enabling Package References")
2. Update all `project.json` files to use package references
3. Test each project: `bunx nx run <project>:<target> --help`
4. Commit changes

## Verification Commands

```bash
# Verify workspace inclusion
bun pm ls | grep @pantheon-org/tools

# Verify path mapping
cat tsconfig.base.json | grep -A2 "@pantheon-org/tools"

# Verify package can be imported
node -e "require.resolve('@pantheon-org/tools')"

# Test executor resolution
bunx nx run <project>:<target> --help

# View project configuration
bunx nx show project <project> --web
```

## Related Documentation

- [Nx Executors - Core Concepts](./concepts.md)
- [Troubleshooting Guide](./troubleshooting.md)
