# Scenario 04: Fix a Generator That Destroys Existing Files

## User Prompt

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

## Expected Behavior

1. Call `tree.exists(path)` before each `tree.write` call and skip writing if the file already exists
2. Log or return information about which files were skipped (e.g. via `console.log`, an output object, or comments in the result)
3. Apply the `tree.exists` guard to all three config files: `.eslintrc.json`, `tsconfig.json`, and `.prettierrc`
4. Preserve the use of `readProjectConfiguration` to derive the root path and preserve the `formatFiles(tree)` call at the end

## Success Criteria

- **tree.exists check before each write**: `generator.ts` calls `tree.exists(path)` before each `tree.write` call and skips writing if the file already exists
- **Skipped files reported**: `generator.ts` logs or returns information about which files were skipped (e.g. `console.log`, output object, or comments in result)
- **All 3 config files covered by the guard**: The `tree.exists` guard applies to all three files: `.eslintrc.json`, `tsconfig.json`, and `.prettierrc`
- **readProjectConfiguration and formatFiles preserved**: `generator.ts` still uses `readProjectConfiguration` to derive the root path and calls `formatFiles(tree)` at the end

## Failure Conditions

- Generator writes files without checking `tree.exists` first, destroying existing customizations
- Generator provides no output or logging indicating which files were skipped
- The `tree.exists` guard is applied to only some of the three config files, leaving gaps
- `readProjectConfiguration` or `formatFiles(tree)` is removed from the implementation
