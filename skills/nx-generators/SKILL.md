---
name: nx-generators
description: Create Nx generators to automate code scaffolding and enforce best practices in Nx workspaces. Covers the Tree API, schema validation, composing generators, template files, and testing with TypeScript.
license: MIT
compatibility: opencode
metadata:
  category: nx-development
  audience: nx-developers
---

# Nx Generators Skill

Guide for creating custom Nx generators to automate code scaffolding and enforce workspace best practices.

## When to Use This Skill

Use this skill when:
- User requests creating a new Nx generator
- Scaffolding code in an Nx workspace
- Automating repetitive project setup tasks
- Enforcing workspace conventions and best practices

## Knowledge Base References

For technical details on Nx generators, refer to these KB articles:

1. **[concepts.md](knowledge-base/concepts.md)** - Generator structure, Tree API, schema system
2. **[template-system.md](knowledge-base/template-system.md)** - EJS templates, file naming, variable injection
3. **[utilities.md](knowledge-base/utilities.md)** - Devkit utilities (names, logger, dependencies)

## Agent Workflow

### Step 1: Understand Requirements

Before creating a generator, clarify:
- What does the generator create? (library, component, configuration)
- What files need to be generated?
- What options should it accept? (required vs optional)
- What conventions should it enforce? (naming, structure, tags)

Ask the user if any requirements are unclear.

### Step 2: Set Up Generator Structure

Create the generator scaffold:

```bash
# If no plugin exists yet
nx add @nx/plugin
nx g @nx/plugin:plugin plugins/<namespace>

# Create generator
nx generate @nx/plugin:generator <generator-name> --project=<plugin-name>
```

This creates:
```
plugins/<namespace>/src/generators/<generator-name>/
├── generator.ts      # Main implementation
├── generator.spec.ts # Tests
├── schema.d.ts       # TypeScript types
├── schema.json       # CLI configuration
└── files/            # Template directory (create manually)
```

### Step 3: Define Schema

Start with the schema to define the generator's interface:

```json
// schema.json
{
  "cli": "nx",
  "id": "<generator-name>",
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "description": "Name of the generated artifact",
      "$default": { "$source": "argv", "index": 0 }
    }
  },
  "required": ["name"]
}
```

**Reference:** [concepts.md](knowledge-base/concepts.md#schema-definitions)

### Step 4: Implement Generator Logic

Follow this pattern:

```typescript
// generator.ts
import {
  Tree, formatFiles, generateFiles,
  joinPathFragments, names, logger
} from "@nx/devkit"

export default async function (tree: Tree, schema: Schema) {
  // 1. Validate input
  if (!schema.name.match(/^[a-z][a-z0-9-]*$/)) {
    throw new Error("Name must be kebab-case")
  }
  
  // 2. Compute paths and variables
  const projectRoot = `libs/${schema.name}`
  const templateVars = {
    ...schema,
    ...names(schema.name),
    tmpl: ""
  }
  
  // 3. Generate files from templates
  generateFiles(
    tree,
    joinPathFragments(__dirname, "./files"),
    projectRoot,
    templateVars
  )
  
  // 4. Format generated files
  await formatFiles(tree)
  
  // 5. Log success
  logger.info(`Generated ${schema.name} successfully`)
}
```

**Reference:** [concepts.md](knowledge-base/concepts.md#tree-api), [utilities.md](knowledge-base/utilities.md)

### Step 5: Create Template Files

Create `files/` directory with templates:

```
files/
├── README.md.template
├── src/
│   ├── index.ts.template
│   └── __name__.ts.template  # __name__ replaced with actual name
```

Use EJS syntax for variable injection:

```typescript
// src/__name__.ts.template
export const <%= propertyName %> = () => {
  return "<%= name %>"
}
```

**Reference:** [template-system.md](knowledge-base/template-system.md)

### Step 6: Write Tests

```typescript
// generator.spec.ts
import { createTreeWithEmptyWorkspace } from "@nx/devkit/testing"
import generator from "./generator"

describe("my-generator", () => {
  it("should generate files", async () => {
    const tree = createTreeWithEmptyWorkspace()
    await generator(tree, { name: "test-lib" })
    
    expect(tree.exists("libs/test-lib/README.md")).toBeTruthy()
  })
})
```

### Step 7: Test Manually

```bash
# Dry run (shows changes without writing)
nx g <plugin-name>:<generator-name> test-lib --dry-run

# Actual run
nx g <plugin-name>:<generator-name> test-lib

# Run tests
nx test <plugin-name>
```

### Step 8: Iterate and Refine

Based on test results:
1. Add error handling for edge cases
2. Improve validation messages
3. Add conditional logic if needed
4. Update documentation

## Common Patterns

### Composing Generators

```typescript
import { libraryGenerator } from "@nx/js"

export default async function (tree: Tree, schema: Schema) {
  await libraryGenerator(tree, { name: schema.name })
  generateFiles(/* ... */)
  await formatFiles(tree)
}
```

### Conditional File Generation

```typescript
export default async function (tree: Tree, schema: Schema) {
  generateFiles(tree, baseSrc, dest, schema)
  
  if (schema.includeTests) {
    generateFiles(tree, testSrc, dest, schema)
  }
}
```

### Updating Project Configuration

```typescript
import { readProjectConfiguration, updateProjectConfiguration } from "@nx/devkit"

const config = readProjectConfiguration(tree, schema.name)
updateProjectConfiguration(tree, schema.name, {
  ...config,
  tags: [...(config.tags || []), "scope:shared"]
})
```

## Troubleshooting

### Generator Not Found
- Ensure plugin is built: `nx build <plugin-name>`
- Use package.json name: `nx g @myorg/plugin:gen`

### Template Variables Not Substituted
- Ensure `tmpl: ""` in variables object
- Check EJS syntax: `<%= var %>` not `{{ var }}`

### Tree Changes Not Persisted
- Call `await formatFiles(tree)` before returning

### Schema Validation Fails
- Verify `schema.json` has correct types and required fields

## Best Practices

1. **Validate Early** - Check inputs before making changes
2. **Use Type Safety** - Import schema types in generator
3. **Always Format** - Call `formatFiles(tree)` before returning
4. **Write Tests** - Test success paths and error cases
5. **Document Options** - Add descriptions to schema.json properties
6. **Use Dry Run** - Test with `--dry-run` first
7. **Compose When Possible** - Reuse existing generators
8. **Log Progress** - Use `logger.info()` for user feedback

## Quick Reference

```typescript
// Essential imports
import {
  Tree, formatFiles, generateFiles,
  joinPathFragments, names, logger,
  readProjectConfiguration,
  updateProjectConfiguration,
  updateJson, installPackagesTask
} from "@nx/devkit"

// Template variables
{
  ...schema,
  ...names(schema.name),
  tmpl: ""  // Remove .template extension
}

// Running generators
nx g <plugin>:<generator> <name> [options] --dry-run
```

## Resources

- **KB Articles:**
  - [concepts.md](knowledge-base/concepts.md) - Core generator concepts
  - [template-system.md](knowledge-base/template-system.md) - Template syntax
  - [utilities.md](knowledge-base/utilities.md) - Helper utilities
- **Official Docs:** [Nx Plugin Development](https://nx.dev/extending-nx/intro)
- **API Reference:** [Nx Devkit](https://nx.dev/reference/devkit)
