# Scenario 02: Create a Note Without Opening It

## User Prompt

The user wants to create a new note in Obsidian with specific content. They do not want the note to open in the editor when it is created.

Create a new note called "Project Alpha" with the following content:

```
# Project Alpha

Initial notes.
```

Make sure the note is not opened in Obsidian after creation.

## Expected Behavior

1. Use the `obsidian create` subcommand to create the note
2. Include the `silent` flag so Obsidian does not open the note in the editor after creation
3. Quote the name parameter because it contains a space: `name="Project Alpha"`
4. Encode newlines in the content parameter using `\n` since the content spans multiple lines within a single-line CLI parameter value

## Success Criteria

- **Uses obsidian create command**: The agent invokes the obsidian CLI with the `create` subcommand
- **Includes silent flag**: The command includes the silent flag so Obsidian does not open the note in the editor
- **Note name is quoted**: The `name=` parameter value is quoted because it contains a space: `name="Project Alpha"`
- **Content encodes newlines**: The content parameter uses `\n` to represent newlines within the single-line CLI parameter value

## Failure Conditions

- Uses a subcommand other than `create` to create the note
- Omits the silent flag, causing Obsidian to open the note after creation
- Leaves the note name unquoted despite it containing a space
- Uses literal newlines in the content parameter instead of `\n` escape sequences
