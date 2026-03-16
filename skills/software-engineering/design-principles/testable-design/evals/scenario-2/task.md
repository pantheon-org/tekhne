# Task: Refactor Concrete Dependency to Injected Interface

The following `OrderService` class instantiates a concrete repository directly. Refactor it to use dependency injection so it can be unit tested without a real database.

## Code to Refactor

```typescript
// src/services/OrderService.ts
import { PostgresOrderRepository } from '../repositories/PostgresOrderRepository'

export class OrderService {
  private repo = new PostgresOrderRepository()

  async getOrderTotal(orderId: string): Promise<number> {
    const order = await this.repo.findById(orderId)
    if (!order) throw new Error(`Order ${orderId} not found`)
    return order.items.reduce((sum, item) => sum + item.price * item.quantity, 0)
  }

  async markShipped(orderId: string): Promise<void> {
    const order = await this.repo.findById(orderId)
    if (!order) throw new Error(`Order ${orderId} not found`)
    order.status = 'shipped'
    await this.repo.save(order)
  }
}
```

## Output Specification

Produce two files:

1. `IOrderRepository.ts` — the interface extracted from the repository, with at minimum `findById` and `save` method signatures.
2. `OrderService.ts` — the refactored service that accepts `IOrderRepository` as a constructor parameter. The refactored class must not import or reference `PostgresOrderRepository`.

Both files must be valid TypeScript.
