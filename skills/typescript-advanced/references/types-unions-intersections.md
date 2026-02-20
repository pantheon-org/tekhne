# Union and Intersection Types

## Union Types

Represent values that can be one of several types:

```typescript
type Result = Success | Failure;
type ID = string | number;
type Status = 'pending' | 'approved' | 'rejected';

function handleResult(result: Result) {
  // Type narrowing required to access specific properties
  if ('data' in result) {
    console.log(result.data);
  }
}
```

## Intersection Types

Combine multiple types into one:

```typescript
type Named = { name: string };
type Aged = { age: number };
type Person = Named & Aged;

const person: Person = {
  name: 'Alice',
  age: 30
};

// Useful for mixins
type Timestamped = { createdAt: Date; updatedAt: Date };
type User = Person & Timestamped;
```

## Discriminated Unions

Union types with a common discriminant property for type-safe narrowing:

```typescript
type Success = {
  kind: 'success';
  data: string;
};

type Failure = {
  kind: 'failure';
  error: Error;
};

type Result = Success | Failure;

function handleResult(result: Result) {
  switch (result.kind) {
    case 'success':
      console.log(result.data); // TypeScript knows this is Success
      break;
    case 'failure':
      console.error(result.error); // TypeScript knows this is Failure
      break;
  }
}
```

## State Machines with Discriminated Unions

Model state machines type-safely:

```typescript
type IdleState = { status: 'idle' };
type LoadingState = { status: 'loading'; startedAt: Date };
type SuccessState = { status: 'success'; data: string };
type ErrorState = { status: 'error'; error: Error };

type State = IdleState | LoadingState | SuccessState | ErrorState;

function transition(state: State): State {
  switch (state.status) {
    case 'idle':
      return { status: 'loading', startedAt: new Date() };
    case 'loading':
      // Can access startedAt here
      return Math.random() > 0.5
        ? { status: 'success', data: 'result' }
        : { status: 'error', error: new Error('Failed') };
    case 'success':
    case 'error':
      return { status: 'idle' };
  }
}
```

## Intersection vs Extension

```typescript
// Intersection - combines types
type A = { a: string };
type B = { b: number };
type C = A & B; // { a: string; b: number }

// Extension - interface inheritance
interface D extends A, B {
  c: boolean;
}

// Intersection allows for more flexible composition
type Mixed = A & { c: boolean } & B;
```

## Best Practices

1. **Use discriminated unions for mutually exclusive states**
2. **Prefer union types for OR relationships**
3. **Use intersection types for AND relationships**
4. **Add exhaustiveness checks with never**
5. **Use literal types in discriminants**

## Common Patterns

### Optional Properties

```typescript
type Config = {
  required: string;
} & (
  | { mode: 'development' }
  | { mode: 'production'; apiKey: string }
);
```

### Branded Types with Intersection

```typescript
type Brand<T, B> = T & { __brand: B };
type UserId = Brand<string, 'UserId'>;
type Email = Brand<string, 'Email'>;

// UserId and Email are incompatible despite both being strings
```

## Anti-Patterns

1. **Don't use unions when discriminated unions are appropriate**
2. **Avoid deep intersection nesting**
3. **Don't forget exhaustiveness checks**
4. **Avoid conflicts in intersection types**
