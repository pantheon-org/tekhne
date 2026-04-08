# Scenario 11: Testable Design Dependency Injection

## User Prompt

"This service is impossible to unit test because it instantiates concrete dependencies. Refactor it to be testable.

```typescript
class OrderService {
  private repo = new PostgresOrderRepository()
  private emailer = new SendGridEmailService()
  private logger = new WinstonLogger()

  async createOrder(userId: string, items: Item[]): Promise<Order> {
    this.logger.info('Creating order', { userId })

    const order = new Order({ userId, items, status: 'pending' })
    await this.repo.save(order)

    await this.emailer.send({
      to: userId,
      subject: 'Order Confirmation',
      body: `Your order ${order.id} is confirmed`
    })

    this.logger.info('Order created', { orderId: order.id })
    return order
  }
}
```"

## Expected Behavior

1. Identify concrete instantiation as testability blocker
2. Suggest extracting interfaces (IOrderRepository, IEmailService, ILogger)
3. Recommend constructor injection of dependencies
4. Show how to provide mocks/stubs in tests
5. Demonstrate unit test with injected test doubles
6. Explain testability benefits

## Success Criteria

- Identifies dependency injection as solution
- Defines at least 2 interface abstractions
- Shows constructor injection pattern
- Provides example of test with mocked dependencies
- Mentions testability, isolation, or maintainability benefits

## Failure Conditions

- Suggests mocking the module-level imports without changing the class design
- Defines interfaces but continues to instantiate concrete classes inside the constructor
- Does not demonstrate a unit test that passes mock implementations
- Uses only 1 interface abstraction, leaving remaining dependencies concrete
- Does not explain why constructor injection improves testability over field instantiation
