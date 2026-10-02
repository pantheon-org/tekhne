# Scenario 5: Produce a Boundary Test Coverage Plan

## User Prompt

You are architecting the test strategy for a new e-commerce service with the following layers:

- **Entities:** `Order`, `OrderItem` (pure business objects, no dependencies)
- **Use Cases:** `CreateOrderUseCase`, `CancelOrderUseCase` (depend on `IOrderRepository` and `IPaymentGateway` interfaces)
- **Adapters:** `PostgresOrderRepository` (implements `IOrderRepository`), `StripePaymentGateway` (implements `IPaymentGateway`), `HttpOrderController` (calls use cases)
- **Infrastructure:** Express web server, PostgreSQL database, Stripe API
