# Scenario 05: Fix "Object is possibly undefined" Errors

## User Prompt

The following TypeScript file has three functions that produce "Object is possibly 'undefined'" compile errors under `strict: true`. A previous developer silenced them using non-null assertions (`!`) or ignored them entirely. Your task is to fix the underlying type model correctly.

```typescript
// accessors.ts

interface Config {
  host?: string;
  port?: number;
  tags?: string[];
}

// Error: Object is possibly 'undefined'
export function getHost(config: Config): string {
  return config.host.toUpperCase();   // host may be undefined
}

// Error: Object is possibly 'undefined'
export function getFirstTag(config: Config): string {
  return config.tags![0];   // non-null assertion hides the real problem
}

// Error: Object is possibly 'undefined'
export function getPort(config: Config): number {
  return config.port!;   // non-null assertion hides the real problem
}
```

Rewrite `accessors.ts` so that:

1. The `!` non-null assertions are removed.
2. Each function guards against `undefined` explicitly before accessing the value.
3. Functions return a sensible default or throw a descriptive error when the optional value is absent — choose the approach that best fits each case and justify it with an inline comment.
4. TypeScript's control-flow analysis confirms the type is narrowed (no residual "possibly undefined" errors).

## Output Specification

Produce a single file `accessors.ts` with the corrected implementations and inline justification comments.

## Expected Behavior

1. Remove all `!` non-null assertion operators on optional property accesses
2. Add an explicit conditional check (`if (x === undefined)`, nullish coalescing, or optional chaining with a fallback) in every function before accessing the potentially undefined value
3. Apply the strategy most appropriate per function: use a default value where absence is recoverable; throw a `TypeError` or `RangeError` where absence is an error
4. Include an inline comment in each function explaining the chosen strategy (default vs. throw)
5. Introduce no `any`, `@ts-ignore`, or new `as` assertions to suppress errors

## Success Criteria

- **All `!` non-null assertions removed**: The output file contains zero non-null assertion operators (`!`) on optional property accesses
- **Each function has an explicit undefined guard**: Every function contains a conditional check before accessing the potentially undefined value
- **Guard strategy is appropriate per function**: Functions that can reasonably return a default value use a default; functions where absence is an error throw a `TypeError` or `RangeError`
- **Inline comments justify each strategy**: Each function has an inline comment explaining why the chosen strategy (default vs. throw) was used
- **No `any`, `@ts-ignore`, or new `as` assertions introduced**: The fix does not introduce any unsafe workarounds

## Failure Conditions

- Agent retains `!` non-null assertion operators in the output
- Agent does not add explicit undefined guards in every function
- Agent applies the same strategy (all defaults or all throws) without distinguishing which is appropriate per function
- Agent does not add inline comments justifying the strategy chosen
- Agent adds `any`, `@ts-ignore`, or `as` assertions to suppress the "possibly undefined" errors
