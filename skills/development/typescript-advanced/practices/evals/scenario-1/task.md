# Scenario: Redesign an Interface to Make an Illegal State Unrepresentable

A payment interface currently has several independent optional fields, and the fields' combinations don't all make sense — a payment can't be both `"refunded"` and carry a `cardLast4` for a bank-transfer method, and a `failureReason` should only exist alongside a failed payment.

```typescript
// payment.ts

interface Payment {
  status: "pending" | "completed" | "failed" | "refunded";
  method: "card" | "bank_transfer";
  cardLast4?: string;      // only meaningful when method is "card"
  bankReference?: string;  // only meaningful when method is "bank_transfer"
  failureReason?: string;  // only meaningful when status is "failed"
  refundedAt?: Date;       // only meaningful when status is "refunded"
}

// Currently compiles, but is nonsensical:
const bad: Payment = {
  status: "completed",
  method: "bank_transfer",
  cardLast4: "4242",       // method is bank_transfer — cardLast4 shouldn't exist
  failureReason: "oops",   // status is completed — failureReason shouldn't exist
};
```

Redesign `payment.ts` so that:

1. `method` and its associated field (`cardLast4` for `"card"`, `bankReference` for `"bank_transfer"`) are unified into a single discriminated union, so a `"card"` payment cannot have a `bankReference` and vice versa.
2. `status` and its associated field (`failureReason` for `"failed"`, `refundedAt` for `"refunded"`) are unified into a second discriminated union, so a `"completed"` payment cannot carry a `failureReason`.
3. Combine both unions (e.g. via an intersection of the two, or a single combined union) so `Payment` as a whole rejects the nonsensical combination shown above at compile time.
4. Include a comment demonstrating that the `bad` example above no longer compiles.

## Output Specification

Produce a single file `payment.ts` containing the redesigned type(s) and a comment showing why the original `bad` example now fails to compile.
