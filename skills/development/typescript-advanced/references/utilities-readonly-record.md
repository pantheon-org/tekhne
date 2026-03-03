# Readonly and Record Utility Types

## Readonly<T>

Makes all properties in T readonly.

### Basic Usage

```typescript
interface User {
  id: string;
  name: string;
  email: string;
}

type ReadonlyUser = Readonly<User>;
// {
//   readonly id: string;
//   readonly name: string;
//   readonly email: string;
// }

const user: ReadonlyUser = {
  id: '123',
  name: 'John',
  email: 'john@example.com'
};

// user.name = 'Jane'; // ❌ Error: Cannot assign to 'name' because it is read-only
```

### Implementation

```typescript
type Readonly<T> = {
  readonly [P in keyof T]: T[P];
};
```

### Use Cases

**Immutable Configuration**

```typescript
interface Config {
  apiUrl: string;
  timeout: number;
  retries: number;
}

const config: Readonly<Config> = {
  apiUrl: 'https://api.example.com',
  timeout: 5000,
  retries: 3
};

// Prevent accidental modification
// config.timeout = 10000; // ❌ Error
```

**Function Parameters**

```typescript
function processUsers(users: readonly User[]) {
  // Cannot modify the array
  // users.push({ ... }); // ❌ Error
  // users[0].name = 'Jane'; // ❌ Error if User is Readonly<User>
  
  return users.map(u => u.name);
}
```

**Const Assertions (Better Alternative)**

```typescript
const config = {
  apiUrl: 'https://api.example.com',
  timeout: 5000,
  retries: 3
} as const;
// Type: {
//   readonly apiUrl: "https://api.example.com";
//   readonly timeout: 5000;
//   readonly retries: 3;
// }
```

### Deep Readonly

For nested objects:

```typescript
type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object
    ? DeepReadonly<T[P]>
    : T[P];
};

interface Config {
  server: {
    host: string;
    port: number;
  };
  database: {
    host: string;
    credentials: {
      user: string;
      password: string;
    };
  };
}

type ImmutableConfig = DeepReadonly<Config>;

const config: ImmutableConfig = { /* ... */ };
// config.server.host = 'localhost'; // ❌ Error
// config.database.credentials.user = 'admin'; // ❌ Error
```

## Record<K, T>

Creates an object type with keys of type K and values of type T.

### Basic Usage

```typescript
type Roles = 'admin' | 'user' | 'guest';

type RolePermissions = Record<Roles, string[]>;
// {
//   admin: string[];
//   user: string[];
//   guest: string[];
// }

const permissions: RolePermissions = {
  admin: ['read', 'write', 'delete'],
  user: ['read', 'write'],
  guest: ['read']
};
```

### Implementation

```typescript
type Record<K extends keyof any, T> = {
  [P in K]: T;
};
```

### Use Cases

**String Dictionary**

```typescript
type StringMap = Record<string, string>;

const translations: StringMap = {
  hello: 'Hola',
  goodbye: 'Adiós',
  welcome: 'Bienvenido'
};
```

**Enum-based Configurations**

```typescript
enum Status {
  Pending = 'pending',
  Approved = 'approved',
  Rejected = 'rejected'
}

type StatusConfig = Record<Status, {
  color: string;
  label: string;
}>;

const statusConfig: StatusConfig = {
  [Status.Pending]: { color: 'yellow', label: 'Pending Review' },
  [Status.Approved]: { color: 'green', label: 'Approved' },
  [Status.Rejected]: { color: 'red', label: 'Rejected' }
};
```

**Type-safe State Handlers**

```typescript
type EventType = 'click' | 'hover' | 'focus';

type EventHandlers = Record<EventType, (event: Event) => void>;

const handlers: EventHandlers = {
  click: (e) => console.log('clicked'),
  hover: (e) => console.log('hovered'),
  focus: (e) => console.log('focused')
};
```

**API Endpoints**

```typescript
type Endpoint = 'users' | 'posts' | 'comments';

type ApiConfig = Record<Endpoint, {
  url: string;
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
}>;

const api: ApiConfig = {
  users: { url: '/api/users', method: 'GET' },
  posts: { url: '/api/posts', method: 'GET' },
  comments: { url: '/api/comments', method: 'GET' }
};
```

## Combining Readonly and Record

### Immutable Dictionary

```typescript
type ImmutableMap<K extends keyof any, T> = Readonly<Record<K, T>>;

const colors: ImmutableMap<string, string> = {
  primary: '#007bff',
  secondary: '#6c757d',
  success: '#28a745'
};

// colors.primary = '#0056b3'; // ❌ Error
// colors['danger'] = '#dc3545'; // ❌ Error (in readonly arrays)
```

### Deep Readonly Record

```typescript
type DeepReadonlyRecord<K extends keyof any, T> = {
  readonly [P in K]: T extends object ? DeepReadonly<T> : T;
};

type Config = DeepReadonlyRecord<string, {
  enabled: boolean;
  options: {
    timeout: number;
  };
}>;
```

## Partial Record

For optional keys:

```typescript
type PartialRecord<K extends keyof any, T> = Partial<Record<K, T>>;

type OptionalPermissions = PartialRecord<Roles, string[]>;

const permissions: OptionalPermissions = {
  admin: ['read', 'write']
  // user and guest not required
};
```

## Best Practices

1. **Use Readonly for immutability** - Prevent accidental mutations
2. **Use Record for string-keyed objects** - More explicit than index signatures
3. **Combine with const assertions** - For literal types
4. **Consider readonly arrays** - `readonly T[]` or `ReadonlyArray<T>`
5. **Use DeepReadonly for nested data** - Ensure complete immutability

## Common Patterns

### Lookup Tables

```typescript
type HttpStatusCode = 200 | 400 | 404 | 500;

const statusMessages: Record<HttpStatusCode, string> = {
  200: 'OK',
  400: 'Bad Request',
  404: 'Not Found',
  500: 'Internal Server Error'
};
```

### Feature Flags

```typescript
type FeatureName = 'darkMode' | 'betaFeatures' | 'analytics';

type FeatureFlags = Record<FeatureName, boolean>;

const features: Readonly<FeatureFlags> = {
  darkMode: true,
  betaFeatures: false,
  analytics: true
};
```

### Error Codes

```typescript
enum ErrorCode {
  NetworkError = 'NETWORK_ERROR',
  ValidationError = 'VALIDATION_ERROR',
  AuthError = 'AUTH_ERROR'
}

type ErrorMessages = Record<ErrorCode, string>;

const errors: Readonly<ErrorMessages> = {
  [ErrorCode.NetworkError]: 'Network connection failed',
  [ErrorCode.ValidationError]: 'Invalid input data',
  [ErrorCode.AuthError]: 'Authentication failed'
};
```

## Common Pitfalls

- **Shallow readonly** - Nested objects are still mutable without DeepReadonly
- **Runtime immutability** - Readonly is compile-time only, use Object.freeze() for runtime
- **Record with string keys** - Might be too permissive, consider literal unions
- **Assuming Record enforces all keys** - `Record<string, T>` allows any string key
