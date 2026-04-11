# Scenario 03: Build Component Generator with Templates

## User Prompt

Your team needs a generator that creates React components with consistent structure. Instead of string concatenation, you should use Nx's template system for maintainability.

## Requirements

The generator should create components with:
- A TypeScript component file named after the component (PascalCase)
- A test file with `.spec.ts` extension
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
- Include `tmpl: ''` in template variables to handle `.template` extension

## Constraints

- Focus on template structure and variable substitution
- Show how file naming tokens work (`__fileName__` → actual name)
- Demonstrate EJS syntax usage

## Expected Behavior

1. Use `generateFiles()` from `@nx/devkit` with all required parameters
2. Pass `names()` transformations (`className`, `propertyName`, `fileName`) as template variables
3. Include `tmpl: ''` in template variables so the `.template` extension is stripped from output files
4. Use `__fileName__` tokens in template file names for dynamic naming
5. Use EJS syntax (`<%= variableName %>`) inside template files for variable substitution
6. Document name transformations and file token replacement in TEMPLATE-GUIDE.md

## Success Criteria

- **Generate files API**: Uses `generateFiles()` from `@nx/devkit` with proper parameters
- **Name transformations**: Template variables include `names()` transformations (`className`, `propertyName`, `fileName`)
- **Template extension stripping**: Template variables include `tmpl: ''` to strip `.template` extension
- **Dynamic file naming**: Template files use `__fileName__` token for dynamic file naming
- **EJS syntax**: Templates use EJS syntax for variable substitution (`<%= variableName %>`)
- **Documentation**: TEMPLATE-GUIDE.md explains name transformations and file token replacement

## Failure Conditions

- Generator uses string concatenation instead of `generateFiles()` from `@nx/devkit`
- Template variables omit `names()` transformations, producing wrong casing (e.g. `user-profile` instead of `UserProfile`)
- `tmpl: ''` is absent, leaving `.template` extensions on output files
- Template files do not use `__fileName__` token, producing literal filenames
- Templates do not use EJS syntax, preventing variable substitution
- TEMPLATE-GUIDE.md is missing or does not explain token replacement and name transformations
