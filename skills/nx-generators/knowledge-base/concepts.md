# Nx Generator Concepts

Core concepts and APIs for creating custom Nx generators.

## Generator Structure

Generators are TypeScript functions that modify the workspace file system:

```
plugins/
└── <namespace>/
    └── src/
        └── generators/
            └── my-generator/
                ├── generator.ts      # Main implementation
                ├── generator.spec.ts # Unit tests
                ├── schema.d.ts       # TypeScript interface
                └── schema.json       # JSON schema config
```

### Basic Generator Signature

```typescript
import { Tree } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Modify tree
  await formatFiles(tree)
  
  // Return post-generation callback (optional)
  return () => {
    installPackagesTask(tree)
  }
}
```

**Key characteristics:**
- Receives `Tree` (virtual file system) and `schema` (user options)
- Returns optional callback for post-generation tasks
- All file operations are in-memory until completion
- Async functions allow composing other generators

## The Tree API

`Tree` represents an in-memory file system that batches changes:

- Changes are staged, not written immediately
- All operations are batched for performance
- Actual file writes happen after generator completes
- Multiple generators can compose without I/O overhead

### Core Tree Methods

**File operations:**

```typescript
// Read/write
tree.exists("path/to/file.ts")
tree.read("path/to/file.ts", "utf-8")
tree.write("path/to/new-file.ts", "content")
tree.delete("path/to/file.ts")
tree.rename("old.ts", "new.ts")

// JSON
readJson(tree, "package.json")
updateJson(tree, "nx.json", (json) => ({ ...json, ... }))
```

**Project configuration:**

```typescript
import {
  readProjectConfiguration,
  updateProjectConfiguration,
  addProjectConfiguration,
  getProjects
} from "@nx/devkit"

// Read config
const config = readProjectConfiguration(tree, "my-app")

// Update config
updateProjectConfiguration(tree, "my-app", {
  ...config,
  tags: ["scope:shared", "type:util"]
})

// Add new project
addProjectConfiguration(tree, "new-lib", {
  root: "libs/new-lib",
  projectType: "library",
  sourceRoot: "libs/new-lib/src",
  targets: { build: { executor: "@nx/js:tsc", options: {} } }
})

// Get all projects
const projects = getProjects(tree)
```

## Schema System

Schemas define generator options with validation and CLI integration.

### TypeScript Schema Interface

```typescript
// schema.d.ts
export interface MyGeneratorSchema {
  name: string
  directory?: string
  style?: "css" | "scss" | "less"
  tags?: string
  skipTests?: boolean
}
```

### JSON Schema Definition

```json
{
  "cli": "nx",
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "$default": { "$source": "argv", "index": 0 },
      "x-prompt": "Library name?"
    },
    "style": {
      "type": "string",
      "enum": ["css", "scss"],
      "default": "css"
    }
  },
  "required": ["name"]
}
```

**Key features:**
- `"$default"` - Positional arguments: `nx g my-generator mylib`
- `"x-prompt"` - Interactive prompts for missing options
- `"alias"` - Shorthand (`--dir` instead of `--directory`)
- `"enum"` - Value validation and restrictions

## Composing Generators

Call other generators from within your generator:

```typescript
import { libraryGenerator } from "@nx/js"
import { componentGenerator } from "@nx/react"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Create base library
  await libraryGenerator(tree, {
    name: schema.name,
    directory: "libs",
    bundler: "tsc"
  })
  
  // Add component to library
  await componentGenerator(tree, {
    name: "MyComponent",
    project: schema.name,
    style: "scss"
  })
  
  // Format all files
  await formatFiles(tree)
}
```

**Benefits:**
- Reuse existing generators
- Build complex scaffolding
- Maintain consistency with official generators
- All operations remain in-memory

## File Generation

### Using generateFiles

```typescript
import { generateFiles, joinPathFragments } from "@nx/devkit"

generateFiles(
  tree,                                      // Tree instance
  joinPathFragments(__dirname, "./files"),   // Template source
  `./libs/${schema.name}`,                   // Destination
  {                                          // Template variables
    name: schema.name,
    description: schema.description,
    tmpl: ""  // Removes .template extension
  }
)
```

### Path Utilities

```typescript
import { joinPathFragments, normalizePath, workspaceRoot } from "@nx/devkit"

// Join path segments
const path = joinPathFragments("libs", schema.name, "src")
// => "libs/my-lib/src"

// Normalize path separators
const normalized = normalizePath("libs\\my-lib\\src")
// => "libs/my-lib/src" (Unix-style)

// Get workspace root
const root = workspaceRoot
// => "/absolute/path/to/workspace"
```

## Formatting and Tasks

### Format Files

```typescript
import { formatFiles } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Make changes to tree
  
  // Format all modified files
  await formatFiles(tree)
}
```

Applies Prettier/linting to all modified files.

### Post-Generation Tasks

```typescript
import { installPackagesTask } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Make changes
  
  await formatFiles(tree)
  
  // Return callback to run after tree commits
  return () => {
    installPackagesTask(tree)
  }
}
```

Tasks run after all file changes are written.

## Testing Generators

```typescript
import { createTreeWithEmptyWorkspace } from "@nx/devkit/testing"
import { readProjectConfiguration } from "@nx/devkit"
import myGenerator from "./generator"

describe("my-generator", () => {
  let tree: Tree
  
  beforeEach(() => {
    tree = createTreeWithEmptyWorkspace()
  })
  
  it("should create a library", async () => {
    await myGenerator(tree, { name: "test-lib" })
    
    const config = readProjectConfiguration(tree, "test-lib")
    expect(config).toBeDefined()
    expect(tree.exists("libs/test-lib/src/index.ts")).toBeTruthy()
  })
})
```

## Validation and Conditional Logic

```typescript
export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Validate name format
  if (!schema.name.match(/^[a-z][a-z0-9-]*$/)) {
    throw new Error("Name must be kebab-case")
  }
  
  // Check if project exists
  const projects = getProjects(tree)
  if (projects.has(schema.name)) {
    throw new Error(`Project ${schema.name} already exists`)
  }
  
  // Conditional file generation
  if (schema.includeTests) {
    generateFiles(tree, "./test-files", "./tests", schema)
  }
}
```
