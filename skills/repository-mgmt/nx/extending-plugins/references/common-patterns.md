# Common Patterns

## Composing Generators

```typescript
import { libraryGenerator } from '@nx/js';

export async function enhancedLibraryGenerator(
  tree: Tree,
  options: Schema
) {
  // Generate base library
  await libraryGenerator(tree, {
    name: options.name,
    directory: 'libs'
  });

  // Add custom files
  generateFiles(
    tree,
    path.join(__dirname, 'files'),
    `libs/${options.name}`,
    options
  );

  await formatFiles(tree);
}
```

## Conditional File Generation

```typescript
export async function myGenerator(tree: Tree, options: Schema) {
  // Always generate base files
  generateFiles(tree, baseFilesPath, projectRoot, options);

  // Conditionally generate test files
  if (options.includeTests) {
    generateFiles(tree, testFilesPath, projectRoot, options);
  }

  await formatFiles(tree);
}
```

## Updating Existing Projects

```typescript
import { updateProjectConfiguration, readProjectConfiguration } from '@nx/devkit';

export async function myGenerator(tree: Tree, options: Schema) {
  const projectConfig = readProjectConfiguration(tree, options.project);

  // Add new target
  projectConfig.targets.myTarget = {
    executor: 'nx:run-commands',
    options: {
      command: 'echo "Hello"'
    }
  };

  updateProjectConfiguration(tree, options.project, projectConfig);
}
```

Always maintain compatibility with supported Nx versions and publish breaking changes as major version bumps.
