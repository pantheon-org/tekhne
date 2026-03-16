# Scenario 1: Refactor `any` Types to Constrained Generics

## Context

You are working on a TypeScript utility library. A colleague submitted the following code
which compiles but is full of unsafe `any` types. Your task is to rewrite it so that all
`any` annotations are eliminated and replaced with appropriate generic constraints or
specific types, without changing the observable behaviour of the functions.

## Input Code

```typescript
// utils.ts

export function getProperty(obj: any, key: string): any {
  return obj[key];
}

export function merge(target: any, source: any): any {
  return { ...target, ...source };
}

export function process(data: any) {
  return data.value;
}

export function wrapInArray(item: any): any[] {
  return [item];
}
```

## Task

Produce a rewritten file `utils.ts` that:

1. Replaces every `any` annotation with a constrained generic, a specific type, or an
   explicit union type that accurately describes what the function accepts and returns.
2. Does not introduce new `any`, `@ts-ignore`, or unchecked `as` assertions.
3. Keeps each function's public signature backward-compatible where possible (callers
   passing objects/strings should still compile).
4. Includes a brief inline comment above each function explaining the constraint chosen.

## Output Specification

Produce a single file `utils.ts` containing the rewritten implementations.
