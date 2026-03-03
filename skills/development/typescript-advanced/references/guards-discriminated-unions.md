# Discriminated Unions Type Guards

## Overview
Discriminated unions (tagged unions) use a common property (discriminant) to enable exhaustive type narrowing and type-safe pattern matching.

## Basic Discriminated Union

```typescript
interface Success {
  status: "success";
  data: { id: number; name: string };
}

interface Error {
  status: "error";
  error: { code: number; message: string };
}

interface Loading {
  status: "loading";
}

type ApiResponse = Success | Error | Loading;

function handleResponse(response: ApiResponse) {
  switch (response.status) {
    case "success":
      // response is narrowed to Success
      console.log(response.data.name);
      break;
    case "error":
      // response is narrowed to Error
      console.error(response.error.message);
      break;
    case "loading":
      // response is narrowed to Loading
      console.log("Loading...");
      break;
  }
}
```

## Exhaustiveness Checking

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
      // Exhaustiveness check: ensures all cases handled
      const _exhaustive: never = shape;
      throw new Error(`Unhandled shape: ${_exhaustive}`);
  }
}
```

## Multiple Discriminants

```typescript
interface NetworkError {
  type: "error";
  category: "network";
  message: string;
}

interface ValidationError {
  type: "error";
  category: "validation";
  fields: string[];
}

interface Success {
  type: "success";
  data: unknown;
}

type Result = NetworkError | ValidationError | Success;

function handleResult(result: Result) {
  if (result.type === "error") {
    // result is narrowed to NetworkError | ValidationError
    if (result.category === "network") {
      // result is narrowed to NetworkError
      console.error(result.message);
    } else {
      // result is narrowed to ValidationError
      console.error("Invalid fields:", result.fields);
    }
  } else {
    // result is narrowed to Success
    console.log(result.data);
  }
}
```

## State Machine with Discriminated Unions

```typescript
type ConnectionState =
  | { status: "disconnected" }
  | { status: "connecting"; startTime: number }
  | { status: "connected"; socket: WebSocket }
  | { status: "error"; error: Error; retryCount: number };

function transition(state: ConnectionState, event: string): ConnectionState {
  switch (state.status) {
    case "disconnected":
      if (event === "connect") {
        return { status: "connecting", startTime: Date.now() };
      }
      return state;
    
    case "connecting":
      if (event === "success") {
        return { status: "connected", socket: new WebSocket("ws://...") };
      }
      if (event === "error") {
        return { status: "error", error: new Error("Failed"), retryCount: 0 };
      }
      return state;
    
    case "connected":
      if (event === "disconnect") {
        state.socket.close();
        return { status: "disconnected" };
      }
      return state;
    
    case "error":
      if (event === "retry" && state.retryCount < 3) {
        return { status: "connecting", startTime: Date.now() };
      }
      return state;
  }
}
```

## Discriminated Union Helper

```typescript
function isOfType<T extends { type: string }, K extends T["type"]>(
  item: T,
  type: K
): item is Extract<T, { type: K }> {
  return item.type === type;
}

type Action =
  | { type: "add"; value: number }
  | { type: "subtract"; value: number }
  | { type: "reset" };

function handle(action: Action) {
  if (isOfType(action, "add")) {
    // action is narrowed to { type: "add"; value: number }
    console.log(`Adding ${action.value}`);
  }
}
```

## Nested Discriminated Unions

```typescript
type ApiCall =
  | {
      method: "GET";
      params: { id: string };
    }
  | {
      method: "POST";
      body: { name: string; age: number };
    }
  | {
      method: "PUT";
      params: { id: string };
      body: { name: string };
    };

function makeRequest(call: ApiCall) {
  switch (call.method) {
    case "GET":
      // call has params, no body
      fetch(`/api/resource/${call.params.id}`);
      break;
    case "POST":
      // call has body, no params
      fetch("/api/resource", {
        method: "POST",
        body: JSON.stringify(call.body)
      });
      break;
    case "PUT":
      // call has both params and body
      fetch(`/api/resource/${call.params.id}`, {
        method: "PUT",
        body: JSON.stringify(call.body)
      });
      break;
  }
}
```

## Best Practices
- Use string literal types for discriminants (not enums)
- Place discriminant property first for clarity
- Always include exhaustiveness checking with never
- Keep discriminant property names consistent across unions
- Use descriptive discriminant values
- Prefer switch over if/else for clarity

## Common Pitfalls
- Using non-literal types as discriminants
- Forgetting exhaustiveness check when adding new variants
- Overlapping discriminants across union members
- Using optional discriminants (must be required)
- Not narrowing before accessing variant-specific properties
