---
name: biomejs
description:
  Resolve BiomeJS linting errors and warnings with fix-forward approach (never ignore/suppress). Covers formatting,
  correctness, suspicious patterns, style, complexity, and performance rules.
license: MIT
compatibility: opencode
metadata:
  category: linting
  tool: biome
---

## What I do

I help resolve all BiomeJS errors and warnings by **fixing the root cause**, never by adding ignore comments or
suppressions.

## Core Principles

1. **NEVER use suppression comments** - Do not use `// biome-ignore`, `// biome-ignore lint`, or any suppression
   directives
2. **Fix forward, not backward** - Address the underlying issue rather than disabling the rule
3. **Prefer automated fixes** - Use `biome check --write` or `biome lint --write` when safe
4. **Manual fixes for unsafe changes** - When Biome flags a change as unsafe, carefully implement the fix manually
5. **Understand the intent** - Each rule exists to catch real issues; fix them properly

## When to use me

- After running `biome check`, `biome lint`, or `biome format` and seeing errors
- When CI fails due to Biome linting violations
- When refactoring code and needing to address Biome warnings
- When reviewing Biome output and unsure how to fix specific rules

## How to resolve errors

### Step 1: Run Biome to see all issues

```bash
# Check all files (dry run - shows issues without fixing)
biome check .

# Check specific file or directory
biome check src/components/

# Show detailed diagnostics with code frames
biome check --verbose .
```

### Step 2: Apply safe fixes automatically

```bash
# Apply all safe fixes (formatting + safe lint fixes)
biome check --write .

# Apply only safe lint fixes
biome lint --write .

# Apply only formatting fixes
biome format --write .
```

### Step 3: Address remaining unsafe issues manually

For each remaining error, understand the rule and fix properly:

## Rule Categories and Fix Strategies

### Formatting Rules

**Issues**: Incorrect indentation, spacing, line breaks, quote style

**Fix**: Use `biome format --write` or adjust manually:

- Follow project's `biome.json` formatter settings
- Use spaces/tabs consistently per config
- Maintain consistent line ending style

### Correctness Rules (High Priority)

These catch actual bugs - **always fix immediately**:

**noUnusedVariables** - Remove unused variables/imports or use them

```typescript
// BAD
const unused = 5; // Never used

// GOOD
const used = 5;
console.log(used);
```

**noUnreachable** - Remove unreachable code after return/throw

```typescript
// BAD
function foo() {
  return 1;
  console.log("never reached"); // Remove this
}

// GOOD
function foo() {
  return 1;
}
```

**noUndeclaredVariables** - Declare variables or import them

```typescript
// BAD
console.log(undeclaredVar); // Not defined

// GOOD
const declaredVar = "value";
console.log(declaredVar);
```

**noDebugger** - Remove `debugger` statements before committing

```typescript
// BAD
function process() {
  debugger; // Remove
  return data;
}

// GOOD
function process() {
  return data;
}
```

### Suspicious Rules

These indicate likely bugs or problematic patterns:

**noExplicitAny** - Use specific types instead of `any`

```typescript
// BAD
function process(data: any) { ... }

// GOOD
interface Data { id: string; value: number }
function process(data: Data) { ... }
// Or use unknown with type guards
function process(data: unknown) {
  if (typeof data === 'string') { ... }
}
```

**noArrayIndexKey** - Use stable unique IDs for React keys

```typescript
// BAD
items.map((item, index) => <div key={index} />)

// GOOD
items.map((item) => <div key={item.id} />)
```

**noDoubleEquals** - Use strict equality (=== !==)

```typescript
// BAD
if (value == null) // type coercion

// GOOD
if (value === null || value === undefined)
// Or if intentional:
if (value == null) // Refactor to be explicit
```

**noConsoleLog** - Remove or replace console.log statements

```typescript
// BAD
console.log("debug"); // In production code

// GOOD
// Use proper logging library
// Or remove if temporary debugging
```

### Style Rules

**useTemplate** - Use template literals instead of string concatenation

```typescript
// BAD
const message = "Hello, " + name + "!";

// GOOD
const message = `Hello, ${name}!`;
```

**useConst** - Use const for variables that don't change

```typescript
// BAD
let x = 5; // Never reassigned

// GOOD
const x = 5;
```

**useSingleVarDeclarator** - Declare one variable per statement

```typescript
// BAD
const a = 1,
  b = 2,
  c = 3;

// GOOD
const a = 1;
const b = 2;
const c = 3;
```

**useNamingConvention** - Follow naming conventions

```typescript
// BAD (depending on config)
const my_variable = 1;
const MyVariable = 1;

// GOOD
const myVariable = 1; // camelCase
const MY_CONSTANT = 1; // CONST_CASE for constants
```

### Complexity Rules

**noForEach** - Use for-of loops for better performance and control

```typescript
// BAD
array.forEach((item) => { ... });

// GOOD
for (const item of array) { ... }
```

**noBannedTypes** - Avoid problematic types (String, Number, Boolean, Object, {})

```typescript
// BAD
function process(obj: Object) { ... }
function process(obj: {}) { ... }

// GOOD
function process(obj: Record<string, unknown>) { ... }
// Or use specific interfaces
interface Config { ... }
function process(obj: Config) { ... }
```

