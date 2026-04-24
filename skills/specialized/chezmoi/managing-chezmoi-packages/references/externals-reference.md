# Chezmoi External Format Reference

Complete specification for chezmoi external dependencies. Externals allow you to include files, archives, and Git repositories from external sources in your dotfiles.

## Overview

Externals are defined in:

- **Legacy**: `home/.chezmoiexternal.<format>` (single file for all externals)
- **Recommended**: `home/.chezmoiexternals/*.toml[.tmpl]` (organized by program)

Files in `.chezmoiexternals/` are automatically treated as external definitions relative to the source directory.

## External Types

### 1. git-repo

Clones or updates a Git repository to a target directory.

**Best for**: Plugin managers, frameworks, complete configurations

**Required fields**:

- `type = "git-repo"`
- `url` - Repository URL (HTTPS or SSH)

**Optional fields**:

- `revision` - Specific commit SHA, tag, or branch (CRITICAL: always use commit SHA)
- `clone.args` - Additional git clone arguments
- `pull.args` - Additional git pull arguments
- `refreshPeriod` - How often to check for updates (e.g., "168h")

**Example**:

```toml
[".zsh/plugins/zsh-autosuggestions"]
type = "git-repo"
url = "https://github.com/zsh-users/zsh-autosuggestions.git"
revision = "85919cd1ffa7d2d5412f6d3fe437ebdbeeec4fc5"        # Pin to SHA
refreshPeriod = "168h"                                       # Update weekly
```

**Security**: Always pin `revision` to a commit SHA for reproducibility.

### 2. file

Downloads a single file from a URL.

**Best for**: Configuration files, themes, single scripts

**Required fields**:

- `type = "file"`
- `url` or `urls` - Download location(s)

**Optional fields**:

- `executable` - Make file executable (boolean)
- `checksum` - SHA-256/384/512 hash for verification
- `refreshPeriod` - How often to re-download
- `encrypted` - Handle encrypted content (boolean)

**Example**:

```toml
[".config/bat/themes/Catppuccin-mocha.tmTheme"]
type = "file"
url = "https://github.com/catppuccin/bat/raw/6810349b28055dce54076712fc05fc68da4b8ec0/themes/Catppuccin%20Mocha.tmTheme"
refreshPeriod = "168h"
```

**With checksum**:

```toml
[".local/bin/tool"]
type = "file"
url = "https://example.com/releases/v1.0.0/tool"
executable = true
checksum = "sha256:abc123def456..."
```

**Security**: Use commit SHA in URL path for GitHub files.

### 3. archive-file

Extracts a specific file from an archive (tar.gz, zip, etc.).

**Best for**: Release binaries, single files within archives

**Required fields**:

- `type = "archive-file"`
- `url` or `urls` - Archive download location
- `path` - Path to file within archive

**Optional fields**:

- `executable` - Make extracted file executable
- `checksum` - Archive checksum verification
- `refreshPeriod` - Update frequency

**Example**:

```toml
[".local/bin/zellij"]
type = "archive-file"
url = "https://github.com/zellij-org/zellij/releases/download/v0.40.0/zellij-x86_64-apple-darwin.tar.gz"
executable = true
path = "zellij"
checksum = "sha256:1234567890abcdef..."
refreshPeriod = "168h"
```

**Security**: Always include `checksum` for release binaries.

### 4. archive

Extracts an entire archive into a target directory.

**Best for**: Complete directory structures, multi-file configurations

**Required fields**:

- `type = "archive"`
- `url` or `urls` - Archive download location

**Optional fields**:

- `exact` - Remove files not in archive (boolean)
- `stripComponents` - Strip leading path components (integer)
- `include` - Only extract matching patterns (array)
- `exclude` - Skip matching patterns (array)
- `format` - Force specific archive format
- `checksum` - Archive verification
- `refreshPeriod` - Update frequency

**Example - Full directory**:

```toml
[".oh-my-zsh"]
type = "archive"
url = "https://github.com/ohmyzsh/ohmyzsh/archive/abc123def456.tar.gz"
exact = true
stripComponents = 1
refreshPeriod = "168h"
```

