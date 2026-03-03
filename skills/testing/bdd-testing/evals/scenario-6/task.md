# Legacy BDD Test Suite Refactoring

## Problem/Feature Description

A mature SaaS application has accumulated a large BDD test suite over 2 years of development. The test suite now has 150+ scenarios and is becoming difficult to maintain. The team has noticed significant duplication in steps, scenarios that are very similar but slightly different, and some feature files that are getting unwieldy.

The QA lead wants to refactor the test suite to reduce maintenance overhead while preserving test coverage. They need to identify duplication patterns, consolidate similar scenarios, and reorganize the suite for better maintainability without losing test value.

## Output Specification

Create the following deliverables:

1. **refactoring-plan.md** - Strategy for removing duplication and improving maintainability
2. **consolidation-examples.feature** - Before/after examples of scenario consolidation
3. **step-library-design.md** - Approach for creating reusable step definitions

Your solution should demonstrate systematic approaches to BDD test suite maintenance and duplication removal.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/current-suite-problems.md ===============
# Current Test Suite Issues

## Duplication Patterns Observed
- Similar login steps repeated across 40+ scenarios
- Multiple variations of "create user" with slight differences  
- Cart management scenarios that overlap significantly
- Payment processing steps that differ only in amounts/methods

## Problematic Scenarios
- User registration scenarios: 12 similar scenarios for different validation cases
- Checkout process: 8 scenarios with 80% identical steps
- Search functionality: 15 scenarios with repeated setup and verification
- Profile management: 10 scenarios sharing common user setup

## Step Definition Issues
- 200+ step definitions with significant overlap
- Steps like "Given user John exists" vs "Given a user exists"  
- Similar Then statements: "order should be created" vs "order is successfully created"

=============== FILE: inputs/sample-duplicated-scenarios.feature ===============
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