# Scenario 04: Regenerate Existing Context with --force

## User Prompt

"Recreate the context. Use --force."

## Expected Behavior

1. Agent detects that `.context/session/ctx/` already exists
2. Because `--force` is present in the arguments, agent does NOT abort
3. Agent removes or overwrites existing `.context/session/ctx/` contents
4. Agent re-runs the full scan, prioritization, manifest generation, and copy/summarize workflow
5. Agent overwrites `.context/session/ctx/manifest.yaml` with a fresh timestamp
6. Agent rewrites `.context/session/CONTEXT-baseline-llm.md`
7. Agent reports that context was regenerated, not created fresh

## Success Criteria

- **Force flag honoured**: Agent proceeds despite `.context/session/ctx/` existing when `--force` is given
- **Guard without force**: When `--force` is absent and `.context/session/ctx/` exists, agent stops with: ".context/session/ctx/ exists. Use --force to recreate."
- **Fresh manifest**: `manifest.yaml` `created` timestamp is updated to the current time
- **Baseline overwritten**: `.context/session/CONTEXT-baseline-llm.md` reflects the regenerated content
- **User informed**: Agent reports that the prior context was overwritten, not silently replaced

## Failure Conditions

- Agent regenerates context without requiring `--force` (overwrites without guard)
- Agent aborts even when `--force` is explicitly provided
- New `manifest.yaml` retains the original `created` timestamp from the first run
- Agent does not report that an overwrite occurred
- Partial state left behind (some old files remain alongside new files)
