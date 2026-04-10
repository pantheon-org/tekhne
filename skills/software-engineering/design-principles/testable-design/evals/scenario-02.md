# Scenario 02: Refactor Concrete Dependency to Injected Interface

## User Prompt

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

## Expected Behavior

1. Produce `IOrderRepository.ts` with at minimum `findById` and `save` method signatures
2. Define `findById` returning a `Promise` of an `Order` or `null`
3. Define `save` accepting an `Order` and returning `Promise<void>`
4. Refactor `OrderService.ts` with a constructor parameter typed to `IOrderRepository` (not `PostgresOrderRepository`)
5. Ensure `OrderService.ts` does not import `PostgresOrderRepository`
6. Preserve `getOrderTotal` and `markShipped` with the same calculation and status-update logic as the original

## Success Criteria

- **IOrderRepository interface file produced**: A file named `IOrderRepository.ts` exists and exports an interface or type named `IOrderRepository`
- **Interface declares findById**: `IOrderRepository` includes a `findById` method signature returning a `Promise` of an `Order` or `null`
- **Interface declares save**: `IOrderRepository` includes a `save` method signature accepting an `Order` and returning `Promise<void>`
- **OrderService accepts interface via constructor**: The refactored `OrderService.ts` has a constructor parameter typed to `IOrderRepository` (not `PostgresOrderRepository`)
- **No concrete import in refactored service**: `OrderService.ts` does not import `PostgresOrderRepository` anywhere
- **Business logic preserved**: `getOrderTotal` and `markShipped` methods are present and contain the same calculation and status-update logic as the original

## Failure Conditions

- `IOrderRepository.ts` is missing or does not export an interface
- `IOrderRepository` is missing a `findById` signature
- `IOrderRepository` is missing a `save` signature
- `OrderService` constructor still accepts `PostgresOrderRepository` directly
- `OrderService.ts` still imports `PostgresOrderRepository`, maintaining the concrete dependency
- `getOrderTotal` or `markShipped` logic is changed or removed
