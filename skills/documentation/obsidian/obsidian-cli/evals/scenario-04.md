# Scenario 04: Append a Task to Today's Daily Note in a Specific Vault

## User Prompt

The user has two Obsidian vaults open: "Work" and "Personal". They want to add a task to today's daily note in the Work vault.

Append the following task to today's daily note in the Work vault:

```
- [ ] Follow up with design team on mockups
```

Make sure the command targets the correct vault and uses the appropriate daily note command.

## Expected Behavior

1. Specify `vault="Work"` as the first parameter to explicitly target the Work vault rather than relying on the most recently focused vault
2. Use the `daily:append` subcommand to append content to today's daily note
3. Pass content containing a valid Markdown checkbox task in the format `- [ ] <task text>`
4. Avoid targeting the Personal vault or omitting the vault parameter in a way that could write to the wrong vault

## Success Criteria

- **Specifies vault="Work" parameter**: The command includes `vault="Work"` as the first parameter to explicitly target the Work vault rather than relying on the most recently focused vault
- **Uses daily:append command**: Uses the `daily:append` subcommand to append content to today's daily note
- **Content is a valid Markdown checkbox task**: The content parameter contains a Markdown checkbox task in the format `- [ ] <task text>`
- **Does not target Personal vault**: The command does not use `vault="Personal"` or omit the vault parameter in a way that could target the wrong vault

## Failure Conditions

- Omits `vault="Work"`, risking appending to whichever vault was most recently focused
- Uses `vault="Personal"` or another incorrect vault name
- Uses a subcommand other than `daily:append` to append to today's daily note
- Content parameter does not use `- [ ]` checkbox format
