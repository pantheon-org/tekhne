# Scenario: Generate Getter Method Names with Key Remapping

A codebase needs a `Getters<T>` type that transforms a plain data type into an interface of getter methods — turning each property `foo: string` into a method `getFoo(): string` — for use in a generated accessor class.

```typescript
// getters.ts

interface Config {
  host: string;
  port: number;
  debug: boolean;
}

// Currently hand-written and must be kept in sync with Config by hand
interface ConfigGetters {
  getHost(): string;
  getPort(): number;
  getDebug(): boolean;
}
```

Rewrite `getters.ts` so that:

1. A generic mapped type `Getters<T>` is defined that remaps each key `K` of `T` to `get${Capitalize<K>}` using an `as` clause in the mapped type, producing a method signature `() => T[K]` for each.
2. `ConfigGetters` is redefined as `Getters<Config>` instead of being hand-written.
3. `Getters<Config>` structurally matches the original hand-written `ConfigGetters` (same method names, same return types).
4. The solution generalizes to any object type, not just `Config` — demonstrate this with a second example type and its derived getters.

## Output Specification

Produce a single file `getters.ts` containing the `Getters<T>` mapped type, `Config`, `ConfigGetters` derived from it, and the second example type with its derived getters.
