# Scenario 05: Design Patterns Humble Object

## User Prompt

"This controller has business logic mixed with HTTP handling, making it hard to test. Refactor it using the Humble Object pattern.

```typescript
class OrderController {
  async createOrder(req: Request, res: Response): Promise<void> {
    // Validation
    if (!req.body.userId || !req.body.items) {
      res.status(400).json({ error: 'Missing fields' })
      return
    }

    // Business logic
    const items = req.body.items
    const total = items.reduce((sum, item) => sum + item.price * item.qty, 0)

    if (total < 10) {
      res.status(400).json({ error: 'Minimum order is $10' })
      return
    }

    // Save to DB
    const order = await db.orders.insert({
      userId: req.body.userId,
      items,
      total,
      status: 'pending'
    })

    // Response
    res.status(201).json({ orderId: order.id, status: order.status })
  }
}
```"

## Expected Behavior

1. Identify mixing of HTTP, validation, business logic, and persistence
2. Suggest Humble Object pattern
3. Extract testable use case (CreateOrderUseCase) with pure business logic
4. Keep controller thin (humble) with minimal logic
5. Show how to unit test use case without HTTP or DB
6. Show how to integration test controller with real/fake use case

## Success Criteria

- Identifies Humble Object pattern or mentions separating testable logic
- Extracts business logic into separate use case class
- Controller becomes thin adapter (maps request to use case to response)
- Shows how use case is testable without infrastructure
- Mentions dependency injection or interface abstraction

## Failure Conditions

- Keeps business logic in the controller and only reorganizes methods
- Does not name or explain the Humble Object pattern
- Extracted use case still depends on Express Request/Response types
- Does not demonstrate how the refactored use case can be unit tested
- Controller retains total calculation or minimum order validation logic
