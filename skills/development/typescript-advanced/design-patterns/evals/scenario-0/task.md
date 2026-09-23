# Scenario: Enforce Required Fields at Compile Time with the Builder Pattern

A `RequestBuilder` class currently allows `.build()` to be called even when required fields haven't been set, producing a runtime error deep in an HTTP client instead of a compile error at the call site.

```typescript
// requestBuilder.ts

interface Request {
  url: string;
  method: string;
  body?: unknown;
}

class RequestBuilder {
  private url?: string;
  private method?: string;
  private body?: unknown;

  setUrl(url: string): this {
    this.url = url;
    return this;
  }

  setMethod(method: string): this {
    this.method = method;
    return this;
  }

  setBody(body: unknown): this {
    this.body = body;
    return this;
  }

  build(): Request {
    // Runtime check only — TypeScript happily compiles new RequestBuilder().build()
    if (!this.url || !this.method) {
      throw new Error("url and method are required");
    }
    return { url: this.url, method: this.method, body: this.body };
  }
}
```

Rewrite `requestBuilder.ts` so that:

1. `.build()` does not compile unless both `setUrl` and `setMethod` have already been called on that builder instance.
2. `setBody` remains optional and can be called in any order relative to the required setters.
3. The type-level enforcement uses generics to track which required setters have been called (not a runtime-only check).
4. `new RequestBuilder().setUrl("/x").build()` (missing `setMethod`) must fail to compile, with a comment demonstrating this.

## Output Specification

Produce a single file `requestBuilder.ts` containing the reworked builder and a commented-out line showing the compile error case.
