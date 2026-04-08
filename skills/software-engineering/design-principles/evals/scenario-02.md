# Scenario 02: Clean Arch Dependency Direction

## User Prompt

"This codebase has dependency direction violations. Identify the problems and suggest fixes.

```typescript
// domain/entities/Order.ts
import { OrderRepository } from '../../infrastructure/database/OrderRepository'

export class Order {
  constructor(
    public id: string,
    public items: Item[],
    private repo: OrderRepository
  ) {}

  async save(): Promise<void> {
    await this.repo.save(this)
  }
}

// application/CreateOrderUseCase.ts
import { Request, Response } from 'express'

export class CreateOrderUseCase {
  async execute(req: Request, res: Response): Promise<void> {
    const order = new Order(req.body.id, req.body.items, new OrderRepository())
    await order.save()
    res.json(order)
  }
}
```"

## Expected Behavior

1. Identify entity importing infrastructure (dep direction violation)
2. Identify use case importing Express types (framework leakage)
3. Suggest entity should be pure (no dependencies)
4. Recommend repository interface in domain, implemented in infrastructure
5. Suggest use case should use input/output DTOs, not Express types

## Success Criteria

- Identifies at least 2 dependency direction violations
- Mentions "dependencies point inward" or Clean Architecture dependency rule
- Proposes interface abstraction for repository
- Suggests removing framework imports from application layer
- Shows entity as pure object without infrastructure dependencies

## Failure Conditions

- Identifies only one violation and misses the other
- Does not mention the Clean Architecture dependency rule
- Suggests fixing violations without introducing interface abstractions
- Proposes moving Express types deeper rather than removing them
- Leaves the entity with an infrastructure dependency
