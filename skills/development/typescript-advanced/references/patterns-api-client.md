# Type-Safe API Client

Building API clients with route configuration and full type safety.

## Basic API Client

```typescript
type Routes = {
  '/users': {
    GET: { response: User[] };
    POST: { body: CreateUserDTO; response: User };
  };
  '/users/:id': {
    GET: { response: User };
    PUT: { body: UpdateUserDTO; response: User };
    DELETE: { response: void };
  };
  '/posts': {
    GET: { query: { page: number; limit: number }; response: Post[] };
  };
};

class ApiClient<T extends Record<string, Record<string, unknown>>> {
  constructor(private baseUrl: string) {}

  async request<
    Path extends keyof T,
    Method extends keyof T[Path]
  >(
    path: Path,
    method: Method,
    config?: T[Path][Method] extends { body: infer B } ? { body: B } : {}
      & T[Path][Method] extends { query: infer Q } ? { query: Q } : {}
  ): Promise<T[Path][Method] extends { response: infer R } ? R : never> {
    const url = new URL(path as string, this.baseUrl);
    
    if (config && 'query' in config) {
      Object.entries(config.query as Record<string, unknown>).forEach(([key, value]) => {
        url.searchParams.append(key, String(value));
      });
    }

    const response = await fetch(url.toString(), {
      method: method as string,
      headers: {
        'Content-Type': 'application/json',
      },
      body: config && 'body' in config 
        ? JSON.stringify(config.body) 
        : undefined,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
  }
}

// Usage with full type safety
const client = new ApiClient<Routes>('https://api.example.com');

// GET /users - no config needed
const users = await client.request('/users', 'GET');
// users is typed as User[]

// POST /users - requires body
const newUser = await client.request('/users', 'POST', {
  body: { name: 'John', email: 'john@example.com' }
});
// newUser is typed as User

// GET /posts - requires query params
const posts = await client.request('/posts', 'GET', {
  query: { page: 1, limit: 10 }
});
// posts is typed as Post[]
```

## With Path Parameters

```typescript
type PathParams<Path extends string> = 
  Path extends `${infer _Start}:${infer Param}/${infer Rest}`
    ? { [K in Param | keyof PathParams<`/${Rest}`>]: string }
    : Path extends `${infer _Start}:${infer Param}`
    ? { [K in Param]: string }
    : {};

class ApiClientWithParams<T extends Record<string, Record<string, unknown>>> {
  async request<
    Path extends keyof T & string,
    Method extends keyof T[Path]
  >(
    path: Path,
    method: Method,
    config: PathParams<Path> extends Record<string, never>
      ? T[Path][Method] extends { body: infer B } ? { body: B } : {}
      : { params: PathParams<Path> } 
        & (T[Path][Method] extends { body: infer B } ? { body: B } : {})
  ): Promise<T[Path][Method] extends { response: infer R } ? R : never> {
    let url = path as string;
    
    if ('params' in config) {
      Object.entries(config.params as Record<string, string>).forEach(([key, value]) => {
        url = url.replace(`:${key}`, value);
      });
    }

    // ... rest of implementation
  }
}

// Usage
const user = await client.request('/users/:id', 'GET', {
  params: { id: '123' }
});
```

## With Response Validation

```typescript
import { z } from 'zod';

type ValidatedRoutes = {
  '/users': {
    GET: { 
      response: User[];
      schema: z.ZodType<User[]>;
    };
  };
};

class ValidatedApiClient<T extends Record<string, Record<string, unknown>>> {
  async request<
    Path extends keyof T,
    Method extends keyof T[Path]
  >(
    path: Path,
    method: Method,
    schema: T[Path][Method] extends { schema: infer S } ? S : never
  ): Promise<T[Path][Method] extends { response: infer R } ? R : never> {
    const response = await fetch(/* ... */);
    const data = await response.json();
    
    // Runtime validation
    return (schema as z.ZodType).parse(data);
  }
}
```

## Best Practices

1. **Define Route Types**: Central route configuration with all endpoints
2. **Type Safety**: Enforce correct method, body, and query params
3. **Path Parameters**: Extract and validate path parameters from route strings
4. **Response Validation**: Use Zod for runtime validation of responses
5. **Error Handling**: Provide typed error responses

## See Also

- patterns-builder.md
- guards-type-predicates.md
- practices-runtime-validation.md
