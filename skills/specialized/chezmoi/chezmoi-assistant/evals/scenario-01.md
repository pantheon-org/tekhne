# Scenario 01: Track a config file and make it machine-specific

## User Prompt

I want to track my `~/.gitconfig` with chezmoi. My work laptop uses `alice@corp.com` as the git email but my personal machine uses `alice@personal.com`. Make it work on both.

## Expected Behavior

1. Run `chezmoi add ~/.gitconfig` to start tracking the file
2. Convert to a template with `chezmoi chattr +template ~/.gitconfig` (or `chezmoi add --template ~/.gitconfig`)
3. Edit the source file (`chezmoi edit ~/.gitconfig`) and replace the hardcoded email with `{{ .chezmoi.hostname }}` or a custom data variable
4. Show the user how to add machine-specific data in `chezmoi.toml` under `[data]`
5. Verify with `chezmoi cat ~/.gitconfig` before applying
6. Apply with `chezmoi apply`

## Success Criteria

- Tracks file with `chezmoi add`
- Converts to template — source file has `.tmpl` suffix
- Uses a template variable (`.chezmoi.hostname`, `.chezmoi.os`, or custom data) to differentiate machines
- Shows `chezmoi cat` or `chezmoi execute-template` to preview before applying
- Does NOT hardcode the email difference as two separate files

## Failure Conditions

- Suggests managing two separate dotfile repos for each machine
- Hardcodes hostname comparison without explaining how to set custom data
- Skips the preview step and goes straight to apply
- Forgets to show how to commit and push the change to git
