# Scenario 10: Testable Design Boundary Testing

## User Prompt

"Design a test strategy for this Clean Architecture application:

- Entities: Order, OrderItem, User
- Use Cases: CreateOrder, CancelOrder, GetOrderHistory
- Adapters: OrderController, OrderRepository (Postgres), EmailGateway (SendGrid)
- Infrastructure: Express server, TypeORM, SendGrid client

What types of tests should we write, and what should each test cover?"

## Expected Behavior

1. Recommend unit tests for entities (pure business logic)
2. Recommend unit tests for use cases (with mocked ports)
3. Recommend integration tests for adapters (with real or in-memory infrastructure)
4. Recommend end-to-end tests for complete workflows
5. Explain boundary testing principle
6. Show example test for each layer

## Success Criteria

- Defines at least 3 test types (unit, integration, e2e)
- Assigns appropriate test type to each layer
- Mentions mocking or test doubles for use case ports
- Recommends integration tests for adapters
- Explains testing at boundaries, not implementation details
- Provides at least 1 concrete test example

## Failure Conditions

- Recommends only end-to-end tests for all layers
- Does not differentiate test strategies between entities, use cases, and adapters
- Omits mocking or test doubles when describing use case tests
- Does not mention boundary testing or testing at architectural seams
- Provides no concrete test example for any layer
