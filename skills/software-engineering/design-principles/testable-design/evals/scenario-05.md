# Scenario 05: Produce a Boundary Test Coverage Plan

## User Prompt

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

## Expected Behavior

1. Assign unit tests with no mocks or stubs to `Order` and `OrderItem` — pure input/output testing
2. Assign unit tests with mocked/stubbed `IOrderRepository` and `IPaymentGateway` to `CreateOrderUseCase` and `CancelOrderUseCase`
3. Assign integration tests to `PostgresOrderRepository` and `StripePaymentGateway`, with explicit mention of real or fake infrastructure
4. Assign an integration test (not a unit test) to `HttpOrderController`, with the use case either real or faked
5. Include at least one end-to-end scenario covering a complete workflow from HTTP request to database
6. Include a boundary summary table with columns `Layer`, `Component`, `Test Type`, and `Dependencies Used` covering all listed components

## Success Criteria

- **Entities assigned unit tests with no dependencies**: The plan specifies unit tests for `Order` and `OrderItem` with no mocks, stubs, or fakes — pure function input/output testing
- **Use cases assigned unit tests with mocked ports**: `CreateOrderUseCase` and `CancelOrderUseCase` are assigned unit tests that use mocks or stubs for `IOrderRepository` and `IPaymentGateway`
- **Adapters assigned integration tests**: `PostgresOrderRepository` and `StripePaymentGateway` are assigned integration tests (not unit tests), with explicit mention of real or fake infrastructure
- **Controller assigned integration test**: `HttpOrderController` is assigned an integration test (not a unit test), with the use case either real or faked
- **At least one end-to-end test described**: The plan includes at least one end-to-end scenario covering a complete workflow from HTTP request to database
- **Boundary summary table present**: A markdown table with columns `Layer`, `Component`, `Test Type`, and `Dependencies Used` is present and covers all listed components

## Failure Conditions

- `Order` or `OrderItem` are assigned integration tests or use test doubles, violating the "pure unit test" principle for entities
- `CreateOrderUseCase` or `CancelOrderUseCase` are assigned integration tests or are tested without mocking their port interfaces
- `PostgresOrderRepository` or `StripePaymentGateway` are assigned unit tests instead of integration tests
- `HttpOrderController` is assigned a unit test instead of an integration test
- The plan contains no end-to-end test description
- The boundary summary table is missing, incomplete, or does not include all four required columns
