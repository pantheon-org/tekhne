# Bun Test Snapshots

## Overview

Snapshot testing captures the output of a function/component and compares it against stored snapshots on subsequent runs.

## Basic Snapshot Testing

```typescript
import { test, expect } from "bun:test";

test("snapshot test", () => {
  const data = {
    id: 1,
    name: "Alice",
    email: "alice@example.com",
    createdAt: new Date("2024-01-01")
  };
  
  expect(data).toMatchSnapshot();
});
```

## Inline Snapshots

```typescript
test("inline snapshot", () => {
  const user = { id: 1, name: "Bob" };
  
  expect(user).toMatchInlineSnapshot(`
    {
      "id": 1,
      "name": "Bob"
    }
  `);
});
```

## Updating Snapshots

```bash
# Update all snapshots
bun test --update-snapshots

# Or use -u flag
bun test -u

# Update snapshots for specific test file
bun test user.test.ts -u
```

## Snapshot File Structure

Snapshots are stored in `__snapshots__` directory:

```
tests/
  user.test.ts
  __snapshots__/
    user.test.ts.snap
```

## Testing API Responses

```typescript
import { test, expect } from "bun:test";

test("API response snapshot", async () => {
  const response = await fetch("http://localhost:3000/api/users/1");
  const data = await response.json();
  
  expect(data).toMatchSnapshot();
});
```

## Testing Rendered Output

```typescript
import { test, expect } from "bun:test";
import { renderToString } from "some-template-engine";

test("template rendering", () => {
  const html = renderToString("user-profile", {
    name: "Alice",
    email: "alice@example.com"
  });
  
  expect(html).toMatchSnapshot();
});
```

## Property Matchers

Use property matchers when some values are dynamic:

```typescript
test("snapshot with dynamic values", () => {
  const user = {
    id: Math.random(),
    name: "Alice",
    createdAt: new Date()
  };
  
  expect(user).toMatchSnapshot({
    id: expect.any(Number),
    createdAt: expect.any(Date)
  });
});
```

## Snapshot Serialization

### Custom Serializer

```typescript
import { test, expect } from "bun:test";

expect.addSnapshotSerializer({
  test: (val) => val instanceof Date,
  serialize: (val) => `Date(${val.toISOString()})`
});

test("custom date serialization", () => {
  const event = {
    name: "Meeting",
    date: new Date("2024-01-01T10:00:00Z")
  };
  
  expect(event).toMatchSnapshot();
  // Snapshot will show: Date(2024-01-01T10:00:00.000Z)
});
```

## Snapshot Testing Patterns

### Testing Error Messages

```typescript
test("error message snapshot", () => {
  const error = validateUser({ email: "invalid" });
  
  expect(error).toMatchSnapshot();
});
```

### Testing Configuration Objects

```typescript
test("config snapshot", () => {
  const config = loadConfig("production");
  
  // Exclude sensitive data
  const { apiKey, ...publicConfig } = config;
  
  expect(publicConfig).toMatchSnapshot();
});
```

### Testing Database Queries

```typescript
test("SQL query snapshot", () => {
  const query = buildQuery({
    table: "users",
    where: { active: true },
    orderBy: "created_at",
    limit: 10
  });
  
  expect(query).toMatchSnapshot();
});
```

## Snapshot Diffing

When a snapshot fails, Bun shows a diff:

```
Expected:
{
  "name": "Alice",
  "age": 30
}

Received:
{
  "name": "Alice",
  "age": 31
}
```

## Best Practices

1. **Commit snapshots to version control** - Track changes over time
2. **Review snapshot changes carefully** - Don't blindly update snapshots
3. **Use property matchers for dynamic data** - Handle timestamps, IDs, random values
4. **Keep snapshots small** - Large snapshots are hard to review
5. **Name tests descriptively** - Make it clear what's being snapshotted
6. **Update intentionally** - Only update when changes are expected

## Anti-patterns

- ❌ Blindly updating snapshots without review
- ❌ Snapshotting large objects with too much irrelevant data
- ❌ Not using property matchers for dynamic values
- ❌ Snapshotting implementation details instead of outputs
- ❌ Using snapshots as the only test (combine with assertions)
- ❌ Not committing snapshot files to version control

## When to Use Snapshots

### Good Use Cases

- ✅ Testing API responses
- ✅ Testing rendered HTML/templates
- ✅ Testing generated code
- ✅ Testing configuration objects
- ✅ Testing serialized data structures

### Poor Use Cases

- ❌ Testing simple values (use `toBe()` or `toEqual()`)
- ❌ Testing implementation details
- ❌ Testing frequently changing data
- ❌ Testing when precision assertions are better

## Snapshot Organization

```typescript
// Group related snapshots
describe("User API", () => {
  test("GET /users/:id", async () => {
    const response = await fetch("/users/1");
    expect(await response.json()).toMatchSnapshot();
  });
  
  test("POST /users", async () => {
    const response = await fetch("/users", {
      method: "POST",
      body: JSON.stringify({ name: "Bob" })
    });
    expect(await response.json()).toMatchSnapshot();
  });
});
```

## Cleaning Up Obsolete Snapshots

```bash
# Bun automatically removes obsolete snapshots when tests are deleted
bun test

# Force cleanup
bun test --update-snapshots
```