**Security**: Use commit SHA in URL for GitHub archives.

## Common Fields

### url vs urls

```toml
# Single URL
url = "https://github.com/user/repo/archive/sha.tar.gz"

# Multiple URLs (tries in order)
urls = [
  "https://cdn.example.com/file.tar.gz",
  "https://github.com/user/repo/releases/download/v1.0.0/file.tar.gz",
]
```

### checksum

```toml
checksum = "sha256:abc123def456..."
checksum = "sha384:abc123def456..."
checksum = "sha512:abc123def456..."
```

**Generate checksum**:

```bash
curl -fsSL <url> | shasum -a 256
```

### refreshPeriod

```toml
refreshPeriod = "24h"  # Daily
refreshPeriod = "168h" # Weekly (recommended for most externals)
refreshPeriod = "720h" # Monthly
```

## Version Pinning Strategies

### Git Repositories — ALWAYS use commit SHA in `revision`

```toml
# CORRECT
revision = "abc123def456..."               # 40-character SHA

# WRONG - mutable reference
revision = "main"
```

### GitHub Raw Files — Include commit SHA in URL path

```toml
# CORRECT
url = "https://github.com/user/repo/raw/abc123def456.../file.ext"

# WRONG
url = "https://github.com/user/repo/raw/main/file.ext"
```

### Release Binaries — Pin version tag AND include checksum

```toml
# CORRECT
url = "https://github.com/user/tool/releases/download/v1.2.3/tool.tar.gz"
checksum = "sha256:abc123..."

# WRONG
url = "https://github.com/user/tool/releases/download/latest/tool.tar.gz"
```

## Templating

### OS Detection

```toml
{{ if eq .chezmoi.os "darwin" }}
[".config/tool-mac"]
type = "file"
url = "https://example.com/tool-mac"
{{ else if eq .chezmoi.os "linux" }}
[".config/tool-linux"]
type = "file"
url = "https://example.com/tool-linux"
{{ end }}
```

### Conditional Installation

```toml
{{ if lookPath "zsh" }}
[".zsh/plugins/plugin"]
type = "git-repo"
url = "https://github.com/user/plugin.git"
{{ end }}
```

### Architecture Detection

```toml
{{ $arch := .chezmoi.arch -}}
[".local/bin/tool"]
type = "archive-file"
url = "https://github.com/user/tool/releases/download/v1.0.0/tool-{{ .chezmoi.os }}-{{ $arch }}.tar.gz"
executable = true
path = "tool"
```

## Archive Formats

Chezmoi automatically detects: `.tar.gz`, `.tgz`, `.tar.bz2`, `.tar.xz`, `.tar.zst`, `.zip`

Force specific format:

```toml
format = "tar.gz"
```

## stripComponents

```toml
# Archive structure: repo-abc123/config/file.conf
[".config/tool"]
type = "archive"
url = "https://github.com/user/repo/archive/abc123.tar.gz"
stripComponents = 1    # Removes "repo-abc123/" prefix
```

## include/exclude Patterns

```toml
[".config/tool"]
type = "archive"
url = "..."
include = ["config/**", "themes/*.json"]
exclude = ["**/*.md", "docs/**"]
```

## Best Practices

1. **Always pin versions**: Commit SHAs for git repos, checksums for files
2. **Use .chezmoiexternals/**: Organize by program (zsh, bat, tmux, nvim, etc.)
3. **Set refreshPeriod**: 168h recommended for most externals
4. **Add comments**: Explain what each external is and why it's included
5. **Template conditionally**: Use `{{ if lookPath "tool" }}` to skip on missing dependencies
6. **Verify checksums**: Always include for release binaries and archives

## Troubleshooting

### External not downloading

```bash
curl -fsSL <url> -o /tmp/test-download
gh api repos/USER/REPO/commits/SHA
chezmoi apply -v
```

### Checksum mismatch

```bash
curl -fsSL <url> | shasum -a 256
# Update checksum in external definition
```

### Template errors

```bash
chezmoi cat home/.chezmoiexternals/program.externals.toml
chezmoi data
```
