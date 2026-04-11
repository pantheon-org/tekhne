# Scenario 04: Show the Current Pin Board

## User Prompt

"/pin show"

## Expected Behavior

1. Agent reads `pins.json` from the session-scoped `.session-logs/<slug>/pins.json` path.
2. If the file is missing or items list is empty, agent responds: `📋 Pin board is empty.`
3. If items exist, agent displays a formatted board with the header `📋 Pin Board (N items)`.
4. Each item is numbered, prefixed with its emoji, and shows content plus detail (if present).
5. Agent does not modify the file (read-only operation).
6. Agent does not add commentary about the board contents.

## Expected Behavior

1. Agent derives the correct `PINS_FILE` path from CWD using git slug logic.
2. Agent reads and parses `pins.json`.
3. Agent renders the board using the canonical display format from the skill.
4. Agent stops after the board display — no additional commentary.

## Success Criteria

- Board is displayed with the `📋 Pin Board (N items)` header.
- Each item uses the format `N. <emoji> <content> (<detail>)`.
- Empty-board case returns exactly `📋 Pin board is empty.` with no additional text.
- File is not modified by the show command.
- Agent does not add any explanation, summary, or commentary after the board.

## Failure Conditions

- Agent modifies or re-writes `pins.json` during a show command.
- Agent outputs commentary such as "Here are your current pins:" before the board.
- Board format does not match the canonical layout (missing header, wrong numbering).
- Agent invents pin items not present in the file.
- Agent returns an error instead of "board is empty" when the file does not exist.
