# Scenario 03: Abort When .context/session/in/ Folder is Missing

## User Prompt

"Create context for this project."

## Expected Behavior

1. Agent checks whether `.context/session/in/` exists
2. Agent finds the directory does not exist
3. Agent immediately stops and outputs the canonical error message: "No .context/session/in/ folder found. Create it and add source files first."
4. Agent does NOT create `.context/session/ctx/` or write any files
5. Agent provides brief guidance on what the user should do next (create the folder and add source documents)

## Success Criteria

- **Guard triggers**: Agent stops before any file creation when `.context/session/in/` is absent
- **Canonical error message**: Output matches exactly: "No .context/session/in/ folder found. Create it and add source files first."
- **No partial artifacts**: `.context/session/ctx/` directory is NOT created
- **Actionable guidance**: Agent explains that the user must create `.context/session/in/` and add documents before retrying

## Failure Conditions

- Agent creates `.context/session/ctx/` despite missing input folder
- Agent writes a manifest or baseline with empty or placeholder sources
- Agent silently completes with no files processed
- Error message differs from the canonical form specified in `reference.md`
- Agent asks clarifying questions instead of stopping with the error
