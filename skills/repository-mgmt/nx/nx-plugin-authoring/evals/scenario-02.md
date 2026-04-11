# Scenario 02: Fix a Generator with Hardcoded Project Paths

## User Prompt

A generator adds a `CHANGELOG.md` file to an existing Nx project. The current implementation hardcodes the path:

```typescript
import { Tree, formatFiles } from '@nx/devkit';

interface Schema {
  projectName: string;
}

export default async function addChangelog(tree: Tree, schema: Schema) {
  const changelogPath = `libs/${schema.projectName}/CHANGELOG.md`;

  if (!tree.exists(changelogPath)) {
    tree.write(changelogPath, `# Changelog\n\n## Unreleased\n`);
  }

  await formatFiles(tree);
}
```

The problem: the path is hardcoded to `libs/`. If the project lives in `packages/` or `apps/`, the file will be written to the wrong location.

Fix the generator so that it derives the correct project root from the project configuration.

Produce:
1. `generators/add-changelog/generator.ts` — corrected implementation

## Expected Behavior

1. Call `readProjectConfiguration(tree, schema.projectName)` and use the returned `.root` property to build the changelog path
2. Remove the hardcoded `libs/` string — derive the path dynamically from project configuration
3. Preserve the `tree.exists` check before writing, to avoid overwriting an existing CHANGELOG.md
4. Preserve the `formatFiles(tree)` call after writing the file

## Success Criteria

- **readProjectConfiguration used to get project root**: `generator.ts` calls `readProjectConfiguration(tree, schema.projectName)` and uses the returned `.root` property to build the changelog path
- **Hardcoded libs/ prefix removed**: `generator.ts` does not have a hardcoded `libs/` string in the file path — path is dynamic based on project config
- **tree.exists check preserved**: `generator.ts` still checks `tree.exists` before writing to avoid overwriting an existing CHANGELOG.md
- **formatFiles still called**: `generator.ts` still calls `formatFiles(tree)` after writing the file

## Failure Conditions

- Generator still hardcodes `libs/` in the path, breaking projects located in `packages/` or other directories
- Generator does not call `readProjectConfiguration`, failing to derive the correct project root
- The `tree.exists` guard is removed, causing existing CHANGELOG.md files to be overwritten silently
- `formatFiles(tree)` is removed, leaving written files inconsistently formatted
