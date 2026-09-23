# Scenario: Write JSDoc That Adds Information the Signature Doesn't Already Give

A public function has no documentation, and a colleague's first draft of JSDoc for it merely restates the parameter and return types the signature already shows, adding no real information.

```typescript
// retryWithBackoff.ts

export async function retryWithBackoff<T>(
  fn: () => Promise<T>,
  maxAttempts: number,
  baseDelayMs: number = 100
): Promise<T> {
  let attempt = 0;
  while (true) {
    try {
      return await fn();
    } catch (err) {
      attempt++;
      if (attempt >= maxAttempts) throw err;
      const delay = baseDelayMs * 2 ** (attempt - 1);
      await new Promise((resolve) => setTimeout(resolve, delay));
    }
  }
}
```

Write JSDoc for `retryWithBackoff` that:

1. Explains the function's actual behavior and intent (retries with exponentially increasing delay), not just its parameter types.
2. Documents what happens when `maxAttempts` is exhausted (the last error is thrown, not swallowed).
3. Includes a `@param` for each parameter that adds information beyond the type (e.g. what `baseDelayMs` scales from, what counts as an "attempt").
4. Includes a realistic `@example` showing a typical call site.
5. Does NOT restate a parameter's type in prose where the signature already makes it obvious (e.g. does not write "`maxAttempts` — a number" with no further detail).

## Output Specification

Produce a single file `retryWithBackoff.ts` containing the original function with the new JSDoc comment added above it.
