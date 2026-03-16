# Scenario 5: Fix "Object is possibly undefined" Errors

## Context

The following TypeScript file has three functions that produce "Object is possibly
'undefined'" compile errors under `strict: true`. A previous developer silenced them
using non-null assertions (`!`) or ignored them entirely. Your task is to fix the
underlying type model correctly.

## Input Code

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

## Task

Rewrite `accessors.ts` so that:

1. The `!` non-null assertions are removed.
2. Each function guards against `undefined` explicitly before accessing the value.
3. Functions return a sensible default or throw a descriptive error when the optional
   value is absent — choose the approach that best fits each case and justify it with
   an inline comment.
4. TypeScript's control-flow analysis confirms the type is narrowed (no residual
   "possibly undefined" errors).

## Output Specification

Produce a single file `accessors.ts` with the corrected implementations and inline
justification comments.
