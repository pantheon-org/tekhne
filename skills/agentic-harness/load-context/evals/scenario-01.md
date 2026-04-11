# Scenario 01: Resume Default Stream

## User Prompt

"Load context"

## Expected Behavior

1. Agent runs `rtk ls -t .context/session/CONTEXT-*llm.md .context/session/done/CONTEXT-*llm.md 2>/dev/null || true` to detect available streams
2. Agent reads `.context/session/CONTEXT-llm.md` (the default stream file)
3. Agent parses the key-value header fields (saved, stream, status, focus, goal)
4. Agent formats and outputs a resume report with Stream, Saved, Focus, Goal always shown
5. Agent includes NextTasks (top 3), Session Context, and Hot Files sections
6. Agent ends report with a single-sentence Next Step based on NextTasks[0] and focus field

## Success Criteria

- Agent uses `rtk` prefix for the shell command (not bare `ls`)
- Agent reads `.context/session/CONTEXT-llm.md` (default filename, no stream name segment)
- Resume report title is `# Session Resume: [stream-name]` or equivalent
- Report includes NextTasks, Session Context, and Hot Files blocks
- Next Step is a single sentence derived from NextTasks[0] + focus field
- Agent does NOT ask the user questions before loading (stream is implicit from no argument)

## Failure Conditions

- Agent uses bare `ls` instead of `rtk ls`
- Agent reads the wrong file (e.g., a named-stream file when no argument given)
- Agent asks the user which stream to load when no argument is provided and only one file exists
- Agent skips the NextTasks or Hot Files blocks in the report
- Agent synthesizes or rewrites context content instead of presenting it directly
- Agent makes multiple sequential round-trips when parallel calls were possible
