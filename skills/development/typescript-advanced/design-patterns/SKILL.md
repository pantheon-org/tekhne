---
name: typescript-advanced-design-patterns
description: TypeScript design patterns that use the type system to enforce correctness at compile time — the builder pattern with required-field tracking, type-safe state machines and event emitters, dependency injection, plugin systems, recursive/deep-readonly utility types, type-safe module encapsulation, advanced generic constraints, and typed API clients. Use when structuring a class or module so invalid usage fails to compile rather than throwing at runtime, e.g. "how do I stop `.build()` being called before required fields are set" or "how do I type an event emitter so payloads match their event name".
allowed-tools: Read, Write, Edit, Bash
---

# TypeScript Design Patterns

Structural patterns that push correctness into the type system: a class or module shaped so the compiler rejects a misuse the runtime would otherwise have to catch.

## Mindset

A pattern here earns its place only if it converts a runtime failure into a compile error. Before reaching for one, ask what invalid usage it should make impossible to write, not just cleaner to read — a builder that still lets `.build()` run with missing fields is a fluent API, not a type-safe one. Prefer composing the type system's own primitives (discriminated unions, conditional types, mapped types) over hand-rolled runtime checks wherever the invalid state can be excluded at the type level instead of merely caught. Treat generics as free specificity, not decoration: a generic parameter with no constraint is barely more useful than `any`, and a pattern that leaves one unconstrained has not finished its job.

## When to Apply

Use this skill when:

- Enforcing required fields at compile time with the builder pattern
- Modeling application or connection state as a type-safe state machine
- Typing an event emitter so payloads are checked against their event name
- Wiring dependency injection with compile-time-verified dependencies
- Building an extensible plugin system with typed hooks
- Writing recursive utility types (`DeepReadonly`, `DeepPartial`, tree structures)
- Encapsulating a module's internals behind a type-safe public surface
- Building a typed API client from a central route configuration
- Validating form fields (sync and async) with type-safe, cross-field-aware validators
- Applying advanced generic constraints for flexible, reusable code

## Use When

- "How do I stop `.build()` from compiling until every required field is set?"
- "How do I type a plugin system so each plugin's hooks are checked?"
- "How do I model a wizard/form flow so an invalid step transition doesn't compile?"
- "How do I make a deeply nested config object read-only at every level?"

## Scope

### In Scope

- Structural design patterns implemented with TypeScript's type system (builder, state machine, event emitter, DI, plugin system, module encapsulation, API client).
- Recursive and deep utility types (`DeepReadonly`, `DeepPartial`, tree/path types).
- Advanced generic constraint patterns.

### Out of Scope

- Runtime type guards and narrowing mechanics that these patterns call into — see the sibling `typescript-type-guards` skill.
- The underlying conditional/mapped/union type mechanics — see the sibling `typescript-type-system` skill.
- Built-in utility types (`Partial`, `Pick`, `ReturnType`) — see the sibling `typescript-utility-types` skill.

## When NOT to Use

Do not use this skill to look up how a conditional type, mapped type, or discriminated union actually works at the type level — the pattern files assume that mechanic and build on top of it. Do not use it as a substitute for `tsc --noEmit` — a pattern that looks type-safe in a reference file still needs the compiler to confirm it holds for your actual types.

## Quick Commands

### Type Check

```bash
npx tsc --noEmit
```

## Anti-Patterns

### NEVER let a builder's `.build()` compile with missing required fields

**WHY:** a builder pattern's entire value is catching an incomplete construction at compile time; if `.build()` accepts an incomplete state, it is no safer than a plain object literal.

**BAD**:

```typescript
class UserBuilder {
  private name?: string;
  setName(name: string) { this.name = name; return this; }
  build() { return { name: this.name }; } // name may be undefined — compiles anyway
}
```

**GOOD**:

```typescript
class UserBuilder<HasName extends boolean = false> {
  private name?: string;
  setName(name: string): UserBuilder<true> {
    this.name = name;
    return this as UserBuilder<true>;
  }
  build(this: UserBuilder<true>): { name: string } {
    return { name: this.name! };
  }
}
```

### NEVER leave a generic type parameter unconstrained when the pattern relies on it having a shape

**WHY:** an unconstrained `<T>` gives the compiler nothing to check, so the pattern degrades to `any` the moment a caller passes something unexpected.

**BAD**:

```typescript
function createInstance<T>(ctor: T, ...args: any[]) {
  return new (ctor as any)(...args);
}
```

**GOOD**:

```typescript
function createInstance<T extends new (...args: any[]) => any>(
  ctor: T,
  ...args: ConstructorParameters<T>
): InstanceType<T> {
  return new ctor(...args);
}
```

### NEVER type an event emitter's `emit`/`on` with a loose string and `any[]`

**WHY:** the whole point of a typed event emitter is that an event name and its payload are checked together; `emit(event: string, ...args: any[])` throws that guarantee away.

**BAD**:

```typescript
class EventEmitter {
  emit(event: string, ...args: any[]): void { /* ... */ }
  on(event: string, handler: (...args: any[]) => void): void { /* ... */ }
}
```

**GOOD**:

```typescript
class EventEmitter<T extends Record<string, (...args: any[]) => void>> {
  emit<K extends keyof T>(event: K, ...args: Parameters<T[K]>): void { /* ... */ }
  on<K extends keyof T>(event: K, handler: T[K]): void { /* ... */ }
}
```

## References

| File | Covers |
| --- | --- |
| `references/builder.md` | Compile-time enforcement of required fields in the builder pattern |
| `references/state-machine.md` | Discriminated-union state machines with compile-time transition validation |
| `references/event-emitter.md` | Type-safe event names and payloads |
| `references/dependency-injection.md` | Compile-time-verified dependency wiring |
| `references/plugin-system.md` | Extensible, typed plugin architectures |
| `references/deep-readonly.md` | `DeepReadonly`, `DeepPartial`, `DeepRequired`, `DeepMutable`, runtime `deepFreeze` |
| `references/recursive-types.md` | Self-referential types: JSON values, path types, tree structures |
| `references/type-safe-module.md` | Encapsulating module internals behind a typed public surface |
| `references/api-client.md` | Typed API clients built from a central route configuration |
| `references/form-validation.md` | Type-safe, cross-field-aware form validation (sync and async validators) |
| `references/branded-types.md` | Nominal typing with Zod-based validated brands and domain-primitive use cases |
| `references/advanced-generics.md` | Complex, composable generic constraint patterns |

- [TypeScript Handbook: Generics](https://www.typescriptlang.org/docs/handbook/2/generics.html)
- [Refactoring Guru: Design Patterns](https://refactoring.guru/design-patterns)
