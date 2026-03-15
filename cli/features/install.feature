Feature: Install command
  The install command places skill symlinks in agent config directories.
  The --dry-run flag shows what would be installed without making changes.

  Scenario: Dry run exits successfully without installing anything
    Given I am at the repository root
    When I run the CLI command "install --dry-run"
    Then the exit code should be 0

  Scenario: Dry run output mentions dry run mode
    Given I am at the repository root
    When I run the CLI command "install --dry-run"
    Then the exit code should be 0
    And the output should contain "dry run"

  Scenario: Help flag shows the command description
    Given I am at the repository root
    When I run the CLI command "install --help"
    Then the exit code should be 0
    And the output should contain "Install skills"

  Scenario: Filtering by domain in dry run shows only matching skills
    Given I am at the repository root
    When I run the CLI command "install --dry-run --skill-domain ci-cd"
    Then the exit code should be 0
