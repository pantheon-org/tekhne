# Scenario: Validate an API Response at the Boundary with Zod

A function fetches a user from an API and casts the response directly to the expected type, trusting the API without checking it. If the API ever returns a differently-shaped payload (a renamed field, a missing property), the bug surfaces far from where the bad data entered the system.

```typescript
// getUser.ts

interface User {
  id: string;
  name: string;
  email: string;
  age: number;
}

async function getUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  const data = await res.json();
  return data as User; // never actually checked
}
```

Rewrite `getUser.ts` so that:

1. A Zod schema `UserSchema` is defined matching `User`'s shape, including that `age` must be a non-negative number.
2. `User` is derived from the schema with `z.infer<typeof UserSchema>` rather than declared separately (so the type and the runtime check cannot drift apart).
3. `getUser` parses the fetched JSON through `UserSchema.parse(...)` instead of using `as User`, so an invalid payload throws a descriptive Zod error instead of silently producing a wrongly-typed object.
4. No `as` assertion remains anywhere in `getUser`.

## Output Specification

Produce a single file `getUser.ts` containing the Zod schema, the derived `User` type, and the corrected `getUser` function.
