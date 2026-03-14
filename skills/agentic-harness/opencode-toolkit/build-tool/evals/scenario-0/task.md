# evals task.md
Create a custom OpenCode tool that searches for TODOs in the codebase. The tool should:
1. Accept a `directory` argument (string, optional, defaults to ".")
2. Accept a `pattern` argument (string, optional, defaults to "TODO")
3. Use bun shell ($) to run a grep command
4. Return the results as a string

Place the file at the correct location for a project-scoped tool.
