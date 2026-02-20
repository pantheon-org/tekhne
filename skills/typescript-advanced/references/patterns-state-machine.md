# Type-Safe State Machines

Implementing state machines with discriminated unions for compile-time state validation.

## Basic State Machine

```typescript
type State =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; data: string }
  | { status: 'error'; error: Error };

type Event =
  | { type: 'FETCH' }
  | { type: 'SUCCESS'; data: string }
  | { type: 'ERROR'; error: Error }
  | { type: 'RESET' };

function reducer(state: State, event: Event): State {
  switch (state.status) {
    case 'idle':
      switch (event.type) {
        case 'FETCH':
          return { status: 'loading' };
        default:
          return state;
      }
    
    case 'loading':
      switch (event.type) {
        case 'SUCCESS':
          return { status: 'success', data: event.data };
        case 'ERROR':
          return { status: 'error', error: event.error };
        default:
          return state;
      }
    
    case 'success':
    case 'error':
      switch (event.type) {
        case 'RESET':
          return { status: 'idle' };
        case 'FETCH':
          return { status: 'loading' };
        default:
          return state;
      }
  }
}

// Usage
let state: State = { status: 'idle' };

state = reducer(state, { type: 'FETCH' });
// state is now { status: 'loading' }

state = reducer(state, { type: 'SUCCESS', data: 'Hello' });
// state is now { status: 'success', data: 'Hello' }

if (state.status === 'success') {
  console.log(state.data); // TypeScript knows data exists here
}
```

## Generic State Machine

```typescript
type StateMachine<
  States extends { status: string },
  Events extends { type: string }
> = {
  initialState: States;
  reducer: (state: States, event: Events) => States;
};

function createStateMachine<
  States extends { status: string },
  Events extends { type: string }
>(config: StateMachine<States, Events>) {
  let state = config.initialState;
  
  return {
    getState: () => state,
    dispatch: (event: Events) => {
      state = config.reducer(state, event);
      return state;
    },
  };
}

// Usage
const machine = createStateMachine<State, Event>({
  initialState: { status: 'idle' },
  reducer
});

machine.dispatch({ type: 'FETCH' });
console.log(machine.getState()); // { status: 'loading' }
```

## With Type Guards

```typescript
function isLoading(state: State): state is { status: 'loading' } {
  return state.status === 'loading';
}

function isSuccess(state: State): state is { status: 'success'; data: string } {
  return state.status === 'success';
}

function isError(state: State): state is { status: 'error'; error: Error } {
  return state.status === 'error';
}

// Usage with type guards
const currentState = machine.getState();

if (isSuccess(currentState)) {
  console.log(currentState.data); // Type-safe access to data
}

if (isError(currentState)) {
  console.log(currentState.error.message); // Type-safe access to error
}
```

## Complex State Machine (Order Processing)

```typescript
type OrderState =
  | { status: 'pending'; orderId: string }
  | { status: 'payment_processing'; orderId: string; paymentId: string }
  | { status: 'confirmed'; orderId: string; paymentId: string; confirmationNumber: string }
  | { status: 'shipping'; orderId: string; trackingNumber: string }
  | { status: 'delivered'; orderId: string; deliveredAt: Date }
  | { status: 'cancelled'; orderId: string; reason: string };

type OrderEvent =
  | { type: 'PROCESS_PAYMENT'; paymentId: string }
  | { type: 'PAYMENT_SUCCESS'; confirmationNumber: string }
  | { type: 'PAYMENT_FAILED' }
  | { type: 'SHIP'; trackingNumber: string }
  | { type: 'DELIVER'; deliveredAt: Date }
  | { type: 'CANCEL'; reason: string };

function orderReducer(state: OrderState, event: OrderEvent): OrderState {
  switch (state.status) {
    case 'pending':
      if (event.type === 'PROCESS_PAYMENT') {
        return {
          status: 'payment_processing',
          orderId: state.orderId,
          paymentId: event.paymentId
        };
      }
      if (event.type === 'CANCEL') {
        return {
          status: 'cancelled',
          orderId: state.orderId,
          reason: event.reason
        };
      }
      return state;
    
    case 'payment_processing':
      if (event.type === 'PAYMENT_SUCCESS') {
        return {
          status: 'confirmed',
          orderId: state.orderId,
          paymentId: state.paymentId,
          confirmationNumber: event.confirmationNumber
        };
      }
      if (event.type === 'PAYMENT_FAILED') {
        return { status: 'pending', orderId: state.orderId };
      }
      return state;
    
    case 'confirmed':
      if (event.type === 'SHIP') {
        return {
          status: 'shipping',
          orderId: state.orderId,
          trackingNumber: event.trackingNumber
        };
      }
      return state;
    
    case 'shipping':
      if (event.type === 'DELIVER') {
        return {
          status: 'delivered',
          orderId: state.orderId,
          deliveredAt: event.deliveredAt
        };
      }
      return state;
    
    case 'delivered':
    case 'cancelled':
      // Terminal states
      return state;
  }
}
```

## With Middleware

```typescript
type Middleware<S, E> = (
  state: S,
  event: E,
  next: (state: S, event: E) => S
) => S;

function createStateMachineWithMiddleware<
  States extends { status: string },
  Events extends { type: string }
>(
  config: StateMachine<States, Events>,
  middleware: Middleware<States, Events>[]
) {
  let state = config.initialState;
  
  const applyMiddleware = (s: States, e: Events): States => {
    let index = 0;
    
    const next = (currentState: States, currentEvent: Events): States => {
      if (index >= middleware.length) {
        return config.reducer(currentState, currentEvent);
      }
      
      const mw = middleware[index++];
      return mw(currentState, currentEvent, next);
    };
    
    return next(s, e);
  };
  
  return {
    getState: () => state,
    dispatch: (event: Events) => {
      state = applyMiddleware(state, event);
      return state;
    },
  };
}

// Usage with logging middleware
const loggingMiddleware: Middleware<State, Event> = (state, event, next) => {
  console.log('Before:', state, event);
  const newState = next(state, event);
  console.log('After:', newState);
  return newState;
};

const machineWithLogging = createStateMachineWithMiddleware<State, Event>(
  { initialState: { status: 'idle' }, reducer },
  [loggingMiddleware]
);
```

## Best Practices

1. **Discriminated Unions**: Use discriminated unions for states
2. **Exhaustive Checking**: Use switch statements with never for exhaustive checking
3. **Type Guards**: Create type guard functions for common state checks
4. **Immutability**: Always return new state objects
5. **Middleware**: Use middleware for cross-cutting concerns like logging

## See Also

- practices-illegal-states.md
- types-discriminated-unions.md
- guards-type-predicates.md
