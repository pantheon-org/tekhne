# Package Ecosystem Selection Guide

Decision framework for choosing the right package management approach in a chezmoi dotfiles repository.

## Decision Tree

```
Is it a package/tool to install?
├─ YES → Does it need to be installed system-wide?
│   ├─ YES → Is this macOS?
│   │   ├─ YES → Use Homebrew (.chezmoidata/packages.yaml)
│   │   └─ NO → Is this Linux?
│   │       ├─ YES → Use system package manager (apt/dnf) or Homebrew
│   │       └─ NO → Use appropriate OS package manager
│   └─ NO → Is it a CLI developer tool (language version manager, etc.)?
│       ├─ YES → Use mise (.mise.toml)
│       ├─ NO → Is it a Python tool (uv tool install)?
│       │   ├─ YES → Use Python requirements (requirements.txt)
│       │   └─ NO → Is it for containers/development environments?
│       │       ├─ YES → Use Docker/devcontainer configs
│       │       └─ NO → Consider chezmoi externals
└─ NO → Is it a file/directory/repo to download?
    └─ YES → Use chezmoi externals (.chezmoiexternals/)
```

## Ecosystem Comparison

| Ecosystem             | Best For                         | Location                                  | Renovate Support | Notes                        |
| --------------------- | -------------------------------- | ----------------------------------------- | ---------------- | ---------------------------- |
| **Homebrew**          | macOS packages, Linux packages   | `.chezmoidata/packages.yaml`              | Built-in         | System-wide installation     |
| **mise**              | CLI dev tools, language runtimes | `.mise.toml`                              | Custom regex     | Per-project or global        |
| **Python/uv**         | Python CLI tools                 | `requirements.txt`                        | Built-in         | Isolated Python environments |
| **Docker**            | Container images                 | `docker-compose.yml`, `devcontainer.json` | Built-in         | Development environments     |
| **Chezmoi Externals** | Files, archives, Git repos       | `.chezmoiexternals/*.toml`                | Custom regex     | Dotfiles, configs, plugins   |
| **CLI Versions**      | Script-installed CLIs            | `cli-versions.toml`                       | Custom regex     | Install script dependencies  |

## Detailed Ecosystem Guides

### 1. Homebrew (brew/cask/mas)

**When to use**:

- macOS GUI applications (casks)
- macOS/Linux CLI tools available in Homebrew
- Mac App Store applications (mas)
- System-level packages

**Location**: `home/.chezmoidata/packages.yaml`

**Format**:

```yaml
packages:
  darwin:
    brews:
      - git
      - node
      - python
    casks:
      - google-chrome
      - visual-studio-code
      - docker
    mas:
      - 497799835 # Xcode (App Store ID)
```

**Installation**: Run script `run_onchange_darwin-install-packages.sh.tmpl`

**Renovate**: Built-in support via `homebrew` datasource

**Pros**:

- Extensive package catalog
- System-wide availability
- Automatic updates via Homebrew
- Well-maintained formulas

**Cons**:

- macOS/Linux only
- Requires admin permissions
- System-wide (can conflict)
- Slower than binary downloads

### 2. Mise (Aqua-based)

**When to use**:

- CLI developer tools (not in Homebrew)
- Language version managers
- Project-specific tool versions
- Tools from GitHub releases

**Location**: `.mise.toml` (global) or `home/dot_config/mise/config.toml` (managed)

**Format**:

```toml
[tools]
"aqua:mikefarah/yq" = "v4.47.1"
"aqua:cli/cli" = "v2.63.2"
node = "20.11.0"
python = "3.12.1"
```

**Renovate**: Custom regex manager for aqua-prefixed tools

**Pros**:

- Fast binary installations
- Per-project versions
- No admin permissions needed
- Extensive tool support via Aqua registry

**Cons**:

- Requires mise installed
- Less familiar than Homebrew
- Some tools may not be available

### 3. Python Requirements (uv tool install)

**When to use**:

- Python CLI tools (uv tool install)
- Python libraries and dependencies
- Tools best installed via pip

**Location**: `home/dot_config/dotfiles/requirements.txt`

**Format**:

```txt
black==24.1.1
ruff==0.2.0
poetry==1.7.1
```

