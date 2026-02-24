---
name: typescript-advanced
description: Comprehensive TypeScript guidance covering compiler configuration, advanced types, utility types, type guards, strict mode workflows, and documentation patterns; use when configuring tsconfig, designing complex generics, making illegal states unrepresentable, fixing type errors, or writing testable and maintainable type-safe APIs.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
---

# TypeScript Advanced

Comprehensive TypeScript guidance covering compiler configuration, advanced types, utilities, type narrowing, best practices, and documentation patterns.

## When to Apply

Use this skill when:

- Configuring TypeScript compiler (tsconfig.json)
- Working with advanced type features (generics, conditionals, mapped types)
- Creating custom utility types and type guards
- Implementing type-first development workflow
- Making illegal states unrepresentable with discriminated unions
- Implementing runtime validation with Zod
- Generating API documentation with JSDoc/TypeDoc
- Creating architectural decision records (ADRs)
- Debugging complex type errors
- Optimizing type checking performance

## Use When

- "Help me do this with strict TypeScript."
- "How should I model this union/generic type?"
- "How do I avoid `any` and unsafe assertions?"
- "How do I fix this TypeScript compile error safely?"

## Scope

### In Scope

- tsconfig and strict-mode guidance.
- Advanced type modeling (generics, conditionals, mapped types).
- Runtime narrowing and type-safe validation patterns.
- TypeScript documentation patterns (JSDoc/TypeDoc, ADRs).

### Out of Scope

- End-to-end testing setup.
- Framework build pipeline setup unrelated to TypeScript design.
- Language-agnostic coding standards.

## Consolidation Note

This skill consolidates **5 original TypeScript skills** (~3,372 lines) into a **~120-line navigation hub** with on-demand reference loading:

- **typescript-type-system** (738 lines) - Compiler config, strict mode, module resolution
- **typescript-utility-types** (832 lines) - Built-in and custom utility types
- **typescript-advanced-types** (724 lines) - Generics, conditional types, mapped types, patterns
- **typescript-best-practices** (270 lines) - Type-first development, illegal states, Zod validation
- **typescript-docs** (808 lines) - JSDoc, TypeDoc, ADRs, framework-specific documentation

## Categories by Priority

| Priority | Category | Impact | Prefix |
| --- | --- | --- | --- |
| CRITICAL | Compiler & Configuration | Essential foundation | `compiler-` |
| CRITICAL | Best Practices & Patterns | Type-first workflow | `practices-` |
| HIGH | Advanced Types | Complex type logic | `types-advanced-` |
| HIGH | Built-in Utilities | Common transformations | `utilities-builtin-` |
| MEDIUM | Custom Utilities | Specialized types | `utilities-custom-` |
| MEDIUM | Type Narrowing | Runtime type guards | `narrowing-` |
| MEDIUM | Documentation | API docs & ADRs | `docs-` |

## How to Use

Read individual reference files for detailed guidance:

```text
references/compiler-strict-mode.md
references/practices-type-first.md
references/types-advanced-conditional.md
references/utilities-builtin-partial.md
references/docs-jsdoc-patterns.md
```

Each reference file contains:

- Concepts and explanations
- Practical TypeScript examples
- Common patterns and use cases
- Best practices and pitfalls
- Performance considerations

## Navigation Workflow

1. **Start with practices** - Understand type-first development workflow
2. **Configure compiler** - Set up tsconfig with strict mode
3. **Apply advanced types** - Use generics, conditionals, mapped types
4. **Use utilities** - Leverage built-in and custom utility types
5. **Implement narrowing** - Add type guards for runtime safety
6. **Document** - Generate API docs with JSDoc/TypeDoc

## Quick Commands

### Type Check

```bash
npx tsc --noEmit
```

### Type Check Single Entry

```bash
npx tsc --noEmit src/index.ts
```

### Generate Docs

```bash
npx typedoc --out docs src/index.ts
```

### Find Unsafe Patterns

```bash
rg -n "\\bany\\b|@ts-ignore| as " src
```

## Anti-Patterns

### NEVER use `any` as a default escape hatch

**WHY**: `any` disables type checking and hides design bugs.

**BAD**:

```typescript
function process(data: any) {
  return data.value;
}
```

**GOOD**:

```typescript
function process<T extends { value: unknown }>(data: T) {
  return data.value;
}
```

### NEVER silence type errors with unchecked assertions

**WHY**: assertions bypass compiler safety without runtime guarantees.

**BAD**:

```typescript
const id = input as string;
```

**GOOD**:

```typescript
if (typeof input !== "string") throw new TypeError("Expected string");
const id = input;
```

### NEVER disable strict-mode findings instead of fixing model issues

**WHY**: strict mode surfaces real defects early.

**BAD**: `@ts-ignore`, `strict: false`, or broad ignore patterns.

**GOOD**: adjust type model, narrow correctly, and keep strict checks enabled.

## References

- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/)
- [TSConfig Reference](https://www.typescriptlang.org/tsconfig)
- [Utility Types](https://www.typescriptlang.org/docs/handbook/utility-types.html)
- [Type Challenges](https://github.com/type-challenges/type-challenges)
