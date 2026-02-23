# Nx Executor Configuration with @nx-bun/nx

Use this reference for detailed target configuration.

## Core Executors

- `@nx-bun/nx:run`: run/serve applications with Bun runtime
- `@nx-bun/nx:build`: bundle applications/libraries
- `@nx-bun/nx:test`: execute tests with Bun test runner

## Example Target Configuration

```json
{
  "targets": {
    "build": {
      "executor": "@nx-bun/nx:build",
      "outputs": ["{options.outputPath}"],
      "options": {
        "entrypoints": ["apps/api/src/main.ts"],
        "outputPath": "dist/apps/api",
        "target": "bun",
        "format": "esm"
      }
    },
    "serve": {
      "executor": "@nx-bun/nx:run",
      "options": {
        "buildTarget": "api:build",
        "hot": true,
        "watch": true
      }
    },
    "test": {
      "executor": "@nx-bun/nx:test",
      "outputs": ["{workspaceRoot}/coverage/{projectRoot}"],
      "options": {
        "timeout": 5000,
        "bail": true
      }
    }
  }
}
```

## Caching Strategy

- Always define `outputs` for cacheable targets.
- Keep named inputs consistent with repo standards.
- Include shared config artifacts (`bunfig.toml`, `tsconfig.base.json`) where relevant.

Example `nx.json` excerpt:

```json
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

## Verification

- `bun nx show project <name> --web` to inspect inferred/effective targets.
- `nx reset` after executor/plugin updates.
