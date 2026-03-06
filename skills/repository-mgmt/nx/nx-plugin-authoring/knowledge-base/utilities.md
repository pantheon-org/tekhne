# Nx Generator Utilities

Essential utilities from `@nx/devkit` for generator development.

## String Manipulation

### names() - Name Transformations

```typescript
import { names } from "@nx/devkit"

const options = names("my-awesome-lib")
// Output: { name, className, propertyName, constantName, fileName }
```

## Logging

```typescript
import { logger } from "@nx/devkit"

logger.info("Starting generator...")
logger.warn("This configuration might cause issues")
logger.error("Failed to create project")
```

## Dependency Management

```typescript
import { addDependenciesToPackageJson, installPackagesTask } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  addDependenciesToPackageJson(
    tree,
    { "lodash": "^4.17.21" },           // dependencies
    { "@types/lodash": "^4.14.0" }      // devDependencies
  )
  
  return () => installPackagesTask(tree)  // Post-generation task
}
```

## Path Utilities

```typescript
import { joinPathFragments, normalizePath, workspaceRoot } from "@nx/devkit"

// Join path segments (OS-agnostic)
const path = joinPathFragments("libs", schema.name, "src")

// Normalize Windows paths to Unix-style
const normalized = normalizePath("libs\\my-lib\\src")

// Get absolute workspace root path
const root = workspaceRoot
```

## Formatting

### formatFiles

```typescript
import { formatFiles } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Make changes to tree
  
  // Format all modified files (Prettier + linting)
  await formatFiles(tree)
}
```

Applies workspace formatting rules to all modified files.

## Project Utilities

### getProjects

```typescript
import { getProjects } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  const projects = getProjects(tree)
  
  // Check if project exists
  if (projects.has(schema.name)) {
    throw new Error(`Project ${schema.name} already exists`)
  }
  
  // List all projects
  projects.forEach((project, name) => {
    logger.info(`Project: ${name} at ${project.root}`)
  })
}
```

**Return type:** `Map<string, ProjectConfiguration>`

### readProjectConfiguration

```typescript
import { readProjectConfiguration } from "@nx/devkit"

const config = readProjectConfiguration(tree, "my-app")

// Access project properties
console.log(config.root)        // "apps/my-app"
console.log(config.sourceRoot)  // "apps/my-app/src"
console.log(config.targets)     // { build: {...}, serve: {...} }
console.log(config.tags)        // ["scope:app", "type:web"]
```

### updateProjectConfiguration

```typescript
import { updateProjectConfiguration, readProjectConfiguration } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  const config = readProjectConfiguration(tree, schema.name)
  
  updateProjectConfiguration(tree, schema.name, {
    ...config,
    tags: [...(config.tags || []), "scope:shared", "type:util"]
  })
}
```

### addProjectConfiguration

```typescript
import { addProjectConfiguration } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  const projectRoot = `libs/${schema.name}`
  
  addProjectConfiguration(tree, schema.name, {
    root: projectRoot,
    projectType: "library",
    sourceRoot: `${projectRoot}/src`,
    targets: {
      build: {
        executor: "@nx/js:tsc",
        options: {
          outputPath: `dist/${projectRoot}`,
          main: `${projectRoot}/src/index.ts`,
          tsConfig: `${projectRoot}/tsconfig.lib.json`
        }
      },
      test: {
        executor: "@nx/jest:jest",
        options: {
          jestConfig: `${projectRoot}/jest.config.ts`
        }
      }
    },
    tags: schema.tags?.split(",") || []
  })
}
```

## Interactive Prompts

### Using enquirer

```typescript
import { prompt } from "enquirer"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Prompt if option not provided
  if (!schema.style) {
    const response = await prompt<{ style: string }>({
      type: "select",
      name: "style",
      message: "Which style format?",
      choices: ["css", "scss", "less"]
    })
    schema.style = response.style
  }
  
  // Confirm action
  const { confirm } = await prompt<{ confirm: boolean }>({
    type: "confirm",
    name: "confirm",
    message: `Create library at libs/${schema.name}?`
  })
  
  if (!confirm) {
    logger.info("Generation cancelled")
    return
  }
}
```

**Common prompt types:**
- `input` - Text input
- `confirm` - Yes/no
- `select` - Single choice
- `multiselect` - Multiple choices

## Workspace Configuration

### updateJson

```typescript
import { updateJson } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  // Update nx.json
  updateJson(tree, "nx.json", (json) => ({
    ...json,
    targetDefaults: {
      ...json.targetDefaults,
      "custom-target": {
        cache: true,
        inputs: ["default", "^default"]
      }
    }
  }))
  
  // Update tsconfig.base.json
  updateJson(tree, "tsconfig.base.json", (json) => ({
    ...json,
    compilerOptions: {
      ...json.compilerOptions,
      paths: {
        ...json.compilerOptions.paths,
        [`@myorg/${schema.name}`]: [`libs/${schema.name}/src/index.ts`]
      }
    }
  }))
}
```

### readJson

```typescript
import { readJson } from "@nx/devkit"

const packageJson = readJson(tree, "package.json")
const nxJson = readJson(tree, "nx.json")

// Check versions
if (packageJson.dependencies["@nx/devkit"]) {
  logger.info(`Using @nx/devkit ${packageJson.dependencies["@nx/devkit"]}`)
}
```
