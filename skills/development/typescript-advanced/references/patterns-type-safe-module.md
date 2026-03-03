# Type-Safe Module Pattern

Create modules with full type safety and encapsulation.

## Basic Module Pattern

```typescript
// math.ts
export interface MathOperations {
  add(a: number, b: number): number;
  subtract(a: number, b: number): number;
}

const createMath = (): MathOperations => {
  // Private implementation
  const cache = new Map<string, number>();
  
  const getCacheKey = (op: string, a: number, b: number) =>
    `${op}:${a}:${b}`;
  
  return {
    add(a, b) {
      const key = getCacheKey('add', a, b);
      if (!cache.has(key)) {
        cache.set(key, a + b);
      }
      return cache.get(key)!;
    },
    subtract(a, b) {
      const key = getCacheKey('sub', a, b);
      if (!cache.has(key)) {
        cache.set(key, a - b);
      }
      return cache.get(key)!;
    }
  };
};

export const math = createMath();
```

## Plugin Architecture

```typescript
interface Plugin<TConfig = unknown> {
  name: string;
  version: string;
  init(config: TConfig): Promise<void>;
  cleanup(): Promise<void>;
}

interface PluginRegistry {
  register<TConfig>(plugin: Plugin<TConfig>): void;
  get<TConfig>(name: string): Plugin<TConfig> | undefined;
  unregister(name: string): void;
}

const createPluginRegistry = (): PluginRegistry => {
  const plugins = new Map<string, Plugin>();
  
  return {
    register(plugin) {
      if (plugins.has(plugin.name)) {
        throw new Error(`Plugin ${plugin.name} already registered`);
      }
      plugins.set(plugin.name, plugin);
    },
    get(name) {
      return plugins.get(name);
    },
    unregister(name) {
      const plugin = plugins.get(name);
      if (plugin) {
        plugin.cleanup();
        plugins.delete(name);
      }
    }
  };
};
```

## Module with Private State

```typescript
export type UserId = string & { readonly __brand: 'UserId' };

export interface UserModule {
  create(email: string): UserId;
  get(id: UserId): { id: UserId; email: string } | undefined;
  delete(id: UserId): boolean;
}

const createUserModule = (): UserModule => {
  let counter = 0;
  const users = new Map<UserId, { id: UserId; email: string }>();
  
  const createUserId = (): UserId => {
    return `user-${++counter}` as UserId;
  };
  
  return {
    create(email) {
      const id = createUserId();
      users.set(id, { id, email });
      return id;
    },
    get(id) {
      return users.get(id);
    },
    delete(id) {
      return users.delete(id);
    }
  };
};

export const userModule = createUserModule();
```

## Best Practices

1. **Export only interfaces**: Keep implementation details private
2. **Use branded types**: Prevent ID mixing across modules
3. **Single instance**: Export one instance, not the factory
4. **Clear boundaries**: Define what's public vs private
5. **Testability**: Allow dependency injection for testing
