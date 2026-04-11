# Scenario 03: Full Expansion with --full Flag

## User Prompt

"Load context --full"

## Expected Behavior

1. Agent detects `--full` in `$ARGUMENTS` and identifies default stream (no stream name present)
2. Agent reads `.context/session/CONTEXT-llm.md`
3. In Phase 2, agent issues parallel Reads for: `project/proposal/tasks.md`, top 3 hot files listed in the context, and `manifest.yaml`
4. If a `## Thinking Artifacts` section exists in the context file, agent reads those artifact files and includes brief summaries in the report
5. Agent formats full resume report including expanded resource sections
6. Agent does NOT attempt to restore or act on tasks read — informational display only

## Success Criteria

- Agent detects `--full` flag and triggers Phase 2 expanded read
- Phase 2 file reads (project files, hot files, manifest) are issued as parallel tool calls in one message
- Thinking Artifact file contents are read and summarized when `## Thinking Artifacts` section is present
- Agent explicitly notes it is displaying task data as informational, not restoring tasks
- Report is more detailed than a default load (additional resource sections visible)
- Total execution completes in a reasonable number of round-trips (not one file per message)

## Failure Conditions

- Agent ignores `--full` flag and performs default (Phase 1 only) load
- Agent reads Phase 2 files sequentially instead of in parallel
- Agent attempts to restore or execute tasks found in the expanded reads
- Agent reads Thinking Artifacts but does not include summaries in the report
- Agent makes no distinction between default and `--full` report output
