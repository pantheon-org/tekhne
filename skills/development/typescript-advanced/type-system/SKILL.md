---
name: typescript-advanced-type-system
description: Advanced TypeScript type-system mechanics — union and intersection types, conditional types with `infer`, mapped types, template literal types, generics and their constraints, index signatures, type assertions, and extracting types from other types (return types, array elements, promise results). Use when designing a complex generic, modeling a type that transforms based on another type, extracting a type from a function or structure with `infer`, or deciding between a type assertion and a real narrowing check.
allowed-tools: Read, Write, Edit, Bash
---

# TypeScript Type System

The type-level mechanics that everything else in this skill family builds on: unions, conditionals, mapped types, template literals, generics, and type-level extraction with `infer`.

## Mindset

Model the problem in types before reaching for a runtime check — a type that makes an invalid combination unrepresentable is a compile-time guarantee, while a runtime check for the same thing only protects the paths that get exercised. Prefer deriving a type from another type (`ReturnType<typeof fn>`, a mapped type, a conditional extraction with `infer`) over hand-writing a parallel type that can drift out of sync with the thing it describes. When a conditional type distributes over a union unexpectedly, that is usually the type system doing exactly what it's told — wrap the checked type in a tuple (`[T] extends [U]`) to opt out of distribution rather than restructuring the type to dodge it. A type assertion (`as`) is a claim, not a check; only reach for one when you have already established the fact some other way (e.g. immediately after a runtime guard), never as a way to silence an error you haven't investigated.

## When to Apply

Use this skill when:

- Modeling a union or intersection type, including discriminated unions at the type-definition level
- Writing a conditional type, especially one using `infer` to extract a nested type
- Building a mapped type or remapping its keys with an `as` clause
- Constructing a template literal type (e.g. typed route parameters)
- Designing a generic function or type with meaningful constraints
- Extracting a type from a function, array, promise, or constructor
- Deciding whether a value needs an index signature or a fixed shape
- Choosing between a type assertion and a real narrowing check

## Use When

- "How do I extract the return type / element type / awaited type from this?"
- "How should I model this union or generic type?"
- "How do I build a type that changes shape based on another type?"
- "What's the difference between `as` and a type guard here?"

## Scope

### In Scope

- Union, intersection, conditional, mapped, and template literal types.
- Generics and their constraints.
- Index signatures and type assertions.
- Type-level extraction with `infer` (return types, array elements, promise values, tuple destructuring, constructor/instance types).

### Out of Scope

- Runtime narrowing and type guards (`typeof`, `instanceof`, custom predicates) — see the sibling `typescript-type-guards` skill, which also owns exhaustiveness checking.
- Built-in utility types that wrap these mechanics for common cases (`Partial`, `Pick`, `ReturnType` as a ready-made utility) — see the sibling `typescript-utility-types` skill for the applied, ready-to-use versions.
- Design patterns that use these mechanics structurally — see the sibling `typescript-design-patterns` skill.

## When NOT to Use

Do not use this skill for a runtime check — narrowing a value at runtime with `typeof`/`instanceof`/a custom predicate belongs in `typescript-type-guards`. Do not use it as a substitute for `tsc --noEmit`; a type that looks correct in isolation can still fail to apply as expected against real call sites.

## Quick Commands

### Type Check

```bash
npx tsc --noEmit
```

### Type Check Single Entry

```bash
npx tsc --noEmit src/index.ts
```

## Anti-Patterns

### NEVER silence a type error with an unchecked assertion instead of modeling the type correctly

**WHY:** an `as` assertion bypasses the compiler without a matching runtime guarantee; the error it silences almost always points at a type that needs to be modeled more precisely, not asserted away.

**BAD**:

```typescript
const id = getValue() as string;
```

**GOOD**:

```typescript
type Result = { kind: "ok"; value: string } | { kind: "error"; message: string };
function unwrap(result: Result): string {
  if (result.kind === "error") throw new Error(result.message);
  return result.value;
}
```

### NEVER hand-write a type that duplicates what `infer` or a mapped type could derive

**WHY:** a hand-written parallel type drifts out of sync with the source it was copied from the first time either one changes.

**BAD**:

```typescript
function getUser() { return { id: 1, name: "Alice" }; }
interface User { id: number; name: string; } // duplicates getUser's actual return shape
```

**GOOD**:

```typescript
function getUser() { return { id: 1, name: "Alice" }; }
type User = ReturnType<typeof getUser>; // stays correct if getUser changes
```

### NEVER leave a generic parameter unconstrained when the type only makes sense for a shape

**WHY:** an unconstrained `<T>` accepts anything, including values the type's own logic assumes won't appear.

**BAD**:

```typescript
type PropType<T, K> = T extends { [P in K]: infer V } ? V : never; // K unconstrained
```

**GOOD**:

```typescript
type PropType<T, K extends keyof T> = T extends { [P in K]: infer V } ? V : never;
```

## References

| File | Covers |
| --- | --- |
| `references/unions-intersections.md` | Union and intersection types |
| `references/conditional-types.md` | Conditional types, `infer`, distributive behavior |
| `references/mapped-types.md` | Mapped types and key remapping mechanics |
| `references/template-literal-types.md` | Template literal types (e.g. typed route parameters) |
| `references/generics.md` | Generic constraints and type parameters |
| `references/index-signatures.md` | Index signatures and dynamic properties |
| `references/type-assertions.md` | `as` assertions and type compatibility |
| `references/infer-extraction.md` | Extracting return types, parameters, array elements, promise values, tuple elements, and instance/`this` types with `infer` |

- [TypeScript Handbook: Conditional Types](https://www.typescriptlang.org/docs/handbook/2/conditional-types.html)
- [TypeScript Handbook: Mapped Types](https://www.typescriptlang.org/docs/handbook/2/mapped-types.html)
- [TypeScript Handbook: Template Literal Types](https://www.typescriptlang.org/docs/handbook/2/template-literal-types.html)
