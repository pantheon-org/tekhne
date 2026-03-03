# Exhaustiveness Checking

## Overview
Exhaustiveness checking ensures all possible cases in a union type are handled, preventing runtime errors from unhandled cases.

## Basic Exhaustiveness with never

```typescript
type Status = "pending" | "success" | "error";

function handleStatus(status: Status): string {
  switch (status) {
    case "pending":
      return "Loading...";
    case "success":
      return "Done!";
    case "error":
      return "Failed!";
    default:
      // If all cases are handled, status is never here
      const _exhaustive: never = status;
      throw new Error(`Unhandled status: ${_exhaustive}`);
  }
}
```

## Exhaustiveness with Discriminated Unions

```typescript
type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "square"; size: number }
  | { kind: "rectangle"; width: number; height: number };

function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle":
      return Math.PI * shape.radius ** 2;
    case "square":
      return shape.size ** 2;
    case "rectangle":
      return shape.width * shape.height;
    default:
      const _exhaustive: never = shape;
      throw new Error(`Unhandled shape kind: ${_exhaustive}`);
  }
}

// If we add a new shape type:
// type Shape = ... | { kind: "triangle"; base: number; height: number };
// TypeScript will error at the _exhaustive assignment, forcing us to handle it
```

## Exhaustiveness Helper Function

```typescript
function assertNever(value: never): never {
  throw new Error(`Unexpected value: ${JSON.stringify(value)}`);
}

type Action =
  | { type: "increment" }
  | { type: "decrement" }
  | { type: "reset"; value: number };

function reducer(state: number, action: Action): number {
  switch (action.type) {
    case "increment":
      return state + 1;
    case "decrement":
      return state - 1;
    case "reset":
      return action.value;
    default:
      return assertNever(action);
  }
}
```

## Exhaustiveness with if/else

```typescript
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

function unwrap<T, E>(result: Result<T, E>): T {
  if (result.ok) {
    return result.value;
  } else if (!result.ok) {
    throw result.error;
  } else {
    // This branch is never reachable if all cases are handled
    const _exhaustive: never = result;
    throw new Error(`Unhandled result: ${_exhaustive}`);
  }
}
```

## Exhaustiveness in Pattern Matching

```typescript
type Option<T> = { _tag: "Some"; value: T } | { _tag: "None" };

function match<T, R>(
  option: Option<T>,
  patterns: {
    Some: (value: T) => R;
    None: () => R;
  }
): R {
  switch (option._tag) {
    case "Some":
      return patterns.Some(option.value);
    case "None":
      return patterns.None();
    default:
      const _exhaustive: never = option;
      throw new Error(`Unhandled option: ${_exhaustive}`);
  }
}

const result = match({ _tag: "Some", value: 42 }, {
  Some: (v) => `Got ${v}`,
  None: () => "Nothing"
});
```

## Exhaustiveness in Array Methods

```typescript
type EventType = "click" | "keypress" | "scroll";

const events: EventType[] = ["click", "keypress", "scroll"];

function getEventHandler(event: EventType): () => void {
  switch (event) {
    case "click":
      return () => console.log("Clicked");
    case "keypress":
      return () => console.log("Key pressed");
    case "scroll":
      return () => console.log("Scrolled");
    default:
      const _exhaustive: never = event;
      throw new Error(`No handler for event: ${_exhaustive}`);
  }
}

events.forEach(event => {
  const handler = getEventHandler(event);
  handler();
});
```

## Exhaustiveness with Enums

```typescript
enum Direction {
  North,
  South,
  East,
  West
}

function move(direction: Direction): { x: number; y: number } {
  switch (direction) {
    case Direction.North:
      return { x: 0, y: 1 };
    case Direction.South:
      return { x: 0, y: -1 };
    case Direction.East:
      return { x: 1, y: 0 };
    case Direction.West:
      return { x: -1, y: 0 };
    default:
      const _exhaustive: never = direction;
      throw new Error(`Unhandled direction: ${_exhaustive}`);
  }
}
```

## Exhaustiveness in Async Handlers

```typescript
type AsyncResult<T> =
  | { status: "loading" }
  | { status: "success"; data: T }
  | { status: "error"; error: Error };

async function handleAsyncResult<T>(result: AsyncResult<T>): Promise<string> {
  switch (result.status) {
    case "loading":
      return "Loading...";
    case "success":
      return `Success: ${JSON.stringify(result.data)}`;
    case "error":
      return `Error: ${result.error.message}`;
    default:
      const _exhaustive: never = result;
      throw new Error(`Unhandled async result: ${_exhaustive}`);
  }
}
```

## Compile-Time Only Exhaustiveness

```typescript
// For cases where throwing is undesirable
function assertExhaustive(value: never): void {
  // Intentionally empty - only for compile-time checking
}

type Theme = "light" | "dark" | "auto";

function applyTheme(theme: Theme) {
  switch (theme) {
    case "light":
      document.body.className = "light";
      break;
    case "dark":
      document.body.className = "dark";
      break;
    case "auto":
      document.body.className = "auto";
      break;
    default:
      // Compile-time error if cases are missing, no runtime cost
      assertExhaustive(theme);
  }
}
```

## Exhaustiveness with Return

```typescript
type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

function isIdempotent(method: HttpMethod): boolean {
  switch (method) {
    case "GET":
    case "PUT":
    case "DELETE":
      return true;
    case "POST":
      return false;
    default:
      const _exhaustive: never = method;
      throw new Error(`Unknown method: ${_exhaustive}`);
  }
}
```

## Best Practices
- Always include exhaustiveness check in switch default
- Use a helper function like assertNever for consistency
- Place exhaustiveness check in default/else branches
- Use descriptive error messages including the value
- Leverage exhaustiveness to catch missing cases during refactoring
- Combine with discriminated unions for type-safe pattern matching

## Common Pitfalls
- Forgetting default case (exhaustiveness not enforced)
- Using any or unknown instead of never (breaks exhaustiveness)
- Returning undefined in default instead of throwing (masks bugs)
- Not including exhaustiveness check in if/else chains
- Catching exhaustiveness errors and continuing (defeats purpose)
