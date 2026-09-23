---
name: typescript-advanced-type-guards
description: Runtime type guards, narrowing, and exhaustiveness checking for TypeScript — typeof/instanceof/in guards, custom and generic type predicates (value is T), discriminated union narrowing, assertion functions (asserts value is T), branded/nominal types validated at runtime, and exhaustive switch statements with a never guard. Use when replacing an unsafe `as` assertion with a real runtime check, fixing "Object is possibly undefined", modeling impossible states out of existence, writing an `is*` predicate, or making a switch over a union fail to compile when a case is missing.
allowed-tools: Read, Write, Edit, Bash
---

# TypeScript Type Guards

Runtime type guards and narrowing: the compile-time type system backed by an actual runtime check, so a type guarantee is real rather than asserted.

## Mindset

A type guard is a promise the compiler will hold you to only if the runtime check underneath it is honest. Never reach for `any` or an unchecked `as` assertion as a shortcut past a type error — both disable the exact safety net a guard exists to provide, and the error they suppress almost always points at a real design gap. Prefer narrowing a value before use over asserting its type after the fact: `if (typeof x !== "string") throw ...` gives the compiler a real fact to reason from; `x as string` gives it nothing but your word. Verify a narrowing actually narrows by re-running `tsc --noEmit` after the change — a guard that looks correct can still fail to narrow if its predicate doesn't match the shape TypeScript expects. Every union that models a real set of states deserves an exhaustive switch with a `never`-typed default branch, so adding a new state without updating every handler is a compile error, not a silent gap.

## When to Apply

Use this skill when:

- Replacing an unsafe `as` assertion with a runtime check that narrows for real
- Writing a custom type predicate (`function is*(x): x is T`)
- Modeling impossible field combinations out of existence with a discriminated union
- Adding an assertion function (`asserts value is T`) to validate and narrow in one call
- Creating branded/nominal types with a validating factory function
- Adding exhaustiveness checking to a switch over a union so new variants can't be silently unhandled
- Fixing "Object is possibly 'undefined'" without reaching for a non-null assertion (`!`)

## Use When

- "How do I fix this TypeScript compile error without an `as` assertion?"
- "How do I check the shape of an `unknown` value at runtime?"
- "How do I make my switch statement error if I add a new case and forget to handle it?"
- "How do I stop two IDs of the same primitive type from being mixed up?"

## Scope

### In Scope

- `typeof`, `instanceof`, `in`, truthiness, and equality narrowing.
- Custom and generic type predicate functions (`value is T`).
- Discriminated union narrowing and exhaustiveness checking with `never`.
- Assertion functions (`asserts condition`, `asserts value is T`).
- Branded/nominal types validated by a runtime type guard.

### Out of Scope

- The type-system mechanics behind a union or conditional type itself — see the sibling `typescript-type-system` skill.
- Zod-based schema validation as a library choice — see `runtime-validation.md` in the sibling `typescript-practices` skill; this skill covers hand-written predicates and assertion functions.
- General design patterns (builder, state machine, event emitter) that *use* guards — see the sibling `typescript-design-patterns` skill.

## When NOT to Use

Do not use this skill for compiler configuration (tsconfig, strict-mode flags — see `typescript-compiler-config`) or for utility-type mechanics like `Pick`/`Omit`/`ReturnType` (see `typescript-utility-types`). Do not use it as a substitute for actually running `tsc --noEmit` — the reference files teach the pattern, but the compiler is the source of truth on whether a guard actually narrows.

## Quick Commands

### Type Check

```bash
npx tsc --noEmit
```

### Find Unsafe Patterns

```bash
rg -n "\\bany\\b|@ts-ignore| as " src
```

## Quick Fixes

### "Object is possibly 'undefined'"

```typescript
// BAD — non-null assertion hides the real problem
function getHost(config: Config): string {
  return config.host!.toUpperCase();
}

// GOOD — guard before access
function getHost(config: Config): string {
  if (config.host === undefined) throw new Error("host is required");
  return config.host.toUpperCase();
}
```

### "Type X is not assignable to type Y"

```typescript
// BAD — forces an incompatible assignment
const id = getValue() as string;

// GOOD — narrow first, then assign
const raw = getValue();
if (typeof raw !== "string") throw new TypeError("Expected string");
const id = raw; // TypeScript now knows id: string
```

## Anti-Patterns

### NEVER use `any` as a default escape hatch

**WHY:** `any` disables type checking and hides the design bug the error was pointing at.

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

**WHY:** an `as` assertion bypasses compiler safety without a matching runtime guarantee.

**BAD**:

```typescript
const id = input as string;
```

**GOOD**:

```typescript
if (typeof input !== "string") throw new TypeError("Expected string");
const id = input;
```

### NEVER use numeric `enum` for a discriminated union tag

**WHY:** numeric enums generate a reverse mapping at runtime (`Direction[0] === "Up"`), inflate bundle size, and let any number be assigned where the enum type is expected — none of which a string-literal union has.

**BAD**:

```typescript
enum Status { Pending, Active, Closed }
function handle(status: Status) { /* Status.Pending accepts any number */ }
```

**GOOD**:

```typescript
type Status = "pending" | "active" | "closed";
function handle(status: Status) { /* only the three literal strings are valid */ }
```

### NEVER let a discriminant literal widen

**WHY:** `let` and object-literal inference widen `"success"` to `string`, which breaks exhaustive narrowing on the union's discriminant even though the runtime value never changes.

**BAD**:

```typescript
let kind = "success"; // inferred as string, not "success"
const result = { kind, data: 42 }; // result.kind: string — narrowing fails
```

**GOOD**:

```typescript
const kind = "success" as const;
const result = { kind, data: 42 } as const; // result.kind: "success"
```

### NEVER leave a switch over a union without an exhaustiveness check

**WHY:** without a `never`-typed default branch, adding a new union variant compiles silently and the new case is unhandled at runtime.

**BAD**:

```typescript
function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle": return Math.PI * shape.radius ** 2;
    case "square": return shape.size ** 2;
    // no default — adding "triangle" to Shape compiles without warning
  }
}
```

**GOOD**:

```typescript
function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle": return Math.PI * shape.radius ** 2;
    case "square": return shape.size ** 2;
    default:
      const _exhaustive: never = shape;
      throw new Error(`Unhandled shape: ${_exhaustive}`);
  }
}
```

## References

| File | Covers |
| --- | --- |
| `references/basic-guards.md` | `typeof`, `instanceof`, `in`, truthiness, and equality narrowing |
| `references/generic-guards.md` | Reusable generic type predicates (`isArrayOf`, `hasProperty`, `isInstance`) |
| `references/discriminated-unions.md` | Tagged unions, multiple discriminants, state-machine narrowing |
| `references/exhaustiveness-checking.md` | The `never` exhaustiveness pattern across switch, if/else, and pattern matching |
| `references/assertion-functions.md` | `asserts condition` and `asserts value is T` functions |
| `references/branded-types-guards.md` | Nominal types validated by a runtime predicate, multi-level brands, unwrap utilities |

- [TypeScript Handbook: Narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html)
- [TypeScript Handbook: Assertion Functions](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#assertion-functions)