**Renovate**: Built-in support via `pypi` datasource

### 4. Docker / Devcontainer

**When to use**:

- Container images for development
- Devcontainer features
- Docker Compose services

**Location**: `home/dot_config/docker-compose/*.yml`, `.devcontainer/devcontainer.json`

**Renovate**: Built-in support for Docker images and devcontainer features

### 5. Chezmoi Externals

**When to use**:

- Shell/editor plugins
- Configuration frameworks
- Theme files
- Binaries from GitHub releases
- Any file/directory to download

**Location**: `home/.chezmoiexternals/*.toml.tmpl`

**Renovate**: Custom regex managers for each pattern

**Pros**:

- No package manager dependencies
- Flexible (files, archives, repos)
- Fast downloads with caching
- Works across all platforms

**Cons**:

- Manual Renovate setup
- No dependency resolution
- Requires careful version pinning

### 6. CLI Versions (Install Script)

**When to use**:

- Tools installed by custom script
- Tools requiring special installation logic
- Cross-platform CLI tools with signature verification

**Location**: `home/dot_config/dotfiles/cli-versions.toml`

**Format**:

```toml
cosign = "v2.5.3"
chezmoi = "v2.56.0"
mise = "v2024.1.0"
```

**Renovate**: Custom regex manager

## Selection Criteria

### By Tool Type

| Tool Type                             | Recommended Ecosystem | Alternative            |
| ------------------------------------- | --------------------- | ---------------------- |
| **System utility** (git, curl)        | Homebrew              | mise                   |
| **GUI application** (VS Code, Chrome) | Homebrew (cask)       | Manual install         |
| **CLI dev tool** (gh, jq, yq)         | mise                  | Homebrew               |
| **Language runtime** (Node, Python)   | mise                  | Homebrew               |
| **Python tool** (black, poetry)       | uv tool install       | mise                   |
| **Shell plugin**                      | Chezmoi externals     | Manual                 |
| **Editor plugin**                     | Chezmoi externals     | Editor package manager |
| **Config framework** (oh-my-zsh)      | Chezmoi externals     | Manual                 |
| **Theme file**                        | Chezmoi externals     | Manual                 |
| **Container image**                   | Docker                | N/A                    |

### By Platform

| Platform           | Primary         | Secondary                   | Notes                    |
| ------------------ | --------------- | --------------------------- | ------------------------ |
| **macOS**          | Homebrew + mise | Chezmoi externals           | Use casks for GUI apps   |
| **Linux**          | mise + apt/dnf  | Homebrew, Chezmoi externals | System packages for deps |
| **Containers**     | Docker + mise   | Chezmoi externals           | Minimal base image       |
| **Cross-platform** | mise            | Chezmoi externals           | Use mise for consistency |

## Best Practices

1. **Minimize ecosystems**: Fewer systems = easier maintenance
2. **Prefer mise for CLI**: Fast, flexible, per-project versions
3. **Use Homebrew for GUI**: macOS applications via casks
4. **Externals for dotfiles**: Plugins, themes, configs
5. **Pin everything**: No mutable references anywhere
6. **Document choices**: Comment why each tool uses its ecosystem
7. **Test cross-platform**: Verify on macOS, Linux, containers
8. **Centralize versions**: Use ecosystem-specific manifests
9. **Automate updates**: Configure Renovate for all ecosystems
10. **Isolate environments**: Avoid global installations when possible

## Quick Reference

| Need            | Use              | Command                                               |
| --------------- | ---------------- | ----------------------------------------------------- |
| Add macOS app   | Homebrew cask    | Edit `packages.yaml`                                  |
| Add CLI tool    | mise             | `mise use <tool>@<version>`                           |
| Add zsh plugin  | Chezmoi external | Create in `.chezmoiexternals/zsh.externals.toml.tmpl` |
| Add Python tool | uv tool install  | Add to `requirements.txt`                             |
| Add binary      | Chezmoi external | Create in `.chezmoiexternals/` with type=archive-file |
| Update versions | Renovate         | Wait for PR or update manually                        |
| Preview changes | Chezmoi          | `chezmoi diff`                                        |
| Apply changes   | Chezmoi          | `chezmoi apply`                                       |
