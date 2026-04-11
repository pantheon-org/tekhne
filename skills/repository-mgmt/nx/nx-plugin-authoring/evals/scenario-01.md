# Scenario 01: Write an Nx Generator Using the Tree API

## User Prompt

A team wants an Nx generator that scaffolds a new shared library with a README. The generator should:
1. Create `libs/<name>/src/index.ts` with a placeholder export
2. Create `libs/<name>/README.md` with the library name as the heading
3. Format the generated files

A developer wrote this first attempt:

```typescript
import { Tree } from '@nx/devkit';
import { writeFileSync } from 'fs';
import { join } from 'path';

interface Schema {
  name: string;
}

export default async function generator(tree: Tree, schema: Schema) {
  const basePath = join(process.cwd(), 'libs', schema.name);

  writeFileSync(join(basePath, 'src/index.ts'), `export const ${schema.name} = {};`);
  writeFileSync(join(basePath, 'README.md'), `# ${schema.name}\n`);
}
```

Fix this generator. Produce:
1. `generators/library-with-readme/generator.ts` — corrected implementation using Tree API
2. `generators/library-with-readme/schema.json` — typed schema for the generator options

## Expected Behavior

1. Replace `fs.writeFileSync` calls with `tree.write()` or `generateFiles()` from `@nx/devkit`
2. Call `formatFiles(tree)` after generating files to ensure consistent formatting
3. Remove `process.cwd()` and `join()` with absolute paths — Tree API paths are workspace-relative
4. Define `name` as a required string field with type and description in `schema.json` (not `schema: any`)

## Success Criteria

- **tree.write or generateFiles used instead of writeFileSync**: `generator.ts` uses `tree.write()` or `generateFiles()` from `@nx/devkit` — no `fs.writeFileSync` or direct filesystem writes
- **formatFiles(tree) called**: `generator.ts` calls `formatFiles(tree)` after generating files to ensure consistent formatting
- **No process.cwd() path construction**: `generator.ts` does not use `process.cwd()` or `join()` with absolute paths — Tree API paths are workspace-relative
- **schema.json has typed fields**: `schema.json` defines `name` as a required string field with type and description (not `schema: any`)

## Failure Conditions

- Generator still uses `fs.writeFileSync` or any other direct filesystem write instead of the Tree API
- Generator does not call `formatFiles(tree)`, leaving generated files inconsistently formatted
- Generator still uses `process.cwd()` or `join()` with absolute paths to construct file locations
- `schema.json` uses a generic `any` type or is missing required field declarations
