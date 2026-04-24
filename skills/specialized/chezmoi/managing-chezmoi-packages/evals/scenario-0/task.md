# Add zsh-autosuggestions to Dotfiles

## Problem Description

A developer maintains their dotfiles with chezmoi and wants to start managing `zsh-autosuggestions` — a popular Zsh plugin that shows command completions based on history. Right now they install it manually on each new machine and occasionally forget, so their shell experience is inconsistent.

They want the plugin to be declared in their dotfiles repo so that `chezmoi apply` on any new machine sets it up automatically. They also want it to stay up-to-date without manual tracking.

The dotfiles repository already uses the layout below. The `.chezmoiexternals/` directory exists but does not yet have a Zsh file.

```
home/
├── .chezmoidata/
│   └── packages.yaml
├── .chezmoiexternals/
│   └── bat.externals.toml
└── dot_zshrc.tmpl
```

## Output Specification

Produce a single file `proposal.md` containing your complete recommendation. It should include:

- The target file path where the external definition should be added
- The full TOML content to add (ready to copy-paste)
- Any Renovate automation rule needed (full JSON5 snippet)
- The commands the developer should run to preview and apply the change

Do not apply any changes — just produce the proposal file.
