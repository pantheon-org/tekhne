# Scenario 05: Load from done/ Archive

## User Prompt

"Load context old-spike"

## Expected Behavior

1. Agent runs detection command: `rtk ls -t .context/session/CONTEXT-*llm.md .context/session/done/CONTEXT-*llm.md`
2. Agent does not find `CONTEXT-old-spike-llm.md` in `.context/session/`
3. Agent finds `CONTEXT-old-spike-llm.md` in `.context/session/done/`
4. Agent reads `.context/session/done/CONTEXT-old-spike-llm.md` and loads the context normally
5. Agent prefixes the resume report with the archive notice indicating the context is archived
6. Report otherwise follows standard resume format (header fields, NextTasks, Session Context, Hot Files, Next Step)

## Success Criteria

- Agent checks `.context/session/done/` when stream is not found in `.context/session/`
- Agent reads the file from the `done/` subfolder path (not root session folder)
- Resume report includes the archive prefix: "Loaded from `.context/session/done/` — this context is archived ({status})"
- Agent does not error out or give "stream not found" when file exists in `done/`
- Standard resume report blocks (NextTasks, Hot Files, Next Step) are all present
- Agent does not warn the user to run `/save-context` (that message is reserved for when no files exist at all)

## Failure Conditions

- Agent reports "stream not found" without checking the `done/` subfolder
- Agent reads the file from the correct path but omits the archive prefix from the report
- Agent errors out instead of loading the archived context
- Agent uses the "No context files found" error message when the file exists in `done/`
- Agent skips standard report blocks because the context came from an archive folder
