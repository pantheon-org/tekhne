# Scenario 01: Refactor `any` Types to Constrained Generics

## User Prompt

You are working on a TypeScript utility library. A colleague submitted the following code which compiles but is full of unsafe `any` types. Your task is to rewrite it so that all `any` annotations are eliminated and replaced with appropriate generic constraints or specific types, without changing the observable behaviour of the functions.

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

Produce a rewritten file `utils.ts` that:

1. Replaces every `any` annotation with a constrained generic, a specific type, or an explicit union type that accurately describes what the function accepts and returns.
2. Does not introduce new `any`, `@ts-ignore`, or unchecked `as` assertions.
3. Keeps each function's public signature backward-compatible where possible (callers passing objects/strings should still compile).
4. Includes a brief inline comment above each function explaining the constraint chosen.

## Output Specification

Produce a single file `utils.ts` containing the rewritten implementations.

## Expected Behavior

1. Eliminate all `: any` or `<any>` occurrences from the output file (excluding comments)
2. Introduce no `@ts-ignore`, `// @ts-nocheck`, or unchecked `as` assertions to silence errors
3. Apply meaningful constraints to each generic parameter (e.g., `extends object`, `extends { value: unknown }`) rather than unconstrained `<T>`
4. Provide explicit, correct return type annotations on every function (e.g., `getProperty` returns `T[K]`, not `unknown`)
5. Include an inline comment above each function explaining why that constraint was chosen

## Success Criteria

- **No `any` annotations remain**: The output file contains zero occurrences of `: any` or `<any>` (excluding comments)
- **No unsafe workarounds introduced**: The output does not introduce `@ts-ignore`, `// @ts-nocheck`, or unchecked `as` assertions
- **Generic constraints are meaningful**: Each generic parameter carries a constraint (`extends object`, `extends { value: unknown }`, etc.) rather than being unconstrained `<T>`
- **Return types are explicit and correct**: Every function has an explicit return type annotation that reflects the actual return value
- **Inline comments explain each constraint**: A comment above each function briefly explains why that generic constraint was chosen

## Failure Conditions

- Agent leaves any `: any` or `<any>` annotations in the output
- Agent adds `@ts-ignore` or `// @ts-nocheck` to suppress errors
- Agent uses unconstrained `<T>` generics without meaningful constraints
- Agent uses `as any` or other unchecked casts to bypass type checking
- Agent omits return type annotations or uses `unknown` where a more specific type is derivable
- Agent omits inline comments explaining the constraint choices
