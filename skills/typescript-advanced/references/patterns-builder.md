# Builder Pattern with Type Safety

Implementing the Builder pattern with compile-time enforcement of required fields.

## Basic Builder

```typescript
interface User {
  id: string;
  name: string;
  email: string;
  age?: number;
  role?: 'admin' | 'user';
}

class UserBuilder {
  private user: Partial<User> = {};

  setId(id: string): this {
    this.user.id = id;
    return this;
  }

  setName(name: string): this {
    this.user.name = name;
    return this;
  }

  setEmail(email: string): this {
    this.user.email = email;
    return this;
  }

  setAge(age: number): this {
    this.user.age = age;
    return this;
  }

  setRole(role: 'admin' | 'user'): this {
    this.user.role = role;
    return this;
  }

  build(): User {
    // Runtime validation of required fields
    if (!this.user.id || !this.user.name || !this.user.email) {
      throw new Error('Missing required fields');
    }
    return this.user as User;
  }
}

// Usage
const user = new UserBuilder()
  .setId('123')
  .setName('John')
  .setEmail('john@example.com')
  .setAge(30)
  .build();
```

## Type-Safe Builder (Compile-Time Enforcement)

```typescript
type RequiredKeys<T> = {
  [K in keyof T]-?: {} extends Pick<T, K> ? never : K
}[keyof T];

type OptionalKeys<T> = {
  [K in keyof T]-?: {} extends Pick<T, K> ? K : never
}[keyof T];

type Builder<T, Set extends keyof T = never> = {
  [K in RequiredKeys<T> as `set${Capitalize<K & string>}`]: (
    value: T[K]
  ) => Builder<T, Set | K>;
} & {
  [K in OptionalKeys<T> as `set${Capitalize<K & string>}`]: (
    value: T[K]
  ) => Builder<T, Set>;
} & (RequiredKeys<T> extends Set
  ? { build(): T }
  : { build: 'Error: Missing required fields' }
);

function createBuilder<T>(): Builder<T> {
  const data: Partial<T> = {};
  
  return new Proxy({} as Builder<T>, {
    get(_, prop: string) {
      if (prop === 'build') {
        return () => data as T;
      }
      
      if (prop.startsWith('set')) {
        return (value: unknown) => {
          const key = prop.slice(3).toLowerCase() as keyof T;
          data[key] = value as T[keyof T];
          return this;
        };
      }
      
      return undefined;
    }
  });
}

// Usage - compile-time enforcement!
const builder = createBuilder<User>();

// builder.build(); // ✗ Error: Missing required fields

const userBuilder = builder
  .setId('123')
  .setName('John');
  // .build(); // ✗ Still error: missing 'email'

const completeUser = userBuilder
  .setEmail('john@example.com')
  .build(); // ✓ Now it works!
```

## Fluent API Builder

```typescript
type FluentBuilder<T, Required extends keyof T = never> = {
  [K in keyof T]: (value: T[K]) => FluentBuilder<T, Required | K>;
} & ([Required] extends [keyof T] 
  ? { build(): T } 
  : {}
);

function fluent<T>(): FluentBuilder<T> {
  const data: Partial<T> = {};
  
  return new Proxy({} as FluentBuilder<T>, {
    get(_, prop: string) {
      if (prop === 'build') {
        return () => data as T;
      }
      
      return (value: unknown) => {
        data[prop as keyof T] = value as T[keyof T];
        return this;
      };
    }
  });
}

// Usage
const user = fluent<User>()
  .id('123')
  .name('John')
  .email('john@example.com')
  .age(30)
  .build();
```

## Immutable Builder

```typescript
class ImmutableUserBuilder {
  private constructor(private readonly user: Partial<User>) {}

  static create(): ImmutableUserBuilder {
    return new ImmutableUserBuilder({});
  }

  setId(id: string): ImmutableUserBuilder {
    return new ImmutableUserBuilder({ ...this.user, id });
  }

  setName(name: string): ImmutableUserBuilder {
    return new ImmutableUserBuilder({ ...this.user, name });
  }

  setEmail(email: string): ImmutableUserBuilder {
    return new ImmutableUserBuilder({ ...this.user, email });
  }

  build(): User {
    if (!this.user.id || !this.user.name || !this.user.email) {
      throw new Error('Missing required fields');
    }
    return this.user as User;
  }
}

// Usage
const user = ImmutableUserBuilder.create()
  .setId('123')
  .setName('John')
  .setEmail('john@example.com')
  .build();
```

## Best Practices

1. **Type Safety**: Use type-level programming to enforce required fields
2. **Immutability**: Consider making builders immutable
3. **Fluent API**: Return `this` for method chaining
4. **Validation**: Validate required fields at build time
5. **Defaults**: Provide sensible defaults for optional fields

## See Also

- patterns-state-machine.md
- practices-illegal-states.md
- types-mapped-types.md
