# Task: Build Component Generator with Templates

Your team needs a generator that creates React components with consistent structure. Instead of string concatenation, you should use Nx's template system for maintainability.

## Requirements

The generator should create components with:
- A TypeScript component file named after the component (PascalCase)
- A test file with .spec.ts extension
- Both files should use proper naming conventions
- Component should export a function with the component name

## Example Usage

```bash
npx nx g my-plugin:component user-profile --directory=shared
```

Should create:
- `libs/shared/user-profile/UserProfile.tsx` (component)
- `libs/shared/user-profile/UserProfile.spec.tsx` (test)

## Current Broken Attempt

Someone tried this with string concatenation:

```typescript
export default async function (tree: Tree, options: { name: string }) {
  const code = `export function ${options.name}() { return null; }`;
  tree.write(`libs/${options.name}.tsx`, code);
}
```

Problems:
- No case conversion (user-profile should become UserProfile)
- Missing test files
- Hard to maintain string templates
- Template extensions not handled

## Your Mission

Build a proper generator using Nx template system.

## Deliverables

Create these files:

1. **generator.ts** - Generator implementation using generateFiles
2. **files/\_\_fileName\_\_.tsx.template** - Component template with EJS variables
3. **files/\_\_fileName\_\_.spec.tsx.template** - Test template
4. **TEMPLATE-GUIDE.md** - Explanation of how the template system works

## Template Requirements

- Use `<%= className %>` for PascalCase component name
- Use `<%= fileName %>` for kebab-case file names
- Use `<%= propertyName %>` for camelCase identifiers
- Include `tmpl: ''` in template variables to handle .template extension

## Constraints

- Focus on template structure and variable substitution
- Show how file naming tokens work (\_\_fileName\_\_ → actual name)
- Demonstrate EJS syntax usage
