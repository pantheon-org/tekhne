# NonNullable, Awaited & Other Utility Types

Utilities for handling nullable types and promises.

## NonNullable<T>

Removes `null` and `undefined` from a type.

```typescript
type MaybeString = string | null | undefined;
type DefiniteString = NonNullable<MaybeString>; // string

type MaybeUser = { name: string } | null | undefined;
type DefiniteUser = NonNullable<MaybeUser>; // { name: string }
```

### Practical Usage

```typescript
function processValue<T>(value: T): NonNullable<T> {
  if (value === null || value === undefined) {
    throw new Error('Value cannot be null or undefined');
  }
  return value;
}

// Filter nulls from arrays
function filterNulls<T>(arr: (T | null | undefined)[]): NonNullable<T>[] {
  return arr.filter((item): item is NonNullable<T> => 
    item !== null && item !== undefined
  );
}
```

## Awaited<T>

Recursively unwraps Promise types.

```typescript
type P1 = Promise<string>;
type R1 = Awaited<P1>; // string

type P2 = Promise<Promise<number>>;
type R2 = Awaited<P2>; // number (recursive unwrap)

type P3 = Promise<{ data: Promise<User> }>;
type R3 = Awaited<P3>; // { data: Promise<User> } (shallow unwrap)
```

### With Async Functions

```typescript
async function fetchUser(): Promise<{ id: string; name: string }> {
  // Implementation
  return { id: '1', name: 'John' };
}

type User = Awaited<ReturnType<typeof fetchUser>>; 
// { id: string; name: string }
```

### Handling Promise Arrays

```typescript
type PromiseArray = Promise<User>[];
type ResolvedArray = Awaited<typeof Promise.all<PromiseArray>>;
// User[]

async function fetchAll<T>(
  promises: Promise<T>[]
): Promise<Awaited<T>[]> {
  return Promise.all(promises);
}
```

## Uppercase, Lowercase, Capitalize, Uncapitalize

String manipulation utilities.

```typescript
type Greeting = 'hello world';

type Loud = Uppercase<Greeting>; // 'HELLO WORLD'
type Quiet = Lowercase<Greeting>; // 'hello world'
type Proper = Capitalize<Greeting>; // 'Hello world'
type Casual = Uncapitalize<'Hello'>; // 'hello'
```

### Template Literal Combinations

```typescript
type HTTPMethod = 'get' | 'post' | 'put' | 'delete';
type UpperMethod = Uppercase<HTTPMethod>;
// 'GET' | 'POST' | 'PUT' | 'DELETE'

type EventName = 'click' | 'focus' | 'blur';
type HandlerName = `on${Capitalize<EventName>}`;
// 'onClick' | 'onFocus' | 'onBlur'
```

## Practical Patterns

### Safe Async Operations

```typescript
type AsyncResult<T> = Promise<
  | { success: true; data: T }
  | { success: false; error: Error }
>;

async function safeAsync<T>(
  promise: Promise<T>
): AsyncResult<T> {
  try {
    const data = await promise;
    return { success: true, data };
  } catch (error) {
    return { success: false, error: error as Error };
  }
}

// Extract successful data type
type SuccessData<T> = T extends { success: true; data: infer D }
  ? D
  : never;

type UserData = Awaited<ReturnType<typeof fetchUser>>;
```

### Non-Nullable Object Properties

```typescript
type User = {
  id: string;
  name: string | null;
  email?: string;
  age: number | undefined;
};

type RequiredUser = {
  [K in keyof User]-?: NonNullable<User[K]>
};
// { id: string; name: string; email: string; age: number }
```

### Promise Resolution Types

```typescript
interface APIEndpoints {
  getUser: () => Promise<User>;
  getPosts: () => Promise<Post[]>;
  getComments: (postId: string) => Promise<Comment[]>;
}

type ResolvedAPI = {
  [K in keyof APIEndpoints]: Awaited<ReturnType<APIEndpoints[K]>>
};
// { getUser: User; getPosts: Post[]; getComments: Comment[] }
```

### Event Name Transformations

```typescript
type DOMEvent = 'click' | 'mouseenter' | 'focus';

type EventHandler<E extends string> = {
  [K in E as `on${Capitalize<K>}`]: (event: Event) => void
};

type Handlers = EventHandler<DOMEvent>;
// {
//   onClick: (event: Event) => void;
//   onMouseenter: (event: Event) => void;
//   onFocus: (event: Event) => void;
// }
```

## Best Practices

1. **Use NonNullable with type guards**: Narrow types safely
2. **Awaited for async return types**: Extract promise values
3. **String utilities for type safety**: Generate consistent naming
4. **Combine with conditional types**: Build flexible utilities
5. **Document unwrapping behavior**: Clarify recursive vs shallow

## Common Use Cases

### Form Validation

```typescript
type FormData = {
  username: string | null;
  email: string | null;
  age: number | null;
};

type ValidatedForm = {
  [K in keyof FormData]: NonNullable<FormData[K]>
};

function validateForm(data: FormData): ValidatedForm {
  const validated: Partial<ValidatedForm> = {};
  for (const key in data) {
    const value = data[key as keyof FormData];
    if (value === null) {
      throw new Error(`${key} is required`);
    }
    (validated as any)[key] = value;
  }
  return validated as ValidatedForm;
}
```

### Async Data Fetching

```typescript
type DataFetcher = {
  user: () => Promise<User>;
  posts: (userId: string) => Promise<Post[]>;
  comments: (postId: string) => Promise<Comment[]>;
};

type FetcherResults = {
  [K in keyof DataFetcher]: Awaited<ReturnType<DataFetcher[K]>>
};
// { user: User; posts: Post[]; comments: Comment[] }
```

## Anti-Patterns

❌ **Manual null checks without types**
```typescript
function bad(value: string | null) {
  if (value !== null) {
    // TypeScript doesn't know value is string here
    return value;
  }
}
```

✅ **Type guards with NonNullable**
```typescript
function good<T>(value: T): NonNullable<T> {
  if (value === null || value === undefined) {
    throw new Error('Null value');
  }
  return value; // TypeScript knows this is NonNullable<T>
}
```

❌ **Ignoring nested promises**
```typescript
type Bad = ReturnType<typeof async1>; // Promise<Promise<T>>
```

✅ **Use Awaited**
```typescript
type Good = Awaited<ReturnType<typeof async1>>; // T
```
