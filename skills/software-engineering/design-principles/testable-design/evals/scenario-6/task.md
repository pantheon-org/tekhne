# Scenario 6: Remove Hidden Dependencies From an Order Processor

## User Prompt

The `OrderProcessor` class below calls `InventoryChecker.isAvailable(...)` statically, uses `NotificationService.getInstance()`, builds `new PaymentService()` inline and stamps orders with `new Date()`. Its only test needs a static-mocking library and takes 20 lines of setup. Refactor it so it can be unit tested with simple test doubles, and write one unit test.

```typescript
class OrderProcessor {
  process(order: Order): void {
    const payment = new PaymentService()
    if (InventoryChecker.isAvailable(order.items)) {
      payment.charge(order.total)
      NotificationService.getInstance().sendConfirmation(order)
      order.processedAt = new Date()
    }
  }
}
```

Repo state: TypeScript project, `src/orders/OrderProcessor.ts`, no existing interfaces.
