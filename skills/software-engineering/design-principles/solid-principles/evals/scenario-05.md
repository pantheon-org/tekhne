# Scenario 05: Apply DIP to Decouple a High-Level Module

## User Prompt

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

## Expected Behavior

1. `INotifier.ts` exists and exports an interface with a `notify` method (or equivalent) returning `Promise<void>`
2. Updated `EmailNotifier.ts` declares `implements INotifier` in its class signature
3. Refactored `OrderProcessor.ts` has a constructor parameter typed to `INotifier`
4. `OrderProcessor.ts` contains no import of `EmailNotifier` anywhere
5. `dip-analysis.md` identifies that `OrderProcessor` (high-level) depended on `EmailNotifier` (low-level concrete), violating DIP
6. `dip-analysis.md` explains that both modules now depend on the `INotifier` abstraction, with `EmailNotifier` implementing it and `OrderProcessor` consuming it

## Success Criteria

- **INotifier interface produced**: `INotifier.ts` exists and exports an interface with a `notify` method (or equivalent) returning `Promise<void>`
- **EmailNotifier implements INotifier**: The updated `EmailNotifier.ts` declares `'implements INotifier'` in its class signature
- **OrderProcessor accepts INotifier via constructor**: The refactored `OrderProcessor.ts` has a constructor parameter typed to `INotifier`
- **OrderProcessor does not import EmailNotifier**: `OrderProcessor.ts` contains no import of `EmailNotifier` anywhere
- **dip-analysis.md explains the violation**: `dip-analysis.md` identifies that `OrderProcessor` (high-level) depended on `EmailNotifier` (low-level concrete), violating DIP
- **dip-analysis.md explains the fix**: `dip-analysis.md` explains that both modules now depend on the `INotifier` abstraction, with `EmailNotifier` implementing it and `OrderProcessor` consuming it

## Failure Conditions

- `INotifier.ts` is missing or has no `notify` method
- Updated `EmailNotifier.ts` does not declare `implements INotifier`
- Refactored `OrderProcessor.ts` still accepts `EmailNotifier` directly instead of `INotifier`
- `OrderProcessor.ts` still imports `EmailNotifier`, maintaining the direct coupling
- `dip-analysis.md` is missing or does not explain the violation and the fix
