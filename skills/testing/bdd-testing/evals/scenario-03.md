# Scenario 03: Step Definition Implementation for Order Processing

## User Prompt

A development team has written comprehensive Gherkin scenarios for their order processing system, but they need to implement the corresponding step definitions. The scenarios were created collaboratively with business stakeholders and use natural business language that needs to be translated into executable code.

The challenge is to create step definitions that maintain the business meaning while implementing the actual test logic. The team wants to ensure that future changes to the underlying system implementation won't break the scenarios, and that the step definitions can be reused across similar scenarios.

Create the following deliverables:

1. **step-definitions.js** - Implementation of step definitions for the provided scenarios
2. **implementation-notes.md** - Documentation of your approach and design decisions
3. **reusability-analysis.md** - Analysis of how steps can be reused across scenarios

## Input Files

The following files are provided. Extract them before beginning.

**inputs/features/order-processing.feature:**
```gherkin
Feature: Order Processing

  Scenario: Process standard order
    Given a customer has items in their cart
    When they complete the checkout process
    Then the order should be confirmed
    And payment should be processed
    And inventory should be updated

  Scenario: Handle insufficient inventory
    Given a customer selects an out-of-stock item
    When they attempt to complete checkout
    Then they should see an inventory warning
    And the order should not be processed

  Scenario: Process expedited shipping
    Given a customer selects expedited shipping
    When they complete checkout with valid payment
    Then the order should be marked for priority processing
    And shipping should be scheduled within 24 hours
```

**inputs/existing-steps.js (bad example — do not replicate):**
```javascript
const { Given, When, Then } = require('@cucumber/cucumber');

// This is BAD - too implementation specific
Given('I click the submit button and validate the form', function() {
  // ...
});
```

## Expected Behavior

1. Preserve and map step definitions directly to the business terminology used in the scenarios
2. Do not expose UI elements, API calls, database queries, or internal method names in step implementations
3. Implement the business intent rather than technical implementation in step definitions
4. Use Given/When/Then imports and correct step definition syntax
5. Create steps that can be reused across different scenarios with similar meaning
6. Have step definitions act as a translation layer between business language and implementation
7. Avoid coupling steps to specific UI elements, selectors, or implementation details that could change

## Success Criteria

- **Business language mapping**: Step definitions preserve and map directly to the business terminology used in the scenarios
- **Avoids implementation details**: Step implementations do NOT expose UI elements, API calls, database queries, or internal method names
- **Maintains scenario meaning**: Step definitions implement the business intent rather than technical implementation
- **Proper Cucumber syntax**: Uses Given/When/Then imports and correct step definition syntax
- **Step reusability**: Creates steps that can be reused across different scenarios with similar meaning
- **Abstraction layer**: Step definitions act as a translation layer between business language and implementation
- **No brittle dependencies**: Steps don't couple to specific UI elements, selectors, or implementation details that could change

## Failure Conditions

- Step definitions do not use the same business terminology as the Gherkin scenarios
- Step implementations expose UI selectors, API endpoint URLs, database queries, or internal method names
- Step definitions implement technical details rather than the business behavior they represent
- `Given`, `When`, `Then` are not imported and used correctly from `@cucumber/cucumber`
- Step definitions are written so narrowly that they cannot be shared across similar scenarios
- Steps directly encode implementation mechanics that would break if the system is refactored
- Steps are tightly coupled to specific UI element names or selectors
