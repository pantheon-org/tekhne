# Set Up Automated Updates for Chezmoi Externals

## Problem Description

A developer has been maintaining their chezmoi dotfiles manually. They recently added several external dependencies but haven't configured any automation to keep them up to date. They want to set up Renovate to automatically propose PRs when their pinned versions become stale.

Their current `home/.chezmoiexternals/` directory contains two files:

```
home/.chezmoiexternals/
├── zsh.externals.toml
└── bat.externals.toml
```

The contents of these files are provided below. Their `renovate.json5` currently has no `customManagers` section.

## Input Files

Extract the following files before beginning.

=============== FILE: inputs/zsh.externals.toml ===============
[".zsh/plugins/zsh-autosuggestions"]
type = "git-repo"
url = "https://github.com/zsh-users/zsh-autosuggestions.git"
revision = "85919cd1ffa7d2d5412f6d3fe437ebdbeeec4fc5"
refreshPeriod = "168h"

[".zsh/plugins/zsh-syntax-highlighting"]
type = "git-repo"
url = "https://github.com/zsh-users/zsh-syntax-highlighting.git"
revision = "e0165eaa730dd0fa321a6a6de74f092fe87630b0"
refreshPeriod = "168h"
=============== END FILE ===============

=============== FILE: inputs/bat.externals.toml ===============
[".config/bat/themes/Catppuccin Mocha.tmTheme"]
type = "file"
url = "https://github.com/catppuccin/bat/raw/6810349b28055dce54076712fc05fc68da4b8ec0/themes/Catppuccin%20Mocha.tmTheme"
refreshPeriod = "168h"
=============== END FILE ===============

## Output Specification

Produce two output files:

1. `updated_zsh.externals.toml` — the zsh externals file with any annotations added
2. `updated_bat.externals.toml` — the bat externals file with any annotations added
3. `renovate_rules.json5` — the complete `customManagers` array to add to `renovate.json5`

Do not modify the actual repository files — just produce the output files.
