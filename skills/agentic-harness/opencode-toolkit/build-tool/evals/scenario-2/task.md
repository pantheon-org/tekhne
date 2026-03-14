# evals task.md
A developer wants to create a tool that runs a long-running Python analysis script on a file. The tool should:
1. Accept a `file` argument
2. Execute a Python script: `python3 analyze.py <file>`
3. Properly handle cancellation if the user aborts the operation
4. Return the output as a string

Implement the tool with proper abort signal handling.
