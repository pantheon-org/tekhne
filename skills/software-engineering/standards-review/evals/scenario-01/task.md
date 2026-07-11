# Scenario 01: Full Standards Review Flow

## Setup

The user's project has a file at `/tmp/standards/my-rules.md` with the following content:

```markdown
## Use TypeScript for all new files

All new source files must be written in TypeScript. Existing JavaScript files may remain
but should be migrated during active development.

## Prefer named exports over default exports

Named exports improve discoverability and refactoring tool support. Use `export function`
or `export const` rather than `export default`.
```

## User Prompt

"Can you review these coding standards for me? The file is at /tmp/standards/my-rules.md."

## Expected Behavior

1. Ask the user for the file path (or recognize it was provided and proceed).
2. Read the file and parse it into 2 individual standards.
3. Tell the user 2 standards were found and confirm they want to review both.
4. Present the first standard using a single question tool call with 3-4 options (Accept, Revise, Reject, Other).
5. Wait for the answer. Then present the second standard in a separate question tool call.
6. After both are answered, present a recap table summarizing both standards and their actions.
7. Ask for confirmation before writing the report.
8. Write a report file to `.context/findings/` (or similar location).

## Success Criteria

- The file path was asked for or accepted.
- The agent correctly identified 2 standards in the file.
- Exactly one standard was presented per question tool call.
- Each standard question offered 3-4 concrete options (Accept, Revise, Reject, + Other).
- A recap table was presented covering all standards.
- Explicit confirmation was requested before writing.
- A report file was written matching the confirmed answers.

## Failure Conditions

- The agent reviews both standards in a single question call.
- The agent skips the file path prompt and proceeds without reading a file.
- The agent fails to parse the standards correctly (e.g. treats the whole file as one standard).
- The agent writes the report without a recap/confirmation step.
- The agent debates the user's answers rather than recording them.
