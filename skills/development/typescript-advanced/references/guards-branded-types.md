# Branded Types Type Guards

## Overview
Branded types (nominal types) create distinct types from primitives using unique symbols or phantom types, preventing accidental mixing of semantically different values with the same underlying type.

## Basic Branded Type

```typescript
type Brand<K, T> = K & { __brand: T };

type UserId = Brand<number, "UserId">;
type ProductId = Brand<number, "ProductId">;

function createUserId(id: number): UserId {
  return id as UserId;
}

function createProductId(id: number): ProductId {
  return id as ProductId;
}

function getUser(id: UserId) { /* ... */ }
function getProduct(id: ProductId) { /* ... */ }

const userId = createUserId(42);
const productId = createProductId(42);

getUser(userId); // ✓ OK
getUser(productId); // ✗ Error: ProductId not assignable to UserId
```

## Branded Type with Validation

```typescript
type Email = Brand<string, "Email">;

function isValidEmail(value: string): value is Email {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value);
}

function createEmail(value: string): Email {
  if (!isValidEmail(value)) {
    throw new Error("Invalid email format");
  }
  return value;
}

function sendEmail(to: Email, subject: string, body: string) {
  // Guaranteed to be valid email
  console.log(`Sending to ${to}: ${subject}`);
}

const email = createEmail("alice@example.com");
sendEmail(email, "Hello", "...");

// sendEmail("invalid", "Hello", "..."); // ✗ Error: string not assignable to Email
```

## Multiple Brand Levels

```typescript
type URL = Brand<string, "URL">;
type SecureURL = Brand<URL, "Secure">;

function createURL(value: string): URL {
  try {
    new globalThis.URL(value);
    return value as URL;
  } catch {
    throw new Error("Invalid URL");
  }
}

function createSecureURL(value: URL): SecureURL {
  if (!value.startsWith("https://")) {
    throw new Error("URL must be secure (https)");
  }
  return value as SecureURL;
}

function fetchSecure(url: SecureURL) {
  // Guaranteed to be valid and secure
  return fetch(url);
}

const url = createURL("https://example.com");
const secureUrl = createSecureURL(url);
fetchSecure(secureUrl);
```

## Branded Numeric Types

```typescript
type PositiveNumber = Brand<number, "Positive">;
type Integer = Brand<number, "Integer">;
type PositiveInteger = Brand<PositiveNumber, "Integer">;

function isPositive(n: number): n is PositiveNumber {
  return n > 0;
}

function isInteger(n: number): n is Integer {
  return Number.isInteger(n);
}

function createPositiveInteger(n: number): PositiveInteger {
  if (!isPositive(n) || !isInteger(n)) {
    throw new Error("Must be a positive integer");
  }
  return n as PositiveInteger;
}

function repeat<T>(item: T, times: PositiveInteger): T[] {
  return Array.from({ length: times }, () => item);
}

const count = createPositiveInteger(5);
repeat("hello", count); // ✓ OK
// repeat("hello", 3.5); // ✗ Error: number not assignable to PositiveInteger
```

## Branded Types with Symbol

```typescript
const UserIdSymbol = Symbol("UserId");
const ProductIdSymbol = Symbol("ProductId");

type UserId = number & { [UserIdSymbol]: true };
type ProductId = number & { [ProductIdSymbol]: true };

function createUserId(id: number): UserId {
  return id as UserId;
}

function createProductId(id: number): ProductId {
  return id as ProductId;
}

// Symbols ensure brands are truly unique
const userId = createUserId(42);
const productId = createProductId(42);

function getUserName(id: UserId): string {
  return `User ${id}`;
}

getUserName(userId); // ✓ OK
// getUserName(productId); // ✗ Error: ProductId not assignable to UserId
```

## Branded Type Utilities

```typescript
type Brand<K, T> = K & { __brand: T };

type Unbrand<T> = T extends Brand<infer K, any> ? K : T;

type UserId = Brand<number, "UserId">;

type UnbrandedUserId = Unbrand<UserId>; // number

function unwrap<T>(value: Brand<T, any>): T {
  return value as T;
}

const userId = 42 as UserId;
const rawId: number = unwrap(userId);
```

## Branded Types in Domain Models

```typescript
type CustomerId = Brand<string, "CustomerId">;
type OrderId = Brand<string, "OrderId">;
type Money = Brand<number, "Money">;

interface Order {
  id: OrderId;
  customerId: CustomerId;
  total: Money;
}

function createOrder(
  customerId: CustomerId,
  total: Money
): Order {
  return {
    id: crypto.randomUUID() as OrderId,
    customerId,
    total
  };
}

function chargeCust omer(id: CustomerId, amount: Money) {
  // Type safety ensures correct IDs and amounts
  console.log(`Charging customer ${id}: $${amount}`);
}

const customerId = "cust_123" as CustomerId;
const total = 99.99 as Money;
const order = createOrder(customerId, total);

chargeCustomer(order.customerId, order.total); // ✓ OK
// chargeCustomer(order.id, order.total); // ✗ Error: OrderId not assignable to CustomerId
```

## Branded Types with Type Guards

```typescript
type NonEmptyString = Brand<string, "NonEmpty">;

function isNonEmpty(value: string): value is NonEmptyString {
  return value.trim().length > 0;
}

function assertNonEmpty(value: string): asserts value is NonEmptyString {
  if (!isNonEmpty(value)) {
    throw new Error("String must not be empty");
  }
}

function greet(name: NonEmptyString) {
  console.log(`Hello, ${name}!`);
}

const input = "Alice";
if (isNonEmpty(input)) {
  greet(input); // ✓ OK - narrowed to NonEmptyString
}

const input2 = "Bob";
assertNonEmpty(input2);
greet(input2); // ✓ OK - asserted as NonEmptyString
```

## Best Practices
- Use branded types for domain primitives with semantic meaning
- Combine with validation functions for runtime safety
- Use symbols for truly unique brands
- Document brand semantics in type alias comments
- Create factory functions that validate and brand values
- Use branded types to make illegal states unrepresentable

## Common Pitfalls
- Forgetting to validate when creating branded types
- Over-using brands for every primitive (only for semantically distinct types)
- Not providing unwrap utilities when needed
- Using brands without type guards (runtime validation)
- Mixing branded and unbranded values without explicit casting
