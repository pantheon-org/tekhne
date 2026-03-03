# Type Narrowing and Guards

## typeof Guards

Use `typeof` to narrow primitive types:

```typescript
function process(value: string | number) {
  if (typeof value === "string") {
    return value.toUpperCase(); // value is string here
  } else {
    return value.toFixed(2); // value is number here
  }
}
```

## instanceof Guards

Narrow to class instances:

```typescript
class Dog {
  bark() { console.log("Woof!"); }
}

class Cat {
  meow() { console.log("Meow!"); }
}

function speak(animal: Dog | Cat) {
  if (animal instanceof Dog) {
    animal.bark(); // animal is Dog here
  } else {
    animal.meow(); // animal is Cat here
  }
}
```

## in Operator

Check for property existence:

```typescript
interface User {
  name: string;
  email: string;
}

interface Guest {
  name: string;
  sessionId: string;
}

function greet(person: User | Guest) {
  if ("email" in person) {
    console.log(person.email); // person is User here
  } else {
    console.log(person.sessionId); // person is Guest here
  }
}
```

## Custom Type Guards

Functions that return `value is Type`:

```typescript
function isString(value: unknown): value is string {
  return typeof value === "string";
}

function isNumber(value: unknown): value is number {
  return typeof value === "number";
}

function process(value: unknown) {
  if (isString(value)) {
    return value.toUpperCase(); // value is string
  }
  if (isNumber(value)) {
    return value.toFixed(2); // value is number
  }
}
```

## Generic Type Guards

```typescript
function isArray<T>(value: unknown): value is T[] {
  return Array.isArray(value);
}

function isRecord<T extends object>(
  value: unknown
): value is Record<string, T> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
```

## Discriminated Union Narrowing

```typescript
type Success = { status: "success"; data: string };
type Failure = { status: "failure"; error: Error };
type Result = Success | Failure;

function handle(result: Result) {
  switch (result.status) {
    case "success":
      console.log(result.data); // result is Success
      break;
    case "failure":
      console.error(result.error); // result is Failure
      break;
  }
}
```

## Assertion Functions

Functions that throw or narrow types:

```typescript
function assert(condition: unknown): asserts condition {
  if (!condition) {
    throw new Error("Assertion failed");
  }
}

function assertIsString(value: unknown): asserts value is string {
  if (typeof value !== "string") {
    throw new Error("Not a string");
  }
}

function process(value: unknown) {
  assertIsString(value);
  // value is string from this point forward
  return value.toUpperCase();
}
```

## Assertion with Parameter

```typescript
function assertIsDefined<T>(
  value: T,
  message?: string
): asserts value is NonNullable<T> {
  if (value === null || value === undefined) {
    throw new Error(message ?? "Value is null or undefined");
  }
}

function process(value: string | null) {
  assertIsDefined(value);
  // value is string here
  return value.toUpperCase();
}
```

## Equality Narrowing

```typescript
function process(value: string | number, check: string | number) {
  if (value === check) {
    // Both value and check are narrowed to their intersection type
    console.log(value.toFixed(2)); // Only works if both can be number
  }
}
```

## Truthiness Narrowing

```typescript
function process(value: string | null | undefined) {
  if (value) {
    // value is string here (null and undefined are falsy)
    return value.toUpperCase();
  }
}

// Be careful with falsy values
function handleNumber(value: number | null) {
  if (value) {
    // This excludes 0, which might not be intended
    return value.toFixed(2);
  }
}
```

## Null/Undefined Checks

```typescript
function process(value: string | null | undefined) {
  if (value !== null && value !== undefined) {
    return value.toUpperCase(); // value is string
  }
  
  // Or use != (non-strict)
  if (value != null) {
    return value.toUpperCase(); // value is string (excludes null and undefined)
  }
}
```

## Array.isArray Narrowing

```typescript
function process(value: string | string[]) {
  if (Array.isArray(value)) {
    return value.join(", "); // value is string[]
  } else {
    return value.toUpperCase(); // value is string
  }
}
```

## Advanced Patterns

### hasProperty Type Guard Factory

```typescript
function hasProperty<K extends string>(
  obj: unknown,
  key: K
): obj is Record<K, unknown> {
  return typeof obj === "object" && obj !== null && key in obj;
}

function process(value: unknown) {
  if (hasProperty(value, "name") && typeof value.name === "string") {
    console.log(value.name.toUpperCase());
  }
}
```

### Type Predicate with Generic Constraint

```typescript
interface Named {
  name: string;
}

function isNamed<T>(value: T): value is T & Named {
  return (
    typeof value === "object" &&
    value !== null &&
    "name" in value &&
    typeof (value as any).name === "string"
  );
}
```

### Exhaustiveness Checking

```typescript
type Shape = Circle | Square | Triangle;

function getArea(shape: Shape): number {
  switch (shape.kind) {
    case "circle":
      return Math.PI * shape.radius ** 2;
    case "square":
      return shape.size ** 2;
    case "triangle":
      return (shape.base * shape.height) / 2;
    default:
      // Ensures all cases are handled
      const _exhaustive: never = shape;
      throw new Error(`Unhandled shape: ${_exhaustive}`);
  }
}
```

## Control Flow Analysis

TypeScript tracks type narrowing through control flow:

```typescript
function process(value: string | number | null) {
  if (value === null) {
    return "No value";
  }
  // value is string | number here
  
  if (typeof value === "string") {
    return value.toUpperCase();
  }
  // value is number here
  return value.toFixed(2);
}
```

## Best Practices

1. **Prefer type guards over type assertions**
2. **Use discriminated unions with narrowing**
3. **Write custom type guards for complex checks**
4. **Use assertion functions for validation**
5. **Add exhaustiveness checks for union types**
6. **Be explicit with null/undefined checks**

## Common Pitfalls

1. **Don't use truthiness for numbers** (0 is falsy)
2. **Remember that `typeof null === "object"`**
3. **Be careful with `==` vs `===` in narrowing**
4. **Don't forget to check all variants in unions**
5. **Avoid overly broad type guards**
6. **Watch out for type narrowing in async/callback contexts**

## Performance Considerations

- Type guards have no runtime cost (they're just checks)
- Assertion functions throw errors (consider performance in hot paths)
- Complex type guards may impact readability
- Prefer built-in narrowing over custom guards when possible
