# Scenario 3: List All Bridges

## User Prompt

"/bridge list"

## Expected Behavior

1. Agent globs for all YAML files in the configured bridges directory.
2. Agent reads and parses each YAML file found.
3. Agent sorts the bridges by date descending, taking the most recent 10.
4. Agent displays the list in the canonical format with the `🔗 Bridges (last 10)` header.
5. Each item shows: number, date, archetype emoji, source → target (or ↔ for bidirectional), description snippet, and strength.
6. If the bridges directory is empty or absent, agent responds: `🔗 No bridges captured yet. Use /bridge <source> → <target>: <description>`.
7. Agent does not modify any files during the list operation.

## Failure Conditions

- Agent modifies or creates YAML files during a list command.
- Agent shows more than 10 bridges without a project filter.
- Agent does not sort by date (wrong order).
- Agent displays raw YAML instead of the formatted list.
- Agent silently returns nothing for an empty directory instead of the empty-state message.
- Agent omits strength from the display line.
