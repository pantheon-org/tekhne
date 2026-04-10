# Scenario 02: Optimize Custom Build Executor

## User Prompt

Your team built a custom Nx executor that processes documentation files, but it has performance issues and the cache never works. Developers are complaining about slow builds.

## Current Problems

1. The executor runs every single time, never uses Nx cache
2. File operations block the entire build process
3. All the processing logic is crammed in the executor file (300+ lines)
4. Target configuration uses fragile relative paths
5. When the executor fails, it just logs errors but returns success

## Your Mission

Refactor the executor to follow Nx best practices for performance and maintainability.

## Current Implementation

**executor.ts:**
```typescript
import { readFileSync, writeFileSync } from 'fs';

export default function buildExecutor(options: any, context: any) {
  console.log('Building docs...');
  
  // All business logic inline (imagine 300 lines here)
  const inputFile = readFileSync(options.inputPath, 'utf-8');
  const processed = inputFile.toUpperCase(); // Processing logic
  writeFileSync(options.outputPath, processed);
  
  console.log('Build failed!');
  return { success: true }; // Always returns true even on failure
}
```

**project.json target:**
```json
{
  "targets": {
    "build-docs": {
      "executor": "../../tools/executors/docs-builder/executor",
      "options": {
        "inputPath": "docs/input.md",
        "outputPath": "dist/docs/output.html"
      }
    }
  }
}
```

**schema.json:**
```json
{
  "type": "object",
  "properties": {
    "inputPath": { "type": "string" },
    "outputPath": { "type": "string" }
  }
}
```

## Deliverables

Create these files:

1. **executor.ts** - Refactored executor implementation
2. **schema.json** - Improved schema
3. **schema.d.ts** - TypeScript types
4. **lib.ts** - Extracted business logic
5. **project-config.json** - Example target configuration showing proper executor reference and outputs
6. **IMPROVEMENTS.md** - Explanation of optimizations

## Constraints

- Do NOT create actual workspace - just the executor code and configs
- Focus on executor structure, async I/O, cache configuration, error handling
- Show proper separation of concerns

## Expected Behavior

1. Return `{ success: boolean }` with correct error handling so failures propagate properly
2. Use package notation (`@scope/package:executor`) in target configuration instead of relative paths
3. Declare an `outputs` array in the target configuration so Nx can cache the task
4. Extract business logic to a separate `lib.ts` file; keep the executor as thin orchestration
5. Use async file I/O (`fs.promises`) instead of synchronous operations
6. Mark required fields in schema and provide defaults where appropriate
7. Document the cache and async I/O improvements in IMPROVEMENTS.md

## Success Criteria

- **Return contract**: Executor returns `{ success: boolean }` with proper error handling
- **Package notation**: `project-config.json` uses package notation (`@scope/package:executor`) not relative paths
- **Cache outputs**: `project-config.json` declares `outputs` array for cache awareness
- **Separation of concerns**: Business logic extracted to separate `lib.ts` file, executor is thin orchestration
- **Async I/O**: Uses async file I/O (`fs.promises`) instead of synchronous operations
- **Schema validation**: Schema marks required fields and provides defaults
- **Documentation**: IMPROVEMENTS.md explains cache benefits and async I/O performance gains

## Failure Conditions

- Executor does not return `{ success: boolean }` or always returns `true` even on failure
- Target configuration still uses relative paths instead of package notation
- Target configuration has no `outputs` array, preventing Nx from caching
- Business logic remains inline in the executor file instead of being extracted to `lib.ts`
- File I/O uses synchronous `readFileSync`/`writeFileSync` instead of async alternatives
- Schema has no required fields or defaults
- IMPROVEMENTS.md is missing or does not explain cache or async I/O benefits
