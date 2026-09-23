# Scenario: Type-Safe Route Parameters with Template Literal Types

An Express-style router currently accepts route paths as plain `string`, so a typo in a path parameter (e.g. `:userId` vs `:userID`) is only caught at runtime.

```typescript
// routes.ts

function registerRoute(path: string, handler: (params: Record<string, string>) => void) {
  // implementation omitted
}

registerRoute("/users/:userId/posts/:postId", (params) => {
  console.log(params.userId, params.postId); // params is just Record<string, string> — no autocomplete, no typo safety
});
```

Rewrite `routes.ts` so that:

1. A template literal type `RouteParams<T extends string>` extracts the named parameters from a route path string (e.g. `RouteParams<"/users/:userId/posts/:postId">` is `{ userId: string; postId: string }`).
2. `registerRoute` is a generic function whose `handler` parameter's `params` argument is typed as `RouteParams<Path>` for the specific `Path` string literal passed in, not a generic `Record<string, string>`.
3. Accessing `params.userId` and `params.postId` in the example call site type-checks, and accessing a parameter not present in the path (e.g. `params.commentId`) is a compile error.
4. The solution handles paths with zero, one, or multiple `:param` segments.

## Output Specification

Produce a single file `routes.ts` containing the `RouteParams<T>` type, the updated `registerRoute` function, and the example call site.
