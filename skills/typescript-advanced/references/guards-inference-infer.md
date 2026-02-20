# Type Inference with infer

## Overview
The `infer` keyword allows extracting types from other types within conditional type expressions, enabling powerful type-level programming.

## Extracting Function Return Type

```typescript
type ReturnType<T> = T extends (...args: any[]) => infer R ? R : never;

function getUser() {
  return { id: 1, name: "Alice" };
}

type User = ReturnType<typeof getUser>; 
// { id: number; name: string; }
```

## Extracting Function Parameters

```typescript
type Parameters<T> = T extends (...args: infer P) => any ? P : never;

function createUser(name: string, age: number) {
  return { name, age };
}

type CreateUserParams = Parameters<typeof createUser>;
// [name: string, age: number]
```

## Extracting Array Element Type

```typescript
type ArrayElement<T> = T extends (infer E)[] ? E : never;

type StringArray = string[];
type Element = ArrayElement<StringArray>; // string

type MixedArray = (string | number)[];
type MixedElement = ArrayElement<MixedArray>; // string | number
```

## Extracting Promise Value

```typescript
type Awaited<T> = T extends Promise<infer U> ? U : T;

async function fetchUser() {
  return { id: 1, name: "Alice" };
}

type User = Awaited<ReturnType<typeof fetchUser>>;
// { id: number; name: string; }
```

## Extracting Object Property Type

```typescript
type PropType<T, K extends keyof T> = T extends { [P in K]: infer V }
  ? V
  : never;

interface User {
  id: number;
  name: string;
  email: string;
}

type EmailType = PropType<User, "email">; // string
```

## Extracting Constructor Parameters

```typescript
type ConstructorParameters<T> = T extends new (...args: infer P) => any
  ? P
  : never;

class User {
  constructor(public name: string, public age: number) {}
}

type UserConstructorParams = ConstructorParameters<typeof User>;
// [name: string, age: number]
```

## Extracting Instance Type

```typescript
type InstanceType<T> = T extends new (...args: any[]) => infer R ? R : never;

class Database {
  connect() { }
  query(sql: string) { }
}

type DB = InstanceType<typeof Database>;
// Database
```

## Nested infer

```typescript
type DeepArrayElement<T> = T extends (infer E)[]
  ? E extends (infer F)[]
    ? F
    : E
  : never;

type NestedArray = string[][];
type Element = DeepArrayElement<NestedArray>; // string
```

## Inferring Tuple Elements

```typescript
type First<T> = T extends [infer F, ...any[]] ? F : never;
type Rest<T> = T extends [any, ...infer R] ? R : never;

type Tuple = [string, number, boolean];
type FirstElement = First<Tuple>; // string
type RestElements = Rest<Tuple>; // [number, boolean]
```

## Inferring Function This Type

```typescript
type ThisType<T> = T extends (this: infer U, ...args: any[]) => any
  ? U
  : never;

interface Database {
  query(this: Database, sql: string): void;
}

type DB = ThisType<Database["query"]>; // Database
```

## Best Practices
- Use infer for extracting types from complex structures
- Combine infer with distributive conditional types
- Name inferred types descriptively (U, V, R for Return, P for Parameters)
- Consider built-in utility types before creating custom ones
- Document complex infer usage

## Common Pitfalls
- infer only works within extends clauses of conditional types
- Multiple infer of same type variable creates union
- Can't infer constrained generic types directly
- Over-nesting makes types hard to understand
