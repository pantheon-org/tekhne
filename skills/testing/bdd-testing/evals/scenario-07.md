# Scenario 07: Legacy BDD Test Suite Refactoring

## User Prompt

A mature SaaS application has accumulated a large BDD test suite over 2 years of development. The test suite now has 150+ scenarios and is becoming difficult to maintain. The team has noticed significant duplication in steps, scenarios that are very similar but slightly different, and some feature files that are getting unwieldy.

The QA lead wants to refactor the test suite to reduce maintenance overhead while preserving test coverage. They need to identify duplication patterns, consolidate similar scenarios, and reorganize the suite for better maintainability without losing test value.

Create the following deliverables:

1. **refactoring-plan.md** - Strategy for removing duplication and improving maintainability
2. **consolidation-examples.feature** - Before/after examples of scenario consolidation
3. **step-library-design.md** - Approach for creating reusable step definitions

Your solution should demonstrate systematic approaches to BDD test suite maintenance and duplication removal.

## Input Files

The following files are provided. Extract them before beginning.

**inputs/current-suite-problems.md:**
```
# Current Test Suite Issues

## Duplication Patterns Observed
- Similar login steps repeated across 40+ scenarios
- Multiple variations of "create user" with slight differences
- Cart management scenarios that overlap significantly
- Payment processing steps that differ only in amounts/methods

## Step Definition Issues
- 200+ step definitions with significant overlap
- Steps like "Given user John exists" vs "Given a user exists"
- Similar Then statements: "order should be created" vs "order is successfully created"
```

**inputs/sample-duplicated-scenarios.feature:**
```gherkin
Feature: User Registration

  Scenario: Register with valid email
    Given I am on the registration page
    When I enter email "user@example.com"
    And I enter password "validpassword123"
    And I click the register button
    Then I should see "Registration successful"
    And I should be logged in
    And I should see my profile page

  Scenario: Register with valid email and strong password
    Given I am on the registration page
    When I enter email "newuser@example.com"
    And I enter password "StrongPassword456!"
    And I click the register button
    Then I should see "Registration successful"
    And I should be logged in
    And I should see my profile page

  Scenario: Register with corporate email
    Given I am on the registration page
    When I enter email "employee@company.com"
    And I enter password "CorporatePass789"
    And I click the register button
    Then I should see "Registration successful"
    And I should be logged in
    And I should see my profile page
```

## Expected Behavior

1. Identify specific patterns of duplication in scenarios and step definitions
2. Demonstrate how to consolidate similar scenarios while preserving test coverage and intent
3. Show how to refactor similar steps into reusable, parameterized definitions
4. Ensure refactored scenarios remain readable and understandable to non-technical stakeholders
5. Provide a methodical process for identifying and eliminating duplication across the test suite
6. Maintain the original business intent and coverage while reducing duplication
7. Create patterns for building reusable, maintainable step definition libraries

## Success Criteria

- **Duplication identification**: Identifies specific patterns of duplication in scenarios and step definitions
- **Scenario consolidation strategy**: Demonstrates how to consolidate similar scenarios while preserving test coverage and intent
- **Step definition refactoring**: Shows how to refactor similar steps into reusable, parameterized definitions
- **Maintains scenario readability**: Ensures refactored scenarios remain readable and understandable to non-technical stakeholders
- **Systematic refactoring approach**: Provides a methodical process for identifying and eliminating duplication across the test suite
- **Preserves test intent**: Maintains the original business intent and coverage while reducing duplication
- **Reusable step library design**: Creates patterns for building reusable, maintainable step definition libraries

## Failure Conditions

- No specific duplication patterns are identified in the analysis
- Similar scenarios are removed without demonstrating how coverage is preserved
- Step definitions are not refactored into parameterized or reusable forms
- Refactored scenarios become harder to read for non-technical stakeholders
- No systematic process is provided for identifying and eliminating duplication
- Business intent or test coverage is lost during the consolidation process
- No patterns or design approach are provided for building a reusable step library
