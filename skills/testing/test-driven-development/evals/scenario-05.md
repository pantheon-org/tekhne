# Scenario 05: Multi-Step Order Processing System

## User Prompt

RetailPlus has outgrown their simple order system and needs a sophisticated multi-step order processing workflow. Orders now require inventory validation, payment processing, fraud detection, shipping calculation, and warehouse notification. Each step can fail independently and requires different retry and rollback strategies.

The business team has defined a complex workflow: orders start in "pending" status, move to "validated" after inventory and fraud checks, then "paid" after payment processing, then "fulfilled" after shipping arrangements, and finally "completed" when shipped. Failed steps should move orders to appropriate error states with clear failure reasons.

The existing system has grown organically with several issues: tests that combine multiple workflow steps making failures hard to diagnose, shared test data that causes tests to affect each other, and complex test scenarios that are difficult to understand and maintain.

Implement a comprehensive order processing system with:

1. Multi-step workflow management (pending → validated → paid → fulfilled → completed)
2. Individual step validation (inventory, fraud, payment, shipping)
3. Error handling and rollback for each step
4. Order status tracking and history
5. Integration points for external services (payment gateway, shipping API)

System should handle:
- Concurrent order processing
- Partial failures and recovery
- Business rule validation at each step
- Audit trail of all status changes

Required files:
- Order processing workflow implementation
- Individual step processors
- Status management system
- Comprehensive test suite demonstrating each workflow step
- Integration examples showing the complete flow

## Expected Behavior

1. Have each test verify only one workflow step or one specific behavior — not the entire order lifecycle
2. Avoid tests that cover create/validate/pay/fulfill in one test case
3. Define all order data, test scenarios, and expected states within the test files
4. Have tests not rely on shared state or data from other tests — each test creates its own data
5. Group tests by workflow step or component — not mixed together randomly
6. Write separate tests for each workflow transition (pending→validated, validated→paid, etc.)
7. Test failure cases independently for each step — not combined into one failure test
8. Name tests to clearly identify which workflow step and scenario is being tested
9. Create minimal required data per test with no excess or irrelevant test objects

## Success Criteria

- **Single behavior per test**: Each test verifies only one workflow step or one specific behavior, not the entire order lifecycle
- **No behavior combination**: Avoids tests that cover create/validate/pay/fulfill in one test case
- **Visible test data**: All order data, test scenarios, and expected states are defined within test files
- **Independent test cases**: Tests don't rely on shared state or data from other tests — each test creates its own data
- **Logical test organization**: Tests are grouped by workflow step or component, not mixed together randomly
- **Clear workflow step tests**: Separate tests for each workflow transition (pending→validated, validated→paid, etc.)
- **Error scenario isolation**: Failure cases tested independently for each step, not combined into one failure test
- **Descriptive test structure**: Test names clearly identify which workflow step and scenario is being tested
- **Clean test setup**: Each test creates minimal required data, no excess or irrelevant test objects

## Failure Conditions

- A single test covers multiple workflow transitions in sequence (create → validate → pay → fulfill)
- Tests combine multiple workflow step behaviors into one assertion block
- Test data is loaded from shared fixtures or defined outside the test files
- Tests depend on state or data set up by other tests in the suite
- Tests for different workflow steps are not organized or grouped logically
- No individual tests exist for specific workflow transitions
- Failure cases for multiple steps are tested in a single combined failure test
- Test names do not indicate which workflow step or scenario is under test
- Tests set up unnecessary data or objects beyond what the test requires
