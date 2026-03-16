Feature: Validate frontmatter command
  The validate frontmatter command checks SKILL.md files for required fields
  and valid YAML. It exits 0 when all files pass and non-zero when any fail.

  Scenario: Validating all skills exits successfully
    Given I am at the repository root
    When I run the CLI command "validate frontmatter"
    Then the exit code should be 0

  Scenario: Validating a specific known-good skill file succeeds
    Given I am at the repository root
    When I run the CLI command "validate frontmatter skills/ci-cd/github-actions/generator/SKILL.md"
    Then the exit code should be 0

  Scenario: Help flag shows the command description
    Given I am at the repository root
    When I run the CLI command "validate frontmatter --help"
    Then the exit code should be 0
    And the output should contain "Validate YAML frontmatter"
