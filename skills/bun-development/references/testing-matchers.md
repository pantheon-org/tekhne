# Bun Test Matchers

## Overview

Bun's test runner provides Jest-compatible matchers for writing assertions in your tests.

## Basic Matchers

```typescript
import { test, expect } from "bun:test";

test("basic matchers", () => {
  // Equality
  expect(2 + 2).toBe(4); // Strict equality (===)
  expect({ name: "Alice" }).toEqual({ name: "Alice" }); // Deep equality
  
  // Truthiness
  expect(true).toBeTruthy();
  expect(false).toBeFalsy();
  expect(null).toBeNull();
  expect(undefined).toBeUndefined();
  expect("value").toBeDefined();
});
```

## Numeric Matchers

```typescript
test("numeric matchers", () => {
  expect(10).toBeGreaterThan(5);
  expect(5).toBeLessThan(10);
  expect(10).toBeGreaterThanOrEqual(10);
  expect(5).toBeLessThanOrEqual(5);
  
  // Floating point comparison
  expect(0.1 + 0.2).toBeCloseTo(0.3); // Handles floating point precision
});
```

## String Matchers

```typescript
test("string matchers", () => {
  expect("Hello World").toMatch(/World/); // Regular expression
  expect("Hello World").toMatch("World"); // Substring
  expect("error").toContain("err"); // Contains substring
});
```

## Array and Object Matchers

```typescript
test("array matchers", () => {
  const array = [1, 2, 3, 4, 5];
  
  expect(array).toHaveLength(5);
  expect(array).toContain(3);
  expect(array).toContainEqual(3);
});

test("object matchers", () => {
  const user = { id: 1, name: "Alice", email: "alice@example.com" };
  
  expect(user).toHaveProperty("name");
  expect(user).toHaveProperty("name", "Alice");
  expect(user).toMatchObject({ name: "Alice" }); // Partial match
});
```

## Exception Matchers

```typescript
test("exception matchers", () => {
  const throwError = () => {
    throw new Error("Something went wrong");
  };
  
  expect(throwError).toThrow(); // Any error
  expect(throwError).toThrow(Error); // Specific error type
  expect(throwError).toThrow("Something went wrong"); // Error message
  expect(throwError).toThrow(/went wrong/); // Error message pattern
});
```

## Promise Matchers

```typescript
test("async matchers", async () => {
  const resolves = Promise.resolve("success");
  const rejects = Promise.reject(new Error("failed"));
  
  await expect(resolves).resolves.toBe("success");
  await expect(rejects).rejects.toThrow("failed");
});
```

## Negation

```typescript
test("negation with .not", () => {
  expect(5).not.toBe(10);
  expect("hello").not.toMatch(/world/);
  expect([1, 2, 3]).not.toContain(4);
});
```

## Type Matchers

```typescript
test("type checking", () => {
  expect("hello").toBeInstanceOf(String);
  expect([1, 2, 3]).toBeInstanceOf(Array);
  expect(new Date()).toBeInstanceOf(Date);
});
```

## Common Patterns

### Testing API Responses

```typescript
test("API response structure", async () => {
  const response = await fetch("/api/users/1");
  const data = await response.json();
  
  expect(response.status).toBe(200);
  expect(data).toMatchObject({
    id: expect.any(Number),
    name: expect.any(String),
    email: expect.stringMatching(/@/)
  });
});
```

### Testing Arrays

```typescript
test("array operations", () => {
  const items = [1, 2, 3, 4, 5];
  
  expect(items).toHaveLength(5);
  expect(items).toEqual(expect.arrayContaining([2, 4]));
  expect(items).toEqual([1, 2, 3, 4, 5]); // Exact match
});
```

### Testing Objects

```typescript
test("object validation", () => {
  const user = {
    id: 1,
    name: "Alice",
    settings: { theme: "dark" }
  };
  
  expect(user).toMatchObject({
    name: "Alice",
    settings: { theme: "dark" }
  });
  
  expect(user).toEqual({
    id: expect.any(Number),
    name: expect.any(String),
    settings: expect.objectContaining({ theme: "dark" })
  });
});
```

## Best Practices

1. **Use toBe for primitives** - Use strict equality for numbers, strings, booleans
2. **Use toEqual for objects/arrays** - Use deep equality for complex types
3. **Use toBeCloseTo for floats** - Handle floating point precision issues
4. **Use specific matchers** - Prefer toHaveLength over .length.toBe
5. **Use toMatchObject for partial matching** - Test only relevant properties
6. **Use descriptive assertions** - Make test failures clear and actionable

## Anti-patterns

- ❌ Using `.toBe()` for objects (use `.toEqual()`)
- ❌ Testing implementation details (test behavior, not internal state)
- ❌ Writing assertions that are too loose (be specific)
- ❌ Not using `.not` for negative assertions
- ❌ Comparing floats with `.toBe()` (use `.toBeCloseTo()`)
