# Scenario 02: Pin a User Correction Mid-Session

## User Prompt

"No, I meant the artisan tier, not developers — they're the primary audience for this feature."

## Expected Behavior

1. Agent acknowledges and corrects its understanding of the target audience.
2. Agent immediately invokes `/pin 🔧 target audience = artisan tier, not developers` without being asked.
3. Agent derives `PINS_FILE` path using the git-root slug logic.
4. Agent reads existing `pins.json` and checks for an exact content duplicate.
5. Agent appends the correction item with `type: "correction"` and `emoji: "🔧"`.
6. Agent writes the updated file and responds with a single line: `📌 Pinned #N: 🔧 target audience = artisan tier, not developers`.
7. Agent immediately resumes the corrected line of work.

## Expected Behavior

1. Agent corrects its understanding in the main response.
2. Agent pins the correction as a `🔧` type item without asking for permission.
3. Agent stores the user's words verbatim — no reformulation.
4. Agent checks limits (5 corrections max, 20 total) before appending.
5. Agent resumes the conversation on the correct topic after the one-line pin confirmation.

## Success Criteria

- Item written to `pins.json` has `type: "correction"` and `emoji: "🔧"`.
- Content stored matches the user's phrasing, not a reworded version.
- Single-line pin confirmation is emitted with no additional commentary.
- Agent's subsequent messages reflect the corrected understanding.
- Limits check runs before appending (no silent overflow beyond 5 corrections).

## Failure Conditions

- Agent omits the pin and only corrects verbally.
- Agent reformulates or cleans up the user's correction text.
- Agent uses a different emoji or type (e.g., `approved` or `scope`) for a correction.
- Agent adds a second paragraph explaining why the correction was pinned.
- Agent stops the conversation after pinning instead of resuming immediately.
