# Conditional Types

## Basic Conditional Types

Types that select one of two possible types based on a condition:

```typescript
type IsString<T> = T extends string ? true : false;

type A = IsString<string>;  // true
type B = IsString<number>;  // false

// Useful for filtering
type NonNullable<T> = T extends null | undefined ? never : T;
```

## The `infer` Keyword

Extract types from other types:

```typescript
// Extract return type from function
type ReturnType<T> = T extends (...args: any[]) => infer R ? R : never;

type Fn = () => string;
type Result = ReturnType<Fn>; // string

// Extract array element type
type ElementType<T> = T extends (infer E)[] ? E : never;

type Items = ElementType<string[]>; // string

// Extract Promise result type
type Awaited<T> = T extends Promise<infer U> ? U : T;

type P = Awaited<Promise<string>>; // string
```

## Distributive Conditional Types

Conditional types distribute over union types:

```typescript
type ToArray<T> = T extends any ? T[] : never;

type Result = ToArray<string | number>;
// Distributes to: ToArray<string> | ToArray<number>
// Results in: string[] | number[]

// Prevent distribution with tuple wrapper
type ToArrayNonDist<T> = [T] extends [any] ? T[] : never;

type Result2 = ToArrayNonDist<string | number>;
// Results in: (string | number)[]
```

## Nested Conditional Types

Chain multiple conditions:

```typescript
type TypeName<T> = 
  T extends string ? "string" :
  T extends number ? "number" :
  T extends boolean ? "boolean" :
  T extends undefined ? "undefined" :
  T extends Function ? "function" :
  "object";

type A = TypeName<string>;    // "string"
type B = TypeName<() => void>; // "function"
```

## Inferring Multiple Types

```typescript
// Extract function parameter types
type Parameters<T> = T extends (...args: infer P) => any ? P : never;

type Fn = (a: string, b: number) => void;
type Params = Parameters<Fn>; // [string, number]

// Extract constructor parameters
type ConstructorParameters<T> = T extends new (...args: infer P) => any
  ? P
  : never;

class User {
  constructor(name: string, age: number) {}
}

type UserParams = ConstructorParameters<typeof User>; // [string, number]
```

## Practical Examples

### Filter Union Types

```typescript
type Exclude<T, U> = T extends U ? never : T;
type Extract<T, U> = T extends U ? T : never;

type Numbers = Exclude<string | number | boolean, string>; // number | boolean
type Strings = Extract<string | number | boolean, string>; // string
```

### Deep Readonly

```typescript
type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object
    ? DeepReadonly<T[P]>
    : T[P];
};

interface Config {
  database: {
    host: string;
    port: number;
  };
}

type ReadonlyConfig = DeepReadonly<Config>;
// All properties (including nested) are readonly
```

### Flatten Array Types

```typescript
type Flatten<T> = T extends Array<infer U> ? U : T;

type Flat = Flatten<string[]>;  // string
type NotFlat = Flatten<string>; // string
```

### Function Type Manipulation

```typescript
// Make all parameters optional
type PartialParameters<T extends (...args: any[]) => any> = 
  T extends (...args: infer P) => infer R
    ? (...args: Partial<P>) => R
    : never;

// Add error parameter to function
type WithError<T extends (...args: any[]) => any> =
  T extends (...args: infer P) => infer R
    ? (...args: [...P, Error?]) => R
    : never;
```

## Advanced Patterns

### Type-Safe API Routes

```typescript
type RouteParams<T extends string> =
  T extends `${infer _Start}:${infer Param}/${infer Rest}`
    ? { [K in Param | keyof RouteParams<Rest>]: string }
    : T extends `${infer _Start}:${infer Param}`
    ? { [K in Param]: string }
    : {};

type Params = RouteParams<'/users/:userId/posts/:postId'>;
// { userId: string; postId: string }
```

### Recursive Conditional Types

```typescript
type JSONValue =
  | string
  | number
  | boolean
  | null
  | JSONValue[]
  | { [key: string]: JSONValue };

type JSONCompatible<T> = 
  T extends JSONValue ? T :
  T extends { toJSON(): infer U } ? U :
  never;
```

## Best Practices

1. **Use conditional types for type transformation**
2. **Leverage `infer` to extract type information**
3. **Be aware of distributive behavior**
4. **Use tuple wrappers to prevent distribution when needed**
5. **Keep conditions readable** - extract to separate types if complex

## Common Pitfalls

1. **Don't forget distribution behavior over unions**
2. **Avoid overly complex nested conditionals**
3. **Be careful with `never` in conditional types**
4. **Don't confuse `infer` placement in different contexts**
5. **Watch out for infinite recursion** in recursive conditional types

## Performance Considerations

- Conditional types can be expensive for the compiler
- Deeply nested conditional types slow down type checking
- Consider caching complex conditional type results
- Limit recursion depth (TypeScript has limits)
