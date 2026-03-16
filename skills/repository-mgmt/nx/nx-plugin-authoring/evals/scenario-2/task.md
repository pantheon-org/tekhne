# Fix a Generator with Hardcoded Project Paths

## Problem Description

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
