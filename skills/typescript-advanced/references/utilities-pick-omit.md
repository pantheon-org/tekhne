# Pick and Omit Utility Types

## Pick<T, K>

Creates a type by picking specific properties from T.

### Basic Usage

```typescript
interface User {
  id: string;
  name: string;
  email: string;
  password: string;
  createdAt: Date;
}

type PublicUser = Pick<User, 'id' | 'name' | 'email'>;
// {
//   id: string;
//   name: string;
//   email: string;
// }

function displayUser(user: PublicUser) {
  // Only id, name, email accessible
  console.log(user.name);
  // console.log(user.password); // ❌ Error
}
```

### Implementation

```typescript
type Pick<T, K extends keyof T> = {
  [P in K]: T[P];
};
```

### Use Cases

**API Response DTOs**

```typescript
interface DbUser {
  id: string;
  name: string;
  email: string;
  passwordHash: string;
  salt: string;
  createdAt: Date;
  updatedAt: Date;
}

type UserResponse = Pick<DbUser, 'id' | 'name' | 'email' | 'createdAt'>;

function getUserById(id: string): UserResponse {
  const dbUser = db.users.findById(id);
  return {
    id: dbUser.id,
    name: dbUser.name,
    email: dbUser.email,
    createdAt: dbUser.createdAt
  };
}
```

**Form Fields**

```typescript
interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  stock: number;
  createdAt: Date;
}

type ProductFormData = Pick<Product, 'name' | 'description' | 'price' | 'stock'>;

function ProductForm() {
  const [formData, setFormData] = useState<ProductFormData>({
    name: '',
    description: '',
    price: 0,
    stock: 0
  });
}
```

**Minimal Interface**

```typescript
interface TodoItem {
  id: string;
  title: string;
  description: string;
  completed: boolean;
  createdAt: Date;
  updatedAt: Date;
  tags: string[];
}

type TodoSummary = Pick<TodoItem, 'id' | 'title' | 'completed'>;

function getTodoList(): TodoSummary[] {
  // Return minimal data for list view
}
```

## Omit<T, K>

Creates a type by omitting specific properties from T.

### Basic Usage

```typescript
interface User {
  id: string;
  name: string;
  email: string;
  password: string;
}

type UserWithoutPassword = Omit<User, 'password'>;
// {
//   id: string;
//   name: string;
//   email: string;
// }

type PublicUser = Omit<User, 'password' | 'email'>;
// {
//   id: string;
//   name: string;
// }
```

### Implementation

```typescript
type Omit<T, K extends keyof any> = Pick<T, Exclude<keyof T, K>>;
```

**Note:** K is `keyof any` (not `keyof T`), allowing omitting keys that don't exist.

### Use Cases

**Remove Sensitive Data**

```typescript
interface Employee {
  id: string;
  name: string;
  email: string;
  salary: number;
  ssn: string;
}

type PublicEmployee = Omit<Employee, 'salary' | 'ssn'>;

function getEmployeeProfile(id: string): PublicEmployee {
  const employee = db.employees.findById(id);
  const { salary, ssn, ...public } = employee;
  return public;
}
```

**Exclude Auto-Generated Fields**

```typescript
interface DbRecord {
  id: string;
  createdAt: Date;
  updatedAt: Date;
  data: any;
}

type CreateRecordInput = Omit<DbRecord, 'id' | 'createdAt' | 'updatedAt'>;

async function createRecord(input: CreateRecordInput): Promise<DbRecord> {
  return db.records.create({
    ...input,
    id: generateId(),
    createdAt: new Date(),
    updatedAt: new Date()
  });
}
```

**Extend and Modify**

```typescript
interface BaseConfig {
  host: string;
  port: number;
  timeout: number;
}

// Replace timeout with different type
interface ExtendedConfig extends Omit<BaseConfig, 'timeout'> {
  timeout: {
    connect: number;
    read: number;
  };
}

const config: ExtendedConfig = {
  host: 'localhost',
  port: 3000,
  timeout: {
    connect: 5000,
    read: 30000
  }
};
```

## Pick vs Omit

Choose based on which is more concise:

```typescript
interface LargeType {
  a: string;
  b: number;
  c: boolean;
  d: Date;
  e: string[];
  f: object;
}

// If you need most properties, use Omit
type MostProps = Omit<LargeType, 'f'>;

// If you need few properties, use Pick
type FewProps = Pick<LargeType, 'a' | 'b'>;
```

## Combining Pick and Omit

### PickAndModify

```typescript
type ModifyProp<T, K extends keyof T, NewType> = 
  Omit<T, K> & Record<K, NewType>;

interface User {
  id: string;
  name: string;
  age: number;
}

// Change age from number to string
type UserWithStringAge = ModifyProp<User, 'age', string>;
// {
//   id: string;
//   name: string;
//   age: string;
// }
```

### PickOptional

```typescript
type PickOptional<T, K extends keyof T> = 
  Pick<T, K> & Partial<Omit<T, K>>;

interface Config {
  host: string;
  port: number;
  ssl: boolean;
  timeout: number;
}

// host and port required, others optional
type MinimalConfig = PickOptional<Config, 'host' | 'port'>;
// {
//   host: string;
//   port: number;
//   ssl?: boolean;
//   timeout?: number;
// }
```

## Best Practices

1. **Use Pick for allowlists** - Explicitly state what you want
2. **Use Omit for denylists** - Remove what you don't want
3. **Prefer Pick for public APIs** - More explicit about exposed fields
4. **Use Omit for extending types** - Remove fields before adding new ones
5. **Combine with other utilities** - Partial, Required, Readonly

## Common Patterns

### API Layer Types

```typescript
interface DbUser {
  id: string;
  username: string;
  email: string;
  passwordHash: string;
  role: string;
}

// Public API response
type UserDto = Omit<DbUser, 'passwordHash'>;

// Create user input
type CreateUserDto = Omit<DbUser, 'id'>;

// Update user input
type UpdateUserDto = Partial<Omit<DbUser, 'id' | 'passwordHash'>>;
```

### Form Handling

```typescript
interface Article {
  id: string;
  title: string;
  content: string;
  authorId: string;
  publishedAt: Date | null;
  createdAt: Date;
}

type ArticleFormData = Pick<Article, 'title' | 'content'>;
type ArticleCreateData = Omit<Article, 'id' | 'createdAt'>;
```

## Common Pitfalls

- **Omitting non-existent keys** - No error, but might indicate typo
- **Using Pick with too many keys** - Consider Omit instead
- **Forgetting to update after schema changes** - Can expose sensitive data
- **Circular dependencies** - Avoid complex Pick/Omit chains
