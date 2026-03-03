# Recursive Types

Define types that reference themselves for nested structures.

## JSON Value

```typescript
type JSONValue =
  | string
  | number
  | boolean
  | null
  | JSONValue[]
  | { [key: string]: JSONValue };

const data: JSONValue = {
  name: 'Alice',
  age: 30,
  address: {
    street: '123 Main St',
    city: 'Boston'
  },
  hobbies: ['reading', 'coding']
};
```

## Deep Partial

```typescript
type DeepPartial<T> = T extends object
  ? { [P in keyof T]?: DeepPartial<T[P]> }
  : T;

interface User {
  id: string;
  profile: {
    name: string;
    address: {
      street: string;
      city: string;
    };
  };
}

const update: DeepPartial<User> = {
  profile: {
    address: {
      city: 'New York' // Other fields optional
    }
  }
};
```

## Deep Readonly

```typescript
type DeepReadonly<T> = T extends object
  ? { readonly [P in keyof T]: DeepReadonly<T[P]> }
  : T;

interface Config {
  database: {
    host: string;
    credentials: {
      username: string;
      password: string;
    };
  };
}

const config: DeepReadonly<Config> = {
  database: {
    host: 'localhost',
    credentials: {
      username: 'admin',
      password: 'secret'
    }
  }
};

// All levels are readonly
// config.database.host = 'new'; // Error
// config.database.credentials.password = 'new'; // Error
```

## Path Type

```typescript
type Path<T> = T extends object
  ? {
      [K in keyof T]: K extends string
        ? T[K] extends object
          ? K | `${K}.${Path<T[K]>}`
          : K
        : never;
    }[keyof T]
  : never;

interface Data {
  user: {
    profile: {
      name: string;
      age: number;
    };
    settings: {
      theme: string;
    };
  };
}

// Valid paths
type DataPath = Path<Data>;
// "user" | "user.profile" | "user.profile.name" | "user.profile.age"
// | "user.settings" | "user.settings.theme"

const path1: DataPath = 'user.profile.name'; // OK
const path2: DataPath = 'user.invalid'; // Error
```

## Tree Structure

```typescript
interface TreeNode<T> {
  value: T;
  children: TreeNode<T>[];
}

const tree: TreeNode<number> = {
  value: 1,
  children: [
    {
      value: 2,
      children: [
        { value: 4, children: [] },
        { value: 5, children: [] }
      ]
    },
    { value: 3, children: [] }
  ]
};

// Recursive function with recursive type
function sumTree(node: TreeNode<number>): number {
  return node.value + node.children.reduce(
    (sum, child) => sum + sumTree(child),
    0
  );
}
```

## Best Practices

1. **Set recursion limits**: TypeScript has depth limits (~50 levels)
2. **Use tail recursion**: When possible for better performance
3. **Test edge cases**: Empty structures, single nodes
4. **Document structure**: Explain the recursive nature
5. **Consider alternatives**: Sometimes iteration is clearer
