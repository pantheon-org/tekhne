# Scenario 04: Resolve a Circular Dependency

## User Prompt

A TypeScript monorepo has two modules with a circular import error:

**`src/modules/payments/payment-service.ts`**
```typescript
import { NotificationService } from '../notifications/notification-service';

export class PaymentService {
  private notifier = new NotificationService();

  async processPayment(userId: string, amount: number): Promise<void> {
    // ... process payment
    await this.notifier.sendPaymentConfirmation(userId, amount);
  }
}
```

**`src/modules/notifications/notification-service.ts`**
```typescript
import { PaymentService } from '../payments/payment-service';

export class NotificationService {
  private paymentService = new PaymentService();

  async sendPaymentConfirmation(userId: string, amount: number): Promise<void> {
    const history = await this.paymentService.getHistory(userId);
    // format and send notification with history
  }
}
```

The circular import is: `PaymentService` → `NotificationService` → `PaymentService`.

Produce:
1. A corrected module structure that eliminates the circular dependency (you can create new files as needed)
2. A `REFACTOR.md` explaining the root cause and the approach taken to break the cycle

## Expected Behavior

1. Eliminate the circular import so neither module directly imports the other
2. Introduce an interface (e.g. `IPaymentHistoryProvider`) or event/callback mechanism rather than keeping direct class coupling
3. Explain in REFACTOR.md that the cycle arose because two modules depend on each other's concrete classes
4. Describe the chosen resolution approach in REFACTOR.md (interface extraction, dependency inversion, shared contract module, or event decoupling)
5. Preserve that `PaymentService` can still trigger payment confirmation and `NotificationService` can still access payment history

## Success Criteria

- **Circular import eliminated**: The refactored code has no import cycle: neither module directly imports the other
- **Interface or event-based decoupling used**: The solution introduces an interface (e.g. `IPaymentHistoryProvider`) or event/callback mechanism rather than keeping direct class coupling
- **REFACTOR.md identifies root cause**: REFACTOR.md explains that the cycle arose because two modules depend on each other's concrete classes
- **REFACTOR.md explains resolution approach**: REFACTOR.md describes the chosen approach (interface extraction, dependency inversion, shared contract module, or event decoupling)
- **Both services remain functional**: `PaymentService` can still trigger payment confirmation and `NotificationService` can still access payment history (behavior preserved)

## Failure Conditions

- The circular import is still present in the refactored code
- The solution keeps direct class instantiation between the two modules without introducing an abstraction
- REFACTOR.md does not identify the concrete class coupling as the root cause
- REFACTOR.md does not describe how the chosen approach breaks the cycle
- Either `PaymentService` loses the ability to trigger notifications or `NotificationService` loses access to payment history
