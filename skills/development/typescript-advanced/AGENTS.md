# typescript-advanced

## Overview

Comprehensive TypeScript guidance covering compiler configuration, advanced types, utilities, type narrowing, best practices, and documentation patterns.

## Structure

```
typescript-advanced/
  SKILL.md       # Main navigation hub
  AGENTS.md      # This file - complete reference guide
  references/    # Detailed reference files by category
```

## Usage

1. Read `SKILL.md` to understand when to apply TypeScript patterns
2. Navigate to categories based on your task
3. Load reference files on-demand
4. Follow workflow: practices → compiler → types → utilities → narrowing → docs

## Reference Categories

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| CRITICAL | Compiler & Configuration | Essential foundation | `compiler-` |
| CRITICAL | Best Practices & Patterns | Type-first workflow | `practices-` |
| HIGH | Advanced Types | Complex type logic | `types-advanced-` |
| HIGH | Built-in Utilities | Common transformations | `utilities-builtin-` |
| MEDIUM | Custom Utilities | Specialized types | `utilities-custom-` |
| MEDIUM | Type Narrowing | Runtime type guards | `narrowing-` |
| MEDIUM | Documentation | API docs & ADRs | `docs-` |

## Available References

**Compiler & Configuration** (`compiler-`):
- `references/compiler-tsconfig.md` - tsconfig.json structure and options
- `references/compiler-strict-mode.md` - Strict mode flags and benefits
- `references/compiler-module-resolution.md` - Module resolution strategies
- `references/compiler-performance.md` - Optimizing type checking speed

**Best Practices & Patterns** (`practices-`):
- `references/practices-type-first.md` - Type-first development workflow
- `references/practices-illegal-states.md` - Making illegal states unrepresentable
- `references/practices-discriminated-unions.md` - Discriminated union patterns
- `references/practices-branded-types.md` - Branded types for domain primitives
- `references/practices-zod-validation.md` - Runtime validation with Zod
- `references/practices-functional-patterns.md` - Functional programming patterns
- `references/practices-exhaustive-checks.md` - Exhaustive switch with never

**Advanced Types** (`types-advanced-`):
- `references/types-advanced-union.md` - Union types and discriminated unions
- `references/types-advanced-intersection.md` - Intersection types
- `references/types-advanced-conditional.md` - Conditional types with infer
- `references/types-advanced-mapped.md` - Mapped types and key remapping
- `references/types-advanced-template-literal.md` - Template literal types
- `references/types-advanced-generics.md` - Generics and constraints
- `references/types-advanced-inference.md` - Type inference techniques

**Built-in Utilities** (`utilities-builtin-`):
- `references/utilities-builtin-partial.md` - Partial<T>, Required<T>
- `references/utilities-builtin-pick-omit.md` - Pick<T, K>, Omit<T, K>
- `references/utilities-builtin-record.md` - Record<K, T>
- `references/utilities-builtin-return-type.md` - ReturnType<T>, Parameters<T>
- `references/utilities-builtin-awaited.md` - Awaited<T> for promises

**Custom Utilities** (`utilities-custom-`):
- `references/utilities-custom-deep-partial.md` - Recursive utility types
- `references/utilities-custom-branded.md` - Branded types for type safety
- `references/utilities-custom-builder.md` - Builder pattern types

**Type Narrowing** (`narrowing-`):
- `references/narrowing-typeof.md` - typeof type guards
- `references/narrowing-instanceof.md` - instanceof type guards
- `references/narrowing-custom-guards.md` - Custom type guard functions
- `references/narrowing-discriminated-unions.md` - Exhaustive checking

**Documentation** (`docs-`):
- `references/docs-jsdoc-patterns.md` - JSDoc best practices and patterns
- `references/docs-typedoc-config.md` - TypeDoc configuration and setup
- `references/docs-adr-template.md` - Architectural Decision Records
- `references/docs-framework-nestjs.md` - NestJS documentation patterns
- `references/docs-framework-react.md` - React component documentation
- `references/docs-framework-angular.md` - Angular documentation with Compodoc
- `references/docs-api-generation.md` - API documentation generation pipeline

---

*Consolidates 5 original skills: typescript-type-system, typescript-utility-types, typescript-advanced-types, typescript-best-practices, typescript-docs*

*48 reference files across 7 categories*
