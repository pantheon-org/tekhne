# Key Remapping in Mapped Types

Advanced property key transformations using `as` clause.

## Basic Key Remapping

```typescript
type Getters<T> = {
  [K in keyof T as `get${Capitalize<string & K>}`]: () => T[K]
};

type User = { name: string; age: number };
type UserGetters = Getters<User>;
// { getName: () => string; getAge: () => number }
```

## Filtering Properties

### Remove Keys

```typescript
type RemoveKind<T> = {
  [K in keyof T as Exclude<K, 'kind'>]: T[K]
};

type WithKind = { kind: 'user'; name: string; age: number };
type WithoutKind = RemoveKind<WithKind>;
// { name: string; age: number }
```

### Filter by Type

```typescript
type PickByType<T, U> = {
  [K in keyof T as T[K] extends U ? K : never]: T[K]
};

interface Mixed {
  id: string;
  name: string;
  count: number;
  active: boolean;
}

type OnlyStrings = PickByType<Mixed, string>;
// { id: string; name: string }
```

## Prefix and Suffix

```typescript
type AddPrefix<T, Prefix extends string> = {
  [K in keyof T as `${Prefix}${string & K}`]: T[K]
};

type AddSuffix<T, Suffix extends string> = {
  [K in keyof T as `${string & K}${Suffix}`]: T[K]
};

type Stats = { views: number; likes: number };

type PrefixedStats = AddPrefix<Stats, 'total'>;
// { totalviews: number; totallikes: number }

type SuffixedStats = AddSuffix<Stats, 'Count'>;
// { viewsCount: number; likesCount: number }
```

## Case Transformations

```typescript
type Capitalize<S extends string> = 
  S extends `${infer F}${infer R}` ? `${Uppercase<F>}${R}` : S;

type CamelToSnake<S extends string> = 
  S extends `${infer T}${infer U}`
    ? U extends Uncapitalize<U>
      ? `${Lowercase<T>}${CamelToSnake<U>}`
      : `${Lowercase<T>}_${CamelToSnake<U>}`
    : S;

type SnakeToCamel<S extends string> =
  S extends `${infer T}_${infer U}`
    ? `${T}${Capitalize<SnakeToCamel<U>>}`
    : S;

type ToSnakeCase<T> = {
  [K in keyof T as CamelToSnake<string & K>]: T[K]
};

type ToCamelCase<T> = {
  [K in keyof T as SnakeToCamel<string & K>]: T[K]
};

type JS = { userId: string; firstName: string };
type DB = ToSnakeCase<JS>;
// { user_id: string; first_name: string }
```

## Conditional Remapping

```typescript
type RenameKey<T, Old extends keyof T, New extends string> = {
  [K in keyof T as K extends Old ? New : K]: T[K]
};

type User = { id: string; name: string };
type UserWithUUID = RenameKey<User, 'id', 'uuid'>;
// { uuid: string; name: string }
```

## Event Name Transformations

```typescript
type EventHandlers<T> = {
  [K in keyof T as `on${Capitalize<string & K>}Change`]: (
    value: T[K],
    oldValue: T[K]
  ) => void
};

interface FormState {
  username: string;
  email: string;
  age: number;
}

type FormHandlers = EventHandlers<FormState>;
// {
//   onUsernameChange: (value: string, oldValue: string) => void;
//   onEmailChange: (value: string, oldValue: string) => void;
//   onAgeChange: (value: number, oldValue: number) => void;
// }
```

## Method Generation

### CRUD Operations

```typescript
type CRUDOperations<T, IdKey extends keyof T> = {
  [K in keyof T as `create${Capitalize<string & K>}`]: (
    value: T[K]
  ) => Promise<T>
} & {
  [K in keyof T as `read${Capitalize<string & K>}`]: (
    id: T[IdKey]
  ) => Promise<T>
} & {
  [K in keyof T as `update${Capitalize<string & K>}`]: (
    id: T[IdKey],
    value: Partial<T[K]>
  ) => Promise<T>
} & {
  [K in keyof T as `delete${Capitalize<string & K>}`]: (
    id: T[IdKey]
  ) => Promise<void>
};
```

### Validators

```typescript
type Validators<T> = {
  [K in keyof T as `validate${Capitalize<string & K>}`]: (
    value: T[K]
  ) => boolean
} & {
  [K in keyof T as `is${Capitalize<string & K>}Valid`]: boolean
};

interface UserInput {
  email: string;
  password: string;
}

type UserValidators = Validators<UserInput>;
// {
//   validateEmail: (value: string) => boolean;
//   isEmailValid: boolean;
//   validatePassword: (value: string) => boolean;
//   isPasswordValid: boolean;
// }
```

