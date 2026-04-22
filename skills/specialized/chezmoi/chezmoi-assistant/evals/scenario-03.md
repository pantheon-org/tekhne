# Scenario 03: Write a script that installs packages only once

## User Prompt

I want to add a script to my chezmoi dotfiles that installs my Homebrew packages, but only runs it on first setup — not every time I run `chezmoi apply`.

## Expected Behavior

1. Create a file named `run_once_before_install-homebrew-packages.sh` in the source directory
2. Explain the prefix breakdown: `run_` (execute, not copy) + `once_` (run only if never run before) + `before_` (run before other changes are applied)
3. Show an example script that calls `brew bundle` or lists packages with `brew install`
4. Explain that chezmoi tracks whether the script has run by hashing its content in `~/.local/share/chezmoi/.chezmoistate.boltdb`
5. Mention that changing the script content triggers a re-run (use `run_onchange_` if that's the desired behaviour)
6. Show how to add as a template (`.tmpl` suffix) if the script needs OS-specific logic

## Success Criteria

- Uses `run_once_before_` prefix (not just `run_` or `once_`)
- Correctly explains that `once_` is content-hashed, not time-based
- Distinguishes `run_once_` vs `run_onchange_` and when to use each
- Mentions `.tmpl` suffix as optional for platform-conditional scripts
- Does NOT suggest using a cron job or external script runner

## Failure Conditions

- Uses `run_` without `once_` — script would execute on every `chezmoi apply`
- Confuses `once_` with `onchange_`
- Recommends a separate shell script outside chezmoi management
- Gets the prefix order wrong (e.g. `before_run_once_` instead of `run_once_before_`)
