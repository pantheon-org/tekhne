# Renovate Integration for Chezmoi Externals

Automated dependency updates for chezmoi external definitions using Renovate's custom regex managers.

## Overview

Renovate can automatically detect and update:

- Git repository commit SHAs
- GitHub release versions and checksums
- File URLs with commit SHAs
- Archive URLs with commit SHAs

## Core Concept

For each external dependency, you need:

1. **Pinned reference** in external definition (commit SHA, version tag)
2. **Renovate rule** in `renovate.json5` to detect and update that reference
3. **Optional annotation** in external file to help Renovate identify the dependency

## Pattern Types

### 1. Git Repository with Revision (git-refs)

**External format**:

```toml
# renovate: repo=marlonrichert/zsh-snap branch=main
[".zsh/znap/zsh-snap"]
type = "git-repo"
url = "https://github.com/marlonrichert/zsh-snap.git"
revision = "25754a45d9ceafe6d7d082c9ebe40a08cb85a4f0"
refreshPeriod = "168h"
```

**Renovate rule**:

```json5
{
  customType: 'regex',
  fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
  matchStrings: [
    '# renovate: repo=(?<depName>.*?) branch=(?<currentValue>.*?)\\n.*?\\n.*?revision = "(?<currentDigest>[a-f0-9]{40})"',
  ],
  datasourceTemplate: 'git-refs',
}
```

### 2. GitHub Archive with Commit SHA (git-refs)

**External format**:

```toml
# renovate: repo=ohmyzsh/ohmyzsh branch=master
[".oh-my-zsh"]
type = "archive"
url = "https://github.com/ohmyzsh/ohmyzsh/archive/abc123def456.tar.gz"
exact = true
stripComponents = 1
```

**Renovate rule**:

```json5
{
  customType: 'regex',
  fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
  matchStrings: [
    '# renovate: repo=(?<depName>.*?) branch=(?<currentValue>.*?)\\n.*?url = "https://github\\.com/[^/]+/[^/]+/archive/(?<currentDigest>[a-f0-9]{40})\\.tar\\.gz"',
  ],
  datasourceTemplate: 'git-refs',
}
```

### 3. GitHub Raw File with Commit SHA (git-refs)

**External format**:

```toml
# renovate: repo=catppuccin/bat branch=main
[".config/bat/themes/Catppuccin-mocha.tmTheme"]
type = "file"
url = "https://github.com/catppuccin/bat/raw/6810349b28055dce54076712fc05fc68da4b8ec0/themes/Catppuccin%20Mocha.tmTheme"
```

**Renovate rule**:

```json5
{
  customType: 'regex',
  fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
  matchStrings: [
    '# renovate: repo=(?<depName>.*?) branch=(?<currentValue>.*?)\\n.*?url = "https://github\\.com/[^/]+/[^/]+/raw/(?<currentDigest>[a-f0-9]{40})/',
  ],
  datasourceTemplate: 'git-refs',
}
```

### 4. GitHub Release with Version Tag (github-releases)

**External format**:

```toml
[".local/bin/zellij"]
type = "archive-file"
url = "https://github.com/zellij-org/zellij/releases/download/v0.40.0/zellij-x86_64-apple-darwin.tar.gz"
executable = true
path = "zellij"
checksum = "sha256:abc123..."
```

**Renovate rule**:

```json5
{
  customType: 'regex',
  fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
  matchStrings: [
    'url = "https://github\\.com/(?<depName>[^/]+/[^/]+)/releases/download/(?<currentValue>v?[0-9.]+)/',
  ],
  datasourceTemplate: 'github-releases',
}
```

### 5. Multiple Files from Same Repo (shared annotation)

**External format**:

```toml
# renovate: repo=catppuccin/bat branch=main sha=6810349b28055dce54076712fc05fc68da4b8ec0

[".config/bat/themes/Catppuccin Latte.tmTheme"]
type = "file"
url = "https://github.com/catppuccin/bat/raw/6810349b28055dce54076712fc05fc68da4b8ec0/themes/Catppuccin%20Latte.tmTheme"

[".config/bat/themes/Catppuccin Mocha.tmTheme"]
type = "file"
url = "https://github.com/catppuccin/bat/raw/6810349b28055dce54076712fc05fc68da4b8ec0/themes/Catppuccin%20Mocha.tmTheme"
```

Single annotation at top; Renovate updates all matching SHAs in the file.

## File Matching Patterns

