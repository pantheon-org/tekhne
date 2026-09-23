---
name: typescript-advanced-utility-types
description: TypeScript's built-in utility types and how to build custom ones — Partial/Required, Pick/Omit, Readonly/Record, Extract/Exclude, NonNullable/Awaited, ReturnType/Parameters/ConstructorParameters/InstanceType, key remapping with `as` in mapped types, and hand-rolled custom mapped types. Use when picking the right built-in utility type instead of hand-writing one, extracting a function's parameter or return type, filtering a union with Extract/Exclude, or remapping object keys.
allowed-tools: Read, Write, Edit, Bash
---

# TypeScript Utility Types

The standard library of type transformations: which built-in utility type fits a given shape change, and how to build a custom one when none does.

## Mindset

Reach for a built-in utility type before writing a custom one — `Partial<T>`, `Pick<T, K>`, `ReturnType<typeof fn>` and their relatives cover the overwhelming majority of shape transformations, and a hand-rolled equivalent is one more thing to keep in sync with the type it derives from. When a built-in genuinely doesn't fit (a transform needs to recurse into nested objects, or apply a key-by-key rename), build the custom version as a mapped type over the real source type rather than a parallel interface maintained by hand. Combine utilities rather than inventing a new named type for every combination: `Partial<Pick<T, K>>` is more honest about what it does than a bespoke `PartialSubset<T, K>` that hides the same composition behind a name.

## When to Apply

Use this skill when:

- Making some or all properties of a type optional or required (`Partial`, `Required`)
- Selecting or excluding specific properties (`Pick`, `Omit`)
- Making a type read-only or building a lookup type (`Readonly`, `Record`)
- Filtering members out of or into a union (`Extract`, `Exclude`)
- Stripping `null`/`undefined` or unwrapping a `Promise` (`NonNullable`, `Awaited`)
- Extracting a function's return type, parameters, constructor parameters, or instance type
- Remapping object keys in a mapped type with an `as` clause
- Building a custom mapped type because no built-in utility fits

## Use When

- "Is there a built-in utility type for this, or do I need to write my own?"
- "How do I extract just the parameter types from this function signature?"
- "How do I make every property in this type optional except one?"
- "How do I rename or filter keys while mapping over a type?"

## Scope

### In Scope

- Every TypeScript built-in utility type (`Partial`, `Required`, `Pick`, `Omit`, `Readonly`, `Record`, `Extract`, `Exclude`, `NonNullable`, `Awaited`, `ReturnType`, `Parameters`, `ConstructorParameters`, `InstanceType`, `ThisParameterType`, `OmitThisParameter`).
- Key remapping in mapped types (`as` clauses, key filtering).
- Custom mapped types for transforms no built-in covers.

### Out of Scope

- The general mapped-type and conditional-type mechanics these utilities are built from — see the sibling `typescript-type-system` skill.
- Deep/recursive variants (`DeepReadonly`, `DeepPartial`) that recurse into nested objects — see `deep-readonly.md` in the sibling `typescript-design-patterns` skill.
- Runtime type guards — see the sibling `typescript-type-guards` skill.

## When NOT to Use

Do not use this skill to look up how mapped types or conditional types work mechanically — it assumes that grounding and focuses on the ready-made utilities built from it. For a *recursive* deep-readonly/deep-partial that walks nested objects, use `typescript-design-patterns`' `deep-readonly.md` instead — the utilities here are shallow, one level deep, matching TypeScript's actual built-ins.

## Quick Commands

### Type Check

```bash
npx tsc --noEmit
```

## Anti-Patterns

### NEVER hand-write a type that a built-in utility already provides

**WHY:** a hand-written equivalent of `Partial<T>` or `Pick<T, K>` doesn't stay in sync when the source type changes, while the built-in always reflects the current shape.

**BAD**:

```typescript
interface UserUpdate {
  name?: string;
  email?: string;
  age?: number;
} // duplicates User, optional — drifts if User changes
```

**GOOD**:

```typescript
type UserUpdate = Partial<User>;
```

### NEVER use `ReturnType<fn>` when you mean `ReturnType<typeof fn>`

**WHY:** `fn` the value and `typeof fn` the type are different things in a type position — passing the value's name directly is a type error, not a silent no-op, but it's a common enough slip to call out explicitly.

**BAD**:

```typescript
function getUser() { return { id: 1 }; }
type User = ReturnType<getUser>; // error: 'getUser' refers to a value
```

**GOOD**:

```typescript
type User = ReturnType<typeof getUser>;
```

### NEVER invent a bespoke named type for a simple composition of built-ins

**WHY:** a name like `PartialSubset<T, K>` hides `Partial<Pick<T, K>>` behind an extra layer a reader has to look up, for no gain over writing the composition directly.

**BAD**:

```typescript
type PartialSubset<T, K extends keyof T> = Partial<Pick<T, K>>;
// used once, adds an indirection with no new meaning
```

**GOOD**:

```typescript
type UserPatch = Partial<Pick<User, "name" | "email">>;
```

## References

| File | Covers |
| --- | --- |
| `references/partial-required.md` | `Partial<T>`, `Required<T>` |
| `references/pick-omit.md` | `Pick<T, K>`, `Omit<T, K>` |
| `references/readonly-record.md` | `Readonly<T>`, `Record<K, T>` |
| `references/extract-exclude.md` | `Extract<T, U>`, `Exclude<T, U>` |
| `references/nonnullable-awaited.md` | `NonNullable<T>`, `Awaited<T>` |
| `references/returntype-parameters.md` | `ReturnType`, `Parameters`, `ConstructorParameters`, `InstanceType`, `ThisParameterType`, `OmitThisParameter` |
| `references/key-remapping.md` | Key remapping with `as` in mapped types |
| `references/custom-mapped-types.md` | Building a custom mapped type when no built-in fits |

- [TypeScript Handbook: Utility Types](https://www.typescriptlang.org/docs/handbook/utility-types.html)
