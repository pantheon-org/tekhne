# Nx Devkit API

> For the full reference, see [Nx Devkit API Reference](https://nx.dev/docs/reference/devkit).

## File Operations

```typescript
import {
  generateFiles,
  readProjectConfiguration,
  updateProjectConfiguration,
  joinPathFragments
} from '@nx/devkit';

// Generate files from templates
generateFiles(tree, templatePath, targetPath, variables);

// Read project config
const config = readProjectConfiguration(tree, projectName);

// Update project config
updateProjectConfiguration(tree, projectName, updatedConfig);
```

## Project Management

```typescript
import {
  addProjectConfiguration,
  removeProjectConfiguration,
  getProjects
} from '@nx/devkit';

// Add new project
addProjectConfiguration(tree, name, config);

// Remove project
removeProjectConfiguration(tree, name);

// Get all projects
const projects = getProjects(tree);
```

## Dependency Management

```typescript
import {
  addDependenciesToPackageJson,
  removeDependenciesFromPackageJson
} from '@nx/devkit';

// Add dependencies
addDependenciesToPackageJson(
  tree,
  { 'lodash': '^4.17.21' },  // dependencies
  { '@types/lodash': '^4.14.0' }  // devDependencies
);
```

## String Utilities

```typescript
import { names } from '@nx/devkit';

const result = names('my-awesome-lib');
// {
//   name: 'my-awesome-lib',
//   className: 'MyAwesomeLib',
//   propertyName: 'myAwesomeLib',
//   constantName: 'MY_AWESOME_LIB',
//   fileName: 'my-awesome-lib'
// }
```
