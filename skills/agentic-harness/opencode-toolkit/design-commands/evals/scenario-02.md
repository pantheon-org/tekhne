# Scenario 02: Create a Fire-and-Forget Spellcheck Command

## User Prompt

Create a `/spellcheck` command that:
1. Runs spell checking on all markdown files silently in the background
2. Produces a single report of all spelling errors found
3. Does NOT need back-and-forth interaction with the user

Should use the most appropriate command frontmatter settings.

## Expected Behavior

1. Set `subtask: true` in the frontmatter to enable fire-and-forget behaviour
2. Include a `description` field in the YAML frontmatter
3. Place the command at `.opencode/command/spellcheck.md`
4. Do not use `$ARGUMENTS` placeholder since this command takes no arguments
5. Write template instructions that tell the agent to produce one final consolidated report

## Success Criteria

- **Uses subtask: true**: Command frontmatter includes `subtask: true` for fire-and-forget behavior
- **Has description in frontmatter**: YAML frontmatter contains a `description` field
- **Correct file location**: Command is in `.opencode/command/spellcheck.md`
- **Does not use $ARGUMENTS unnecessarily**: Does not use argument placeholders since this command takes no arguments
- **Clear single-output instruction**: Template instructs the agent to produce one final consolidated report

## Failure Conditions

- Omits `subtask: true` from the frontmatter, defaulting to interactive mode
- Omits the `description` field from YAML frontmatter
- Places the command outside `.opencode/command/`
- Uses `$ARGUMENTS` even though this command accepts no user input
- Instructs the agent to produce multiple incremental outputs instead of a single consolidated report
