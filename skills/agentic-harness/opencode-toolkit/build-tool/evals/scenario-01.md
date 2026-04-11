# Scenario 01: Create a Custom Tool to Search for TODOs

## User Prompt

Create a custom OpenCode tool that searches for TODOs in the codebase. The tool should:
1. Accept a `directory` argument (string, optional, defaults to ".")
2. Accept a `pattern` argument (string, optional, defaults to "TODO")
3. Use bun shell ($) to run a grep command
4. Return the results as a string

Place the file at the correct location for a project-scoped tool.

## Expected Behavior

1. Import `tool` from `@opencode-ai/plugin` (not from `@opencode-ai/sdk`)
2. Place the file in the `.opencode/tool/` directory
3. Add `.describe()` to both the `directory` and `pattern` arguments
4. Implement the `execute` handler to return a string (not an object or `Promise<object>`)
5. Mark both arguments as optional with defaults using `.optional()` or `.default(...)`

## Success Criteria

- **Correct import from @opencode-ai/plugin**: Imports `tool` from `@opencode-ai/plugin`, NOT from `@opencode-ai/sdk`
- **Correct file location**: File placed in `.opencode/tool/` directory
- **All schema args have .describe()**: Both `directory` and `pattern` arguments include `.describe()` calls
- **Execute returns a string**: The execute handler returns a string (not an object or Promise<object>)
- **Optional args with defaults**: Uses `.optional()` or `.default(...)` for the optional arguments

## Failure Conditions

- Imports `tool` from `@opencode-ai/sdk` instead of `@opencode-ai/plugin`
- Places the file outside the `.opencode/tool/` directory
- Omits `.describe()` on one or both schema arguments
- The `execute` handler returns an object or non-string value
- Marks both arguments as required with no defaults
