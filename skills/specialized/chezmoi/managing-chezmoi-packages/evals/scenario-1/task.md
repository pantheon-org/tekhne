# Add Three Tools to a macOS Dotfiles Repo

## Problem Description

A macOS developer wants to declare three new tools in their chezmoi dotfiles so they are automatically available on any new machine. They currently manage packages in a dotfiles repo but haven't yet added these three:

1. **Warp** — a GUI terminal emulator (macOS only, distributed as a downloadable app)
2. **yq** — a command-line YAML processor (small binary, used across projects, in the Aqua registry)
3. **catppuccin bat theme** — a color theme file for the `bat` command-line tool (a single `.tmTheme` file hosted on GitHub)

The repo already has the following structure:

```
home/
├── .chezmoidata/
│   └── packages.yaml          # existing Homebrew manifest
├── .chezmoiexternals/
│   └── (empty directory)
└── dot_config/
    └── mise/
        └── config.toml        # existing mise tools config
```

The current `packages.yaml`:
```yaml
packages:
  darwin:
    brews: []
    casks: []
```

The current `mise/config.toml`:
```toml
[tools]
node = "22.0.0"
```

## Output Specification

Produce a file `proposal.md` with your recommendation for where each tool belongs and what changes to make. Include the exact content for each file that needs to be created or modified.

Do not apply any changes — document the proposal only.
