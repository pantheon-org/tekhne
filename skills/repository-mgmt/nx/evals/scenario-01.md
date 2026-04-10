# Scenario 01: Fix Broken Library Generator

## User Prompt

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

## Expected Behavior

1. Replace all direct `fs.writeFileSync` calls with Tree API methods (`tree.write()`, `generateFiles()`, or `updateJson()`)
2. Add typed interface in `schema.d.ts` and mark required fields in `schema.json`
3. Call `readProjectConfiguration()` before any `updateProjectConfiguration()` and preserve existing targets and tags
4. Validate project tags and boundaries before cross-scope imports, throwing errors on violations
5. Call `formatFiles(tree)` after all file generation
6. Check `tree.exists()` before writing files and handle conflicts gracefully
7. Derive all paths from `readProjectConfiguration().root` instead of hardcoding `libs/` or `apps/`

## Success Criteria

- **Tree API usage**: Generator uses `tree.write()`, `generateFiles()`, or `updateJson()` exclusively for filesystem operations. No `fs.writeFileSync` or direct filesystem writes.
- **Schema validation**: Schema includes typed interface (`schema.d.ts`), required fields marked in `schema.json`, and validation guards in generator code.
- **Read before mutate**: Generator calls `readProjectConfiguration()` before `updateProjectConfiguration()` and preserves existing targets/tags.
- **Boundary enforcement**: Generator validates project tags/boundaries before cross-scope imports and throws errors for violations.
- **Format after generation**: Generator calls `formatFiles(tree)` after `generateFiles()` or file writes.
- **Existence check**: Generator checks `tree.exists()` before overwriting files and handles conflicts gracefully.
- **No hardcoded paths**: Generator derives paths from `readProjectConfiguration().root` instead of hardcoding `libs/` or `apps/` paths.

## Failure Conditions

- Generator still uses `fs.writeFileSync` or any direct filesystem writes instead of the Tree API
- Schema has no typed interface or required field markers; generator performs no input validation
- Generator overwrites project configuration without reading existing state first
- Generator adds cross-scope imports without checking project tags
- Generator skips `formatFiles(tree)`, producing inconsistently formatted output
- Generator overwrites existing files without any existence check
- Generator hardcodes `libs/` or `apps/` prefixes instead of reading paths from project configuration
