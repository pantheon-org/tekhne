# Type-Safe Event Emitter

Building event emitters with full type safety for event names and payloads.

## Basic Event Emitter

```typescript
type Events = {
  'user:login': { userId: string; timestamp: Date };
  'user:logout': { userId: string };
  'data:update': { id: number; data: unknown };
};

class TypedEventEmitter<T extends Record<string, unknown>> {
  private listeners = new Map<keyof T, Set<(payload: unknown) => void>>();

  on<K extends keyof T>(event: K, handler: (payload: T[K]) => void): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(handler as (payload: unknown) => void);
  }

  emit<K extends keyof T>(event: K, payload: T[K]): void {
    const handlers = this.listeners.get(event);
    if (handlers) {
      handlers.forEach(handler => handler(payload));
    }
  }

  off<K extends keyof T>(event: K, handler: (payload: T[K]) => void): void {
    const handlers = this.listeners.get(event);
    if (handlers) {
      handlers.delete(handler as (payload: unknown) => void);
    }
  }
}

// Usage with full type safety
const emitter = new TypedEventEmitter<Events>();

emitter.on('user:login', (payload) => {
  // payload is typed as { userId: string; timestamp: Date }
  console.log(payload.userId, payload.timestamp);
});

emitter.emit('user:login', { 
  userId: '123', 
  timestamp: new Date() 
}); // ✓

// emitter.emit('user:login', { wrong: 'data' }); // ✗ Type error
```

## With Once Support

```typescript
class EnhancedEventEmitter<T extends Record<string, unknown>> {
  private listeners = new Map<keyof T, Set<(payload: unknown) => void>>();
  private onceListeners = new Map<keyof T, Set<(payload: unknown) => void>>();

  once<K extends keyof T>(event: K, handler: (payload: T[K]) => void): void {
    if (!this.onceListeners.has(event)) {
      this.onceListeners.set(event, new Set());
    }
    this.onceListeners.get(event)!.add(handler as (payload: unknown) => void);
  }

  emit<K extends keyof T>(event: K, payload: T[K]): void {
    // Call regular listeners
    const handlers = this.listeners.get(event);
    if (handlers) {
      handlers.forEach(handler => handler(payload));
    }

    // Call and remove once listeners
    const onceHandlers = this.onceListeners.get(event);
    if (onceHandlers) {
      onceHandlers.forEach(handler => handler(payload));
      this.onceListeners.delete(event);
    }
  }
}
```

## Async Event Emitter

```typescript
class AsyncEventEmitter<T extends Record<string, unknown>> {
  private listeners = new Map<keyof T, Set<(payload: unknown) => Promise<void>>>();

  on<K extends keyof T>(event: K, handler: (payload: T[K]) => Promise<void>): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(handler as (payload: unknown) => Promise<void>);
  }

  async emit<K extends keyof T>(event: K, payload: T[K]): Promise<void> {
    const handlers = this.listeners.get(event);
    if (handlers) {
      await Promise.all(
        Array.from(handlers).map(handler => handler(payload))
      );
    }
  }
}
```

## Best Practices

1. **Define Event Types**: Use a type alias for all events
2. **Strong Typing**: Ensure event names and payloads are strictly typed
3. **Memory Management**: Provide `off` method to prevent memory leaks
4. **Error Handling**: Wrap handler calls in try-catch blocks
5. **Async Support**: Use AsyncEventEmitter for async handlers

## See Also

- patterns-builder.md
- patterns-state-machine.md
- guards-type-predicates.md
