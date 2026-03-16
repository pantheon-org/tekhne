# Task: Apply DIP to Decouple a High-Level Module

The `OrderProcessor` class directly instantiates a low-level `EmailNotifier`. Refactor to satisfy the Dependency Inversion Principle.

## Code to Refactor

```typescript
// src/notifications/EmailNotifier.ts
export class EmailNotifier {
  async notify(recipient: string, subject: string, body: string): Promise<void> {
    // Sends email via SMTP
    console.log(`Email to ${recipient}: [${subject}] ${body}`)
  }
}

// src/OrderProcessor.ts
import { EmailNotifier } from './notifications/EmailNotifier'

export class OrderProcessor {
  private notifier = new EmailNotifier()

  async process(orderId: string, userEmail: string): Promise<void> {
    // ... order processing logic ...
    await this.notifier.notify(
      userEmail,
      'Order Confirmed',
      `Your order ${orderId} has been confirmed.`
    )
  }
}
```

## Output Specification

Produce:

1. `INotifier.ts` — an interface that `EmailNotifier` will implement. The interface must be defined in the application/domain layer, not in the infrastructure layer.
2. `EmailNotifier.ts` — updated to implement `INotifier`.
3. `OrderProcessor.ts` — refactored to depend on `INotifier` via constructor injection. It must not import `EmailNotifier`.
4. `dip-analysis.md` — explain the DIP violation and the direction of the dependency before and after the refactor (one short paragraph each).
