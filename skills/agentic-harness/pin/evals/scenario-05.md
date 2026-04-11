# Scenario 05: Remove a Stale Pin by Number

## User Prompt

"/pin rm 3"

## Expected Behavior

1. Agent reads `pins.json` from the session-scoped path.
2. Agent searches the `items` array for an item with `id === 3`.
3. If found, agent removes that item from the array and writes the updated file.
4. Agent responds with a single line: `🗑️ Pin #3 removed.`
5. `next_id` is NOT reset — it continues from its current value (stable numbering rule).
6. If pin #3 is not found, agent responds: `⚠️ Pin #3 not found.`
7. If no number is provided (e.g., `/pin rm`), agent responds: `⚠️ Usage: /pin rm <number>`

## Expected Behavior

1. Agent parses the command correctly to extract the pin number.
2. Agent finds and removes the matching item by `id` field.
3. Agent writes the updated file with the item removed.
4. Agent emits the one-line removal confirmation.
5. Agent does not renumber remaining items (gaps stay per stable-numbering rule).
6. Agent handles the not-found case with the correct warning format.

## Success Criteria

- Item with `id: 3` is absent from `pins.json` after the command.
- Remaining items retain their original `id` values (no renumbering).
- `next_id` in the file is unchanged.
- Response is exactly `🗑️ Pin #3 removed.` — one line, no extra commentary.
- Not-found case returns `⚠️ Pin #N not found.` rather than an error or silence.
- Missing-number case returns the usage hint `⚠️ Usage: /pin rm <number>`.

## Failure Conditions

- Agent renumbers remaining pins after removal (violates stable-numbering rule).
- Agent resets `next_id` after removal.
- Agent removes the wrong item (matches by position rather than `id` field).
- Agent adds commentary such as "Done! The stale pin has been removed."
- Agent silently does nothing if the pin is not found instead of warning.
- Agent clears all pins instead of removing only pin #3.
