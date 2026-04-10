# Scenario 03: Multi-Channel Notification Service

## User Prompt

TechCorp's customer engagement platform needs a notification system that can send alerts via multiple channels: email, SMS, and push notifications. Currently, notifications are hardcoded to only send emails, but marketing campaigns require reaching customers through their preferred communication channels.

The business requirements specify that the system should gracefully handle failures (like invalid email addresses or SMS delivery issues) without breaking the entire notification flow. The service needs to support different notification types like welcome messages, password resets, and promotional offers.

Additionally, the system must be testable in development without actually sending real emails or SMS messages, as this has caused issues with accidental spam to customers during testing.

Build a notification service with the following capabilities:

1. Send notifications through multiple channels (email, SMS, push)
2. Handle different notification types and templates
3. Provide error handling for channel failures
4. Support configuration of which channels to use per notification type

Design the system to be:
- Easily testable without external service calls
- Extensible for new notification channels
- Resilient to individual channel failures

Required files:
- Notification service implementation
- Channel provider implementations (email, SMS, etc.)
- Comprehensive test suite
- Example usage demonstrating the service capabilities

## Expected Behavior

1. Have the service receive notification channels through constructor parameters — not create them internally
2. Replace real email/SMS services with test doubles/mocks in tests
3. Use interfaces or contracts for notification channels to enable easy mocking
4. Define all test data (notification content, recipients, etc.) within the test files
5. Have tests run without requiring real external services or network calls
6. Verify that mocked services are called with correct parameters in tests
7. Test failure cases by configuring mocks to simulate errors
8. Focus tests on service behavior — not testing the mocked dependencies themselves
9. Use no random values or timestamps that could cause flaky tests

## Success Criteria

- **Constructor dependency injection**: Service receives notification channels through constructor/parameters, not creating them internally
- **External dependencies mocked**: Tests replace real email/SMS services with test doubles/mocks
- **Interface-based design**: Uses interfaces/contracts for notification channels to enable easy mocking
- **Test data visibility**: All test data (notification content, recipients, etc.) is defined within the test files
- **Independent test execution**: Tests run without requiring real external services or network calls
- **Mock verification**: Tests verify that mocked services are called with correct parameters
- **Error scenario testing**: Tests cover failure cases by configuring mocks to simulate errors
- **Clear test boundaries**: Tests focus on service behavior, not testing the mocked dependencies themselves
- **Deterministic test data**: No random values or timestamps that could cause flaky tests

## Failure Conditions

- Service instantiates its own email/SMS providers internally rather than accepting them via constructor
- Tests call real external services or require network connectivity
- Notification channels have no interface or contract, making them impossible to mock
- Test data is loaded from external files or shared fixtures rather than defined inline
- Tests depend on network availability or external service state
- Tests do not verify that mocked notification providers were called with correct arguments
- No tests cover channel failure or error scenarios
- Tests assert on mock internal behavior rather than the notification service's behavior
- Tests use `Date.now()`, `Math.random()`, or other non-deterministic values
