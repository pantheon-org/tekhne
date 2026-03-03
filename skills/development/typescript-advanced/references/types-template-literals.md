# Template Literal Types

## Basic Template Literals

Create types from string templates:

```typescript
type World = "world";
type Greeting = `hello ${World}`; // "hello world"

type EmailDomain = "gmail.com" | "yahoo.com";
type Email = `user@${EmailDomain}`;
// "user@gmail.com" | "user@yahoo.com"
```

## String Manipulation Utilities

Built-in type utilities for string transformation:

```typescript
type Uppercase<S extends string> = intrinsic;
type Lowercase<S extends string> = intrinsic;
type Capitalize<S extends string> = intrinsic;
type Uncapitalize<S extends string> = intrinsic;

type Loud = Uppercase<"hello">; // "HELLO"
type Quiet = Lowercase<"WORLD">; // "world"
type Proper = Capitalize<"john">; // "John"
type Lower = Uncapitalize<"Smith">; // "smith"
```

## Event Handler Generation

```typescript
type EventNames = "click" | "focus" | "blur";
type EventHandlers = `on${Capitalize<EventNames>}`;
// "onClick" | "onFocus" | "onBlur"

type PropEventHandler<T extends string> = `on${Capitalize<T>}Changed`;

type FormEvents = PropEventHandler<"username" | "email">;
// "onUsernameChanged" | "onEmailChanged"
```

## Path Building

```typescript
type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";
type Endpoint = "users" | "posts" | "comments";
type ApiRoute = `/${Endpoint}`;
// "/users" | "/posts" | "/comments"

type RestEndpoint = `${HttpMethod} /${Endpoint}`;
// "GET /users" | "POST /users" | ... (12 combinations)
```

## Pattern Matching with `infer`

Extract parts from template literal types:

```typescript
type ExtractRouteParams<T extends string> =
  T extends `${infer _Start}:${infer Param}/${infer Rest}`
    ? Param | ExtractRouteParams<Rest>
    : T extends `${infer _Start}:${infer Param}`
    ? Param
    : never;

type Params = ExtractRouteParams<"/users/:userId/posts/:postId">;
// "userId" | "postId"
```

## CamelCase Conversion

```typescript
type CamelCase<S extends string> =
  S extends `${infer First}_${infer Rest}`
    ? `${Lowercase<First>}${Capitalize<CamelCase<Rest>>}`
    : Lowercase<S>;

type Result = CamelCase<"hello_world_example">;
// "helloWorldExample"
```

## Type-Safe CSS Properties

```typescript
type CSSUnits = "px" | "em" | "rem" | "%";
type CSSValue<T extends number> = `${T}${CSSUnits}`;

type Width = CSSValue<100>; // "100px" | "100em" | "100rem" | "100%"

// With specific properties
type Padding = CSSValue<0 | 4 | 8 | 16>;
```

## Database Query Builder

```typescript
type TableName = "users" | "posts" | "comments";
type Operation = "select" | "insert" | "update" | "delete";
type Query = `${Operation} from ${TableName}`;

// Type-safe query builder
type WhereClause<T extends string> = `where ${T}`;
type FullQuery<T extends TableName, C extends string> =
  `select from ${T} ${WhereClause<C>}`;
```

## Accessor Generation

```typescript
type Accessor<T, K extends keyof T> = {
  [P in K as `get${Capitalize<string & P>}`]: () => T[P];
} & {
  [P in K as `set${Capitalize<string & P>}`]: (value: T[P]) => void;
};

interface User {
  name: string;
  age: number;
}

type UserAccessors = Accessor<User, keyof User>;
// {
//   getName: () => string;
//   setName: (value: string) => void;
//   getAge: () => number;
//   setAge: (value: number) => void;
// }
```

## Environment Variables

```typescript
type EnvPrefix = "VITE_" | "NEXT_PUBLIC_";
type EnvVar<T extends string> = `${EnvPrefix}${Uppercase<T>}`;

type AppEnvVars = EnvVar<"api_url" | "api_key">;
// "VITE_API_URL" | "VITE_API_KEY" | "NEXT_PUBLIC_API_URL" | "NEXT_PUBLIC_API_KEY"

// Type-safe env access
declare const process: {
  env: Record<AppEnvVars, string>;
};
```

## SQL Table Schema

```typescript
type Column = "id" | "name" | "email" | "created_at";
type SqlType = "INTEGER" | "TEXT" | "TIMESTAMP";
type ColumnDef = `${Column} ${SqlType}`;

type TableSchema = {
  [K in Column]: ColumnDef;
};
```

## Advanced Patterns

### URL Builder

```typescript
type Protocol = "http" | "https";
type Domain = string;
type Path = string;
type Url = `${Protocol}://${Domain}/${Path}`;

type ApiUrl<E extends string> = `https://api.example.com/${E}`;
```

### Versioned API Endpoints

```typescript
type Version = "v1" | "v2" | "v3";
type Resource = "users" | "posts";
type VersionedEndpoint = `/api/${Version}/${Resource}`;
// "/api/v1/users" | "/api/v1/posts" | "/api/v2/users" | ...
```

### Type-Safe Event Names

```typescript
type DomainEvent<T extends string> = `domain:${T}`;
type SystemEvent<T extends string> = `system:${T}`;

type UserEvents = DomainEvent<"user:created" | "user:updated">;
// "domain:user:created" | "domain:user:updated"
```

## Best Practices

1. **Use template literals for systematic naming patterns**
2. **Combine with mapped types for property generation**
3. **Leverage string manipulation utilities**
4. **Use `infer` for pattern extraction**
5. **Keep template expressions readable**

## Common Pitfalls

1. **Don't create excessively large union types** (combinatorial explosion)
2. **Be aware of recursion limits** in pattern matching
3. **Avoid overly complex template expressions**
4. **Remember string manipulation utilities are case-sensitive**
5. **Watch out for empty string edge cases**

## Performance Considerations

- Template literal types can create very large unions
- Each combination is expanded at compile time
- Limit the number of union members in template positions
- Consider using string branding instead for open-ended strings
