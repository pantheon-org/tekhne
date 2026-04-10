# Scenario 03: Implement Abort Signal Handling in a Long-Running Tool

## User Prompt

A developer wants to create a tool that runs a long-running Python analysis script on a file. The tool should:
1. Accept a `file` argument
2. Execute a Python script: `python3 analyze.py <file>`
3. Properly handle cancellation if the user aborts the operation
4. Return the output as a string

Implement the tool with proper abort signal handling.

## Expected Behavior

1. Destructure `abort` from the context parameter in the `execute` function
2. Either check `abort.aborted` early to short-circuit, or pass `{ signal: abort }` to the shell command to propagate cancellation
3. Return a string from the `execute` function (using `.text()` or equivalent), not an object
4. Add a `.describe()` call to the `file` argument schema
5. Import `tool` from `@opencode-ai/plugin` and structure the tool with a `description` and `args`

## Success Criteria

- **Accepts abort signal from context**: Destructures `abort` from the context parameter in execute
- **Checks abort.aborted or passes signal**: Either checks `abort.aborted` early or passes `{ signal: abort }` to the shell command
- **Execute returns string**: Returns a string (`.text()` or equivalent), not an object
- **Schema arg has .describe()**: The `file` argument has a `.describe()` call
- **Correct import and tool structure**: Uses `tool` from `@opencode-ai/plugin` with proper description and args

## Failure Conditions

- Ignores the abort signal entirely and provides no cancellation support
- Returns an object or raw process result instead of a string from `execute`
- Omits `.describe()` from the `file` argument
- Imports from the wrong package (e.g., `@opencode-ai/sdk`)
- Passes the abort signal but in an incorrect way that does not propagate cancellation to the subprocess
