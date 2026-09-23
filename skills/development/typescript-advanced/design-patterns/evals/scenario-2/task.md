# Scenario: Model a Connection Lifecycle as a Type-Safe State Machine

A WebSocket wrapper currently tracks its connection state with a single class holding several optional fields, which allows nonsensical combinations (e.g. a `socket` present while `status` is `"disconnected"`) and lets a caller call `send()` while disconnected.

```typescript
// connection.ts

class Connection {
  status: string = "disconnected";
  socket?: WebSocket;
  retryCount: number = 0;
  lastError?: Error;

  connect(): void { /* sets status = "connecting", eventually "connected" */ }
  send(data: string): void {
    this.socket?.send(data); // silently no-ops if not connected — no compile-time signal
  }
}
```

Rewrite `connection.ts` so that:

1. Connection state is modelled as a discriminated union (`{ status: "disconnected" } | { status: "connecting"; startTime: number } | { status: "connected"; socket: WebSocket } | { status: "error"; error: Error; retryCount: number }`).
2. A `transition(state, event)` function returns a new state given the current state and an event string, using exhaustive switching on `status` with a `never` guard in the default branch.
3. A `send(state, data)` function only compiles when given a state whose `status` is `"connected"` (i.e. its parameter type is narrowed to that specific union member, not the whole union).
4. A commented-out call demonstrates that `send(state, data)` fails to compile when `state`'s type is the full union rather than the narrowed `"connected"` member.

## Output Specification

Produce a single file `connection.ts` containing the discriminated union, `transition`, `send`, and the example usage including the commented-out compile-error case.
