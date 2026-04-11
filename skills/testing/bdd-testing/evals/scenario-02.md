# Scenario 02: CI/CD Integration for BDD Test Suite

## User Prompt

A software development team has just completed implementing their BDD test suite for a customer management system. The DevOps team needs to integrate these tests into their continuous integration pipeline, but they're encountering challenges with test execution and reporting.

The team needs to set up different test execution strategies: running the full suite for release builds, running only critical scenarios during development, and generating reports for stakeholder review. They also need to be able to quickly identify missing test implementations before spending time on full test runs.

Create the following deliverables:

1. **test-execution-script.sh** - Script that demonstrates different ways to run the BDD tests
2. **ci-integration.md** - Documentation of the test execution approach
3. **test-report-example.json** - Sample of the generated test report format

Your solution should demonstrate various test execution methods and report generation capabilities.

## Input Files

The following files are provided. Extract them before beginning.

**inputs/package.json:**
```json
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
```

**inputs/features/customer-management.feature:**
```gherkin
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

## Expected Behavior

1. Use `npx cucumber-js features` or similar command to run all feature tests
2. Use the `--dry-run` flag to detect missing step definitions without full execution
3. Use the `--tags` parameter with logical operators (e.g., `@smoke and not @wip`) for tag-based filtering
4. Use `--format json:` to generate machine-readable reports for CI/reporting
5. Demonstrate at least 3 different ways to run Cucumber tests (full, filtered, dry-run)
6. Correctly specify the `features` directory or specific `.feature` files in commands
7. Mention or handle non-zero exit codes on test failures
8. Organize generated reports in an appropriate directory structure

## Success Criteria

- **Basic cucumber execution**: Uses `npx cucumber-js features` or similar command to run all feature tests
- **Dry-run validation**: Uses `--dry-run` flag to detect missing step definitions without full execution
- **Tag-based filtering**: Uses `--tags` parameter with logical operators (e.g., `@smoke and not @wip`)
- **JSON report generation**: Uses `--format json:` to generate machine-readable reports for CI/reporting
- **Multiple execution methods**: Demonstrates at least 3 different ways to run Cucumber tests (full, filtered, dry-run)
- **Proper file paths**: Correctly specifies `features` directory or specific `.feature` files in commands
- **Exit code awareness**: Mentions or handles non-zero exit codes on test failures
- **Report file organization**: Organizes generated reports in an appropriate directory structure

## Failure Conditions

- No basic Cucumber execution command is demonstrated
- `--dry-run` flag is not used for step definition validation
- Tag filtering with `--tags` is not demonstrated
- `--format json:` is not used for report generation
- Fewer than 3 different execution methods are shown
- Feature file paths are not correctly specified in commands
- Non-zero exit codes on test failure are not mentioned or handled
- Generated reports are not organized in a directory structure
