# Generic Type Guards

## Overview
Generic type guards provide reusable, type-safe narrowing functions that work across different types.

## Basic Generic Type Predicate

```typescript
function isArray<T>(value: T | T[]): value is T[] {
  return Array.isArray(value);
}

const value: string | string[] = getSomeValue();
if (isArray(value)) {
  // value is narrowed to string[]
  value.map(s => s.toUpperCase());
}
```

## Generic Property Check

```typescript
function hasProperty<T, K extends string>(
  obj: T,
  key: K
): obj is T & Record<K, unknown> {
  return typeof obj === "object" && obj !== null && key in obj;
}

const data: unknown = { name: "Alice", age: 30 };
if (hasProperty(data, "name")) {
  // data is narrowed to have name property
  console.log(data.name);
}
```

## Generic Instance Check

```typescript
function isInstance<T>(
  value: unknown,
  constructor: new (...args: any[]) => T
): value is T {
  return value instanceof constructor;
}

const val: unknown = new Date();
if (isInstance(val, Date)) {
  // val is narrowed to Date
  console.log(val.getTime());
}
```

## Generic Array Element Check

```typescript
function isArrayOf<T>(
  value: unknown,
  guard: (item: unknown) => item is T
): value is T[] {
  return Array.isArray(value) && value.every(guard);
}

function isString(value: unknown): value is string {
  return typeof value === "string";
}

const data: unknown = ["a", "b", "c"];
if (isArrayOf(data, isString)) {
  // data is narrowed to string[]
  data.forEach(s => console.log(s.toUpperCase()));
}
```

## Generic Optional Value Check

```typescript
function isDefined<T>(value: T | undefined | null): value is T {
  return value !== undefined && value !== null;
}

const values = [1, undefined, 2, null, 3];
const defined = values.filter(isDefined); // number[]
```

## Generic Type Match

```typescript
type TypeMap = {
  string: string;
  number: number;
  boolean: boolean;
};

function isType<K extends keyof TypeMap>(
  value: unknown,
  type: K
): value is TypeMap[K] {
  return typeof value === type;
}

const val: unknown = "hello";
if (isType(val, "string")) {
  // val is narrowed to string
  console.log(val.toUpperCase());
}
```

## Best Practices
- Use descriptive generic parameter names
- Constrain generics when possible for better type safety
- Combine generic guards for complex validation
- Consider performance for hot paths
- Document expected guard behavior

## Common Pitfalls
- Over-constraining generics limits reusability
- Forgetting to check for null/undefined in object guards
- Runtime checks must match type predicate guarantees
- Generic guards can't narrow discriminated unions directly
