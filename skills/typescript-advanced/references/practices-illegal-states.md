# Making Illegal States Unrepresentable

Use the type system to prevent invalid states at compile time. If an invalid combination of values cannot be represented in the type system, it cannot occur at runtime.

## Core Principle

**Bad design allows invalid states:**
```typescript
interface RequestState {
  loading: boolean;
  data?: string;
  error?: Error;
}

// All of these are possible but invalid:
const invalid1: RequestState = { loading: true, data: 'result', error: new Error() };
const invalid2: RequestState = { loading: false }; // No data and no error
```

**Good design makes invalid states unrepresentable:**
```typescript
type RequestState =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; data: string }
  | { status: 'error'; error: Error };

// Compiler prevents invalid combinations
function processRequest(state: RequestState) {
  if (state.status === 'success') {
    console.log(state.data); // ✓ data is always available
  }
  if (state.status === 'error') {
    console.log(state.error); // ✓ error is always available
  }
}
```

## Pattern: Discriminated Unions

Use a discriminant field to distinguish between union variants:

```typescript
type PaymentMethod =
  | { type: 'credit_card'; cardNumber: string; cvv: string }
  | { type: 'paypal'; email: string }
  | { type: 'bank_transfer'; accountNumber: string; routingNumber: string };

function processPayment(method: PaymentMethod) {
  switch (method.type) {
    case 'credit_card':
      // TypeScript knows cardNumber and cvv are available
      return chargeCreditCard(method.cardNumber, method.cvv);
    case 'paypal':
      // TypeScript knows email is available
      return chargePaypal(method.email);
    case 'bank_transfer':
      // TypeScript knows accountNumber and routingNumber are available
      return chargeBankTransfer(method.accountNumber, method.routingNumber);
  }
}
```

## Pattern: Branded Types

Prevent mixing incompatible IDs:

```typescript
type UserId = string & { readonly __brand: 'UserId' };
type OrderId = string & { readonly __brand: 'OrderId' };

function getUser(id: UserId): Promise<User> { /* ... */ }
function getOrder(id: OrderId): Promise<Order> { /* ... */ }

// Constructor functions enforce validation
function createUserId(id: string): UserId {
  if (!id.match(/^user_[0-9a-f]+$/)) {
    throw new Error('Invalid user ID format');
  }
  return id as UserId;
}

function createOrderId(id: string): OrderId {
  if (!id.match(/^order_[0-9a-f]+$/)) {
    throw new Error('Invalid order ID format');
  }
  return id as OrderId;
}

const userId = createUserId('user_123');
const orderId = createOrderId('order_456');

getUser(userId);      // ✓ OK
getUser(orderId);     // ✗ Compile error: can't pass OrderId to UserId parameter
```

## Pattern: Required vs Optional Fields

Be explicit about what fields are required in different contexts:

```typescript
// Creation: email and name required
type CreateUser = {
  email: string;
  name: string;
};

// Update: all fields optional
type UpdateUser = Partial<CreateUser>;

// Database row: all fields present including generated ones
type User = CreateUser & {
  id: string;
  createdAt: Date;
  updatedAt: Date;
};

// Public API response: no sensitive fields
type UserResponse = Omit<User, 'email'>;
```

## Pattern: Const Assertions for Unions

Keep arrays and types in sync automatically:

```typescript
const ROLES = ['admin', 'user', 'guest'] as const;
type Role = typeof ROLES[number]; // 'admin' | 'user' | 'guest'

// Adding a role to the array automatically adds it to the type
const EXTENDED_ROLES = [...ROLES, 'moderator'] as const;
type ExtendedRole = typeof EXTENDED_ROLES[number];

// Type guard based on const array
function isValidRole(role: string): role is Role {
  return ROLES.includes(role as Role);
}
```

## Pattern: Exhaustive Switch Checks

Use `never` to catch unhandled cases at compile time:

```typescript
type Status = 'active' | 'inactive' | 'pending';

function processStatus(status: Status): string {
  switch (status) {
    case 'active':
      return 'processing';
    case 'inactive':
      return 'skipped';
    case 'pending':
      return 'queued';
    default: {
      // If we add a new status and forget to handle it,
      // this will be a compile error
      const _exhaustive: never = status;
      throw new Error(`unhandled status: ${_exhaustive}`);
    }
  }
}
```

## Real-World Example: Form State

**Bad: Allows invalid states**
```typescript
interface FormState {
  pristine: boolean;
  dirty: boolean;
  submitting: boolean;
  submitted: boolean;
  valid: boolean;
  errors?: Record<string, string>;
}

// Invalid: both pristine and dirty can be true
// Invalid: can be submitting and submitted
// Invalid: can be valid but have errors
```

**Good: Only valid states possible**
```typescript
type FormState =
  | { status: 'pristine' }
  | { status: 'editing'; errors?: Record<string, string> }
  | { status: 'submitting'; errors?: Record<string, string> }
  | { status: 'submitted'; result: SubmitResult }
  | { status: 'error'; error: Error };

function handleSubmit(state: FormState) {
  if (state.status === 'submitting' || state.status === 'submitted') {
    // Can't submit while already submitting or after submitted
    return;
  }
  if (state.status === 'editing' && state.errors) {
    // Can't submit with validation errors
    return;
  }
  // Submit logic
}
```

## Benefits

- **Compiler enforces correctness** - Invalid states are compile errors
- **Less defensive code** - No need to check for impossible conditions
- **Better refactoring** - Adding/removing states updates all usage sites
- **Self-documenting** - Type definition shows all possible states

## References

- [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Making Impossible States Impossible](https://www.youtube.com/watch?v=IcgmSRJHu_8)
