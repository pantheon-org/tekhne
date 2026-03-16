Feature: Readme command
  The readme update command regenerates the skill catalog in docs and README.md.
  The --dry-run flag shows the diff without writing any files.

  Scenario: Help shows the command description
    Given I am at the repository root
    When I run the CLI command "readme update --help"
    Then the exit code should be 0
    And the output should contain "Update skill catalog"

  Scenario: Dry run exits successfully
    Given I am at the repository root
    When I run the CLI command "readme update --dry-run"
    Then the exit code should be 0

  Scenario: Dry run output reports discovered tiles
    Given I am at the repository root
    When I run the CLI command "readme update --dry-run"
    Then the exit code should be 0
    And the output should contain "tiles"
