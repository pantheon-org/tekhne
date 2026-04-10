# Scenario 01: Read a Note by Name

## User Prompt

The user wants to retrieve the contents of one of their Obsidian notes.

Read the contents of the note called "Meeting Notes 2024-01-15" from the user's Obsidian vault and display the result.

The note is stored somewhere inside the vault but you do not know its exact folder location.

## Expected Behavior

1. Use the `obsidian read` subcommand to retrieve the note
2. Use the `file=` parameter to target the note by name (wikilink-style resolution) since the exact vault path is unknown
3. Quote the note name because it contains spaces: `file="Meeting Notes 2024-01-15"`
4. Avoid using the `path=` parameter since the exact vault-root path is not known

## Success Criteria

- **Uses obsidian read command**: The agent invokes the obsidian CLI with the `read` subcommand
- **Uses file= parameter**: The agent uses `file=` to target the note by name (wikilink-style), not `path=`
- **Note name is quoted**: The note name value is quoted because it contains spaces: `file="Meeting Notes 2024-01-15"`
- **Does not use path= parameter**: The agent does not use `path=` since the exact vault-root path is not known

## Failure Conditions

- Uses a subcommand other than `read` to retrieve note content
- Uses `path=` parameter instead of `file=` when the exact vault path is unknown
- Leaves the note name unquoted despite it containing spaces
- Speculates an absolute `path=` value when only the note name is known
