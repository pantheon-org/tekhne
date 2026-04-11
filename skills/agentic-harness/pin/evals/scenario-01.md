# Scenario 01: Auto-Pin an Approved Recommendation

## User Prompt

"Yes, go with Bun as the package manager — it's faster and we already use it elsewhere."

## Expected Behavior

1. Agent responds to the user's approval normally.
2. Agent immediately invokes `/pin ✅ use Bun as package manager` without being asked.
3. Agent derives the `PINS_FILE` path from CWD using the `git rev-parse` slug logic.
4. Agent creates the `.session-logs/<slug>/` directory if absent.
5. Agent reads existing `pins.json` (or initialises `{"items":[],"next_id":1}`).
6. Agent checks for an exact content duplicate and finds none.
7. Agent appends the new approved item, increments `next_id`, and writes the file.
8. Agent responds with exactly one line: `📌 Pinned #1: ✅ use Bun as package manager`.
9. Agent resumes the prior conversation thread immediately.

## Success Criteria

- Pin is written to `pins.json` with `type: "approved"` and `emoji: "✅"`.
- Response is a single line matching `📌 Pinned #N: ✅ <content>` with no extra commentary.
- Agent does not ask permission before pinning.
- Agent resumes the conversation after the one-line acknowledgement.
- `next_id` is incremented in the written file.

## Failure Conditions

- Agent asks "Should I pin this?" before pinning.
- Agent adds commentary or explanation about the pin content.
- Agent writes the pin but does not respond with the expected one-line format.
- Agent pins with the wrong type (e.g., `scope` instead of `approved`).
- Agent fails to create the directory before writing the file.
- Agent stops the conversation after pinning instead of resuming.
