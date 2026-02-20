# Type Assertions & Type Compatibility

## Type Assertions

Type assertions tell TypeScript to treat a value as a specific type.

### as Syntax (Preferred)

```typescript
const value: unknown = "hello";
const strLength = (value as string).length;

// DOM elements
const input = document.querySelector('input') as HTMLInputElement;
input.value = 'text';
```

### Angle Bracket Syntax (JSX-incompatible)

```typescript
const value: unknown = "hello";
const strLength = (<string>value).length;
```

**Note:** Use `as` syntax - it works in JSX/TSX files.

### Non-null Assertion (!)

```typescript
function getUser(id: string) {
  const user = users.find(u => u.id === id);
  // Assert user is not null/undefined
  return user!.name;
}

// Optional chaining is safer
return users.find(u => u.id === id)?.name;
```

**Warning:** Only use `!` when you're certain the value exists. Prefer optional chaining.

### Const Assertions

```typescript
// Infer literal types instead of widening
const config = {
  endpoint: '/api/users',
  method: 'GET'
} as const;
// Type: { readonly endpoint: "/api/users"; readonly method: "GET"; }

// Array becomes readonly tuple
const colors = ['red', 'green', 'blue'] as const;
// Type: readonly ["red", "green", "blue"]

// Use in function parameters
function request<T extends string>(method: T) {
  // method is the literal type, not string
}
request(config.method); // ✅ method is "GET"
```

### Satisfies Operator (TypeScript 4.9+)

```typescript
// Validate type without widening
const config = {
  endpoint: '/api/users',
  method: 'GET',
  timeout: 5000
} satisfies Config;

// config.method is still "GET", not string
// Unlike 'as Config', satisfies preserves literal types

type Color = { r: number; g: number; b: number } | string;

const palette = {
  red: { r: 255, g: 0, b: 0 },
  blue: '#0000FF'
} satisfies Record<string, Color>;

// palette.red.r is accessible (object structure preserved)
// With 'as Record<string, Color>', red.r would error
```

## Type Compatibility

TypeScript uses structural typing (shape-based).

### Structural Typing

```typescript
interface Point {
  x: number;
  y: number;
}

class Point2D {
  constructor(public x: number, public y: number) {}
}

const point: Point = new Point2D(10, 20); // ✅ Compatible
```

### Function Compatibility

```typescript
type Handler = (msg: string, code: number) => void;

// ✅ Fewer parameters is compatible
const handler: Handler = (msg: string) => {
  console.log(msg);
};

// ❌ More parameters is not compatible
const handler2: Handler = (msg: string, code: number, extra: boolean) => {};
```

### Bivariance in Method Parameters

```typescript
interface Animal { name: string; }
interface Dog extends Animal { breed: string; }

class Vet {
  treat(animal: Animal) {}
}

class DogVet extends Vet {
  // ⚠️ Allowed but unsound
  treat(dog: Dog) {}
}
```

## Best Practices

1. **Prefer type guards over assertions** - Type guards are type-safe
2. **Use `satisfies` for validation without widening** - Preserves literal types
3. **Const assertions for literal types** - Use `as const` for immutable literal types
4. **Avoid non-null assertion (!)** - Use optional chaining or type guards instead
5. **Double assertions as last resort** - `value as unknown as TargetType` indicates design issue

## Common Pitfalls

- **Overusing `as`** - Defeats type checking, prefer narrowing
- **Non-null assertion on uncertain values** - Can cause runtime errors
- **Ignoring structural compatibility** - Extra properties can cause issues with fresh object literals
