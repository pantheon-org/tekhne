Feature: Skill structure validation (skill-validator-rs)
  The Rust skill-validator-rs checks a skill's directory layout, frontmatter,
  tokens, markdown, and links. It is built from source and exits 0 for a
  well-formed skill.

  Scenario: Validator help shows the structure check
    Given I am at the repository root
    When I show the skill validator help
    Then the exit code should be 0
    And the output should contain "structure"

  Scenario: A well-formed skill passes structure validation
    Given I am at the repository root
    When I validate the skill "agentic-harness/professional-honesty"
    Then the exit code should be 0
