# Bun Test Mocking

## Overview

Bun provides built-in mocking capabilities through `mock()` and `mock.module()` for testing code with dependencies.

## Function Mocking

### Basic Mock Functions

```typescript
import { test, expect, mock } from "bun:test";

test("mock function", () => {
  const mockFn = mock(() => "mocked value");
  
  expect(mockFn()).toBe("mocked value");
  expect(mockFn).toHaveBeenCalled();
  expect(mockFn).toHaveBeenCalledTimes(1);
});
```

### Mock with Return Values

```typescript
test("mock return values", () => {
  const mockFn = mock();
  
  mockFn.mockReturnValue(42);
  expect(mockFn()).toBe(42);
  
  mockFn.mockReturnValueOnce(1)
        .mockReturnValueOnce(2)
        .mockReturnValue(3);
  
  expect(mockFn()).toBe(1);
  expect(mockFn()).toBe(2);
  expect(mockFn()).toBe(3);
  expect(mockFn()).toBe(3);
});
```

### Mock with Implementation

```typescript
test("mock implementation", () => {
  const mockFn = mock((x: number, y: number) => x + y);
  
  expect(mockFn(2, 3)).toBe(5);
  expect(mockFn).toHaveBeenCalledWith(2, 3);
});
```

## Module Mocking

### Mock Entire Module

```typescript
import { test, expect, mock } from "bun:test";

mock.module("./database", () => ({
  default: {
    query: mock(() => Promise.resolve([{ id: 1, name: "Test" }])),
    connect: mock(() => Promise.resolve()),
    disconnect: mock(() => Promise.resolve())
  }
}));

test("database operations", async () => {
  const db = await import("./database");
  
  const results = await db.default.query("SELECT * FROM users");
  expect(results).toHaveLength(1);
  expect(db.default.query).toHaveBeenCalled();
});
```

### Partial Module Mock

```typescript
import { test, expect, mock } from "bun:test";
import * as userService from "./user-service";

mock.module("./user-service", () => ({
  ...userService,
  sendEmail: mock(() => Promise.resolve(true)) // Mock only sendEmail
}));

test("user creation without email", async () => {
  const { createUser, sendEmail } = await import("./user-service");
  
  await createUser({ name: "Alice", email: "alice@example.com" });
  expect(sendEmail).toHaveBeenCalledWith("alice@example.com");
});
```

## Spy Functions

### Spying on Object Methods

```typescript
import { test, expect, spyOn } from "bun:test";

test("spy on method", () => {
  const calculator = {
    add: (a: number, b: number) => a + b
  };
  
  const spy = spyOn(calculator, "add");
  
  calculator.add(2, 3);
  
  expect(spy).toHaveBeenCalledWith(2, 3);
  expect(spy).toHaveReturnedWith(5);
});
```

### Spy with Mock Implementation

```typescript
test("spy with mock implementation", () => {
  const logger = {
    log: (message: string) => console.log(message)
  };
  
  const spy = spyOn(logger, "log").mockImplementation(() => {
    // Do nothing instead of logging
  });
  
  logger.log("test message");
  
  expect(spy).toHaveBeenCalledWith("test message");
});
```

## Mock Assertions

```typescript
test("mock assertions", () => {
  const mockFn = mock();
  
  mockFn("arg1", "arg2");
  mockFn("arg3");
  
  // Called assertions
  expect(mockFn).toHaveBeenCalled();
  expect(mockFn).toHaveBeenCalledTimes(2);
  
  // Argument assertions
  expect(mockFn).toHaveBeenCalledWith("arg1", "arg2");
  expect(mockFn).toHaveBeenNthCalledWith(1, "arg1", "arg2");
  expect(mockFn).toHaveBeenNthCalledWith(2, "arg3");
  expect(mockFn).toHaveBeenLastCalledWith("arg3");
});
```

## Resetting Mocks

```typescript
import { test, expect, mock, beforeEach } from "bun:test";

const mockFn = mock();

beforeEach(() => {
  mockFn.mockClear(); // Clear call history
  // or
  mockFn.mockReset(); // Clear call history and implementation
  // or
  mockFn.mockRestore(); // Restore original implementation (for spies)
});

test("first test", () => {
  mockFn();
  expect(mockFn).toHaveBeenCalledTimes(1);
});

test("second test", () => {
  // mockFn call count is reset
  expect(mockFn).not.toHaveBeenCalled();
});
```

## Common Patterns

### Mocking HTTP Fetch

```typescript
import { test, expect, mock } from "bun:test";

mock.module("global", () => ({
  fetch: mock(() => Promise.resolve({
    json: () => Promise.resolve({ id: 1, name: "Test" }),
    status: 200,
    ok: true
  }))
}));

test("fetch API data", async () => {
  const response = await fetch("/api/users/1");
  const data = await response.json();
  
  expect(data).toEqual({ id: 1, name: "Test" });
});
```

### Mocking Timers

```typescript
import { test, expect, mock } from "bun:test";

test("delayed execution", () => {
  const callback = mock();
  
  setTimeout(callback, 1000);
  
  // Fast-forward time
  jest.advanceTimersByTime(1000);
  
  expect(callback).toHaveBeenCalledTimes(1);
});
```

### Mocking File System

```typescript
import { test, expect, mock } from "bun:test";

mock.module("node:fs/promises", () => ({
  readFile: mock(() => Promise.resolve("file contents")),
  writeFile: mock(() => Promise.resolve()),
  exists: mock(() => Promise.resolve(true))
}));

test("file operations", async () => {
  const fs = await import("node:fs/promises");
  
  const content = await fs.readFile("test.txt", "utf-8");
  expect(content).toBe("file contents");
  expect(fs.readFile).toHaveBeenCalledWith("test.txt", "utf-8");
});
```

## Best Practices

1. **Mock at the boundary** - Mock external dependencies, not internal logic
2. **Reset mocks between tests** - Use beforeEach to clear mock state
3. **Verify mock calls** - Assert that mocks were called with expected arguments
4. **Mock minimal scope** - Only mock what's necessary for the test
5. **Use spies for partial mocking** - Spy on real objects when you need partial behavior

## Anti-patterns

- ❌ Mocking too much (over-mocking makes tests brittle)
- ❌ Not resetting mocks between tests (causes test interdependence)
- ❌ Mocking implementation details (mock external dependencies only)
- ❌ Complex mock setup (simplify or refactor the code under test)
- ❌ Not asserting mock behavior (verify mocks were called correctly)
