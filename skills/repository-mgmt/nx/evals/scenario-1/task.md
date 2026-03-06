# Task: Optimize Custom Build Executor

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
