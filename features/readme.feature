Feature: Skill catalog generation (scripts/catalog)
  The relocated scripts/catalog generator regenerates the skill catalog in
  README.md and the docs tiles page. The --dry-run flag reports what it would
  write without changing any tracked files.

  Scenario: Dry run exits successfully
    Given I am at the repository root
    When I run the skill catalog update in dry-run mode
    Then the exit code should be 0

  Scenario: Dry run output reports discovered tiles
    Given I am at the repository root
    When I run the skill catalog update in dry-run mode
    Then the exit code should be 0
    And the output should contain "tiles"
