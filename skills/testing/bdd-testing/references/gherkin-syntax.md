# Gherkin Syntax Fundamentals

## Keywords

| Keyword            | Purpose                 | Description                            |
| ------------------ | ----------------------- | -------------------------------------- |
| `Feature`          | High-level description  | What feature is being tested           |
| `Scenario`         | Test case               | A single example of behavior           |
| `Given`            | Precondition            | Setup state before action              |
| `When`             | Action                  | The behavior being tested              |
| `Then`             | Expected outcome        | What should happen                     |
| `And`              | Additional steps        | Extends Given/When/Then                |
| `But`              | Contrasting step        | Negative assertion (optional)          |
| `Background`       | Setup for all scenarios | Runs before each scenario in a feature |
| `Scenario Outline` | Data-driven tests       | Run same scenario with different data  |
| `Examples`         | Test data               | Data table for Scenario Outline        |

## Feature Structure

```gherkin
Feature: Game Lobby Management
  As a player
  I want to create and join game lobbies
  So that I can play with other players

  Background:
    Given the game server is running
    And the database is seeded with test data

  Scenario: Host creates a new lobby
    Given I am logged in as "Alice"
    When I create a new lobby with mode "race"
    Then the lobby should exist
    And I should be the host
    And the lobby should have 1 player

  Scenario: Player joins an existing lobby
    Given a lobby exists with host "Bob"
    And I am logged in as "Charlie"
    When I join the lobby
    Then the lobby should have 2 players
    And I should not be the host
```

## Given/When/Then Pattern

**Given**: Arrange (setup preconditions)
- Sets up the initial state
- Creates test data
- Navigates to starting point

**When**: Act (perform action)
- The behavior being tested
- Usually ONE action per scenario
- Represents user interaction

**Then**: Assert (verify outcome)
- Expected results
- Validates behavior
- Can have multiple assertions

## Scenario Outline (Data-Driven Tests)

```gherkin
Scenario Outline: Player score calculation
  Given a player finishes in position <position>
  When the game ends
  Then the player should receive <score> points

  Examples:
    | position | score |
    | 1        | 500   |
    | 2        | 300   |
    | 3        | 200   |
    | 4        | 100   |
```

This generates 4 scenarios automatically.

## Tags

Tags organize and filter scenarios:

```gherkin
@mvp @lobby
Feature: Lobby Management

  @smoke
  Scenario: Create lobby
    Given I am logged in
    When I create a lobby
    Then the lobby should exist

  @wip @bug-123
  Scenario: Handle host disconnect
    Given a lobby exists with 3 players
    When the host disconnects
    Then the next player should become host
```

**Run specific tags:**

```bash
# Run only @mvp scenarios
npx cucumber-js --tags "@mvp"

# Run @mvp but skip @wip
npx cucumber-js --tags "@mvp and not @wip"

# Run @smoke OR @critical
npx cucumber-js --tags "@smoke or @critical"
```
