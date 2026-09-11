# Chezmoi Command Reference

## Getting Started

```bash
chezmoi doctor                    # check for common problems
chezmoi init                      # create source dir + git repo
chezmoi init $GITHUB_USERNAME     # clone dotfiles from GitHub
chezmoi init --apply $GITHUB_USERNAME  # clone + apply immediately
```

## Daily Commands

```bash
chezmoi add ~/.zshrc              # start tracking a file
chezmoi add --template ~/.zshrc   # track as template
chezmoi edit ~/.zshrc             # open source file in $EDITOR
chezmoi edit --apply ~/.zshrc     # edit then apply immediately
chezmoi status                    # summary of pending changes (A/D/M)
chezmoi diff                      # show what apply would change
chezmoi apply                     # apply all pending changes
chezmoi apply ~/.zshrc            # apply one file
chezmoi cd                        # open shell in source directory
```

## Inspection & Debugging

```bash
chezmoi cat ~/.zshrc              # print rendered target (no apply)
chezmoi data                      # print available template variables as JSON
chezmoi execute-template          # render a template from stdin
chezmoi source-path ~/.zshrc      # show source path for a target file
chezmoi target-path               # show target path for a source file
chezmoi managed                   # list all managed files
chezmoi unmanaged                 # list unmanaged files in target
chezmoi doctor                    # diagnose environment problems
```

## Attribute Management

```bash
chezmoi chattr +template ~/.zshrc
chezmoi chattr +encrypted ~/.netrc
chezmoi chattr +executable ~/.local/bin/script
chezmoi chattr -private ~/.config/foo
```

## Multi-Machine Sync

```bash
chezmoi update                    # git pull + apply
# Commit and push changes:
chezmoi cd && git add -A && git commit -m "..." && git push
```

## Encryption

```bash
# With age (recommended):
chezmoi add --encrypt ~/.ssh/id_rsa
# Configure in chezmoi.toml:
# [age]
#   identity = "~/.config/chezmoi/key.txt"
#   recipient = "age1..."
```

## Status Output Key

| Symbol | Meaning |
|--------|---------|
| `A` | File will be added |
| `D` | File will be deleted |
| `M` | File will be modified |
| `R` | File will be run (script) |
