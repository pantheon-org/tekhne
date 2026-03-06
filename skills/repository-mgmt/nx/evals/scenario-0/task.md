# Task: Fix Broken Library Generator

You've inherited an Nx workspace with a custom generator that creates shared libraries. The generator has several bugs that developers have reported:

1. Files sometimes get created even when they already exist (overwriting work)
2. The generator fails when required options are missing
3. Generated code has inconsistent formatting
4. Paths are hardcoded which breaks when the workspace structure changes

## Your Mission

Fix the generator implementation to address all reported issues.

## Environment

- Workspace is at `/tmp/nx-workspace`
- Generator is at `tools/my-plugin/src/generators/library/generator.ts`
- Schema is at `tools/my-plugin/src/generators/library/schema.json`
- Template files are in `tools/my-plugin/src/generators/library/files/`

## Current Implementation

The generator currently looks like this:

```typescript
import { Tree } from '@nx/devkit';
import { writeFileSync } from 'fs';

export default async function (tree: Tree, options: any) {
  // Create library directory
  writeFileSync(`libs/${options.name}/src/index.ts`, 
    `export * from './lib/${options.name}';`
  );
  
  writeFileSync(`libs/${options.name}/README.md`,
    `# ${options.name}`
  );
  
  return { success: true };
}
```

The schema.json is:

```json
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "directory": {
      "type": "string"
    }
  }
}
```

## Deliverables

Create these files in your output:

1. **generator.ts** - Fixed generator implementation
2. **schema.json** - Improved schema with proper validation
3. **schema.d.ts** - TypeScript interface for the schema
4. **FIXES.md** - Explanation of what you fixed and why

## Constraints

- Do NOT create actual Nx workspace files - just the generator code
- Focus on the generator logic, schema, and TypeScript interface
- Keep solutions practical and implementable
