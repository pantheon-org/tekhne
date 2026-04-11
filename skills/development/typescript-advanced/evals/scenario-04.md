# Scenario 04: Replace Unsafe `as` Assertions with Runtime Type Guards

## User Prompt

A developer wrote the following parser module. It uses `as` type assertions throughout to force values into the expected types. While it compiles, the assertions bypass the compiler's safety guarantees — if the API ever returns an unexpected shape, the code will fail silently at runtime.

```typescript
// parser.ts

export function parseUserId(raw: unknown): string {
  return raw as string;
}

export function parseCount(raw: unknown): number {
  return raw as number;
}

export function parseUser(raw: unknown): { id: string; name: string } {
  return raw as { id: string; name: string };
}

export function parseConfig(raw: unknown): { debug: boolean; timeout: number } {
  return raw as { debug: boolean; timeout: number };
}
```

Rewrite `parser.ts` so that:

1. Every `as` assertion is removed.
2. Each function validates the input at runtime before returning it, throwing a descriptive `TypeError` if the input does not match the expected shape.
3. After validation, TypeScript narrows the type so that no `as` assertion is needed for the return value.
4. Type predicate functions (`is*`) are used where appropriate to keep the validation logic reusable and readable.

## Output Specification

Produce a single file `parser.ts` containing the rewritten parser functions.

## Expected Behavior

1. Remove all `as` assertion keywords from function bodies
2. Add a runtime check in each function that throws a `TypeError` with a descriptive message when the input does not match the expected shape
3. Define at least one `is*` type predicate function (with `value is T` return type annotation) to validate object shapes
4. After each runtime check, TypeScript's control-flow analysis narrows the type so the function returns the correct type without further assertions
5. Introduce no new `any` annotations, `@ts-ignore`, or `// @ts-nocheck` directives

## Success Criteria

- **No `as` assertions remain in function bodies**: The output file contains zero uses of the `as` keyword in function bodies (comments excluded)
- **Each function throws a descriptive `TypeError` on invalid input**: Every parser function has a runtime check that throws a `TypeError` with a message identifying which value was invalid
- **Type predicate functions are used for object-shape validation**: At least one `is*` type predicate function (with `value is T` return type annotation) is defined and used
- **TypeScript narrows to the correct type after guard**: After each runtime check, TypeScript's control-flow analysis narrows the type correctly without further assertions
- **No new `any` or `@ts-ignore` introduced**: The rewritten file introduces no `any` annotations, `@ts-ignore`, or `// @ts-nocheck` directives

## Failure Conditions

- Agent retains `as string`, `as number`, or similar assertions in function bodies
- Agent does not add runtime type checks that throw `TypeError` on invalid input
- Agent does not define any `is*` type predicate functions for object validation
- Agent adds `@ts-ignore` or `any` to bypass type errors instead of using proper runtime guards
- Agent produces code that TypeScript cannot narrow correctly after the runtime check (e.g., still requires an `as` assertion after the guard)
