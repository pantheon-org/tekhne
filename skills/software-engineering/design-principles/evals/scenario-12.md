# Scenario 12: Testable Design Layer Isolation

## User Prompt

"This use case is slow to test because it depends on real infrastructure. Refactor it to enable fast unit tests.

```typescript
class CreateOrderUseCase {
  async execute(input: CreateOrderInput): Promise<CreateOrderOutput> {
    // Validate inventory (calls real inventory API)
    const response = await fetch(`${process.env.INVENTORY_API}/check`, {
      method: 'POST',
      body: JSON.stringify({ items: input.items })
    })
    const { available } = await response.json()

    if (!available) {
      throw new Error('Items not in stock')
    }

    // Save to database (real Postgres connection)
    const order = await db.orders.insert({
      userId: input.userId,
      items: input.items,
      status: 'pending'
    })

    // Send email (real SendGrid API call)
    await sendgrid.send({
      to: input.userId,
      subject: 'Order Confirmation',
      text: `Your order ${order.id} is confirmed`
    })

    return { orderId: order.id, status: order.status }
  }
}
```"

## Expected Behavior

1. Identify direct infrastructure dependencies as testability problem
2. Suggest extracting interfaces (IInventoryService, IOrderRepository, IEmailService)
3. Recommend constructor injection
4. Show how to provide in-memory or stub implementations in tests
5. Demonstrate fast unit test with injected test doubles
6. Explain layer isolation principle

## Success Criteria

- Identifies infrastructure dependencies as slow test cause
- Proposes at least 3 interface abstractions
- Shows constructor injection of dependencies
- Provides example unit test with mocked/stubbed dependencies
- Mentions fast feedback, isolation, or layer separation
- Test runs without network calls or database

## Failure Conditions

- Wraps fetch and db calls in try/catch blocks and presents this as a fix
- Does not introduce interface abstractions; uses jest.mock on module imports instead
- Proposes fewer than 3 interface abstractions for the 3 infrastructure calls
- Unit test still requires a running database or network access
- Does not explain layer isolation or why infrastructure must not leak into use cases
