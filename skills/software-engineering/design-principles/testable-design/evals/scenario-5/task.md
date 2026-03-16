# Task: Produce a Boundary Test Coverage Plan

You are architecting the test strategy for a new e-commerce service with the following layers:

- **Entities:** `Order`, `OrderItem` (pure business objects, no dependencies)
- **Use Cases:** `CreateOrderUseCase`, `CancelOrderUseCase` (depend on `IOrderRepository` and `IPaymentGateway` interfaces)
- **Adapters:** `PostgresOrderRepository` (implements `IOrderRepository`), `StripePaymentGateway` (implements `IPaymentGateway`), `HttpOrderController` (calls use cases)
- **Infrastructure:** Express web server, PostgreSQL database, Stripe API

## Output Specification

Produce a file `test-coverage-plan.md` that contains a coverage plan with the following sections:

### 1. Unit Tests
For each entity and use case, specify:
- What is being tested
- Which test doubles are used (if any)
- What the test verifies (behavior, not implementation details)

### 2. Integration Tests
For each adapter, specify:
- What is being tested
- Whether it uses real or fake infrastructure and why
- What the test verifies

### 3. End-to-End Tests
Describe at least one complete workflow test and specify what it covers.

### 4. Boundary Summary Table
A markdown table with columns: Layer | Component | Test Type | Dependencies Used.

The plan must follow the testable design principle that entities are unit tested with no dependencies, use cases are unit tested with mocked ports, adapters are integration tested, and controllers are integration tested.
