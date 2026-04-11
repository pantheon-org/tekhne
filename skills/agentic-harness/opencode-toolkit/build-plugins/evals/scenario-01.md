# Scenario 01: Block Dangerous Bash Commands with a Plugin

## User Prompt

Create an OpenCode plugin that blocks any bash command containing "rm -rf" from executing. The plugin should:
1. Intercept tool execution before it runs
2. Check if the command is a bash/shell tool call containing "rm -rf"
3. Throw an error to block it with message "Blocked: destructive rm -rf detected"
4. Allow all other commands through unchanged

## Expected Behavior

1. Implement the plugin as an async factory function that returns a hooks object
2. Use the `tool.execute.before` hook to intercept tool execution before it runs
3. Check whether the command is a bash/shell call containing "rm -rf"
4. Use `throw new Error(...)` to block the command — never `return false`
5. Import the `Plugin` type from `@opencode-ai/plugin`
6. Declare the `tool.execute.before` handler as `async`

## Success Criteria

- **Uses tool.execute.before hook**: Plugin uses the `tool.execute.before` hook to intercept execution
- **Throws to block (not returns false)**: Uses `throw new Error(...)` to block execution, NOT `return false`
- **Plugin is async factory returning hooks**: Plugin is an async function that returns a hooks object
- **Correct import from @opencode-ai/plugin**: Imports `Plugin` type from `@opencode-ai/plugin`
- **Async hook handler**: The `tool.execute.before` handler is declared as `async`

## Failure Conditions

- Uses `return false` instead of `throw new Error(...)` to block execution
- Implements the plugin as a plain object rather than an async factory function returning hooks
- Imports from a package other than `@opencode-ai/plugin`
- Uses a synchronous hook handler instead of `async`
- Does not intercept tool execution before it runs (e.g., uses `tool.execute.after` instead)
