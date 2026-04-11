# Scenario 02: Save with 'done' Status Triggers Archive

## User Prompt

"Save context stream 'db-migration' — status done, we shipped the migration."

## Expected Behavior

1. Agent identifies stream name `db-migration` and status `done` from arguments
2. Agent synthesizes conversation into required template sections
3. Agent writes `.context/session/CONTEXT-db-migration-llm.md`
4. Agent runs `scripts/upsert-index.sh` with status `done` BEFORE moving the file
5. Agent runs `mkdir -p .context/session/done/`
6. Agent moves `.context/session/CONTEXT-db-migration-llm.md` to `.context/session/done/`
7. Agent reports: "Archived to .context/session/done/ (status: done)"

## Success Criteria

- **Archive triggered**: File is moved to `.context/session/done/` because status is `done`
- **Upsert before move**: `scripts/upsert-index.sh` is called before the `mv` command
- **Archive message**: Agent outputs the exact phrase "Archived to .context/session/done/ (status: done)"
- **Original path cleared**: `.context/session/CONTEXT-db-migration-llm.md` no longer exists at the original location after archival
- **`done/` directory created**: `mkdir -p .context/session/done/` is run before the move

## Failure Conditions

- Agent does not move the file to `done/` when status is `done`
- Agent moves the file before running `upsert-index.sh`
- Agent archives the default stream (`CONTEXT-llm.md`) — default stream must never be moved
- Agent archives the baseline (`CONTEXT-baseline-llm.md`) — baseline must never be moved
- Archive message is missing or uses a different format
- `done/` directory creation step is skipped (relying on the directory already existing)
