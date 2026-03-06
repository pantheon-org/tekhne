# Task: Fix Project Configuration Generator

You have a generator that adds a new build target to existing projects, but it's causing major problems:

1. It overwrites the entire project.json, deleting existing targets
2. It adds imports across project boundaries without checking tags
3. Paths are hardcoded to a specific workspace structure

## Current Broken Implementation

```typescript
import { Tree } from '@nx/devkit';
import { writeFileSync } from 'fs';

export default async function (tree: Tree, options: { project: string }) {
  // PROBLEM 1: Overwrites entire config
  const newConfig = {
    name: options.project,
    targets: {
      'custom-build': {
        executor: '@nx/js:tsc',
        options: { outputPath: 'dist/libs/' + options.project }
      }
    }
  };
  
  tree.write(`libs/${options.project}/project.json`, JSON.stringify(newConfig));
  
  // PROBLEM 2: No boundary checks
  tree.write(
    `libs/${options.project}/src/index.ts`,
    `import { util } from '@myorg/internal-api';` // Cross-boundary import!
  );
}
```

## Reported Issues

**Issue #1**: "The generator deleted all my existing targets!"
- Developer had `build`, `test`, and `lint` targets
- After running generator, only `custom-build` remains

**Issue #2**: "CI fails with boundary violation errors"
- Generator adds imports from `scope:internal` to `scope:public` libraries
- Nx enforce-module-boundaries rule fails

**Issue #3**: "Generator breaks when we reorganize folders"
- Hardcoded `libs/` prefix fails with different workspace layouts
- Some projects are in `packages/`, others in `libs/`

## Your Mission

Fix the generator to safely update configurations.

## Deliverables

Create:

1. **generator.ts** - Fixed implementation using proper APIs
2. **SAFETY-GUIDE.md** - Explain the fixes and why they're necessary

## Key Requirements

- Read existing configuration before modifying
- Preserve all existing targets/settings
- Get project path from configuration, not hardcoded
- Verify boundaries before cross-project imports (check tags)

## Test Scenario

Existing project config:
```json
{
  "name": "my-lib",
  "root": "packages/libs/my-lib",
  "tags": ["scope:public", "type:util"],
  "targets": {
    "build": { "executor": "@nx/js:tsc" },
    "test": { "executor": "@nx/jest:jest" }
  }
}
```

Generator should ADD custom-build target while preserving build and test.

## Constraints

- No actual workspace files - just the generator code
- Focus on safe config mutations and boundary checking
