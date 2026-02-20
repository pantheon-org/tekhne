# Dependency Injection Pattern

Implement type-safe dependency injection for testable, maintainable code.

## Basic DI Container

```typescript
type ServiceFactory<T> = () => T;
type ServiceIdentifier<T> = symbol & { __type?: T };

class Container {
  private services = new Map<symbol, ServiceFactory<any>>();
  
  register<T>(id: ServiceIdentifier<T>, factory: ServiceFactory<T>): void {
    this.services.set(id, factory);
  }
  
  resolve<T>(id: ServiceIdentifier<T>): T {
    const factory = this.services.get(id);
    if (!factory) {
      throw new Error(`Service not registered: ${id.toString()}`);
    }
    return factory();
  }
}

// Service identifiers
const DATABASE_SERVICE = Symbol('database') as ServiceIdentifier<Database>;
const USER_SERVICE = Symbol('user') as ServiceIdentifier<UserService>;

// Registration
const container = new Container();
container.register(DATABASE_SERVICE, () => new Database());
container.register(USER_SERVICE, () => {
  const db = container.resolve(DATABASE_SERVICE);
  return new UserService(db);
});

// Resolution
const userService = container.resolve(USER_SERVICE);
```

## Constructor Injection

```typescript
interface Database {
  query<T>(sql: string): Promise<T[]>;
}

interface Logger {
  log(message: string): void;
}

class UserRepository {
  constructor(
    private readonly db: Database,
    private readonly logger: Logger
  ) {}
  
  async findById(id: string) {
    this.logger.log(`Finding user: ${id}`);
    return this.db.query(`SELECT * FROM users WHERE id = ?`);
  }
}

// DI Container provides dependencies
const db: Database = createDatabase();
const logger: Logger = createLogger();
const userRepo = new UserRepository(db, logger);
```

## Token-based Injection

```typescript
interface InjectionToken<T> {
  readonly _type?: T;
}

function createToken<T>(description: string): InjectionToken<T> {
  return { _type: undefined } as InjectionToken<T>;
}

const DB_TOKEN = createToken<Database>('Database');
const LOGGER_TOKEN = createToken<Logger>('Logger');
const USER_REPO_TOKEN = createToken<UserRepository>('UserRepository');

class TypedContainer {
  private providers = new Map<InjectionToken<any>, any>();
  
  provide<T>(token: InjectionToken<T>, value: T): void {
    this.providers.set(token, value);
  }
  
  inject<T>(token: InjectionToken<T>): T {
    const value = this.providers.get(token);
    if (!value) {
      throw new Error('Token not provided');
    }
    return value;
  }
}

// Usage
const container = new TypedContainer();
container.provide(DB_TOKEN, createDatabase());
container.provide(LOGGER_TOKEN, createLogger());
container.provide(
  USER_REPO_TOKEN,
  new UserRepository(
    container.inject(DB_TOKEN),
    container.inject(LOGGER_TOKEN)
  )
);
```

## Scoped Services

```typescript
type ServiceLifetime = 'singleton' | 'transient' | 'scoped';

interface ServiceDescriptor<T> {
  lifetime: ServiceLifetime;
  factory: ServiceFactory<T>;
  instance?: T;
}

class ScopedContainer {
  private descriptors = new Map<symbol, ServiceDescriptor<any>>();
  private scopeInstances = new Map<symbol, any>();
  
  register<T>(
    id: symbol,
    factory: ServiceFactory<T>,
    lifetime: ServiceLifetime = 'transient'
  ): void {
    this.descriptors.set(id, { lifetime, factory });
  }
  
  resolve<T>(id: symbol): T {
    const descriptor = this.descriptors.get(id);
    if (!descriptor) {
      throw new Error('Service not registered');
    }
    
    switch (descriptor.lifetime) {
      case 'singleton':
        if (!descriptor.instance) {
          descriptor.instance = descriptor.factory();
        }
        return descriptor.instance;
      
      case 'scoped':
        if (!this.scopeInstances.has(id)) {
          this.scopeInstances.set(id, descriptor.factory());
        }
        return this.scopeInstances.get(id);
      
      case 'transient':
        return descriptor.factory();
    }
  }
  
  createScope(): ScopedContainer {
    const scope = new ScopedContainer();
    scope.descriptors = this.descriptors;
    return scope;
  }
}
```

## Testing with DI

```typescript
// Production
class RealDatabase implements Database {
  async query<T>(sql: string): Promise<T[]> {
    // Real database query
    return [];
  }
}

// Test
class MockDatabase implements Database {
  queries: string[] = [];
  
  async query<T>(sql: string): Promise<T[]> {
    this.queries.push(sql);
    return [] as T[];
  }
}

// Test setup
const mockDb = new MockDatabase();
const logger = createLogger();
const userRepo = new UserRepository(mockDb, logger);

await userRepo.findById('123');
expect(mockDb.queries).toContain('SELECT * FROM users WHERE id = ?');
```

## Best Practices

1. **Use interfaces**: Depend on abstractions, not concrete classes
2. **Constructor injection**: Prefer constructor over property injection
3. **Explicit dependencies**: Make dependencies visible in constructor
4. **Avoid service locator**: Pass dependencies explicitly
5. **Lifetime management**: Choose appropriate service lifetimes
6. **Type safety**: Use tokens or symbols for type-safe resolution
