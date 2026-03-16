# Fix a Generator That Destroys Existing Files

## Problem Description

A generator scaffolds initial configuration for a project. It has a bug: it overwrites existing files without warning the user.

```typescript
import { Tree, readProjectConfiguration, formatFiles } from '@nx/devkit';

interface Schema {
  projectName: string;
}

export default async function addConfig(tree: Tree, schema: Schema) {
  const projectConfig = readProjectConfiguration(tree, schema.projectName);
  const root = projectConfig.root;

  // Write default config files
  tree.write(`${root}/.eslintrc.json`, JSON.stringify({ root: true, extends: ['@myorg/eslint-config'] }, null, 2));
  tree.write(`${root}/tsconfig.json`, JSON.stringify({ extends: '../../tsconfig.base.json', compilerOptions: {} }, null, 2));
  tree.write(`${root}/.prettierrc`, JSON.stringify({ singleQuote: true }, null, 2));

  await formatFiles(tree);
}
```

A developer who ran this generator on a project with a customized `tsconfig.json` lost their changes.

Fix the generator so it:
1. Does not overwrite files that already exist
2. Informs the caller which files were skipped

Produce: `generators/add-config/generator.ts`
