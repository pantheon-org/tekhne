# Scenario 07: Solid Dip Violation

## User Prompt

"This OrderService is tightly coupled to concrete implementations. Refactor it to follow the Dependency Inversion Principle.

```typescript
class OrderService {
  private repo = new PostgresOrderRepository()
  private notifier = new EmailNotificationService()

  async createOrder(userId: string, items: Item[]): Promise<Order> {
    const order = new Order({ userId, items, status: 'pending' })
    await this.repo.save(order)
    await this.notifier.sendOrderConfirmation(order)
    return order
  }
}
```"

## Expected Behavior

1. Identify DIP violation (depending on concrete classes, not abstractions)
2. Suggest extracting IOrderRepository and INotificationService interfaces
3. Recommend constructor injection of dependencies
4. Explain benefits: testability, flexibility, decoupling

## Success Criteria

- Identifies DIP violation or mentions "depend on abstractions, not concretions"
- Proposes at least 2 interface abstractions
- Shows constructor injection pattern
- Mentions testability or ability to swap implementations

## Failure Conditions

- Does not identify that the class depends on concrete implementations
- Introduces interfaces but still instantiates concrete classes inside the service
- Uses property injection or service locator instead of constructor injection
- Does not explain testability or flexibility benefits of the refactoring
- Mentions DIP without providing a concrete code example of the fix