## Practical Patterns

### API Route Types

```typescript
type APIRoutes<T> = {
  [K in keyof T as `/api/${string & K}`]: T[K]
};

interface Endpoints {
  users: User[];
  posts: Post[];
  comments: Comment[];
}

type Routes = APIRoutes<Endpoints>;
// {
//   '/api/users': User[];
//   '/api/posts': Post[];
//   '/api/comments': Comment[];
// }
```

### Redux Actions

```typescript
type Actions<T> = {
  [K in keyof T as `SET_${Uppercase<string & K>}`]: {
    type: `SET_${Uppercase<string & K>}`;
    payload: T[K];
  }
}[keyof T];

interface State {
  user: User;
  theme: 'light' | 'dark';
  loading: boolean;
}

type StateActions = Actions<State>;
// { type: 'SET_USER'; payload: User }
// | { type: 'SET_THEME'; payload: 'light' | 'dark' }
// | { type: 'SET_LOADING'; payload: boolean }
```

### Database Column Mapping

```typescript
type DBColumns<T> = {
  [K in keyof T as `${string & K}_column`]: {
    name: string;
    type: T[K] extends string ? 'TEXT'
      : T[K] extends number ? 'INTEGER'
      : T[K] extends boolean ? 'BOOLEAN'
      : 'JSON';
    nullable: boolean;
  }
};
```

## Complex Key Transformations

### Extract Prefix

```typescript
type ExtractPrefix<S extends string, Prefix extends string> =
  S extends `${Prefix}${infer Rest}` ? Rest : never;

type RemovePrefix<T, Prefix extends string> = {
  [K in keyof T as ExtractPrefix<string & K, Prefix>]: T[K]
};

interface PrefixedAPI {
  api_getUser: () => User;
  api_createUser: (data: UserInput) => User;
  other: string;
}

type CleanAPI = RemovePrefix<PrefixedAPI, 'api_'>;
// { getUser: () => User; createUser: (data: UserInput) => User }
```

### Nested Key Paths

```typescript
type PathKeys<T, Prefix extends string = ''> = {
  [K in keyof T]: K extends string
    ? T[K] extends object
      ? `${Prefix}${K}` | PathKeys<T[K], `${Prefix}${K}.`>
      : `${Prefix}${K}`
    : never
}[keyof T];

type User = {
  name: string;
  address: {
    street: string;
    city: string;
  };
};

type UserPaths = PathKeys<User>;
// 'name' | 'address' | 'address.street' | 'address.city'
```

## Best Practices

1. **Use `string &` for string keys**: Ensures key is string type
2. **Leverage template literals**: Build complex key patterns
3. **Combine with conditional types**: Filter and transform simultaneously
4. **Document naming conventions**: Explain key transformation rules
5. **Test with various inputs**: Verify remapping works with edge cases

## Common Use Cases

### React Props

```typescript
type EventProps<T> = {
  [K in keyof T as `on${Capitalize<string & K>}`]: (value: T[K]) => void
};

interface InputState {
  value: string;
  focused: boolean;
}

type InputProps = InputState & EventProps<InputState>;
// {
//   value: string;
//   focused: boolean;
//   onValue: (value: string) => void;
//   onFocused: (value: boolean) => void;
// }
```

### GraphQL Field Selection

```typescript
type SelectFields<T, Selected extends keyof T> = {
  [K in Selected as `select${Capitalize<string & K>}`]: boolean
};
```

## Anti-Patterns

❌ **Unclear naming transformations**
```typescript
type Bad<T> = {
  [K in keyof T as `x${string & K}y`]: T[K]
}; // What does 'x' and 'y' mean?
```

✅ **Clear semantic naming**
```typescript
type Good<T> = {
  [K in keyof T as `fetch${Capitalize<string & K>}`]: () => Promise<T[K]>
};
```

❌ **Over-complex key logic**
```typescript
type Bad<T> = {
  [K in keyof T as K extends `${infer A}_${infer B}_${infer C}`
    ? /* complex nested logic */
    : K]: T[K]
};
```

✅ **Break into helper types**
```typescript
type ParseKey<K> = /* ... */;
type Good<T> = {
  [K in keyof T as ParseKey<K>]: T[K]
};
```
