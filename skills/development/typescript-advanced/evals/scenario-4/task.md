# Scenario 4: Replace Unsafe `as` Assertions with Runtime Type Guards

## Context

A developer wrote the following parser module. It uses `as` type assertions throughout
to force values into the expected types. While it compiles, the assertions bypass the
compiler's safety guarantees — if the API ever returns an unexpected shape, the code
will fail silently at runtime.

## Input Code

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

## Task

Rewrite `parser.ts` so that:

1. Every `as` assertion is removed.
2. Each function validates the input at runtime before returning it, throwing a
   descriptive `TypeError` if the input does not match the expected shape.
3. After validation, TypeScript narrows the type so that no `as` assertion is needed
   for the return value.
4. Type predicate functions (`is*`) are used where appropriate to keep the validation
   logic reusable and readable.

## Output Specification

Produce a single file `parser.ts` containing the rewritten parser functions.
