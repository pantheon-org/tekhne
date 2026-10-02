# Scenario 7: Review a Brittle Test Suite

## User Prompt

Review the test file below and tell the team what is wrong with it and how to fix the design behind it. Do not just rewrite the assertions.

```typescript
describe('OrderService', () => {
  it('creates an order', async () => {
    const db = await connectToRealPostgres()
    const service = new OrderService(db)
    const spy = jest.spyOn(service as any, 'validateItems')
    await service.createOrder(input)
    expect(spy).toHaveBeenCalledTimes(1)
  })
  it.skip('sends the confirmation email', () => {
    // too hard to test, SMTP is hard-coded inside createOrder
  })
})
```

Repo state: `OrderService` constructs its SMTP client inside `createOrder`.
