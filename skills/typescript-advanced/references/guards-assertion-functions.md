# Assertion Functions

## Overview
Assertion functions use the `asserts` keyword to narrow types by throwing errors when conditions fail, rather than returning boolean values.

## Basic Assertion Function

```typescript
function assert(condition: unknown, message?: string): asserts condition {
  if (!condition) {
    throw new Error(message || "Assertion failed");
  }
}

const value: string | null = getSomeValue();
assert(value !== null, "Value must be non-null");
// value is narrowed to string after this line
console.log(value.toUpperCase());
```

## Assertion with Type Predicate

```typescript
function assertIsString(value: unknown): asserts value is string {
  if (typeof value !== "string") {
    throw new TypeError(`Expected string, got ${typeof value}`);
  }
}

const data: unknown = "hello";
assertIsString(data);
// data is narrowed to string
console.log(data.toUpperCase());
```

## Assertion for Object Shape

```typescript
interface User {
  id: number;
  name: string;
  email: string;
}

function assertIsUser(value: unknown): asserts value is User {
  if (
    typeof value !== "object" ||
    value === null ||
    !("id" in value) ||
    !("name" in value) ||
    !("email" in value)
  ) {
    throw new TypeError("Invalid user object");
  }
}

const data: unknown = { id: 1, name: "Alice", email: "alice@example.com" };
assertIsUser(data);
// data is narrowed to User
console.log(data.email);
```

## Assertion for Non-null

```typescript
function assertDefined<T>(
  value: T,
  message?: string
): asserts value is NonNullable<T> {
  if (value === undefined || value === null) {
    throw new Error(message || "Value must be defined");
  }
}

const config: { apiKey?: string } = getConfig();
assertDefined(config.apiKey, "API key is required");
// config.apiKey is narrowed to string
makeApiCall(config.apiKey);
```

## Assertion for Array Elements

```typescript
function assertAllStrings(value: unknown[]): asserts value is string[] {
  if (!value.every(item => typeof item === "string")) {
    throw new TypeError("All elements must be strings");
  }
}

const data: unknown[] = ["a", "b", "c"];
assertAllStrings(data);
// data is narrowed to string[]
data.forEach(s => console.log(s.toUpperCase()));
```

## Assertion with Custom Error

```typescript
class ValidationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ValidationError";
  }
}

function assertValidEmail(value: string): asserts value is string {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(value)) {
    throw new ValidationError(`Invalid email: ${value}`);
  }
}

const email: string = "user@example.com";
assertValidEmail(email);
// email is validated and remains string type
sendEmail(email);
```

## Best Practices
- Use assertion functions at API boundaries
- Throw descriptive error messages
- Consider custom error classes for categorization
- Use assertions for invariants that must hold
- Document why assertions are necessary
- Prefer type guards for conditional logic

## Common Pitfalls
- Assertions terminate execution on failure (not for optional validation)
- Don't use assertions for expected error conditions
- Forgetting to handle errors thrown by assertions
- Over-using assertions when type guards are more appropriate
- Assertions can't narrow discriminated unions effectively
