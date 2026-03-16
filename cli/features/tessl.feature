Feature: Tessl command
  The tessl command manages the Tessl registry lifecycle. The manage subcommand
  handles import/lint/review/publish. The publish-check subcommand validates
  tiles before publishing by running tessl skill lint locally.

  Scenario: manage subcommand help shows usage
    Given I am at the repository root
    When I run the CLI command "tessl manage --help"
    Then the exit code should be 0
    And the output should contain "Manage skill lifecycle"

  Scenario: publish-check subcommand help shows usage
    Given I am at the repository root
    When I run the CLI command "tessl publish-check --help"
    Then the exit code should be 0
    And the output should contain "Validate tiles"

  Scenario: publish-check validates a well-formed tile
    Given I am at the repository root
    When I run the CLI command "tessl publish-check skills/ci-cd/github-actions"
    Then the exit code should be 0
    And the output should contain "All checks passed"
