# Advanced Generics Patterns

Master complex generic type patterns for flexible, reusable code.

## Generic Constraints with keyof

```typescript
function getProperty<T, K extends keyof T>(obj: T, key: K): T[K] {
  return obj[key];
}

const user = { name: 'Alice', age: 30 };
const name = getProperty(user, 'name'); // string
const age = getProperty(user, 'age'); // number
// getProperty(user, 'invalid'); // Error
```

## Multiple Type Parameters with Constraints

```typescript
function merge<T extends object, U extends object>(obj1: T, obj2: U): T & U {
  return { ...obj1, ...obj2 };
}

const result = merge(
  { name: 'Alice' },
  { age: 30 }
); // { name: string; age: number }
```

## Conditional Type Parameters

```typescript
type ApiResponse<T, E = Error> = 
  | { success: true; data: T }
  | { success: false; error: E };

function handleResponse<T, E = Error>(
  response: ApiResponse<T, E>
): T {
  if (response.success) {
    return response.data;
  }
  throw response.error;
}
```

## Generic Type Inference

```typescript
function createPair<T, U>(first: T, second: U): [T, U] {
  return [first, second];
}

// TypeScript infers types
const pair1 = createPair('hello', 42); // [string, number]
const pair2 = createPair(true, { x: 10 }); // [boolean, { x: number }]
```

## Generic Factories

```typescript
interface Factory<T> {
  create(...args: any[]): T;
}

class UserFactory implements Factory<User> {
  create(name: string, email: string): User {
    return { id: generateId(), name, email };
  }
}

class ProductFactory implements Factory<Product> {
  create(name: string, price: number): Product {
    return { id: generateId(), name, price };
  }
}

function register<T>(factory: Factory<T>): void {
  // Register factory
}
```

## Higher-Kinded Types Simulation

```typescript
interface HKT {
  readonly _URI?: unknown;
  readonly _A?: unknown;
}

interface URItoKind<A> {
  Option: Option<A>;
  Either: Either<Error, A>;
  Array: Array<A>;
}

type Kind<URI extends keyof URItoKind<any>, A> = URItoKind<A>[URI];

// Functor interface
interface Functor<F extends keyof URItoKind<any>> {
  map<A, B>(fa: Kind<F, A>, f: (a: A) => B): Kind<F, B>;
}

// Option functor
type Option<A> = { _tag: 'Some'; value: A } | { _tag: 'None' };

const OptionFunctor: Functor<'Option'> = {
  map(fa, f) {
    return fa._tag === 'Some'
      ? { _tag: 'Some', value: f(fa.value) }
      : { _tag: 'None' };
  }
};
```

## Variadic Generic Functions

```typescript
function compose<A, B, C>(
  f: (b: B) => C,
  g: (a: A) => B
): (a: A) => C {
  return (a) => f(g(a));
}

const addOne = (n: number) => n + 1;
const double = (n: number) => n * 2;
const addOneAndDouble = compose(double, addOne);

addOneAndDouble(5); // 12
```

## Generic Builder Pattern

```typescript
class QueryBuilder<T, Selected extends keyof T = never> {
  private selectedFields: Set<keyof T> = new Set();
  private whereClause?: string;
  
  select<K extends keyof T>(
    ...fields: K[]
  ): QueryBuilder<T, Selected | K> {
    fields.forEach(f => this.selectedFields.add(f));
    return this as any;
  }
  
  where(clause: string): this {
    this.whereClause = clause;
    return this;
  }
  
  build(): Pick<T, Selected> {
    // Build query
    return {} as Pick<T, Selected>;
  }
}

interface User {
  id: string;
  name: string;
  email: string;
  age: number;
}

const query = new QueryBuilder<User>()
  .select('name', 'email')
  .where('age > 18')
  .build(); // { name: string; email: string }
```

## Recursive Generic Constraints

```typescript
type Flatten<T> = T extends Array<infer U>
  ? U extends Array<any>
    ? Flatten<U>
    : U
  : T;

type Result1 = Flatten<number>; // number
type Result2 = Flatten<number[]>; // number
type Result3 = Flatten<number[][]>; // number
type Result4 = Flatten<number[][][]>; // number
```

## Generic Type Guards

```typescript
function isArrayOf<T>(
  arr: unknown,
  guard: (item: unknown) => item is T
): arr is T[] {
  return Array.isArray(arr) && arr.every(guard);
}

function isString(value: unknown): value is string {
  return typeof value === 'string';
}

function isNumber(value: unknown): value is number {
  return typeof value === 'number';
}

const data: unknown = ['a', 'b', 'c'];

if (isArrayOf(data, isString)) {
  data.forEach(s => s.toUpperCase()); // OK, data is string[]
}
```

## Best Practices

1. **Infer when possible**: Let TypeScript infer generics
2. **Constrain generics**: Use extends for better type safety
3. **Default type parameters**: Provide sensible defaults
4. **Avoid over-generalization**: Don't make everything generic
5. **Name meaningfully**: Use descriptive type parameter names
6. **Document constraints**: Explain complex generic constraints
