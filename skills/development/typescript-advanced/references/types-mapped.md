# Mapped Types

## Basic Mapped Types

Transform properties of existing types:

```typescript
type Readonly<T> = {
  readonly [P in keyof T]: T[P];
};

type Partial<T> = {
  [P in keyof T]?: T[P];
};

type Required<T> = {
  [P in keyof T]-?: T[P]; // Remove optional modifier
};

interface User {
  name: string;
  age?: number;
}

type ReadonlyUser = Readonly<User>;
// { readonly name: string; readonly age?: number }

type CompleteUser = Required<User>;
// { name: string; age: number }
```

## Key Remapping

Transform property keys with `as`:

```typescript
type Getters<T> = {
  [P in keyof T as `get${Capitalize<string & P>}`]: () => T[P];
};

interface Person {
  name: string;
  age: number;
}

type PersonGetters = Getters<Person>;
// {
//   getName: () => string;
//   getAge: () => number;
// }
```

## Filtering Properties

Use conditional types with `never` to filter:

```typescript
type PickByType<T, U> = {
  [P in keyof T as T[P] extends U ? P : never]: T[P];
};

interface Mixed {
  name: string;
  age: number;
  active: boolean;
  score: number;
}

type StringProps = PickByType<Mixed, string>;
// { name: string }

type NumberProps = PickByType<Mixed, number>;
// { age: number; score: number }
```

## Modifiers

Add or remove modifiers:

```typescript
// Remove readonly
type Mutable<T> = {
  -readonly [P in keyof T]: T[P];
};

// Remove optional
type Concrete<T> = {
  [P in keyof T]-?: T[P];
};

// Add both
type Frozen<T> = {
  readonly [P in keyof T]-?: T[P];
};
```

## Nested Mapped Types

Recursively transform nested objects:

```typescript
type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object
    ? DeepPartial<T[P]>
    : T[P];
};

type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object
    ? DeepReadonly<T[P]>
    : T[P];
};

interface Config {
  database: {
    host: string;
    port: number;
    credentials: {
      username: string;
      password: string;
    };
  };
}

type PartialConfig = DeepPartial<Config>;
// All properties at all levels are optional
```

## Template Literal Key Remapping

Combine with template literal types:

```typescript
type EventHandlers<T> = {
  [P in keyof T as `on${Capitalize<string & P>}Change`]: (
    value: T[P]
  ) => void;
};

interface Form {
  username: string;
  email: string;
  age: number;
}

type FormHandlers = EventHandlers<Form>;
// {
//   onUsernameChange: (value: string) => void;
//   onEmailChange: (value: string) => void;
//   onAgeChange: (value: number) => void;
// }
```

## Conditional Mapped Types

```typescript
type Nullable<T> = {
  [P in keyof T]: T[P] | null;
};

type NullableStrings<T> = {
  [P in keyof T]: T[P] extends string ? T[P] | null : T[P];
};

interface Data {
  name: string;
  age: number;
  email: string;
}

type Result = NullableStrings<Data>;
// {
//   name: string | null;
//   age: number;
//   email: string | null;
// }
```

## Advanced Patterns

### Proxied Properties

```typescript
type Proxied<T> = {
  [P in keyof T]: {
    get(): T[P];
    set(value: T[P]): void;
  };
};
```

### Prefixed Properties

```typescript
type Prefix<T, P extends string> = {
  [K in keyof T as `${P}${Capitalize<string & K>}`]: T[K];
};

type User = { name: string; age: number };
type OldUser = Prefix<User, 'old'>;
// { oldName: string; oldAge: number }
```

### Omit by Type

```typescript
type OmitByType<T, U> = {
  [P in keyof T as T[P] extends U ? never : P]: T[P];
};

interface Mixed {
  name: string;
  age: number;
  active: boolean;
}

type NoStrings = OmitByType<Mixed, string>;
// { age: number; active: boolean }
```

### Readonly Keys

```typescript
type ReadonlyKeys<T> = {
  [P in keyof T]-?: (<F>() => F extends { [Q in P]: T[P] } ? 1 : 2) extends
    (<F>() => F extends { -readonly [Q in P]: T[P] } ? 1 : 2)
    ? never
    : P;
}[keyof T];
```

### Required Keys

```typescript
type RequiredKeys<T> = {
  [K in keyof T]-?: {} extends Pick<T, K> ? never : K;
}[keyof T];

type OptionalKeys<T> = {
  [K in keyof T]-?: {} extends Pick<T, K> ? K : never;
}[keyof T];
```

## Practical Examples

### Form State

```typescript
type FormState<T> = {
  [P in keyof T]: {
    value: T[P];
    error?: string;
    touched: boolean;
  };
};
```

### Async Versions

```typescript
type Promisify<T> = {
  [P in keyof T]: T[P] extends (...args: infer A) => infer R
    ? (...args: A) => Promise<R>
    : T[P];
};
```

### Builder Pattern

```typescript
type Builder<T> = {
  [P in keyof T as `set${Capitalize<string & P>}`]: (value: T[P]) => Builder<T>;
} & {
  build(): T;
};
```

## Best Practices

1. **Use built-in mapped types** (Partial, Readonly, Pick, Omit) when possible
2. **Leverage key remapping for type-safe transformations**
3. **Combine with conditional types for filtering**
4. **Use template literals for systematic renaming**
5. **Be careful with recursive mapped types** (performance)

## Common Pitfalls

1. **Don't map over union types** - use distributive conditional types instead
2. **Avoid overly complex key remapping logic**
3. **Be careful with recursive depth limits**
4. **Don't forget about modifier removal** (-readonly, -?)
5. **Watch out for preserving optional modifiers**

## Performance Considerations

- Mapped types can be expensive for large types
- Recursive mapped types multiply compilation time
- Consider caching results for frequently-used transformations
- Keep nesting depth reasonable
