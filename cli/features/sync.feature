Feature: Sync command
  The sync opencode command cleans broken symlinks, syncs MCP config, and links
  Tessl tiles. The --dry-run flag shows what would change without writing anything.

  Scenario: Help shows the command description
    Given I am at the repository root
    When I run the CLI command "sync opencode --help"
    Then the exit code should be 0
    And the output should contain "Sync OpenCode"

  Scenario: Dry run exits successfully
    Given I am at the repository root
    When I run the CLI command "sync opencode --dry-run"
    Then the exit code should be 0

  Scenario: Dry run output confirms no changes were made
    Given I am at the repository root
    When I run the CLI command "sync opencode --dry-run"
    Then the exit code should be 0
    And the output should contain "Dry run"
