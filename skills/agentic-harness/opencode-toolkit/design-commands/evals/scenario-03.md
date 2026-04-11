# Scenario 03: Fix Anti-Patterns in a Broken Command Template

## User Prompt

A user has a `/review` command that currently looks like this:

```markdown
---
description: Review code
agent: plan
---

Review $ARGUMENTS for bugs. After fixing $ARGUMENTS, run /test to verify.
```

Identify the problems with this command and produce a corrected version.

## Expected Behavior

1. Identify that `agent: plan` is a read-only mode and cannot fix bugs or modify files
2. Remove the `run /test` instruction — agents cannot invoke slash commands from within a command template; replace with `npm test` or a direct shell equivalent
3. Avoid using `$ARGUMENTS` twice in the same template; restructure to reference the target once without duplication
4. Either remove the `agent:` key entirely (defaulting to the build agent) or change it to `agent: build` for file-modifying work

## Success Criteria

- **Identifies agent: plan issue**: Explains that `agent: plan` is read-only and cannot fix bugs/modify files
- **Removes or fixes slash command reference**: Removes `run /test` (agents can't invoke slash commands) and replaces with `npm test` or similar
- **Fixes double $ARGUMENTS usage**: Does not use `$ARGUMENTS` twice in the same template; restructures to avoid duplication
- **Fixes agent to build or removes it**: Either removes `agent:` (uses default) or changes to `agent: build` for file-modifying work

## Failure Conditions

- Leaves `agent: plan` in the corrected version without explaining or fixing the read-only limitation
- Keeps the `run /test` instruction or replaces it with another slash command invocation
- Still uses `$ARGUMENTS` more than once in the corrected template
- Does not address the `agent: plan` issue at all
