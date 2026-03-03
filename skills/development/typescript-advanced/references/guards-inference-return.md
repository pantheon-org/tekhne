# Type Inference with ReturnType

## Overview
ReturnType and related inference utilities extract function-related types without manual type annotations.

## Basic ReturnType

```typescript
function getUser() {
  return {
    id: 1,
    name: "Alice",
    email: "alice@example.com"
  };
}

type User = ReturnType<typeof getUser>;
// { id: number; name: string; email: string; }
```

## ReturnType with Generics

```typescript
function fetchData<T>(url: string): Promise<T> {
  return fetch(url).then(res => res.json());
}

type FetchResult = ReturnType<typeof fetchData>;
// Promise<T> - note T is still generic
```

## ReturnType with async Functions

```typescript
async function loadUser(id: number) {
  const response = await fetch(`/api/users/${id}`);
  return response.json();
}

type LoadUserReturn = ReturnType<typeof loadUser>;
// Promise<any>

// Better: use Awaited to unwrap Promise
type User = Awaited<ReturnType<typeof loadUser>>;
```

## Parameters Type

```typescript
function createUser(name: string, age: number, email: string) {
  return { name, age, email };
}

type CreateUserParams = Parameters<typeof createUser>;
// [name: string, age: number, email: string]

function callCreate(...args: CreateUserParams) {
  return createUser(...args);
}
```

## ConstructorParameters

```typescript
class Database {
  constructor(
    public host: string,
    public port: number,
    public credentials: { user: string; password: string }
  ) {}
}

type DbParams = ConstructorParameters<typeof Database>;
// [host: string, port: number, credentials: { user: string; password: string }]

function createDatabase(...args: DbParams) {
  return new Database(...args);
}
```

## InstanceType

```typescript
class ApiClient {
  async get(url: string) { /* ... */ }
  async post(url: string, data: any) { /* ... */ }
}

type Client = InstanceType<typeof ApiClient>;
// ApiClient

function wrapClient(client: Client) {
  return {
    get: client.get.bind(client),
    post: client.post.bind(client)
  };
}
```

## ThisParameterType

```typescript
interface Context {
  userId: number;
  permissions: string[];
}

function checkPermission(this: Context, permission: string) {
  return this.permissions.includes(permission);
}

type Ctx = ThisParameterType<typeof checkPermission>;
// Context
```

## OmitThisParameter

```typescript
function greet(this: { name: string }, message: string) {
  return `${message}, ${this.name}!`;
}

type GreetFn = OmitThisParameter<typeof greet>;
// (message: string) => string

const boundGreet: GreetFn = greet.bind({ name: "Alice" });
```

## Combining Inference Utilities

```typescript
async function fetchUsers(limit: number, offset: number) {
  const response = await fetch(`/api/users?limit=${limit}&offset=${offset}`);
  return response.json() as { id: number; name: string }[];
}

// Extract parameter types
type FetchUsersParams = Parameters<typeof fetchUsers>;
// [limit: number, offset: number]

// Extract return type without Promise wrapper
type Users = Awaited<ReturnType<typeof fetchUsers>>;
// { id: number; name: string }[]

// Create wrapper function
function cachedFetchUsers(...args: FetchUsersParams): ReturnType<typeof fetchUsers> {
  // Use cached value if available
  return fetchUsers(...args);
}
```

## Best Practices
- Use ReturnType to avoid duplicating type definitions
- Combine with Awaited for async functions
- Use Parameters for function wrappers and proxies
- Consider ConstructorParameters for factory patterns
- Document why inference is preferred over explicit types

## Common Pitfalls
- ReturnType on generic functions preserves generics
- Doesn't infer literal types without const assertions
- typeof required when passing functions
- Inference doesn't work on overloaded functions (uses last signature)
