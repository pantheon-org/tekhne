# Module Organization Patterns

## Single-Function Modules

```typescript
// src/utils/formatDate.ts
export const formatDate = (date: Date): string => {
  return date.toISOString().split('T')[0];
};

// src/utils/parseDate.ts
export const parseDate = (str: string): Date => {
  return new Date(str);
};

// src/utils/index.ts (barrel export)
export { formatDate } from './formatDate.js';
export { parseDate } from './parseDate.js';
```

## Feature-Based Organization

```
src/
├── features/
│   ├── auth/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── api/
│   │   ├── types.ts
│   │   └── index.ts
│   ├── users/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── api/
│   │   ├── types.ts
│   │   └── index.ts
│   └── dashboard/
│       ├── components/
│       ├── hooks/
│       ├── api/
│       ├── types.ts
│       └── index.ts
└── shared/
    ├── components/
    ├── hooks/
    └── utils/
```

## Collocated Tests

```
src/
├── utils/
│   ├── formatDate.ts
│   ├── formatDate.test.ts
│   ├── parseDate.ts
│   └── parseDate.test.ts
```

## Type-Only Exports

```typescript
// types.ts
export type User = {
  id: string;
  name: string;
};

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

// consumer.ts
import type { User, Result } from './types.js';
```

## Barrel Exports

```typescript
// components/index.ts
export { Button } from './Button.js';
export { Input } from './Input.js';
export { Modal } from './Modal.js';
export type { ButtonProps } from './Button.js';
export type { InputProps } from './Input.js';

// Usage
import { Button, Input } from './components/index.js';
import type { ButtonProps } from './components/index.js';
```

## Module Boundaries

```typescript
// Good: Clear module boundaries
// auth/api.ts
export const login = async (credentials: Credentials): Promise<User> => {
  // ...
};

// auth/index.ts
export { login } from './api.js';
export type { Credentials, User } from './types.js';

// Bad: Leaky abstractions
// auth/index.ts
export { login } from './api.js';
export { validatePassword } from './internal/validation.js'; // Internal detail
```

## Dependency Direction

```typescript
// Good: Dependencies point inward
// domain/user.ts (core domain)
export type User = {
  id: string;
  email: string;
};

// api/userApi.ts (infrastructure)
import type { User } from '../domain/user.js';

export const fetchUser = async (id: string): Promise<User> => {
  // ...
};

// Bad: Core depends on infrastructure
// domain/user.ts
import { fetchUser } from '../api/userApi.js'; // Wrong direction
```

## Facade Pattern

```typescript
// database/index.ts (facade)
import { createConnection } from './connection.js';
import { createQueryBuilder } from './queryBuilder.js';
import { createMigrationRunner } from './migrations.js';

export const createDatabase = () => {
  const connection = createConnection();
  const queryBuilder = createQueryBuilder(connection);
  const migrationRunner = createMigrationRunner(connection);

  return {
    query: queryBuilder.query,
    execute: queryBuilder.execute,
    migrate: migrationRunner.run,
  };
};

// Usage
import { createDatabase } from './database/index.js';
const db = createDatabase();
```

## Adapter Pattern

```typescript
// ports/logger.ts (interface)
export interface Logger {
  info(message: string): void;
  error(message: string, error?: Error): void;
}

// adapters/consoleLogger.ts
import type { Logger } from '../ports/logger.js';

export const createConsoleLogger = (): Logger => ({
  info: (message) => console.log(`[INFO] ${message}`),
  error: (message, error) =>
    console.error(`[ERROR] ${message}`, error),
});

// adapters/fileLogger.ts
import type { Logger } from '../ports/logger.js';

export const createFileLogger = (filePath: string): Logger => ({
  info: (message) => {
    /* write to file */
  },
  error: (message, error) => {
    /* write to file */
  },
});
```

## Monorepo Package Structure

```
packages/
├── core/
│   ├── src/
│   ├── package.json
│   └── tsconfig.json
├── ui/
│   ├── src/
│   ├── package.json
│   └── tsconfig.json
└── api/
    ├── src/
    ├── package.json
    └── tsconfig.json
```

## Best Practices

1. **One function per file** - Easy to find, test, and understand
2. **Collocate related files** - Tests, types, and implementation together
3. **Use barrel exports** - Clean public API
4. **Feature-based folders** - Group by feature, not by type
5. **Clear module boundaries** - Public API vs internal implementation
6. **Dependency direction** - Core domain doesn't depend on infrastructure
7. **Type-only imports** - Use `import type` when possible
8. **Avoid circular dependencies** - Refactor to break cycles

## Anti-Patterns

```typescript
// Bad: Circular dependencies
// a.ts
import { b } from './b.js';
export const a = () => b();

// b.ts
import { a } from './a.js';
export const b = () => a();

// Bad: Deep nesting
src/components/shared/common/ui/base/Button.tsx

// Good: Flat structure
src/components/Button.tsx

// Bad: Mixed concerns
// userService.ts contains database, validation, and business logic

// Good: Separated concerns
// domain/user.ts - business logic
// repositories/userRepository.ts - database
// validators/userValidator.ts - validation
```
