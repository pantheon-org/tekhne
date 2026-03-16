Feature: Audit command
  The audit command evaluates skill quality. The status and summary subcommands
  read existing audit data from .context/audits/ without calling any external AI.

  Scenario: Top-level help shows available subcommands
    Given I am at the repository root
    When I run the CLI command "audit --help"
    Then the exit code should be 0
    And the output should contain "Skill quality audit"

  Scenario: skill subcommand help shows usage
    Given I am at the repository root
    When I run the CLI command "audit skill --help"
    Then the exit code should be 0
    And the output should contain "Audit a single skill"

  Scenario: all subcommand help shows usage
    Given I am at the repository root
    When I run the CLI command "audit all --help"
    Then the exit code should be 0
    And the output should contain "Audit all skills"

  Scenario: status subcommand runs and reports compliance
    Given I am at the repository root
    When I run the CLI command "audit status"
    Then the exit code should be 0
    And the output should contain "Audit Status"

  Scenario: summary subcommand generates a report when audit data exists
    Given I am at the repository root
    When I run the CLI command "audit summary"
    Then the exit code should be 0
    And the output should contain "Audit Summary"
