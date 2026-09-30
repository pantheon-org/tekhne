Feature: goal-tracker keeps one active goal per session
  Several conversations can work in one repository at once, each with its own
  goal. With --session, goal.sh answers "what's left" for that session's goal
  only; without it, the old rule of one active goal per repository still holds.

  Scenario: Each session sees only its own goal
    Given a goal "Ship the importer" is active for session "s-one"
    And a goal "Fix the flaky test" is active for session "s-two"
    When I run the goal status for session "s-two"
    Then the exit code should be 0
    And the output should contain "Fix the flaky test"
    And the output should not contain "Ship the importer"

  Scenario: Check validates only the session's goal
    Given a goal "Ship the importer" is active for session "s-one"
    And a goal "Fix the flaky test" is active for session "s-two"
    When I run the goal check for session "s-one"
    Then the exit code should be 0
    And the output should contain "ship-the-importer"

  Scenario: Without a session, two active goals still refuse to answer
    Given a goal "Ship the importer" is active for session "s-one"
    And a goal "Fix the flaky test" is active for session "s-two"
    When I run the goal status without a session
    Then the exit code should be 1
    And the output should contain "More than one active goal"

  Scenario: A new goal records its session
    When I create the goal "Write the docs" for session "s-three"
    Then the exit code should be 0
    And the new goal file should contain "session: s-three"
