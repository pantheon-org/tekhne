# Scenario 03: List All Bridges

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

## Expected Behavior

1. Agent uses `Glob` to enumerate YAML files in the bridges directory.
2. Agent parses YAML content to extract display fields.
3. Agent sorts by `date` field descending.
4. Agent caps output at 10 bridges regardless of total count.
5. Agent uses bidirectional notation (`↔`) where `direction: bidirectional`.

## Success Criteria

- Output header is `🔗 Bridges (last 10)` (or fewer if less than 10 exist).
- Each line shows date, emoji, source, arrow notation, target, description, and strength.
- List is sorted by date descending (most recent first).
- Bidirectional bridges use `↔` arrow, one-way bridges use `→`.
- Empty-directory case returns the correct empty-state message with usage hint.
- No YAML files are created or modified during listing.

## Failure Conditions

- Agent modifies or creates YAML files during a list command.
- Agent shows more than 10 bridges without a project filter.
- Agent does not sort by date (wrong order).
- Agent displays raw YAML instead of the formatted list.
- Agent silently returns nothing for an empty directory instead of the empty-state message.
- Agent omits strength from the display line.
