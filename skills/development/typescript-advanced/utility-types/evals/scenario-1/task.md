# Scenario: Type a Factory Function with Extraction Utilities

A generic factory function currently accepts a constructor and its arguments typed as `any[]`, so a caller passing the wrong number or type of constructor arguments isn't caught until the object is actually constructed at runtime.

```typescript
// factory.ts

class ApiClient {
  constructor(public baseUrl: string, public timeoutMs: number) {}
}

function createInstance(ctor: any, ...args: any[]): any {
  return new ctor(...args);
}

const client = createInstance(ApiClient, "https://api.example.com", 5000);
// createInstance(ApiClient, "https://api.example.com") — missing timeoutMs, compiles anyway
```

Rewrite `factory.ts` so that:

1. `createInstance` is generic over the constructor type, with a constraint requiring it to be a class constructor (`new (...args: any[]) => any`).
2. The `args` parameter's type is extracted from the constructor using `ConstructorParameters<T>`, not `any[]`.
3. The return type is extracted from the constructor using `InstanceType<T>`, not `any`.
4. A commented-out call demonstrates that calling `createInstance(ApiClient, "https://api.example.com")` (missing the second argument) fails to compile.

## Output Specification

Produce a single file `factory.ts` containing the corrected `createInstance` function, the `ApiClient` class unchanged, and the example usage including the commented-out compile-error case.
