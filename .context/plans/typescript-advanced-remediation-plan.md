---
plan_date: 2026-02-23
skill_name: typescript-advanced
source_audit: .context/audits/typescript-advanced-audit-2026-02-22.md
---

# Remediation Plan: typescript-advanced

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 87/120 (72%) |
| **Current Grade** | C |
| **Target Score** | 100+/120 (B+ or higher) |
| **Priority** | High - Priority improvements required |
| **Estimated Effort** | Medium (3-4 hours) |

## Critical Issues to Address

| Issue | Severity | Dimension | Current | Target |
| --- | --- | --- | ---: | ---: |
| Weak anti-pattern quality | High | D3 | 8/15 | 13/15 |
| Weak practical usability | High | D8 | 8/15 | 13/15 |
| Weak pattern recognition | High | D7 | 6/10 | 9/10 |
| Moderate specification compliance | Medium | D4 | 10/15 | 13/15 |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - High Priority

**Goal**: Increase D3 score from 8/15 to 13/15

**File**: `skills/typescript-advanced/SKILL.md`

**Step 1.1**: Add explicit anti-patterns with NEVER/WHY/BAD-GOOD

````markdown
## Anti-Patterns

### NEVER Use `any` Type

- **WHY**: `any` disables TypeScript's type checking completely.
- **Consequence**: Loss of type safety, runtime errors, no IDE support.
- **BAD**: `function process(data: any) { return data.value; }`
- **GOOD**: `function process<T extends { value: unknown }>(data: T) { return data.value; }`

### NEVER Use Type Assertions to Silence Errors

- **WHY**: Assertions bypass the compiler without runtime checks.
- **Consequence**: Hidden type errors, unexpected runtime failures.
- **BAD**: `const value = data as string;`
- **GOOD**: `const value = typeof data === 'string' ? data : throw new TypeError('Expected string');`

### NEVER Create Unused Type Parameters

- **WHY**: Unused type parameters add complexity without benefit.
- **Consequence**: Confusing API, harder to understand intent.
- **BAD**: `interface Container<T> { value: string; }` // T is unused
- **GOOD**: `interface Container<T> { value: T; }`

### NEVER Over-Nest Conditional Types

- **WHY**: Deep nesting makes types unreadable and slow to compile.
- **Consequence**: Long compile times, hard to debug type errors.
- **BAD**: `type X = A extends B ? C extends D ? E extends F ? G : H : I : J;`
- **GOOD**: Break into named intermediate types:
  ```typescript
  type Inner = E extends F ? G : H;
  type Middle = C extends D ? Inner : I;
  type X = A extends B ? Middle : J;
  ```

### NEVER Ignore Strict Mode Errors

- **WHY**: Strict mode catches real bugs before runtime.
- **Consequence**: Null reference errors, undefined behavior.
- **BAD**: Adding `// @ts-ignore` or disabling strict mode.
- **GOOD**: Fix the underlying type issue.

### NEVER Use Implicit Any in Callbacks

- **WHY**: Callbacks without types default to `any`.
- **Consequence**: Loss of type inference in callback bodies.
- **BAD**: `items.map(item => item.value)`
- **GOOD**: `items.map((item: Item) => item.value)`

### NEVER Create Cyclical Type Dependencies

- **WHY**: TypeScript cannot resolve circular type references.
- **Consequence**: Compile errors, infinite type expansion.
- **BAD**: `type A = { b: B }; type B = { a: A };`
- **GOOD**: Use interfaces with import types: `type A = { b: import('./b').B };`
````

---

### Phase 2: Practical Usability (D8) - High Priority

**Goal**: Increase D8 score from 8/15 to 13/15

**File**: `skills/typescript-advanced/SKILL.md`

**Step 2.1**: Add executable commands section

````markdown
## Quick Commands

### Type Checking

```bash
# Run type check
npx tsc --noEmit

# Type check specific file
npx tsc --noEmit src/types/index.ts

# Generate declaration files
npx tsc --declaration --emitDeclarationOnly
```

### Type Analysis

```bash
# Show type of expression (requires ts-node)
npx ts-node -e "console.log(typeof myVariable)"

# Trace type resolution
npx tsc --traceResolution
```

### Utility Type Inspection

```bash
# Create a type test file
cat > type-test.ts << 'EOF'
import type { MyComplexType } from './types';
type Test = MyComplexType;
// Hover over Test to see resolved type
EOF
```

### Documentation Generation

```bash
# Generate TypeDoc documentation
npx typedoc --out docs src/index.ts
```
````

**Step 2.2**: Add verification checklist

```markdown
## Verification Checklist

After modifying TypeScript types:

- [ ] `npx tsc --noEmit` passes with no errors
- [ ] No `any` types in new code (unless explicitly justified)
- [ ] All generics have type parameters used
- [ ] Complex types have JSDoc comments
- [ ] Type exports are documented in index.ts
- [ ] No circular type dependencies
- [ ] Utility types are preferred over repetition
```

**Step 2.3**: Add common patterns with copy-paste examples

````markdown
## Common Patterns

### Branded Types for Type-Safe IDs

