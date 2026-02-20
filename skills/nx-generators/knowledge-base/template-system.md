# Nx Generator Template System

Template-driven file generation using EJS syntax and Nx conventions.

## Overview

Generators use template files to create workspace files with variable substitution:

- Templates use **EJS syntax** for variable injection
- File naming supports **token replacement** (`__name__` → actual value)
- Extension handling via **`.template` suffix** (removed during generation)
- Template location is **colocated** with generator code

## Template Directory Structure

```
generators/
└── my-generator/
    ├── generator.ts
    ├── schema.json
    └── files/                    # Template directory
        ├── README.md.template    # .template removed
        ├── src/
        │   ├── __name__.ts.template
        │   └── index.ts.template
        └── __name__/             # __name__ replaced
            └── config.json.template
```

## Using generateFiles

```typescript
import { generateFiles, joinPathFragments } from "@nx/devkit"

export default async function (tree: Tree, schema: MyGeneratorSchema) {
  generateFiles(
    tree,                                      // Tree instance
    joinPathFragments(__dirname, "./files"),   // Template source dir
    `./libs/${schema.name}`,                   // Destination path
    {                                          // Template variables
      name: schema.name,
      description: schema.description || "No description",
      author: "MyOrg",
      tmpl: ""  // Controls .template extension removal
    }
  )
}
```

### Parameters

1. **tree**: Tree instance for file operations
2. **srcFolder**: Absolute path to template directory (use `__dirname`)
3. **target**: Destination path (relative to workspace root)
4. **substitutions**: Object with template variables

## EJS Template Syntax

### Variable Injection

```markdown
<!-- README.md.template -->
# <%= name %>

Author: <%= author %>
Created: <%= new Date().toISOString() %>
```

### Conditional Logic

```typescript
// config.ts.template
export const config = {
  name: "<%= name %>",
  <% if (includeTests) { %>
  testDir: "tests",
  <% } %>
  version: "1.0.0"
}
```

### Loops

```typescript
// tags.ts.template
export const tags = [
  <% tags.forEach((tag, i) => { %>
  "<%= tag %>"<%= i < tags.length - 1 ? ',' : '' %>
  <% }) %>
]
```

## File Naming Conventions

Template file names support token replacement:

```
files/
├── __name__.ts.template        # Replaced with schema.name
├── __directory__/              # Replaced with schema.directory
└── README.md.template          # No replacement
```

**Example:**

```typescript
generateFiles(tree, srcDir, "./libs/my-lib", {
  name: "my-lib",
  directory: "shared",
  tmpl: ""
})
```

**Result:** `__name__.ts.template` → `my-lib.ts`, `__directory__/` → `shared/`

### Extension Handling

The `tmpl` variable controls `.template` extension removal:

```typescript
// Remove .template extension (common practice)
generateFiles(tree, srcDir, destDir, { tmpl: "" })
```

## Advanced Template Patterns

### Using names Utility

```typescript
import { names } from "@nx/devkit"

const nameVariants = names(schema.name)
// { className, propertyName, constantName, fileName }

generateFiles(tree, srcDir, destDir, {
  ...nameVariants,
  tmpl: ""
})
```

**Template:**

```typescript
// __fileName__.ts.template
export class <%= className %> {
  private <%= propertyName %>: string
  public static <%= constantName %> = "<%= name %>"
}
```

### Multi-line Conditionals

```typescript
// tsconfig.json.template
{
  "extends": "<%= relativePathToRoot %>tsconfig.base.json",
  "compilerOptions": {
    "module": "commonjs",
    <% if (enableDecorators) { %>
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    <% } %>
    <% if (strict) { %>
    "strict": true,
    "noImplicitAny": true,
    <% } %>
    "outDir": "./dist"
  }
}
```

## Template Organization

### Single Directory (Simple)

```
my-generator/
├── generator.ts
└── files/
    └── src/index.ts.template
```

### Multiple Sets (Complex)

```
my-generator/
├── generator.ts
├── files-base/
├── files-react/
└── files-angular/
```

```typescript
// Generate base + conditional framework files
generateFiles(tree, joinPathFragments(__dirname, "./files-base"), root, schema)

if (schema.framework === "react") {
  generateFiles(tree, joinPathFragments(__dirname, "./files-react"), root, schema)
}
```

## Template Best Practices

1. **Type-Safe Variables** - Spread schema for type safety: `{ ...schema, ...names(schema.name), tmpl: "" }`
2. **Provide Defaults** - Handle optional variables: `description: schema.description || "No description"`
3. **Validate Before Generation** - Check inputs before calling `generateFiles()`
4. **Format Generated Files** - Always call `await formatFiles(tree)` after generation

## Common Template Examples

### TypeScript Module

```typescript
// index.ts.template
<% if (hasDescription) { %>
/** <%= description %> */
<% } %>
export const <%= propertyName %> = () => "<%= name %>"
```

### Configuration

```json
// config.json.template
{
  "name": "<%= name %>",
  "features": <%= JSON.stringify(features || []) %>
}
```

### Test File

```typescript
// __fileName__.spec.ts.template
import { <%= className %> } from "./<%= fileName %>"

describe("<%= className %>", () => {
  it("should be defined", () => {
    expect(<%= className %>).toBeDefined()
  })
})
```

## Debugging Templates

```typescript
// Log generated files
tree.listChanges().forEach((change) => {
  logger.info(`Generated: ${change.path}`)
})

// Validate template variables
console.log("Template vars:", JSON.stringify(templateVars, null, 2))
```

**Dry run:** `nx g my-generator mylib --dry-run` (shows generated files without writing)
