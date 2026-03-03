# Bun Test Framework

## Overview

Bun includes a built-in test runner (`bun:test`) that's fast, zero-configuration, and Jest-compatible.

## Basic Test Structure

```typescript
import { describe, test, expect } from "bun:test";

describe("User authentication", () => {
  test("should create new user", () => {
    const user = { id: 1, name: "Alice" };
    expect(user.id).toBe(1);
    expect(user.name).toEqual("Alice");
  });
});
```

## Running Tests

```bash
# Run all tests
bun test

# Run specific test file
bun test auth.test.ts

# Watch mode
bun test --watch

# Coverage
bun test --coverage
```

## Test Lifecycle Hooks

```typescript
import { beforeAll, afterAll, beforeEach, afterEach, describe, test } from "bun:test";

describe("Database operations", () => {
  beforeAll(() => {
    // Setup database connection once
  });

  afterAll(() => {
    // Close database connection
  });

  beforeEach(() => {
    // Clear data before each test
  });

  afterEach(() => {
    // Cleanup after each test
  });

  test("should insert record", () => {
    // Test code
  });
});
```

## Async Testing

```typescript
import { test, expect } from "bun:test";

test("async operations", async () => {
  const data = await fetchData();
  expect(data).toBeDefined();
});

test("promise rejection", async () => {
  await expect(failingPromise()).rejects.toThrow("Error message");
});
```

## Test Organization

```typescript
// Group related tests
describe("User service", () => {
  describe("create()", () => {
    test("with valid data", () => { });
    test("with invalid data", () => { });
  });

  describe("delete()", () => {
    test("existing user", () => { });
    test("non-existent user", () => { });
  });
});
```

## Best Practices

1. **Use Bun's built-in test runner** - No need for Jest, Vitest, or Mocha
2. **Organize tests with describe/test** - Group related tests logically
3. **Use lifecycle hooks** - Setup/teardown with beforeAll/afterAll/beforeEach/afterEach
4. **Test async code properly** - Use async/await and expect().rejects/resolves
5. **Make tests deterministic** - Avoid flaky tests with proper awaiting

## Anti-patterns

- ❌ Using external test runners (Bun's is faster)
- ❌ Forgetting to clean up resources (use afterEach/afterAll)
- ❌ Testing implementation details (test public APIs)
- ❌ Writing flaky tests (ensure deterministic behavior)
