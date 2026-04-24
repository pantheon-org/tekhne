# Reorganize Chezmoi Externals Into Program-Specific Files

## Problem Description

A developer has been managing chezmoi externals since before the `.chezmoiexternals/` directory feature existed. All their external dependencies live in a single `home/.chezmoiexternal.toml.tmpl` file and it has grown unwieldy — 80+ lines with Zsh plugins, bat themes, tmux config, and Neovim plugins all mixed together.

They want to migrate to the modern directory-based approach where each program's externals live in their own file under `.chezmoiexternals/`. They're worried about breaking their setup and want a safe migration plan they can execute step by step.

The current file is provided below. A run script at `home/run_onchange_externals.sh.tmpl` currently contains the line:
```bash
checksum="{{ include "home/.chezmoiexternal.toml.tmpl" | sha256sum }}"
```

## Input Files

Extract the following files before beginning.

=============== FILE: inputs/chezmoiexternal.toml.tmpl ===============
{{ if lookPath "zsh" }}
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
{{ end }}

{{ if lookPath "bat" }}
[".config/bat/themes/Catppuccin Mocha.tmTheme"]
type = "file"
url = "https://github.com/catppuccin/bat/raw/6810349b28055dce54076712fc05fc68da4b8ec0/themes/Catppuccin%20Mocha.tmTheme"
refreshPeriod = "168h"
{{ end }}

[".config/tmux/.tmux"]
type = "git-repo"
url = "https://github.com/gpakosz/.tmux.git"
revision = "23f6e11e65657406be2b2557148d831c631778d7"
refreshPeriod = "24h"
=============== END FILE ===============

=============== FILE: inputs/run_onchange_externals.sh.tmpl ===============
#!/bin/bash
checksum="{{ include "home/.chezmoiexternal.toml.tmpl" | sha256sum }}"
echo "Externals checksum: $checksum"
=============== END FILE ===============

## Output Specification

Produce the following output files representing the migrated state:

1. `output/zsh.externals.toml.tmpl` — Zsh plugin externals
2. `output/bat.externals.toml.tmpl` — bat theme externals
3. `output/tmux.externals.toml` — tmux config external (no template needed)
4. `output/run_onchange_externals.sh.tmpl` — updated run script with fixed include references
5. `migration_plan.md` — a step-by-step plan describing the safe migration sequence including when to run `chezmoi diff` and when it is safe to delete the original file
