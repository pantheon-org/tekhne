# Scenario 03: Fix Global vs Project-Level Permission Scope

## User Prompt

A developer added their project's `npm run deploy:prod` command to the global OpenCode allowlist at `~/.config/opencode/opencode.json` because they were tired of seeing permission prompts. Now other projects on their machine are also allowed to run `npm run deploy:prod` without prompting.

Explain why this is a problem and show the correct way to configure this permission.

## Expected Behavior

1. Explain that placing permissions in `~/.config/opencode/opencode.json` applies them to ALL projects on the machine, not just the intended one
2. Recommend moving the permission to the project-root `opencode.json` to scope it to only that project
3. Specify that the deploy command should be removed from the global config file
4. Show the correct JSON structure for a project-scoped bash permission in `opencode.json`

## Success Criteria

- **Identifies global config bleed problem**: Explains that global config applies to ALL projects, not just the intended one
- **Recommends project-level opencode.json**: Suggests moving the permission to the project-root `opencode.json` instead of global config
- **Removes from global config**: Explains to remove the deploy command from `~/.config/opencode/opencode.json`
- **Shows correct project config**: Shows the correct JSON structure for project-scoped bash permission

## Failure Conditions

- Does not explain that global config affects all projects on the machine
- Recommends keeping the permission in the global config file
- Fails to instruct the developer to remove the permission from `~/.config/opencode/opencode.json`
- Does not show the correct project-scoped JSON structure for the permission
