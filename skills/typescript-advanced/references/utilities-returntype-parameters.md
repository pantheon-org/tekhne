# ReturnType, Parameters & Function Utility Types

Function type extraction utilities for working with function signatures.

## ReturnType<T>

Extracts the return type of a function type.

```typescript
type GetUser = () => { id: string; name: string };
type User = ReturnType<GetUser>; // { id: string; name: string }

function fetchData(): Promise<{ data: string[] }> {
  return Promise.resolve({ data: [] });
}
type FetchResult = ReturnType<typeof fetchData>; // Promise<{ data: string[] }>
```

## Parameters<T>

Extracts function parameter types as a tuple.

```typescript
function createUser(name: string, age: number, active: boolean) {
  return { name, age, active };
}
type CreateUserParams = Parameters<typeof createUser>; // [string, number, boolean]

// Use in wrapper functions
function logAndCreate(...args: Parameters<typeof createUser>) {
  console.log('Creating user:', args);
  return createUser(...args);
}
```

## ConstructorParameters<T>

Extracts constructor parameter types.

```typescript
class User {
  constructor(public name: string, public age: number) {}
}
type UserConstructorParams = ConstructorParameters<typeof User>; // [string, number]

// Factory pattern
function createInstance<T extends new (...args: any[]) => any>(
  ctor: T,
  ...args: ConstructorParameters<T>
): InstanceType<T> {
  return new ctor(...args);
}
```

## InstanceType<T>

Extracts the instance type of a constructor function.

```typescript
class Database {
  connect() {}
  query(sql: string) {}
}
type DB = InstanceType<typeof Database>; // Database

// With generic constructors
function createService<T extends new () => any>(
  Service: T
): InstanceType<T> {
  return new Service();
}
```

## ThisParameterType<T>

Extracts the type of `this` parameter from a function.

```typescript
interface User {
  name: string;
  greet(this: User): void;
}

type UserThis = ThisParameterType<User['greet']>; // User
```

## OmitThisParameter<T>

Removes `this` parameter from function type.

```typescript
function greet(this: User, message: string) {
  console.log(`${this.name}: ${message}`);
}

type GreetFn = OmitThisParameter<typeof greet>; // (message: string) => void
```

## Practical Patterns

### API Client Type Safety

```typescript
interface API {
  getUser: (id: string) => Promise<User>;
  createUser: (data: UserInput) => Promise<User>;
  updateUser: (id: string, data: Partial<UserInput>) => Promise<User>;
}

// Extract all parameter types
type APIParams = {
  [K in keyof API]: Parameters<API[K]>
};
// Result: { getUser: [string], createUser: [UserInput], ... }

// Extract all return types
type APIReturns = {
  [K in keyof API]: ReturnType<API[K]>
};
```

### Function Composition

```typescript
function compose<A extends any[], B, C>(
  f: (...args: A) => B,
  g: (b: B) => C
): (...args: A) => C {
  return (...args: A) => g(f(...args));
}

const add = (a: number, b: number) => a + b;
const toString = (n: number) => n.toString();

const addAndStringify = compose(add, toString);
// Type: (a: number, b: number) => string
```

### Callback Type Extraction

```typescript
function addEventListener(
  event: string,
  callback: (e: MouseEvent) => void
): void {
  // Implementation
}

type EventCallback = Parameters<typeof addEventListener>[1];
// (e: MouseEvent) => void
```

## Best Practices

1. **Use with typeof for values**: `ReturnType<typeof fn>` not `ReturnType<fn>`
2. **Prefer over manual extraction**: Let TypeScript infer types from implementation
3. **Combine with generics**: Create flexible utility functions
4. **Document extracted types**: Add type aliases for clarity
5. **Use in factory patterns**: Ensure type safety in creation functions

## Common Patterns

### Type-Safe Event Emitters

```typescript
type EventMap = {
  'user:created': (user: User) => void;
  'user:updated': (id: string, user: User) => void;
  'user:deleted': (id: string) => void;
};

class EventEmitter<T extends Record<string, (...args: any[]) => void>> {
  on<K extends keyof T>(
    event: K,
    callback: T[K]
  ): void {}
  
  emit<K extends keyof T>(
    event: K,
    ...args: Parameters<T[K]>
  ): void {}
}

const emitter = new EventEmitter<EventMap>();
emitter.on('user:created', (user) => {}); // user: User
emitter.emit('user:updated', '123', user); // Type-safe
```

### Middleware Typing

```typescript
type Middleware<Req, Res> = (
  req: Req,
  res: Res,
  next: () => void
) => void;

type ExtractMiddlewareReq<T> = T extends Middleware<infer R, any> ? R : never;
type ExtractMiddlewareRes<T> = T extends Middleware<any, infer R> ? R : never;
```

## Anti-Patterns

❌ **Manual type extraction**
```typescript
type BadReturn = { data: string[] }; // Hardcoded
```

✅ **Use ReturnType**
```typescript
type GoodReturn = ReturnType<typeof fetchData>; // Derived
```

❌ **Ignoring rest parameters**
```typescript
function wrapper(first: string) {} // Missing ...rest
```

✅ **Spread Parameters**
```typescript
function wrapper(...args: Parameters<typeof original>) {
  return original(...args);
}
```
