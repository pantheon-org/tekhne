---
title: Nx Generators Guide
category: nx-development
last_updated: January 2026
nx_version: 22
---

# Nx Generators Guide

## What Are Generators?

Generators automate file and project scaffolding with repeatable conventions. They use the Tree API for filesystem operations and schema-driven options for type-safe inputs.

## Generator Implementation

**Basic structure:**
```typescript
import { Tree, formatFiles, generateFiles, joinPathSegments } from '@nx/devkit';
import * as path from 'path';
import { GeneratorSchema } from './schema';

export async function myGenerator(tree: Tree, options: GeneratorSchema) {
  const projectRoot = `libs/${options.name}`;
  
  generateFiles(
    tree,
    path.join(__dirname, 'files'),
    projectRoot,
    options
  );
  
  await formatFiles(tree);
}

export default myGenerator;
```

## Tree API Operations

**Write files:**
```typescript
tree.write('path/to/file.ts', 'content');
```

**Read files:**
```typescript
const content = tree.read('path/to/file.ts', 'utf-8');
```

**Check existence:**
```typescript
if (tree.exists('path/to/file.ts')) {
  // file exists
}
```

**Delete files:**
```typescript
tree.delete('path/to/file.ts');
```

**List files:**
```typescript
tree.children('libs/my-lib/src');
```

**Update JSON:**
```typescript
import { updateJson } from '@nx/devkit';

updateJson(tree, 'package.json', (json) => {
  json.scripts.build = 'nx build';
  return json;
});
```

## Project Configuration

**Read project:**
```typescript
import { readProjectConfiguration } from '@nx/devkit';

const project = readProjectConfiguration(tree, 'my-lib');
console.log(project.root); // libs/my-lib
console.log(project.targets); // { build: {...}, test: {...} }
```

**Update project:**
```typescript
import { updateProjectConfiguration } from '@nx/devkit';

const project = readProjectConfiguration(tree, 'my-lib');
project.targets.lint = {
  executor: '@nx/linter:eslint',
  options: {
    lintFilePatterns: [`${project.root}/**/*.ts`]
  }
};
updateProjectConfiguration(tree, 'my-lib', project);
```

**Add new project:**
```typescript
import { addProjectConfiguration } from '@nx/devkit';

addProjectConfiguration(tree, 'my-lib', {
  root: 'libs/my-lib',
  projectType: 'library',
  sourceRoot: 'libs/my-lib/src',
  targets: {
    build: {
      executor: '@nx/js:tsc',
      outputs: ['{options.outputPath}'],
      options: {
        outputPath: 'dist/libs/my-lib',
        main: 'libs/my-lib/src/index.ts',
        tsConfig: 'libs/my-lib/tsconfig.lib.json'
      }
    }
  }
});
```

## Template Generation

**Directory structure:**
```
src/generators/my-generator/
├── generator.ts
├── schema.json
├── schema.d.ts
└── files/
    ├── src/
    │   └── index.ts.template
    ├── README.md.template
    └── __name__/
        └── __fileName__.ts.template
```

**Template files use EJS syntax:**
```typescript
// files/src/index.ts.template
export * from './<%= name %>';

export function <%= propertyName %>(): string {
  return '<%= name %> works!';
}
```

**File name substitution:**
- `__name__` → replaced with `options.name`
- `__fileName__` → replaced with custom option values
- `__directory__` → replaced with `options.directory`

**Generate from templates:**
```typescript
import { generateFiles } from '@nx/devkit';
import * as path from 'path';

generateFiles(
  tree,
  path.join(__dirname, 'files'),
  `libs/${options.name}`,
  {
    ...options,
    template: '', // removes .template extension
    propertyName: names(options.name).propertyName
  }
);
```

## Schema-Driven Options

**schema.json:**
```json
{
  "$schema": "http://json-schema.org/schema",
  "cli": "nx",
  "id": "my-generator",
  "title": "My Generator",
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "description": "Library name",
      "$default": {
        "$source": "argv",
        "index": 0
      },
      "x-prompt": "What name would you like to use?"
    },
    "directory": {
      "type": "string",
      "description": "Directory where the library is placed"
    },
    "tags": {
      "type": "string",
      "description": "Tags to add to project (comma-delimited)"
    },
    "skipFormat": {
      "type": "boolean",
      "description": "Skip formatting files",
      "default": false
    }
  },
  "required": ["name"]
}
```

**schema.d.ts:**
```typescript
export interface GeneratorSchema {
  name: string;
  directory?: string;
  tags?: string;
  skipFormat?: boolean;
}
```

## Utility Helpers

**Name transformations:**
```typescript
import { names } from '@nx/devkit';

const result = names('my-lib');
// {
//   name: 'my-lib',
//   className: 'MyLib',
//   propertyName: 'myLib',
//   constantName: 'MY_LIB',
//   fileName: 'my-lib'
// }
```

**Path utilities:**
```typescript
import { joinPathSegments, normalizePath } from '@nx/devkit';

const fullPath = joinPathSegments('libs', options.name, 'src');
const normalized = normalizePath('libs\\my-lib\\src'); // libs/my-lib/src
```

**Dependency management:**
```typescript
import { addDependenciesToPackageJson } from '@nx/devkit';

addDependenciesToPackageJson(
  tree,
  { '@my-org/shared': '^1.0.0' },
  { '@my-org/build-tools': '^1.0.0' }
);
```

## Validation and Testing

**Dry run validation:**
```bash
npx nx g @my-org/my-plugin:library test --dry-run
```

**Unit test example:**
```typescript
import { createTreeWithEmptyWorkspace } from '@nx/devkit/testing';
import { Tree, readProjectConfiguration } from '@nx/devkit';
import { libraryGenerator } from './generator';

describe('library generator', () => {
  let tree: Tree;

  beforeEach(() => {
    tree = createTreeWithEmptyWorkspace();
  });

  it('should create library', async () => {
    await libraryGenerator(tree, { name: 'test' });
    
    const config = readProjectConfiguration(tree, 'test');
    expect(config.root).toBe('libs/test');
    expect(tree.exists('libs/test/src/index.ts')).toBeTruthy();
    expect(tree.exists('libs/test/README.md')).toBeTruthy();
  });

  it('should add tags', async () => {
    await libraryGenerator(tree, { name: 'test', tags: 'scope:shared,type:util' });
    
    const config = readProjectConfiguration(tree, 'test');
    expect(config.tags).toEqual(['scope:shared', 'type:util']);
  });
});
```

## Best Practices

1. **Use Tree API exclusively** - never direct filesystem writes
2. **Read before mutate** - always `readProjectConfiguration` before `updateProjectConfiguration`
3. **Validate boundaries** - check tags and dependency rules before cross-scope imports
4. **Format generated files** - call `formatFiles(tree)` after generation
5. **Test with dry-run** - validate before broad rollouts
6. **Compose generators** - call other generators to reduce duplication

## References

- [Nx Devkit Generators](https://nx.dev/extending-nx/recipes/local-generators)
- [Tree API Reference](tree-api-reference.md)
- [Schema Design Patterns](schema-design-patterns.md)
- [Template Engine Guide](template-engine-guide.md)
