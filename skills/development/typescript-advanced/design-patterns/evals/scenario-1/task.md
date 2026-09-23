# Scenario: Type-Safe Event Emitter

A notification system's event emitter currently types both `emit` and `on` with a loose `string` event name and `any[]` payload, so a caller can emit an event with the wrong payload shape and nothing catches it until runtime.

```typescript
// emitter.ts

class NotificationEmitter {
  private handlers: Record<string, ((...args: any[]) => void)[]> = {};

  on(event: string, handler: (...args: any[]) => void): void {
    (this.handlers[event] ??= []).push(handler);
  }

  emit(event: string, ...args: any[]): void {
    (this.handlers[event] ?? []).forEach((h) => h(...args));
  }
}

const emitter = new NotificationEmitter();
emitter.on("user:created", (user: { id: string; name: string }) => console.log(user.name));
emitter.emit("user:created", { id: "1" }); // missing `name` — compiles anyway
```

Rewrite `emitter.ts` so that:

1. `NotificationEmitter` is generic over an event map type (`Record<string, (...args: any[]) => void>`-shaped) defined for this notification system, with at least two events: `"user:created"` carrying `{ id: string; name: string }`, and `"user:deleted"` carrying `{ id: string }`.
2. `on(event, handler)` requires `handler` to match the parameter types for that specific `event` key.
3. `emit(event, ...args)` requires `args` to match the parameter types for that specific `event` key.
4. The example call site is updated so that `emitter.emit("user:created", { id: "1" })` (missing `name`) fails to compile, shown as a commented-out line.

## Output Specification

Produce a single file `emitter.ts` containing the generic `NotificationEmitter`, the event map type, and the example call site including the commented-out compile-error case.
