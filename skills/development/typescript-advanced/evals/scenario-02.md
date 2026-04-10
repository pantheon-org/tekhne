# Scenario 02: Model State with Discriminated Unions

## User Prompt

A team is building a data-fetching hook and currently represents its state with a single interface that has many optional fields. This allows impossible combinations — for example, `data` and `error` being set at the same time, or `isLoading: true` while `data` is present.

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

Rewrite `fetchState.ts` so that:

1. The state is modelled as a discriminated union with a `status` discriminant field.
2. The three valid states are: `idle`, `loading`, and `done` (where `done` carries either `data` or `error`, but not both).
3. It is a compile-time error to construct a state with both `data` and `error` present.
4. A helper function `handleState<T>(state: FetchState<T>)` is included that uses exhaustive switching on `status` and returns a human-readable description string. The function must trigger a TypeScript compile error if a new variant is added but not handled (use the `never` trick).

## Output Specification

Produce a single file `fetchState.ts` containing the type definitions and the `handleState` helper function.

## Expected Behavior

1. Assign a distinct literal `status` field to each member of the union (`'idle'`, `'loading'`, `'done-ok'`/`'done-error'` or equivalent)
2. Ensure no single union member type includes both a `data` field and an `error` field simultaneously — the original impossible combination would be a type error
3. In `handleState`, use a `switch` on the discriminant with a `default` branch that assigns the narrowed value to `never`, ensuring compile-time exhaustiveness
4. Introduce no `any`, `@ts-ignore`, or unchecked `as` casts
5. Include a comment explaining why discriminated unions prevent illegal states compared to the original optional-fields approach

## Success Criteria

- **Discriminant field is present on every variant**: Each member of the union has a literal `status` field with a distinct string value
- **Impossible combination is structurally prevented**: No single union member type includes both a `data` field and an `error` field simultaneously
- **Exhaustive switch with `never` guard**: The `handleState` function uses a `switch` on the discriminant with a `default` branch assigning to `never`
- **No `any` or unsafe assertions introduced**: The output contains no `any`, `@ts-ignore`, or unchecked `as` casts
- **Explanation comment included**: A comment in the file explains why discriminated unions prevent illegal states

## Failure Conditions

- Agent uses a single interface with optional fields rather than a discriminated union
- Agent does not add a literal `status` discriminant field to each variant
- Agent allows both `data` and `error` to appear on the same variant type
- Agent omits the `never` guard in the `default` branch of the `switch`
- Agent introduces `any`, `@ts-ignore`, or unchecked `as` casts
- Agent omits the explanation comment about discriminated unions