```json5
// All .chezmoiexternals/ files
fileMatch: ["^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$"]

// Specific program file
fileMatch: ["^home/\\.chezmoiexternals/zsh\\.externals\\.toml(\\.tmpl)?$"]

// Legacy single-file format
fileMatch: ["^home/\\.chezmoiexternal\\.toml(\\.tmpl)?$"]
```

## Annotation Patterns

### Inline (Recommended)

Place immediately before external definition:

```toml
# renovate: repo=user/repo branch=main
[".path/to/file"]
type = "git-repo"
url = "https://github.com/user/repo.git"
revision = "<commit-sha>"
```

### URL Extraction (No Annotation)

For consistent URL formats, Renovate can extract info directly:

```toml
[".local/bin/tool"]
type = "archive-file"
url = "https://github.com/user/tool/releases/download/v1.0.0/tool.tar.gz"
executable = true
```

## Datasource Types

| Datasource        | Best For                                        |
| ----------------- | ----------------------------------------------- |
| `git-refs`        | Commit SHAs (revisions, archive/file URL SHAs)  |
| `github-releases` | Release binary URLs with version tags           |
| `github-tags`     | Git tags (alternative to github-releases)       |

## Complete Example Configuration

```json5
{
  extends: ['config:base'],
  customManagers: [
    // Git repos with revision field
    {
      customType: 'regex',
      fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
      matchStrings: [
        '# renovate: repo=(?<depName>.*?) branch=(?<currentValue>.*?)\\n.*?\\n.*?revision = "(?<currentDigest>[a-f0-9]{40})"',
      ],
      datasourceTemplate: 'git-refs',
    },
    // GitHub archives with commit SHA
    {
      customType: 'regex',
      fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
      matchStrings: [
        '# renovate: repo=(?<depName>.*?) branch=(?<currentValue>.*?)\\n.*?url = "https://github\\.com/[^/]+/[^/]+/archive/(?<currentDigest>[a-f0-9]{40})\\.tar\\.gz"',
      ],
      datasourceTemplate: 'git-refs',
    },
    // GitHub raw files with commit SHA
    {
      customType: 'regex',
      fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
      matchStrings: [
        '# renovate: repo=(?<depName>.*?) branch=(?<currentValue>.*?)\\n.*?url = "https://github\\.com/[^/]+/[^/]+/raw/(?<currentDigest>[a-f0-9]{40})/',
      ],
      datasourceTemplate: 'git-refs',
    },
    // GitHub releases
    {
      customType: 'regex',
      fileMatch: ['^home/\\.chezmoiexternals/.*\\.toml(\\.tmpl)?$'],
      matchStrings: [
        'url = "https://github\\.com/(?<depName>[^/]+/[^/]+)/releases/download/(?<currentValue>v?[0-9.]+)/',
      ],
      datasourceTemplate: 'github-releases',
    },
  ],
}
```

## Testing Renovate Rules

1. **Validate regex**: Use regex101.com with your external file content as test string
2. **Dry-run locally**: `LOG_LEVEL=debug renovate --dry-run --require-config=false`
3. **Test in PR**: Commit outdated SHA, wait for Renovate to propose update

## Common Issues

### Renovate Not Detecting Dependency

1. Check `fileMatch` pattern includes your file
2. Verify regex named capture groups exist and match
3. Ensure datasource is appropriate
4. Check Renovate logs for parse errors

### Updates Break Checksums

Solutions:
1. Accept manual checksum updates in Renovate PRs
2. Run a post-update script to compute and update checksums
3. Use `autoReplaceStringTemplate` to preserve file structure

### Branch Not Found

Verify annotation's branch name matches the repository default branch (`main` vs `master`).

## Reference Commands

```bash
# Get latest commit SHA
gh api repos/USER/REPO/commits/BRANCH --jq .sha

# Get latest release
gh api repos/USER/REPO/releases/latest --jq .tag_name

# Compute checksum
curl -fsSL <url> | shasum -a 256

# Force update externals
chezmoi update --force
```

## Best Practices

1. **Use annotations**: Add `# renovate:` comments for clarity and maintainability
2. **Group by program**: Organize externals in `.chezmoiexternals/PROGRAM.externals.toml`
3. **Consistent formatting**: Use same pattern across similar externals
4. **Document patterns**: Add comments in `renovate.json5` explaining each rule
5. **Test thoroughly**: Validate rules before relying on automation
6. **Pin everything**: Always use immutable references
