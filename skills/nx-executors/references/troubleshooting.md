---
title: Nx Executors - Troubleshooting
category: nx-development
last_updated: January 2026
---

# Nx Executors - Troubleshooting

## Common Issues and Solutions

### "Unable to resolve @scope/tools:executor-name"

**Error Message:**

```shell
NX Cannot find executor '@pantheon-org/tools:dev-proxy' in workspace
Unable to resolve @pantheon-org/tools:dev-proxy
```

**Cause:** Package reference not properly configured.

**Solution:** Verify all three requirements are met:

1. ✅ **Workspace inclusion** - Check `package.json`:

   ```bash
   grep -A5 '"workspaces"' package.json
   # Should include: "tools/executors"
   ```

2. ✅ **Path mapping** - Check `tsconfig.base.json`:

   ```bash
   cat tsconfig.base.json | grep -A2 "@pantheon-org/tools"
   # Should map to: ["tools/src/index.ts"]
   ```

3. ✅ **Entry point exists** - Verify file:

   ```bash
   ls -la tools/src/index.ts
   ```

4. ✅ **Main field** - Check `tools/executors/package.json`:

   ```bash
   cat tools/executors/package.json | grep '"main"'
   # Should be: "main": "../src/index.ts"
   ```

**Quick Fix:**

```bash
# Reinstall workspace
bun install

# Verify package resolution
node -e "require.resolve('@pantheon-org/tools')"
```

---

### Executor Not Found in executors.json

**Error Message:**

```shell
Cannot find executor 'my-executor' in executors.json
```

**Cause:** Executor not registered in the registry.

**Solution:** Add to `tools/executors/executors.json`:

```json
{
  "executors": {
    "my-executor": {
      "implementation": "./my-executor/executor",
      "schema": "./my-executor/schema.json",
      "description": "My executor description"
    }
  }
}
```

**Verify:**

```bash
cat tools/executors/executors.json | grep -A4 '"my-executor"'
```

---

### Schema Validation Errors

**Error Message:**

```shell
Schema validation failed:
- Property 'option1' is required but not provided
- Property 'option2' has invalid type (expected number, got string)
```

**Cause:** Options don't match schema definition.

**Solution:**

1. **Check schema requirements:**

   ```bash
   bunx nx run project:target --help
   ```

2. **Compare schema with usage:**

   ```bash
   # View schema
   cat tools/executors/my-executor/schema.json

   # View usage
   cat packages/my-project/project.json
   ```

3. **Fix mismatches:**

   ```json
   {
     "targets": {
       "my-task": {
         "executor": "@pantheon-org/tools:my-executor",
         "options": {
           "option1": "required-value",
           "option2": 123 // Must be number, not string
         }
       }
     }
   }
   ```

---

### Executor Fails or Doesn't Update

**Symptoms:**

- Executor completes silently without expected actions
- Changes to executor code don't take effect
- Import resolution errors

**Common Causes & Solutions:**

1. **Enable debugging:**

   ```bash
   bunx nx run project:target --verbose
   bunx nx run project:target --dry-run
   ```

2. **Clear Nx cache:**

   ```bash
   bunx nx reset
   bunx nx run project:target --skip-nx-cache
   ```

3. **Fix missing dependencies:**

   ```bash
   cd tools/executors
   bun add @nx/devkit
   ```

4. **Verify imports:**

   ```typescript
   // Correct
   import { ExecutorContext } from "@nx/devkit";
   import { MyExecutorSchema } from "./schema";
   ```

5. **Reinstall workspace:**

   ```bash
   rm -rf node_modules .bun
   bun install
   ```

---

## Testing and Verification

### Test Executor Resolution

```bash
# Show help (verifies executor can be resolved and schema is valid)
bunx nx run <project>:<target> --help

# Dry run (verifies executor logic without side effects)
bunx nx run <project>:<target> --dry-run

# Verbose mode (shows detailed execution logs)
bunx nx run <project>:<target> --verbose
```

### Verify Package Structure

```bash
# Check workspace
bun pm ls | grep @pantheon-org/tools

# Check path mapping
cat tsconfig.base.json | grep -A2 "@pantheon-org/tools"

# Check package import
node -e "require.resolve('@pantheon-org/tools')"

# Check executors registry
cat tools/executors/executors.json | jq '.executors'
```

### View Project Configuration

```bash
# CLI view
bunx nx show project <project>

# Web UI (interactive)
bunx nx show project <project> --web

# JSON export
bunx nx show project <project> --json
```

---

## Performance Optimization

**For slow executor execution:**

1. **Use async operations (not sync):**

   ```typescript
   // Good: Non-blocking
   const data = await fs.promises.readFile("file.json");
   ```

2. **Enable Nx caching:**

   ```json
   {
     "targets": {
       "my-task": {
         "cache": true,
         "inputs": ["..."],
         "outputs": ["..."]
       }
     }
   }
   ```

---

## Related Documentation

- [Nx Executors - Core Concepts](./concepts.md)
- [Workspace Setup Guide](./workspace-setup.md)
- [Nx Troubleshooting Docs](https://nx.dev/troubleshooting)
