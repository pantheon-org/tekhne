# Extract and Exclude Utility Types

## Exclude<T, U>

Removes types from T that are assignable to U.

### Basic Usage

```typescript
type AllColors = 'red' | 'green' | 'blue' | 'yellow';

type PrimaryColors = Exclude<AllColors, 'yellow'>;
// Type: 'red' | 'green' | 'blue'

type NonBlueColors = Exclude<AllColors, 'blue' | 'green'>;
// Type: 'red' | 'yellow'
```

### Implementation

```typescript
type Exclude<T, U> = T extends U ? never : T;
```

**How it works:** Distributive conditional type - checks each member of T union.

### Use Cases

**Filter Union Types**

```typescript
type Status = 'pending' | 'approved' | 'rejected' | 'cancelled';

type ActiveStatus = Exclude<Status, 'cancelled'>;
// 'pending' | 'approved' | 'rejected'

function processActiveOrder(status: ActiveStatus) {
  // status cannot be 'cancelled'
}
```

**Remove null/undefined**

```typescript
type MaybeString = string | null | undefined;

type DefiniteString = Exclude<MaybeString, null | undefined>;
// Type: string

// Note: NonNullable does the same thing
type DefiniteString2 = NonNullable<MaybeString>;
// Type: string
```

**Filter by Type**

```typescript
type Mixed = string | number | boolean | null | undefined;

type OnlyPrimitives = Exclude<Mixed, null | undefined>;
// string | number | boolean

type OnlyStringsAndNumbers = Exclude<Mixed, boolean | null | undefined>;
// string | number
```

**Remove Function Types**

```typescript
type Value = string | number | (() => void) | (() => string);

type NonFunction = Exclude<Value, Function>;
// string | number
```

## Extract<T, U>

Extracts from T only types that are assignable to U (opposite of Exclude).

### Basic Usage

```typescript
type AllColors = 'red' | 'green' | 'blue' | 'yellow';

type WarmColors = Extract<AllColors, 'red' | 'yellow' | 'orange'>;
// Type: 'red' | 'yellow' (orange doesn't exist in AllColors)
```

### Implementation

```typescript
type Extract<T, U> = T extends U ? T : never;
```

### Use Cases

**Filter to Specific Types**

```typescript
type Mixed = string | number | boolean | null;

type StringOrNumber = Extract<Mixed, string | number>;
// string | number

type OnlyString = Extract<Mixed, string>;
// string
```

**Extract Function Types**

```typescript
type Value = string | number | (() => void) | (() => string);

type OnlyFunctions = Extract<Value, Function>;
// (() => void) | (() => string)
```

**Extract Object Types**

```typescript
type Mixed = string | number | { name: string } | { id: number };

type OnlyObjects = Extract<Mixed, object>;
// { name: string } | { id: number }
```

**Pattern Matching with Strings**

```typescript
type Events = 'onClick' | 'onHover' | 'onFocus' | 'handleClick' | 'handleSubmit';

// Extract events starting with 'on'
type OnEvents = Extract<Events, `on${string}`>;
// 'onClick' | 'onHover' | 'onFocus'

type HandleEvents = Extract<Events, `handle${string}`>;
// 'handleClick' | 'handleSubmit'
```

## Combining Extract and Exclude

### Complex Filtering

```typescript
type ApiResponse = 
  | { status: 'success'; data: any }
  | { status: 'error'; error: string }
  | { status: 'loading' }
  | { status: 'idle' };

// Extract responses with data/error
type ResponseWithPayload = Extract<ApiResponse, { data: any } | { error: any }>;
// { status: 'success'; data: any } | { status: 'error'; error: string }

// Exclude loading states
type CompletedResponse = Exclude<ApiResponse, { status: 'loading' | 'idle' }>;
// { status: 'success'; data: any } | { status: 'error'; error: string }
```

### Type-Safe Event Handlers

```typescript
type EventMap = {
  click: MouseEvent;
  keydown: KeyboardEvent;
  scroll: Event;
  custom: CustomEvent;
};

type EventName = keyof EventMap;

// Extract only standard DOM events
type StandardEvents = Extract<EventName, 'click' | 'keydown' | 'scroll'>;
// 'click' | 'keydown' | 'scroll'

// Exclude standard events (get custom ones)
type CustomEvents = Exclude<EventName, 'click' | 'keydown' | 'scroll'>;
// 'custom'
```

## NonNullable<T>

Built-in utility using Exclude:

```typescript
type NonNullable<T> = Exclude<T, null | undefined>;

type MaybeString = string | null | undefined;
type DefiniteString = NonNullable<MaybeString>;
// string
```

## Practical Patterns

### Filter Props by Value Type

```typescript
type User = {
  id: string;
  name: string;
  age: number;
  isActive: boolean;
  email: string;
};

// Get keys with string values
type StringKeys<T> = {
  [K in keyof T]: T[K] extends string ? K : never;
}[keyof T];

type UserStringKeys = StringKeys<User>;
// 'id' | 'name' | 'email'

// Extract string properties
type StringProps = Pick<User, Extract<keyof User, StringKeys<User>>>;
```

### Exclude Internal Properties

```typescript
interface Component {
  _id: string;
  _internal: boolean;
  name: string;
  value: string;
}

type PublicKeys<T> = Exclude<keyof T, `_${string}`>;
type PublicComponent = Pick<Component, PublicKeys<Component>>;
// {
//   name: string;
//   value: string;
// }
```

### Type-Safe Union Filtering

```typescript
type Shape =
  | { kind: 'circle'; radius: number }
  | { kind: 'square'; size: number }
  | { kind: 'rectangle'; width: number; height: number };

type RoundShapes = Extract<Shape, { kind: 'circle' }>;
// { kind: 'circle'; radius: number }

type NonCircleShapes = Exclude<Shape, { kind: 'circle' }>;
// { kind: 'square'; size: number } | { kind: 'rectangle'; width: number; height: number }
```

## Best Practices

1. **Use Exclude for denylists** - Remove unwanted types from unions
2. **Use Extract for allowlists** - Keep only wanted types from unions
3. **Combine with conditional types** - Build complex type transformations
4. **Use NonNullable for null safety** - Remove null/undefined explicitly
5. **Extract with template literals** - Pattern match string literal types

## Common Patterns

### API Status Types

```typescript
type ApiStatus = 'idle' | 'loading' | 'success' | 'error';

type ActiveStatus = Exclude<ApiStatus, 'idle'>;
// 'loading' | 'success' | 'error'

type CompletedStatus = Extract<ApiStatus, 'success' | 'error'>;
// 'success' | 'error'
```

### Permission Filtering

```typescript
type Permission = 'read' | 'write' | 'delete' | 'admin';

type BasicPermissions = Exclude<Permission, 'admin'>;
// 'read' | 'write' | 'delete'

type ModifyPermissions = Extract<Permission, 'write' | 'delete'>;
// 'write' | 'delete'
```

## Common Pitfalls

- **Excluding non-existent types** - No error, returns original type
- **Extract with no matches** - Returns `never`
- **Forgetting distributive behavior** - Works on union members, not object properties
- **Using on non-union types** - Exclude/Extract are for unions, not object shapes
