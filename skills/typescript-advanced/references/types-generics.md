# Generics

## Basic Generics

Type variables that allow components to work with any type:

```typescript
function identity<T>(value: T): T {
  return value;
}

const num = identity(42);        // T inferred as number
const str = identity("hello");   // T inferred as string
```

## Generic Constraints

Restrict type parameters with `extends`:

```typescript
interface HasLength {
  length: number;
}

function logLength<T extends HasLength>(value: T): void {
  console.log(value.length);
}

logLength("hello");     // OK
logLength([1, 2, 3]);   // OK
logLength({ length: 5 }); // OK
// logLength(42);       // Error: number has no length
```

## Multiple Type Parameters

```typescript
function pair<T, U>(first: T, second: U): [T, U] {
  return [first, second];
}

const result = pair("age", 30); // [string, number]

// With constraints
function merge<T extends object, U extends object>(
  obj1: T,
  obj2: U
): T & U {
  return { ...obj1, ...obj2 };
}
```

## Generic Interfaces

```typescript
interface Container<T> {
  value: T;
  map<U>(fn: (value: T) => U): Container<U>;
}

interface Response<T> {
  data: T;
  status: number;
  headers: Record<string, string>;
}

const userResponse: Response<User> = {
  data: { id: 1, name: 'Alice' },
  status: 200,
  headers: {}
};
```

## Generic Classes

```typescript
class Box<T> {
  constructor(private value: T) {}
  
  getValue(): T {
    return this.value;
  }
  
  map<U>(fn: (value: T) => U): Box<U> {
    return new Box(fn(this.value));
  }
}

const numBox = new Box(42);
const strBox = numBox.map(n => n.toString());
```

## Default Type Parameters

```typescript
interface Config<T = string> {
  value: T;
}

const stringConfig: Config = { value: "hello" };
const numberConfig: Config<number> = { value: 42 };

// Useful for API responses
interface ApiResponse<T = unknown> {
  data: T;
  error?: string;
}
```

## Generic Type Aliases

```typescript
type Result<T, E = Error> = 
  | { success: true; value: T }
  | { success: false; error: E };

type Nullable<T> = T | null;
type AsyncFn<T, Args extends any[]> = (...args: Args) => Promise<T>;
```

## Constrained Keys

```typescript
function getProperty<T, K extends keyof T>(obj: T, key: K): T[K] {
  return obj[key];
}

const user = { name: 'Alice', age: 30 };
const name = getProperty(user, 'name'); // string
const age = getProperty(user, 'age');   // number
// getProperty(user, 'invalid');        // Error
```

## Generic Inference

TypeScript infers generic types from usage:

```typescript
// Inferred from argument
function wrap<T>(value: T) {
  return { value };
}

const wrapped = wrap(42); // { value: number }

// Inferred from return type
function parse<T>(json: string): T {
  return JSON.parse(json);
}

const user: User = parse('{"name":"Alice"}'); // T inferred as User
```

## Advanced Generic Patterns

### Builder Pattern

```typescript
class QueryBuilder<T> {
  private conditions: string[] = [];
  
  where(condition: string): QueryBuilder<T> {
    this.conditions.push(condition);
    return this;
  }
  
  execute(): T[] {
    // Execute query
    return [];
  }
}
```

### Type-Safe Event Emitter

```typescript
type EventMap = Record<string, any>;

class TypedEmitter<Events extends EventMap> {
  on<K extends keyof Events>(
    event: K,
    handler: (payload: Events[K]) => void
  ): void {
    // Implementation
  }
  
  emit<K extends keyof Events>(event: K, payload: Events[K]): void {
    // Implementation
  }
}

interface AppEvents {
  'user:login': { userId: string; timestamp: Date };
  'user:logout': { userId: string };
}

const emitter = new TypedEmitter<AppEvents>();
emitter.on('user:login', (payload) => {
  // payload is correctly typed as { userId: string; timestamp: Date }
});
```

## Best Practices

1. **Use descriptive generic names** (T for type, K for key, V for value)
2. **Add constraints to generics when possible**
3. **Leverage type inference** instead of explicit type arguments
4. **Use default type parameters for common cases**
5. **Avoid over-constraining generics**

## Common Pitfalls

1. **Don't use generics when concrete types suffice**
2. **Avoid too many type parameters** (>3 is often a code smell)
3. **Don't forget to constrain when needed**
4. **Avoid losing type information** by being too generic
