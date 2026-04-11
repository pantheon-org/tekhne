# Scenario 04: Save When No Prior Context File Exists

## User Prompt

"Save context — this is the first save for this project."

## Expected Behavior

1. Agent checks `.context/session/` for existing CONTEXT files
2. Agent finds no prior CONTEXT file for this stream
3. Agent proceeds to synthesize the session from the current conversation
4. Agent writes a new `.context/session/CONTEXT-llm.md` (default stream since no name given)
5. Agent runs `scripts/upsert-index.sh`; script outputs "APPENDED:" if INDEX.md exists, or "SKIP:" if it does not
6. Agent reports the new file path and confirms this is a fresh save
7. Agent does NOT error or warn about the absence of a prior context file

## Success Criteria

- **No prior file required**: Agent completes the save without needing an existing CONTEXT file
- **Default stream used**: With no stream argument and no prior `/load-context`, agent writes `CONTEXT-llm.md`
- **INDEX handled gracefully**: `upsert-index.sh` is invoked; if INDEX.md is absent, the SKIP message is accepted and does not block the save
- **Fresh save reported**: Agent confirms a new file was created (not updated)
- **Template complete**: All required template sections present in the new file

## Failure Conditions

- Agent aborts or errors because no prior CONTEXT file exists
- Agent skips the `upsert-index.sh` call on a first-time save
- Agent writes to a named stream file instead of the default `CONTEXT-llm.md`
- Agent asks the user to confirm whether to create a new file (should proceed automatically)
- Any required template section is missing from the freshly written file
