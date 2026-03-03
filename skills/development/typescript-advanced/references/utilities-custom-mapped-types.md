# Custom Mapped Types

Creating reusable type transformations with mapped types.

## Basic Mapped Type Pattern

```typescript
type Mapped<T> = {
  [K in keyof T]: T[K]
};

// With transformation
type Nullable<T> = {
  [K in keyof T]: T[K] | null
};

type Optional<T> = {
  [K in keyof T]?: T[K]
};
```

## Modifier Flags

### Adding/Removing readonly

```typescript
// Add readonly
type Immutable<T> = {
  +readonly [K in keyof T]: T[K]
};

// Remove readonly
type Mutable<T> = {
  -readonly [K in keyof T]: T[K]
};
```

### Adding/Removing optional

```typescript
// Make all optional
type AllOptional<T> = {
  [K in keyof T]?: T[K]
};

// Make all required
type AllRequired<T> = {
  [K in keyof T]-?: T[K]
};
```

## Advanced Mapped Types

### Getters

```typescript
type Getters<T> = {
  [K in keyof T as `get${Capitalize<string & K>}`]: () => T[K]
};

type User = { name: string; age: number };
type UserGetters = Getters<User>;
// { getName: () => string; getAge: () => number }
```

### Setters

```typescript
type Setters<T> = {
  [K in keyof T as `set${Capitalize<string & K>}`]: (value: T[K]) => void
};

type UserSetters = Setters<User>;
// { setName: (value: string) => void; setAge: (value: number) => void }
```

### Combined Accessors

```typescript
type Accessors<T> = Getters<T> & Setters<T>;

class UserClass implements Accessors<User> {
  private data: User = { name: '', age: 0 };
  
  getName() { return this.data.name; }
  setName(value: string) { this.data.name = value; }
  getAge() { return this.data.age; }
  setAge(value: number) { this.data.age = value; }
}
```

## Promisify

Convert sync functions to async.

```typescript
type Promisify<T> = {
  [K in keyof T]: T[K] extends (...args: infer A) => infer R
    ? (...args: A) => Promise<R>
    : T[K]
};

interface SyncAPI {
  getUser: (id: string) => User;
  createUser: (data: UserInput) => User;
}

type AsyncAPI = Promisify<SyncAPI>;
// {
//   getUser: (id: string) => Promise<User>;
//   createUser: (data: UserInput) => Promise<User>;
// }
```

## DeepReadonly & DeepPartial

Recursive type transformations.

```typescript
type DeepReadonly<T> = {
  readonly [K in keyof T]: T[K] extends object
    ? DeepReadonly<T[K]>
    : T[K]
};

type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends object
    ? DeepPartial<T[K]>
    : T[K]
};

type Config = {
  database: {
    host: string;
    port: number;
    credentials: {
      username: string;
      password: string;
    };
  };
};

type ImmutableConfig = DeepReadonly<Config>;
// All properties readonly, including nested
```

## Filtering Properties

### By Type

```typescript
type PickByType<T, U> = {
  [K in keyof T as T[K] extends U ? K : never]: T[K]
};

type OmitByType<T, U> = {
  [K in keyof T as T[K] extends U ? never : K]: T[K]
};

interface Mixed {
  id: string;
  name: string;
  age: number;
  active: boolean;
}

type StringProps = PickByType<Mixed, string>;
// { id: string; name: string }

type NotStrings = OmitByType<Mixed, string>;
// { age: number; active: boolean }
```

### By Key Pattern

```typescript
type PickByKeyPrefix<T, Prefix extends string> = {
  [K in keyof T as K extends `${Prefix}${string}` ? K : never]: T[K]
};

interface User {
  id: string;
  name: string;
  isActive: boolean;
  isAdmin: boolean;
  createdAt: Date;
}

type BooleanFlags = PickByKeyPrefix<User, 'is'>;
// { isActive: boolean; isAdmin: boolean }
```

## Practical Patterns

### API Response Wrapper

```typescript
type APIResponse<T> = {
  [K in keyof T]: {
    data: T[K];
    loading: boolean;
    error: Error | null;
  }
};

interface Data {
  user: User;
  posts: Post[];
}

type WrappedData = APIResponse<Data>;
// {
//   user: { data: User; loading: boolean; error: Error | null };
//   posts: { data: Post[]; loading: boolean; error: Error | null };
// }
```

### Event Handlers

```typescript
type EventHandlers<T> = {
  [K in keyof T as `on${Capitalize<string & K>}Change`]: (
    value: T[K]
  ) => void
};

interface FormData {
  username: string;
  email: string;
  age: number;
}

type FormHandlers = EventHandlers<FormData>;
// {
//   onUsernameChange: (value: string) => void;
//   onEmailChange: (value: string) => void;
//   onAgeChange: (value: number) => void;
// }
```

### Validation Schema

```typescript
type ValidationSchema<T> = {
  [K in keyof T]: {
    required: boolean;
    validator: (value: T[K]) => boolean;
    message: string;
  }
};

const userSchema: ValidationSchema<User> = {
  name: {
    required: true,
    validator: (v) => v.length > 0,
    message: 'Name is required'
  },
  age: {
    required: true,
    validator: (v) => v > 0,
    message: 'Age must be positive'
  }
};
```

## Boxed Types

Wrap primitives in objects.

```typescript
type Boxed<T> = {
  [K in keyof T]: { value: T[K] }
};

type Unbox<T> = {
  [K in keyof T]: T[K] extends { value: infer V } ? V : T[K]
};

interface Config {
  port: number;
  host: string;
}

type BoxedConfig = Boxed<Config>;
// { port: { value: number }; host: { value: string } }

type UnboxedConfig = Unbox<BoxedConfig>;
// { port: number; host: string }
```

## Best Practices

1. **Name mapped types clearly**: Use descriptive names like `Promisify`, `Getters`
2. **Document transformations**: Explain what the mapped type does
3. **Limit recursion depth**: Deep recursion can impact performance
4. **Use with const assertions**: Preserve literal types
5. **Test with complex types**: Ensure mapped types work with edge cases

## Common Patterns

### State Management

```typescript
type StateSlice<T> = {
  [K in keyof T as `set${Capitalize<string & K>}`]: (value: T[K]) => void
} & {
  [K in keyof T as `get${Capitalize<string & K>}`]: () => T[K]
} & {
  [K in keyof T as `reset${Capitalize<string & K>}`]: () => void
};
```

### Dependency Injection

```typescript
type Injectable<T> = {
  [K in keyof T]: T[K] extends (...args: infer A) => infer R
    ? (...args: [...A, dependencies: Record<string, any>]) => R
    : T[K]
};
```

## Anti-Patterns

❌ **Overly complex mapped types**
```typescript
type Bad<T> = {
  [K in keyof T as K extends string
    ? T[K] extends number
      ? `num_${K}`
      : T[K] extends string
      ? `str_${K}`
      : K
    : K]: T[K]
};
```

✅ **Split into smaller utilities**
```typescript
type PrefixNumbers<T> = { [K in keyof T as /* ... */]: T[K] };
type PrefixStrings<T> = { [K in keyof T as /* ... */]: T[K] };
type Good<T> = PrefixNumbers<T> & PrefixStrings<T>;
```
