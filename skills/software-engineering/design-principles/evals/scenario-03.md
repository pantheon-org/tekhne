# Scenario 03: Clean Arch Use Case Design

## User Prompt

"Design a CreateOrder use case following Clean Architecture principles. The use case should:
- Accept order data (userId, items)
- Validate items are in stock
- Calculate total price
- Save order to database
- Send confirmation email

Show the use case interface, input/output types, and dependencies."

## Expected Behavior

1. Define input DTO (CreateOrderInput with userId, items)
2. Define output DTO (CreateOrderOutput with orderId, status)
3. Define port interfaces (IOrderRepository, IInventoryService, INotificationService)
4. Show use case depending on abstractions, not concrete implementations
5. Demonstrate orchestration of entities and gateways
6. Keep use case free of presentation/infrastructure concerns

## Success Criteria

- Defines explicit input and output types
- Defines at least 2 port interfaces
- Shows dependency injection of ports in constructor
- Use case contains only orchestration logic (no HTTP, DB, or framework code)
- Demonstrates single responsibility (one workflow)

## Failure Conditions

- Omits input or output DTOs and uses raw request/response objects
- Imports Express, database clients, or email SDKs directly in the use case
- Does not define port interfaces; uses concrete classes instead
- Mixes multiple workflows or side concerns into the same use case class
- Instantiates dependencies inside the use case rather than injecting them
