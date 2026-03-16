# Resolve a Circular Dependency

## Problem Description

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
