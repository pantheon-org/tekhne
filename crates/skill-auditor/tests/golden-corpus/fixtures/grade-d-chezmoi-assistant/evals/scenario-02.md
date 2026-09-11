# Scenario 02: Bootstrap chezmoi on a new machine

## User Prompt

I just got a new MacBook. I have my dotfiles in a GitHub repo at github.com/alice/dotfiles. Walk me through setting up chezmoi from scratch.

## Expected Behavior

1. Install chezmoi (e.g. `brew install chezmoi` or `sh -c "$(curl -fsLS get.chezmoi.io)"`)
2. Run `chezmoi init --apply alice` to clone the repo and apply all dotfiles in one step
3. Explain that chezmoi clones to `~/.local/share/chezmoi` and runs `chezmoi apply` automatically
4. If the user has run scripts (`run_once_`), explain they will execute during this step
5. Mention `chezmoi doctor` to verify the environment after setup

## Success Criteria

- Uses `chezmoi init --apply $GITHUB_USERNAME` (single command bootstrap)
- Explains what the command does (clone + apply)
- Mentions that `run_once_` scripts execute during apply
- Suggests `chezmoi doctor` for post-setup verification
- Does NOT require manual `git clone` before running chezmoi

## Failure Conditions

- Tells user to manually `git clone` then `chezmoi apply` separately when `init --apply` covers both
- Omits mentioning that run scripts will fire during first apply
- Skips the install step for a brand-new machine
- Does not mention how to verify success
