# TypeScript Strict Mode

## Overview

`"strict": true` enables all strict type checking options. It's the foundation of type-safe TypeScript.

## Strict Mode Flags

### Core Flags (enabled by `strict`)

**noImplicitAny** - Disallow implicit `any` types
```typescript
// Error with noImplicitAny
function greet(name) { // Parameter 'name' implicitly has an 'any' type
  console.log(`Hello, ${name}`);
}

// Fix: Add explicit type
function greet(name: string) {
  console.log(`Hello, ${name}`);
}
```

**strictNullChecks** - `null` and `undefined` require explicit handling
```typescript
// Error with strictNullChecks
function getLength(str: string | null) {
  return str.length; // Object is possibly 'null'
}

// Fix: Check for null
function getLength(str: string | null) {
  return str?.length ?? 0;
}
```

**strictFunctionTypes** - Stricter checking of function parameter types
```typescript
type Handler = (value: string) => void;
type GenericHandler = (value: string | number) => void;

// Error: Type 'GenericHandler' is not assignable to 'Handler'
const handler: Handler = (value: string | number) => {};
```

**strictBindCallApply** - Check `bind`, `call`, and `apply` arguments
```typescript
function greet(name: string, age: number) {
  console.log(`${name} is ${age} years old`);
}

greet.call(undefined, "Alice", "30"); // Error: Argument of type 'string' not assignable to 'number'
```

**strictPropertyInitialization** - Ensure class properties are initialized
```typescript
class User {
  name: string; // Error: Property 'name' has no initializer
  age: number = 0; // OK: Has initializer
  email!: string; // OK: Definite assignment assertion
  
  constructor() {
    this.name = ""; // OK: Initialized in constructor
  }
}
```

**noImplicitThis** - Disallow implicit `any` for `this`
```typescript
// Error with noImplicitThis
const obj = {
  name: "Alice",
  greet() {
    return function() {
      console.log(this.name); // 'this' implicitly has type 'any'
    };
  }
};

// Fix: Use arrow function
const obj = {
  name: "Alice",
  greet() {
    return () => {
      console.log(this.name); // OK: Arrow function captures 'this'
    };
  }
};
```

**alwaysStrict** - Emit `"use strict"` in generated JavaScript
- Enables JavaScript strict mode
- Prevents common pitfalls (e.g., accidental globals)

## Additional Recommended Flags

**noUnusedLocals** - Error on unused local variables
```typescript
function calculate() {
  const x = 10; // Error: 'x' is declared but never used
  return 5;
}
```

**noUnusedParameters** - Error on unused function parameters
```typescript
function greet(name: string, age: number) { // Error: 'age' is declared but never used
  console.log(`Hello, ${name}`);
}

// Fix: Prefix with underscore
function greet(name: string, _age: number) {
  console.log(`Hello, ${name}`);
}
```

**noImplicitReturns** - Error if not all code paths return a value
```typescript
function getStatus(value: number): string {
  if (value > 0) {
    return "positive";
  }
  // Error: Not all code paths return a value
}

// Fix: Handle all paths
function getStatus(value: number): string {
  if (value > 0) {
    return "positive";
  }
  return "non-positive";
}
```

**noFallthroughCasesInSwitch** - Error on switch fallthrough
```typescript
function getDay(day: number): string {
  switch (day) {
    case 0:
      return "Sunday";
    case 1: // Error: Fallthrough case in switch
      console.log("Monday");
    case 2:
      return "Tuesday";
  }
  return "Unknown";
}
```

**noUncheckedIndexedAccess** - Index signatures return `T | undefined`
```typescript
const colors: Record<string, string> = { red: "#ff0000" };

// Without noUncheckedIndexedAccess
const red = colors["red"]; // Type: string

// With noUncheckedIndexedAccess
const red = colors["red"]; // Type: string | undefined
if (red) {
  console.log(red.toUpperCase()); // Safe
}
```

**exactOptionalPropertyTypes** - Distinguish `undefined` from missing
```typescript
interface Config {
  timeout?: number;
}

// Without exactOptionalPropertyTypes
const config: Config = { timeout: undefined }; // OK

// With exactOptionalPropertyTypes
const config: Config = { timeout: undefined }; // Error
const config: Config = {}; // OK
```

## Migration Strategy

Enabling strict mode in an existing codebase:

1. **Enable flags incrementally**
```json
{
  "compilerOptions": {
    "noImplicitAny": true,
    // Fix errors, then enable next flag
    "strictNullChecks": true,
    // Continue until all strict flags enabled
  }
}
```

2. **Use `@ts-expect-error` for temporary suppression**
```typescript
// @ts-expect-error TODO: Fix null handling
const value = maybeNull.property;
```

3. **Fix by priority**
   - Start with noImplicitAny (easiest)
   - Then strictNullChecks (most impactful)
   - Finally other flags

4. **Use `strictNullChecks` migration helpers**
```typescript
// Option 1: Non-null assertion (use sparingly)
const value = maybeNull!.property;

// Option 2: Optional chaining
const value = maybeNull?.property;

// Option 3: Nullish coalescing
const value = maybeNull ?? defaultValue;
```

## Benefits

- **Catch errors at compile time** - Before runtime
- **Better IDE support** - More accurate autocomplete
- **Self-documenting code** - Types show intent
- **Easier refactoring** - Type errors guide changes
- **Prevent null reference errors** - strictNullChecks catches most

## Best Practices

1. **Enable strict mode in new projects** - Start with `"strict": true`
2. **Enable noUncheckedIndexedAccess** - Not included in `strict` but highly recommended
3. **Enable exactOptionalPropertyTypes** - Clearer optional semantics
4. **Use `unknown` over `any`** - Forces type checking
5. **Avoid `!` assertions** - Prefer explicit null checks
6. **Add JSDoc for complex types** - Helps other developers

## Common Pitfalls

- **Using `as any` to bypass errors** - Defeats purpose of strict mode
- **Overusing `!` assertions** - Hides potential null errors
- **Not enabling noUncheckedIndexedAccess** - Still allows unsafe index access
- **Disabling strict for quick fixes** - Technical debt accumulates
