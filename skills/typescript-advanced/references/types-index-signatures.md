# Index Signatures & Dynamic Properties

## Basic Index Signatures

Allow dynamic property access with consistent types.

```typescript
interface StringDictionary {
  [key: string]: string;
}

const dict: StringDictionary = {
  name: 'John',
  role: 'Developer'
};

console.log(dict['name']); // ✅ string
console.log(dict['anything']); // ✅ string (might be undefined at runtime!)
```

## Numeric Index Signatures

```typescript
interface NumberArray {
  [index: number]: string;
}

const arr: NumberArray = ['a', 'b', 'c'];
console.log(arr[0]); // ✅ string
```

## Mixed with Known Properties

```typescript
interface Config {
  name: string;
  version: number;
  [key: string]: string | number; // Known properties must match
}

const config: Config = {
  name: 'app',
  version: 1,
  author: 'John', // ✅ Additional properties allowed
  year: 2024
};
```

**Rule:** Known property types must be assignable to the index signature type.

## Template Literal in Index Signatures (TypeScript 4.4+)

```typescript
interface Getters {
  [K: `get${string}`]: () => any;
}

const obj: Getters = {
  getName: () => 'John',
  getAge: () => 30,
  // setName: () => {} // ❌ Error: must start with 'get'
};
```

## Record Utility Type

```typescript
// Shorthand for index signature
type StringMap = Record<string, string>;

// With specific keys
type Roles = Record<'admin' | 'user' | 'guest', string[]>;

const permissions: Roles = {
  admin: ['read', 'write', 'delete'],
  user: ['read', 'write'],
  guest: ['read']
};
```

## Restricting Keys

```typescript
// Only specific keys allowed
type AllowedKeys = 'name' | 'age' | 'email';
type UserData = Record<AllowedKeys, string>;

const user: UserData = {
  name: 'John',
  age: '30',
  email: 'john@example.com'
  // role: 'admin' // ❌ Error: role not in AllowedKeys
};
```

## Combining with Mapped Types

```typescript
type Optional<T> = {
  [K in keyof T]?: T[K];
};

type WithDefaults<T> = {
  [K in keyof T]: T[K] | null;
};

interface User {
  name: string;
  age: number;
}

type PartialUser = Optional<User>;
// { name?: string; age?: number; }

type NullableUser = WithDefaults<User>;
// { name: string | null; age: number | null; }
```

## noUncheckedIndexedAccess

Stricter type checking for index access:

```typescript
// tsconfig.json
{
  "compilerOptions": {
    "noUncheckedIndexedAccess": true
  }
}

interface Dict {
  [key: string]: string;
}

const dict: Dict = { name: 'John' };
const value = dict['anything']; // Type: string | undefined (safer!)
```

## Best Practices

1. **Use Record<K, V> for simple cases** - More concise than index signatures
2. **Enable noUncheckedIndexedAccess** - Catches potential undefined access
3. **Prefer known properties** - Index signatures are for truly dynamic data
4. **Consider Map for dynamic data** - Better runtime guarantees
5. **Document expected keys** - Comment what keys are expected

## Common Patterns

### Environment Variables

```typescript
interface ProcessEnv {
  NODE_ENV: 'development' | 'production' | 'test';
  [key: string]: string | undefined;
}

declare const process: {
  env: ProcessEnv;
};
```

### API Response with Dynamic Fields

```typescript
interface ApiResponse<T> {
  status: number;
  data: T;
  [key: string]: unknown; // Additional metadata
}
```

### Configuration with Sections

```typescript
interface AppConfig {
  [section: string]: {
    [key: string]: string | number | boolean;
  };
}

const config: AppConfig = {
  database: {
    host: 'localhost',
    port: 5432
  },
  cache: {
    enabled: true,
    ttl: 3600
  }
};
```

## Common Pitfalls

- **Assuming indexed values exist** - Always check for undefined
- **Too permissive types** - Index signature `any` defeats type safety
- **Mixing incompatible known properties** - Known properties must match index signature type
- **Using index signatures for fixed shapes** - Use explicit properties instead
