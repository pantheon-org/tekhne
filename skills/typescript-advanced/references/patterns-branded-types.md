# Branded Types (Nominal Typing)

Create distinct types from primitives to prevent mixing incompatible values.

## Basic Branded Type

```typescript
type Brand<K, T> = K & { __brand: T };

type UserId = Brand<string, 'UserId'>;
type ProductId = Brand<string, 'ProductId'>;

// Factory functions
const createUserId = (id: string): UserId => id as UserId;
const createProductId = (id: string): ProductId => id as ProductId;

const userId = createUserId('user-123');
const productId = createProductId('prod-456');

// Type error: can't mix branded types
// getUser(productId); // Error!
function getUser(id: UserId) {
  return { id, name: 'Alice' };
}
```

## Validated Branded Types

```typescript
type Email = Brand<string, 'Email'>;
type PhoneNumber = Brand<string, 'PhoneNumber'>;

const createEmail = (value: string): Email => {
  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
    throw new Error('Invalid email');
  }
  return value as Email;
};

const createPhoneNumber = (value: string): PhoneNumber => {
  if (!/^\+?[\d\s-()]+$/.test(value)) {
    throw new Error('Invalid phone number');
  }
  return value as PhoneNumber;
};

// Usage
const email = createEmail('user@example.com'); // OK
// const invalid = createEmail('not-an-email'); // Throws
```

## Numeric Branded Types

```typescript
type PositiveInt = Brand<number, 'PositiveInt'>;
type Percentage = Brand<number, 'Percentage'>;

const createPositiveInt = (n: number): PositiveInt => {
  if (!Number.isInteger(n) || n <= 0) {
    throw new Error('Must be a positive integer');
  }
  return n as PositiveInt;
};

const createPercentage = (n: number): Percentage => {
  if (n < 0 || n > 100) {
    throw new Error('Must be between 0 and 100');
  }
  return n as Percentage;
};
```

## Branded Types with Zod

```typescript
import { z } from 'zod';

const UserIdSchema = z.string().uuid().brand('UserId');
type UserId = z.infer<typeof UserIdSchema>;

const EmailSchema = z.string().email().brand('Email');
type Email = z.infer<typeof EmailSchema>;

// Validation
const userId = UserIdSchema.parse('550e8400-e29b-41d4-a716-446655440000');
const email = EmailSchema.parse('user@example.com');
```

## Use Cases

1. **Prevent ID mixing**: UserId vs ProductId
2. **Domain constraints**: PositiveInt, NonEmptyString
3. **Validated formats**: Email, URL, PhoneNumber
4. **Units**: Meters, Seconds, Dollars
5. **Security**: SanitizedHtml, HashedPassword
