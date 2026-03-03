# Type-First Development

Type-first development means defining types before implementation. This workflow ensures type safety from the start and lets the compiler guide completeness.

## Workflow

1. **Define the data model** - types, interfaces, and schemas first
2. **Define function signatures** - input/output types before logic
3. **Implement to satisfy types** - let the compiler guide completeness
4. **Validate at boundaries** - runtime checks where data enters the system

## Example: API Endpoint Type-First

```typescript
// 1. Define data model first
interface User {
  id: string;
  email: string;
  name: string;
  createdAt: Date;
}

type CreateUserRequest = Omit<User, 'id' | 'createdAt'>;
type UpdateUserRequest = Partial<CreateUserRequest>;

// 2. Define function signatures
async function createUser(req: CreateUserRequest): Promise<User>;
async function updateUser(id: string, req: UpdateUserRequest): Promise<User>;
async function getUser(id: string): Promise<User | null>;

// 3. Implement to satisfy types
async function createUser(req: CreateUserRequest): Promise<User> {
  const id = crypto.randomUUID();
  const createdAt = new Date();
  const user: User = { ...req, id, createdAt };
  await db.users.insert(user);
  return user;
}
```

## Benefits

- **Compiler catches missing cases** - Exhaustiveness checking on switch statements
- **IDE provides autocomplete** - Types guide what properties/methods are available
- **Refactoring is safer** - Type errors show all affected code
- **Documentation is built-in** - Types serve as contracts
- **Runtime errors reduced** - Many bugs caught at compile time

## Type-First vs Code-First

**Code-First (avoid):**
```typescript
// Implementation first, types inferred
function processData(data) {
  if (data.status === 'active') {
    return data.items.map(i => i.value);
  }
  return [];
}
```

**Type-First (preferred):**
```typescript
// Types first, implementation second
type Status = 'active' | 'inactive';

interface DataItem {
  value: number;
}

interface Data {
  status: Status;
  items: DataItem[];
}

function processData(data: Data): number[] {
  if (data.status === 'active') {
    return data.items.map(i => i.value);
  }
  return [];
}
```

## Integration with Runtime Validation

Types are compile-time only. Use Zod to bridge compile and runtime:

```typescript
import { z } from 'zod';

// Schema as source of truth
const UserSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  name: z.string().min(1),
  createdAt: z.date(),
});

// Infer TypeScript type from schema
type User = z.infer<typeof UserSchema>;

// Validate at runtime boundaries
function parseUser(data: unknown): User {
  return UserSchema.parse(data);
}
```

## When to Define Types

- **Domain models** - Always define first
- **API contracts** - Define before implementing endpoints
- **Complex business logic** - Define input/output types first
- **Library exports** - Public API types defined first
- **Internal utilities** - Can rely on type inference for simple helpers

## References

- [TypeScript Handbook - Type Inference](https://www.typescriptlang.org/docs/handbook/type-inference.html)
- [Zod Documentation](https://zod.dev/)
