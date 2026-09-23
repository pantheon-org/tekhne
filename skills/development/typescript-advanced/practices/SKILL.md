---
name: typescript-advanced-practices
description: Type-first TypeScript coding practices — designing types before implementation, making illegal states unrepresentable with discriminated unions, runtime validation with Zod at system boundaries, and module organization (single-function modules, barrel files, encapsulation). Use when starting a new feature by asking what the types should look like first, spotting an interface with too many optional fields that let impossible combinations compile, or deciding how to validate data coming from outside the type system (an API response, form input, environment variables).
allowed-tools: Read, Write, Edit, Bash
---

# TypeScript Practices

Workflow-level habits: design types first, make bad states impossible to construct, validate at the boundary where untyped data enters, and organize modules so the type-first discipline stays easy to keep up.

## Mindset

Design the types before writing the implementation — a type is a specification the compiler checks on every subsequent change, and writing it first forces the impossible states question to be answered up front rather than discovered later as a bug. An interface with many optional fields is usually a discriminated union that hasn't been modeled yet: if `data` and `error` can theoretically both be set, that combination will eventually occur, and the fix is a type that excludes it, not a runtime check that catches it after the fact. Runtime validation belongs at the boundary — the one place untyped data (an API response, form input, an environment variable) enters the typed world — not scattered through the code that consumes it; validate once there, and let the type system carry the guarantee everywhere downstream.

## When to Apply

Use this skill when:

- Starting a new feature and deciding what the types should look like before writing implementation
- Spotting an interface with many optional fields that allow an impossible combination to compile
- Validating data at a system boundary (API responses, form input, environment variables) with Zod
- Deciding how to organize a module's exports (single-function modules, barrel files, internal encapsulation)

## Use When

- "Where do I even start typing this feature?"
- "This interface has five optional fields — some combinations don't make sense. How do I fix that?"
- "How do I validate this API response is actually shaped the way I expect?"
- "Should this be one big module or several small ones?"

## Scope

### In Scope

- Type-first development workflow.
- Making illegal states unrepresentable with discriminated unions.
- Runtime validation with Zod at system boundaries.
- Module organization patterns (single-function modules, barrel exports, encapsulation).

### Out of Scope

- The type-guard/predicate mechanics used to validate at a boundary — see the sibling `typescript-type-guards` skill for hand-written predicates.
- The discriminated-union *type* mechanics themselves — see the sibling `typescript-type-system` skill; this skill covers the practice of reaching for one, not how unions narrow.
- Compiler configuration — see the sibling `typescript-compiler-config` skill.

## When NOT to Use

Do not use this skill to look up narrowing syntax or how a discriminated union's exhaustiveness check works mechanically — see `typescript-type-guards` for that. Do not use it as a substitute for `tsc --noEmit` — a type-first design still needs the compiler to confirm every consumer respects it.

## Quick Commands

### Type Check

```bash
npx tsc --noEmit
```

### Validate at a Boundary (Zod)

```bash
npm install zod
```

## Anti-Patterns

### NEVER model a set of mutually exclusive fields as independent optionals

**WHY:** independent optional fields let combinations compile that should be impossible, so the invalid combination becomes a runtime bug instead of a compile error.

**BAD**:

```typescript
interface FetchState<T> {
  isLoading: boolean;
  data?: T;
  error?: Error; // data and error can both be set — impossible in practice, allowed by the type
}
```

**GOOD**:

```typescript
type FetchState<T> =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "done"; data: T }
  | { status: "done"; error: Error };
```

### NEVER trust external data without validating it at the boundary

**WHY:** a type annotation on `fetch().json()` is a claim, not a check — the actual runtime value could be anything, and nothing enforces the annotation matches reality.

**BAD**:

```typescript
async function getUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  return res.json(); // asserted to be User, never verified
}
```

**GOOD**:

```typescript
const UserSchema = z.object({ id: z.string(), name: z.string() });

async function getUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  return UserSchema.parse(await res.json());
}
```

### NEVER let implementation details leak past a module's intended public surface

**WHY:** exporting internals a module wasn't designed to expose removes the compiler's ability to catch a caller depending on something that was meant to change freely.

**BAD**:

```typescript
// db.ts
export let connection: Connection; // internal state exported directly
export function connect() { connection = createConnection(); }
```

**GOOD**:

```typescript
// db.ts
let connection: Connection;
export function connect() { connection = createConnection(); }
export function getConnection(): Connection {
  if (!connection) throw new Error("Not connected");
  return connection;
}
```

## References

| File | Covers |
| --- | --- |
| `references/type-first.md` | Type-first development workflow |
| `references/illegal-states.md` | Making illegal states unrepresentable with discriminated unions |
| `references/runtime-validation.md` | Zod-based validation at system boundaries |
| `references/module-patterns.md` | Single-function modules, barrel exports, encapsulation |

- [Zod Documentation](https://zod.dev/)
- [Parse, Don't Validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
