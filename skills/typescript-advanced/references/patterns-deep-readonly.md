# Deep Readonly and Deep Partial

Recursive utility types for nested objects.

## Deep Readonly

```typescript
type DeepReadonly<T> = {
  readonly [K in keyof T]: T[K] extends object
    ? T[K] extends Function
      ? T[K]
      : DeepReadonly<T[K]>
    : T[K];
};

interface Config {
  server: {
    host: string;
    port: number;
    ssl: {
      enabled: boolean;
      cert: string;
    };
  };
  database: {
    host: string;
    credentials: {
      username: string;
      password: string;
    };
  };
}

type ReadonlyConfig = DeepReadonly<Config>;

const config: ReadonlyConfig = {
  server: {
    host: 'localhost',
    port: 3000,
    ssl: {
      enabled: true,
      cert: '/path/to/cert'
    }
  },
  database: {
    host: 'localhost',
    credentials: {
      username: 'user',
      password: 'pass'
    }
  }
};

// config.server.port = 4000; // ✗ Error: readonly
// config.server.ssl.enabled = false; // ✗ Error: readonly
// config.database.credentials.password = 'new'; // ✗ Error: readonly
```

## Deep Partial

```typescript
type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends object
    ? T[K] extends Function
      ? T[K]
      : DeepPartial<T[K]>
    : T[K];
};

function updateConfig(
  config: Config,
  updates: DeepPartial<Config>
): Config {
  return {
    ...config,
    ...updates,
    server: { ...config.server, ...updates.server },
    database: { ...config.database, ...updates.database }
  };
}

// Usage - all fields optional at all levels
const updated = updateConfig(config, {
  server: {
    port: 4000,
    ssl: {
      enabled: false
    }
  }
}); // ✓
```

## Deep Required

```typescript
type DeepRequired<T> = {
  [K in keyof T]-?: T[K] extends object
    ? T[K] extends Function
      ? T[K]
      : DeepRequired<T[K]>
    : T[K];
};

interface PartialConfig {
  server?: {
    host?: string;
    port?: number;
  };
  database?: {
    host?: string;
  };
}

type CompleteConfig = DeepRequired<PartialConfig>;
// All nested fields are now required
```

## Deep Mutable

```typescript
type DeepMutable<T> = {
  -readonly [K in keyof T]: T[K] extends object
    ? T[K] extends Function
      ? T[K]
      : DeepMutable<T[K]>
    : T[K];
};

type MutableConfig = DeepMutable<ReadonlyConfig>;

const mutable: MutableConfig = { /* ... */ };
mutable.server.port = 4000; // ✓ Now allowed
```

## Deep Freeze (Runtime)

```typescript
function deepFreeze<T extends object>(obj: T): DeepReadonly<T> {
  Object.freeze(obj);
  
  Object.keys(obj).forEach(key => {
    const value = obj[key as keyof T];
    if (typeof value === 'object' && value !== null && !Object.isFrozen(value)) {
      deepFreeze(value);
    }
  });
  
  return obj as DeepReadonly<T>;
}

// Usage
const frozen = deepFreeze(config);
// frozen.server.port = 4000; // ✗ Runtime error
```

## Handling Arrays and Tuples

```typescript
type DeepReadonlyArray<T> = {
  readonly [K in keyof T]: T[K] extends object
    ? T[K] extends (infer U)[]
      ? ReadonlyArray<DeepReadonly<U>>
      : DeepReadonly<T[K]>
    : T[K];
};

interface DataWithArrays {
  items: Array<{
    id: number;
    tags: string[];
  }>;
}

type ReadonlyData = DeepReadonlyArray<DataWithArrays>;
// items is ReadonlyArray<{ readonly id: number; readonly tags: ReadonlyArray<string> }>
```

## Handling Maps and Sets

```typescript
type DeepReadonlyWithCollections<T> = {
  readonly [K in keyof T]: T[K] extends Map<infer K, infer V>
    ? ReadonlyMap<K, DeepReadonly<V>>
    : T[K] extends Set<infer U>
    ? ReadonlySet<DeepReadonly<U>>
    : T[K] extends object
    ? DeepReadonly<T[K]>
    : T[K];
};
```

## Best Practices

1. **Immutability**: Use DeepReadonly for configuration objects
2. **Updates**: Use DeepPartial for update/patch operations
3. **Runtime Enforcement**: Combine with Object.freeze for runtime immutability
4. **Performance**: Be aware that deep recursion can impact compilation time
5. **Collections**: Handle arrays, Maps, and Sets appropriately

## Common Pitfalls

1. **Functions**: Don't try to make functions readonly
2. **Circular References**: Deep types can cause infinite recursion
3. **Performance**: Very deep nesting can slow down TypeScript compiler
4. **Runtime vs Compile-Time**: Type-level readonly doesn't prevent runtime mutation

## See Also

- types-mapped-types.md
- utilities-readonly-record.md
- practices-illegal-states.md
