# Scenario 05: Update an Existing Context File

## User Prompt

"Save context stream 'feature-x' — updating our checkpoint, added the unit tests."

## Expected Behavior

1. Agent identifies stream name `feature-x`
2. Agent finds existing `.context/session/CONTEXT-feature-x-llm.md`
3. Agent reads the existing file to incorporate prior Next tasks and progression into the new synthesis
4. Agent re-synthesizes the session: merges prior progression with new activity since the last save
5. Agent overwrites `.context/session/CONTEXT-feature-x-llm.md` with the updated content
6. Agent runs `scripts/upsert-index.sh`, which updates (not appends) the existing INDEX.md row for this stream
7. Agent reports: file updated, INDEX row updated (shows "UPDATED:" from upsert script)

## Success Criteria

- **Existing file updated**: `CONTEXT-feature-x-llm.md` is overwritten (not duplicated or renamed)
- **INDEX row updated**: `upsert-index.sh` outputs "UPDATED:" confirming the existing row was replaced
- **Saved timestamp refreshed**: The `saved:` field in the file reflects the current time, not the previous save time
- **Prior context incorporated**: Session section reflects continuity — it references prior decisions, not only the most recent messages
- **Token budget maintained**: Updated file remains within 1200-1500 token limit

## Failure Conditions

- Agent creates a second file (e.g., `CONTEXT-feature-x-2-llm.md`) instead of overwriting
- INDEX.md receives a duplicate row (upsert should replace, not append)
- `saved:` timestamp is unchanged from the previous version
- Agent omits prior progression from the Session section, treating this as a first-time save
- Agent archives the file to `done/` without status being `done` or `parked`