```typescript
// Definition
type Brand<T, B> = T & { __brand: B };
type UserId = Brand<string, 'UserId'>;
type OrderId = Brand<string, 'OrderId'>;

// Usage
function getUser(id: UserId): User { /* ... */ }
function getOrder(id: OrderId): Order { /* ... */ }

// Creation (requires explicit cast at boundary)
const userId = 'user-123' as UserId;
const orderId = 'order-456' as OrderId;

getUser(orderId); // Error: OrderId is not assignable to UserId
```

### Exhaustive Matching with Discriminated Unions

```typescript
type Shape = 
  | { kind: 'circle'; radius: number }
  | { kind: 'square'; size: number }
  | { kind: 'rectangle'; width: number; height: number };

function area(shape: Shape): number {
  switch (shape.kind) {
    case 'circle': return Math.PI * shape.radius ** 2;
    case 'square': return shape.size ** 2;
    case 'rectangle': return shape.width * shape.height;
    default: const _exhaustive: never = shape; // Ensures all cases handled
  }
}
```

### Type-Safe Event Emitters

```typescript
type EventMap = {
  userCreated: { id: string; name: string };
  userDeleted: { id: string };
};

function emit<K extends keyof EventMap>(
  event: K, 
  data: EventMap[K]
): void {
  // Implementation
}
```
````

---

### Phase 3: Pattern Recognition (D7) - High Priority

**Goal**: Increase D7 score from 6/10 to 9/10

**File**: `skills/typescript-advanced/SKILL.md`

**Step 3.1**: Enhance frontmatter description

```yaml
---
name: typescript-advanced
description: |
  Comprehensive TypeScript guidance covering compiler configuration, advanced types, 
  utility types, type guards, best practices, and documentation.
  
  Use when: configuring tsconfig, working with complex types, implementing type-safe patterns,
  making illegal states unrepresentable, or generating API documentation.
  
  Triggers: "tsconfig", "strict mode", "generics", "conditional types", "mapped types", 
  "utility types", "Partial", "Pick", "Omit", "ReturnType", "type guards", "typeof",
  "instanceof", "type narrowing", "template literal types", "type-first development",
  "JSDoc", "TypeDoc", "Zod validation", "discriminated unions", "branded types".
---
```

**Step 3.2**: Add trigger phrases section

```markdown
## When to Use This Skill

### Explicit Triggers
- "How to configure tsconfig"
- "TypeScript strict mode"
- "Create a generic type"
- "Use utility types"
- "Type guard patterns"
- "Make illegal states unrepresentable"
- "Template literal types"
- "Discriminated unions"

### Implicit Triggers
- User mentions type errors they don't understand
- User is designing a type-safe API
- User needs runtime type validation (suggest Zod)
- User is working with complex type inference
- User mentions `any` or type casting
```

---

### Phase 4: Specification Compliance (D4) - Medium Priority

**Goal**: Increase D4 score from 10/15 to 13/15

**File**: `skills/typescript-advanced/SKILL.md`

**Step 4.1**: Ensure complete frontmatter

```yaml
---
name: typescript-advanced
description: |
  Comprehensive TypeScript guidance covering compiler configuration, advanced types, 
  utility types, type guards, best practices, and documentation.
  
  Use when: configuring tsconfig, working with complex types, implementing type-safe patterns.
keywords:
  - TypeScript
  - tsconfig
  - strict mode
  - generics
  - conditional types
  - mapped types
  - utility types
  - type guards
  - discriminated unions
  - branded types
---
```

**Step 4.2**: Add explicit scope

```markdown
## Scope

### In Scope
- tsconfig.json configuration
- Advanced type constructs (generics, conditionals, mapped)
- Utility type patterns
- Type guard implementation
- Runtime validation integration (Zod)
- TypeDoc generation
- Type-safe design patterns

### Out of Scope
- JavaScript fundamentals
- React TypeScript patterns (separate skill)
- Node.js TypeScript patterns (separate skill)
- Build tool configuration (webpack, vite)
```

---

## Verification Commands

```bash
# Run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh typescript-advanced --json

# Verify no any types in skill examples
grep -r "any" skills/typescript-advanced/SKILL.md | grep -v "NEVER Use"

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills

# Validate markdown
bunx markdownlint-cli2 "skills/typescript-advanced/**/*.md"
```

## Success Criteria

| Criterion | Metric |
| --- | --- |
| Total Score | >= 100/120 |
| Grade | B+ or higher |
| D3: Anti-Pattern Quality | >= 13/15 |
| D8: Practical Usability | >= 13/15 |
| D7: Pattern Recognition | >= 9/10 |
| D4: Specification Compliance | >= 13/15 |
| Anti-patterns with BAD/GOOD | >= 6 |
| Executable commands | >= 5 |

## Timeline

| Phase | Duration | Dependencies |
| --- | --- | --- |
| Phase 1: Anti-Patterns | 1.5 hours | None |
| Phase 2: Practical Usability | 1.5 hours | Phase 1 |
| Phase 3: Pattern Recognition | 1 hour | None |
| Phase 4: Specification Compliance | 30 min | None |
| Verification | 30 min | All phases |

**Total Estimated Time**: 4.5 hours
