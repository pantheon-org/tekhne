# Partial and Required Utility Types

## Partial<T>

Makes all properties in T optional.

### Basic Usage

```typescript
interface User {
  id: string;
  name: string;
  email: string;
  age: number;
}

type PartialUser = Partial<User>;
// {
//   id?: string;
//   name?: string;
//   email?: string;
//   age?: number;
// }

function updateUser(id: string, updates: Partial<User>) {
  // Apply only the provided fields
}

updateUser('123', { name: 'John' }); // ✅ Only name provided
updateUser('123', { age: 30, email: 'john@example.com' }); // ✅
```

### Implementation

```typescript
type Partial<T> = {
  [P in keyof T]?: T[P];
};
```

### Use Cases

**API Updates (PATCH)**

```typescript
async function patchUser(id: string, updates: Partial<User>) {
  return fetch(`/api/users/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(updates)
  });
}
```

**Form State**

```typescript
interface FormData {
  username: string;
  password: string;
  email: string;
}

function FormComponent() {
  const [formData, setFormData] = useState<Partial<FormData>>({});
  
  const updateField = (field: keyof FormData, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };
}
```

**Default Configuration**

```typescript
interface Config {
  host: string;
  port: number;
  timeout: number;
  retries: number;
}

const defaultConfig: Config = {
  host: 'localhost',
  port: 3000,
  timeout: 5000,
  retries: 3
};

function createConfig(overrides: Partial<Config>): Config {
  return { ...defaultConfig, ...overrides };
}

const config = createConfig({ port: 8080 }); // ✅ Only override port
```

## Required<T>

Makes all properties in T required.

### Basic Usage

```typescript
interface User {
  id?: string;
  name?: string;
  email?: string;
}

type CompleteUser = Required<User>;
// {
//   id: string;
//   name: string;
//   email: string;
// }

function saveUser(user: Required<User>) {
  // All fields guaranteed to exist
  console.log(user.id.toUpperCase()); // ✅ No undefined check needed
}
```

### Implementation

```typescript
type Required<T> = {
  [P in keyof T]-?: T[P]; // -? removes optional modifier
};
```

### Use Cases

**Database Operations**

```typescript
interface DraftPost {
  title?: string;
  content?: string;
  authorId?: string;
  publishedAt?: Date;
}

// Database requires all fields
function insertPost(post: Required<DraftPost>) {
  db.posts.insert(post);
}

// Validation before saving
function validateAndSave(draft: DraftPost) {
  if (isComplete(draft)) {
    insertPost(draft as Required<DraftPost>);
  }
}

function isComplete(draft: DraftPost): draft is Required<DraftPost> {
  return !!(draft.title && draft.content && draft.authorId);
}
```

**API Response Validation**

```typescript
interface ApiUser {
  id?: string;
  name?: string;
  email?: string;
}

function validateUser(user: ApiUser): Required<ApiUser> {
  if (!user.id || !user.name || !user.email) {
    throw new Error('Invalid user data');
  }
  return user as Required<ApiUser>;
}
```

## Combining Partial and Required

### Selective Optionality

```typescript
type PartialBy<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

interface User {
  id: string;
  name: string;
  email: string;
  age: number;
}

// Make only age optional
type UserWithOptionalAge = PartialBy<User, 'age'>;
// {
//   id: string;
//   name: string;
//   email: string;
//   age?: number;
// }
```

### Selective Requirements

```typescript
type RequiredBy<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

interface DraftUser {
  id?: string;
  name?: string;
  email?: string;
}

// Require only name and email
type MinimalUser = RequiredBy<DraftUser, 'name' | 'email'>;
// {
//   id?: string;
//   name: string;
//   email: string;
// }
```

## Deep Partial/Required

For nested objects:

```typescript
type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

type DeepRequired<T> = {
  [P in keyof T]-?: T[P] extends object ? DeepRequired<T[P]> : T[P];
};

interface NestedConfig {
  server: {
    host: string;
    port: number;
    ssl: {
      enabled: boolean;
      cert: string;
    };
  };
}

type PartialConfig = DeepPartial<NestedConfig>;
// All nested properties become optional

const config: PartialConfig = {
  server: {
    port: 8080
    // ssl not required, host not required
  }
};
```

## Best Practices

1. **Use Partial for updates** - PATCH operations, form updates
2. **Use Required for validation** - Ensure complete data before processing
3. **Combine with type guards** - Validate before treating as Required
4. **Consider deep variants** - For nested objects
5. **Document assumptions** - Comment why fields are optional/required

## Common Pitfalls

- **Assuming Partial values exist** - Always check before accessing
- **Over-using Partial** - Makes API unclear about required fields
- **Type assertion without validation** - Don't cast to Required without checking
- **Ignoring nested optionality** - Use DeepPartial for nested objects
