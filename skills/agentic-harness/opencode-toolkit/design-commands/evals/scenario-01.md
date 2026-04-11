# Scenario 01: Create a Parameterized Deploy Command

## User Prompt

Create a `/deploy` slash command that:
1. Accepts an environment name as an argument (e.g. `/deploy staging`)
2. Runs `git status` to check for uncommitted changes first
3. Injects the current git log (last 5 commits) into the prompt
4. Runs the appropriate deploy script for the specified environment

Place the command file in the correct location.

## Expected Behavior

1. Place the command file at `.opencode/command/deploy.md`
2. Use `$ARGUMENTS` or `$1` in the template to insert the environment name provided by the user
3. Use `!` prefix for shell command injection (e.g., `` !`git log --oneline -5` ``) to dynamically include the git log
4. Include a YAML frontmatter block with a `description` field
5. Do not instruct the agent to invoke `/deploy` or other slash commands recursively

## Success Criteria

- **Correct file location**: Command file is placed in `.opencode/command/deploy.md`
- **Uses $ARGUMENTS or $1 for environment**: Template uses `$ARGUMENTS` or `$1` to insert the environment name
- **Uses ! for shell command injection**: Uses `` !`git log --oneline -5` `` or similar for dynamic git context
- **Description frontmatter present**: Has a YAML frontmatter block with a `description` field
- **Agent does NOT call /deploy recursively**: Instructions do not tell the agent to invoke `/deploy` or other slash commands

## Failure Conditions

- Places the command file outside `.opencode/command/`
- Uses a hardcoded environment name instead of `$ARGUMENTS` or `$1`
- Does not inject the git log dynamically (e.g., hardcodes example output or omits it entirely)
- Omits YAML frontmatter or the `description` field
- Instructs the agent to call `/deploy` or another slash command within the template
