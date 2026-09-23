# Scenario: Replace Hand-Written Duplicate Types with Built-In Utility Types

A codebase has several hand-written interfaces that duplicate a subset or variant of an existing `User` interface, and they have already drifted (`UserPreview` is missing a field `User` gained later).

```typescript
// userTypes.ts

interface User {
  id: string;
  name: string;
  email: string;
  role: "admin" | "member";
  createdAt: Date;
}

// Hand-written subset — missing `role`, which UserPreview should have had from the start
interface UserPreview {
  id: string;
  name: string;
}

// Hand-written "everything but id" — drifts if User's fields change
interface UserUpdate {
  name?: string;
  email?: string;
  role?: "admin" | "member";
  createdAt?: Date;
}

// Hand-written read-only variant
interface ReadonlyUser {
  readonly id: string;
  readonly name: string;
  readonly email: string;
  readonly role: "admin" | "member";
  readonly createdAt: Date;
}
```

Rewrite `userTypes.ts` so that:

1. `UserPreview` is derived from `User` using `Pick`, and includes `role` (fixing the drift) by picking exactly `"id" | "name" | "role"`.
2. `UserUpdate` is derived from `User` using `Omit` and `Partial` in combination, excluding `id` and `createdAt` and making the rest optional.
3. `ReadonlyUser` is derived from `User` using the built-in `Readonly` utility type, not a hand-written duplicate.
4. `User` itself is unchanged, and no new interfaces duplicate its shape by hand.

## Output Specification

Produce a single file `userTypes.ts` containing `User` unchanged and the three derived types built from utility types.
