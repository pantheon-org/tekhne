# CI/CD Integration for BDD Test Suite

## Problem/Feature Description

A software development team has just completed implementing their BDD test suite for a customer management system. The DevOps team needs to integrate these tests into their continuous integration pipeline, but they're encountering challenges with test execution and reporting.

The team needs to set up different test execution strategies: running the full suite for release builds, running only critical scenarios during development, and generating reports for stakeholder review. They also need to be able to quickly identify missing test implementations before spending time on full test runs.

## Output Specification

Create the following deliverables:

1. **test-execution-script.sh** - Script that demonstrates different ways to run the BDD tests
2. **ci-integration.md** - Documentation of the test execution approach
3. **test-report-example.json** - Sample of the generated test report format

Your solution should demonstrate various test execution methods and report generation capabilities.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: inputs/package.json ===============
{
  "name": "customer-management-bdd",
  "version": "1.0.0",
  "scripts": {
    "test": "cucumber-js"
  },
  "devDependencies": {
    "@cucumber/cucumber": "^9.0.0"
  }
}

=============== FILE: inputs/features/customer-management.feature ===============
Feature: Customer Management
  
  @smoke
  Scenario: Create new customer
    Given I am on the customer creation page
    When I enter valid customer details
    Then the customer should be created successfully
  
  @integration @wip
  Scenario: Update customer information
    Given a customer exists in the system
    When I update their contact information
    Then the changes should be saved
  
  @smoke @critical
  Scenario: Search for customers
    Given customers exist in the system
    When I search by customer name
    Then matching customers should be displayed
```