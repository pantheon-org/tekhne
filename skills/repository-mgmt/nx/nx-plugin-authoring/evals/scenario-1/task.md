# Write an Nx Generator Using the Tree API

## Problem Description

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
