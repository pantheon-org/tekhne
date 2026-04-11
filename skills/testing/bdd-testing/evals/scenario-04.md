# Scenario 04: User Account Management Test Suite

## User Prompt

A fintech startup is developing user account management features and needs to create a robust BDD test suite. They've had issues in the past with flaky tests that fail unpredictably and scenarios that work individually but fail when run together as a suite.

The team needs to ensure their test scenarios can run in any order, in parallel, and independently without affecting each other. They want to prevent the technical debt that comes from interdependent tests and ensure reliable CI/CD pipeline execution.

Create the following deliverables:

1. **account-management.feature** - Feature file with multiple scenarios for user account operations
2. **independence-analysis.md** - Analysis of how each scenario maintains independence
3. **execution-strategy.md** - Documentation of how scenarios can be run independently

Create scenarios that demonstrate proper independence and structure.

## Input Files

The following file is provided. Extract it before beginning.

```
# Account Management Requirements

## Core Features
- User registration with email verification
- Password reset functionality
- Profile information updates
- Account deactivation
- Login attempt tracking

## Test Considerations
- Tests should not depend on each other
- Each test should clean up after itself
- No shared test data between scenarios
- Tests should work in any execution order
```

## Expected Behavior

1. Follow proper Gherkin Given/When/Then structure with clear steps in all scenarios
2. Have each scenario create or mock its own prerequisites rather than depending on other scenarios
3. Ensure scenarios do NOT assume data or state created by other scenarios
4. Design scenarios to produce consistent results regardless of execution order
5. Include Given steps in each scenario for all required context and data setup
6. Focus each scenario on testing one specific behavior or user journey
7. Design scenarios to not leave state that affects other scenarios

## Success Criteria

- **Given/When/Then structure**: All scenarios follow proper Gherkin Given/When/Then structure with clear steps
- **Scenario independence**: Each scenario creates or mocks its own prerequisites rather than depending on other scenarios
- **No cross-scenario coupling**: Scenarios do NOT assume data or state created by other scenarios
- **Deterministic execution**: Scenarios are designed to produce consistent results regardless of execution order
- **Self-contained setup**: Each scenario includes its own Given steps for required context and data
- **Single behavior focus**: Each scenario focuses on testing one specific behavior or user journey
- **Cleanup consideration**: Scenarios are designed to not leave state that affects other scenarios

## Failure Conditions

- Any scenario does not follow Given/When/Then structure
- A scenario relies on data or state established by another scenario
- Scenarios assume shared state or data created elsewhere in the suite
- Scenario outcomes vary depending on which other scenarios ran before it
- A scenario lacks Given steps and assumes pre-existing context
- A single scenario tests multiple unrelated behaviors
- Scenarios leave shared state that causes other scenarios to fail or behave differently
