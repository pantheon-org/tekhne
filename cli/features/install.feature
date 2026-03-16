Feature: Install command
  The install command places namespaced skill symlinks (domain--subdomain--skill)
  in agent config directories. The --dry-run flag previews all changes without
  writing anything to disk.

  Scenario: Help flag shows the command description
    Given I am at the repository root
    When I run the CLI command "install --help"
    Then the exit code should be 0
    And the output should contain "Install skills"

  Scenario: Dry run exits successfully and confirms no changes were made
    Given I am at the repository root
    When I run the CLI command "install --dry-run"
    Then the exit code should be 0
    And the output should contain "Dry run completed"

  Scenario: Dry run reports how many skills were discovered
    Given I am at the repository root
    When I run the CLI command "install --dry-run"
    Then the exit code should be 0
    And the output should contain "Found"

  Scenario: Dry run always shows an installation summary
    Given I am at the repository root
    When I run the CLI command "install --dry-run"
    Then the exit code should be 0
    And the output should contain "Installation Summary"

  Scenario: Filtering by domain excludes skills from other domains
    Given I am at the repository root
    When I run the CLI command "install --dry-run --skill-domain ci-cd"
    Then the exit code should be 0
    And the output should contain "Excluded"

  Scenario: Filtering by subdomain excludes skills from other subdomains
    Given I am at the repository root
    When I run the CLI command "install --dry-run --skill-subdomain github-actions"
    Then the exit code should be 0
    And the output should contain "Excluded"

  Scenario: Installing for multiple agents in dry run reports each agent
    Given I am at the repository root
    When I run the CLI command "install --dry-run --agent opencode cursor"
    Then the exit code should be 0
    And the output should contain "opencode"
    And the output should contain "cursor"

  Scenario Outline: Install mode is reflected in target directory
    Given I am at the repository root
    When I run the CLI command "install --dry-run <flags>"
    Then the exit code should be 0
    And the output should contain "Mode: <mode>"
    And the output should contain "<path_fragment>"

    Examples:
      | flags    | mode   | path_fragment       |
      |          | local  | .agents/skills      |
      | --global | global | .config/opencode/skills |

  Scenario: Global flag for cursor targets the cursor-specific config directory
    Given I am at the repository root
    When I run the CLI command "install --dry-run --global --agent cursor"
    Then the exit code should be 0
    And the output should contain ".cursor/skills"

  Scenario: An unknown agent name is rejected by the CLI parser
    Given I am at the repository root
    When I run the CLI command "install --agent not-a-real-agent"
    Then the exit code should be 2
    And the output should contain "not-a-real-agent"
