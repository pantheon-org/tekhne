# Scenario 0: Block Dangerous Bash Commands with tool.execute.before

## User Prompt

Create an OpenCode plugin that blocks any bash command containing "rm -rf" from executing. The plugin should:
1. Intercept tool execution before it runs
2. Check if the command is a bash/shell tool call containing "rm -rf"
3. Throw an error to block it with message "Blocked: destructive rm -rf detected"
4. Allow all other commands through unchanged

## Output Specification

Produce the complete plugin source file implementing this behaviour.