**useSimplifiedLogicExpression** - Simplify complex boolean logic

```typescript
// BAD
if (a === true) { ... }
if (b === false) { ... }

// GOOD
if (a) { ... }
if (!b) { ... }
```

### Performance Rules

**noAccumulatingSpread** - Avoid spread in reduce (creates new objects each iteration)

```typescript
// BAD (O(n²) complexity)
array.reduce((acc, item) => ({ ...acc, [item.key]: item }), {});

// GOOD (O(n) complexity)
const result = {};
for (const item of array) {
  result[item.key] = item;
}
```

**noDelete** - Use undefined assignment or Map/Set instead of delete

```typescript
// BAD
delete obj.property;

// GOOD
obj.property = undefined;
// Or use Map for dynamic keys
const map = new Map();
map.set("key", value);
map.delete("key"); // OK for Map
```

## Configuration-Specific Issues

### biome.json Not Found

Ensure `biome.json` or `biome.jsonc` exists in project root:

```json
{
  "$schema": "https://biomejs.dev/schemas/1.9.4/schema.json",
  "organizeImports": {
    "enabled": true
  },
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true
    }
  },
  "formatter": {
    "enabled": true,
    "indentStyle": "space",
    "indentWidth": 2
  }
}
```

### Import Organization

Run `biome check --write` to automatically organize imports. Manual organization:

- Group imports: external libs → internal absolute → relative
- Sort alphabetically within groups
- Remove unused imports

### File-Specific Issues

Some issues require project-level thinking:

**noGlobalAssign** - Don't modify global objects

```typescript
// BAD
Array.prototype.custom = () => {};
window.globalVar = 1;

// GOOD
// Extend via proper subclassing or utility functions
function customArrayMethod(array) { ... }
```

**noRestrictedGlobals** - Use allowed globals only

```typescript
// BAD
const name = "value"; // Uses global window.name
const status = 200; // Uses global window.status

// GOOD
const userName = "value";
const httpStatus = 200;
```

## Common Workflows

### Before Committing Code

```bash
# 1. Check everything
biome check .

# 2. Apply safe fixes
biome check --write .

# 3. Review remaining issues manually
biome check --verbose .

# 4. Fix each remaining issue (NO suppression comments!)
```

### CI/CD Integration

```bash
# In CI, fail on any issues (don't use --write)
biome check .

# Or with specific error formatting
biome check --error-on-warnings --reporter=github .
```

#### Debugging CI Failures Locally

When CI pipelines fail due to Biome errors, always replicate the issue locally first:

```bash
# Run the exact same command CI runs (check your .github/workflows/validate-pr.yml)
biome ci --reporter=github --diagnostic-level=error . --verbose

# Or if using biome check in CI:
biome check --error-on-warnings --reporter=github .

# Compare local results with CI output to identify environment differences
```

**Why run locally first?**

- Faster iteration than waiting for CI
- Can use `--write` flag to auto-fix issues locally
- Identifies environment-specific issues (e.g., Biome version mismatches, config resolution)
- Allows using `--verbose` for detailed diagnostics
- Prevents commit noise from trial-and-error fixes

**Common CI/Local discrepancies:**

1. **Different Biome versions** - Ensure local version matches CI: `bunx biome --version`
2. **Config not found** - CI might run from different working directory; use explicit `--config-path`
3. **Line ending differences** - Windows (CRLF) vs Linux (LF); configure `formatter.lineEnding` in biome.json
4. **File paths** - CI may check files you haven't modified; run on entire codebase locally: `biome check .`

**Steps to resolve CI failures:**

1. Run the same Biome command locally that failed in CI
2. Apply fixes with `biome check --write .` (or manual fixes for unsafe changes)
3. Verify all issues resolved: `biome check .`
4. Commit and push changes

### Large Refactors

```bash
# Format everything first
biome format --write .

# Fix safe linting issues
biome lint --write .

# Tackle remaining issues file by file
biome check --verbose src/specific-file.ts
```

## Emergency Recovery

If you encounter Biome errors that block work:

1. **Check if it's a configuration error**

   ```bash
   biome check --config-path=./biome.json --verbose
   ```

2. **Ensure Biome is up to date**

   ```bash
   npm update @biomejs/biome
   # or
   yarn upgrade @biomejs/biome
   ```

3. **Validate biome.json syntax**

   ```bash
   npx @biomejs/biome migrate --write
   ```

4. **Check for file encoding issues**
   - Ensure files are UTF-8 encoded
   - Check for BOM markers that might confuse parser

## Remember

✅ **DO**:

- Fix the underlying issue
- Use `biome check --write` for safe fixes
- Remove unused code
- Add proper types
- Simplify complex expressions
- Follow project conventions

❌ **NEVER**:

- Add `// biome-ignore` comments
- Use `// biome-ignore lint` suppressions
- Disable rules globally to avoid fixing issues
- Commit code with intentional Biome violations

## Getting More Help

For specific rule documentation:

- Run `biome explain <RULE_NAME>` (e.g., `biome explain noDebugger`)
- Visit https://biomejs.dev/linter/rules/
- Check error messages for specific guidance
- Reference: https://biomejs.dev/reference/diagnostics/
