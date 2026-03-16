# Scenario 2: Model State with Discriminated Unions

## Context

A team is building a data-fetching hook and currently represents its state with a single
interface that has many optional fields. This allows impossible combinations — for example,
`data` and `error` being set at the same time, or `isLoading: true` while `data` is
present.

## Input Code

```typescript
// fetchState.ts

interface FetchState<T> {
  isLoading: boolean;
  data?: T;
  error?: Error;
  statusCode?: number;
}

// Example usage with an impossible combination that TypeScript currently allows:
const state: FetchState<string> = {
  isLoading: false,
  data: "hello",
  error: new Error("oops"),   // impossible: can't have both data and error
  statusCode: 200,
};
```

## Task

Rewrite `fetchState.ts` so that:

1. The state is modelled as a discriminated union with a `status` discriminant field.
2. The three valid states are: `idle`, `loading`, and `done` (where `done` carries either
   `data` or `error`, but not both).
3. It is a compile-time error to construct a state with both `data` and `error` present.
4. A helper function `handleState<T>(state: FetchState<T>)` is included that uses
   exhaustive switching on `status` and returns a human-readable description string.
   The function must trigger a TypeScript compile error if a new variant is added but
   not handled (use the `never` trick).

## Output Specification

Produce a single file `fetchState.ts` containing the type definitions and the
`handleState` helper function.
