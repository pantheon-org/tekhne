---
name: typescript-advanced
description: |-
  Comprehensive TypeScript guidance covering compiler configuration, advanced types, utility types, type guards, best practices, and documentation. Use when: configuring tsconfig, working with complex types, implementing type-safe patterns, making illegal states unrepresentable, or generating API documentation.
  
  Keywords: TypeScript, tsconfig, strict mode, generics, conditional types, mapped types, utility types, Partial, Pick, Omit, ReturnType, type guards, typeof, instanceof, type narrowing, template literal types, type-first development, JSDoc, TypeDoc, Zod validation, discriminated unions, branded types
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

## Consolidation Note

This skill consolidates **5 original TypeScript skills** (~3,372 lines) into a **~120-line navigation hub** with on-demand reference loading:

- **typescript-type-system** (738 lines) - Compiler config, strict mode, module resolution
- **typescript-utility-types** (832 lines) - Built-in and custom utility types
- **typescript-advanced-types** (724 lines) - Generics, conditional types, mapped types, patterns
- **typescript-best-practices** (270 lines) - Type-first development, illegal states, Zod validation
- **typescript-docs** (808 lines) - JSDoc, TypeDoc, ADRs, framework-specific documentation

## Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| CRITICAL | Compiler & Configuration | Essential foundation | `compiler-` |
| CRITICAL | Best Practices & Patterns | Type-first workflow | `practices-` |
| HIGH | Advanced Types | Complex type logic | `types-advanced-` |
| HIGH | Built-in Utilities | Common transformations | `utilities-builtin-` |
| MEDIUM | Custom Utilities | Specialized types | `utilities-custom-` |
| MEDIUM | Type Narrowing | Runtime type guards | `narrowing-` |
| MEDIUM | Documentation | API docs & ADRs | `docs-` |

## How to Use

Read individual reference files for detailed guidance:

```
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

## References

- https://www.typescriptlang.org/docs/handbook/
- https://www.typescriptlang.org/tsconfig
- https://www.typescriptlang.org/docs/handbook/utility-types.html
- https://github.com/type-challenges/type-challenges
