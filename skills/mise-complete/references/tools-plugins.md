# Tool Plugins

Understanding Mise's plugin system and available tool plugins.

## Core vs Plugin Tools

### Core Tools (Built-in)

Mise has built-in support for popular tools (no plugins needed):

```toml
[tools]
# Core tools - faster, no plugin installation
node = "20.10.0"
python = "3.12.0"
java = "21"
go = "1.21"
rust = "1.75"
```

### Plugin Tools

Other tools use the asdf plugin ecosystem:

```bash
# List available plugins
mise plugins ls-remote

# Install a plugin
mise plugins install postgres

# Use the plugin
mise use postgres@16.1
```

## Common Plugin Tools

### Databases

```toml
[tools]
postgres = "16.1"
mysql = "8.2"
redis = "7.2"
mongodb = "7.0"
```

### Cloud CLI Tools

```toml
[tools]
awscli = "2.13"
gcloud = "450.0"
kubectl = "1.28"
helm = "3.13"
```

### Build Tools

```toml
[tools]
cmake = "3.27"
make = "4.4"
bazel = "7.0"
```

## Package Manager Backends

### npm Backend

Install npm packages as tools:

```toml
[tools]
"npm:typescript" = "5.3"
"npm:prettier" = "3.1"
"npm:eslint" = "8.55"
```

### cargo Backend

Install cargo packages as tools:

```toml
[tools]
"cargo:eza" = "latest"
"cargo:ripgrep" = "latest"
"cargo:fd-find" = "latest"
```

### go Backend

Install Go tools:

```toml
[tools]
"go:github.com/golangci/golangci-lint/cmd/golangci-lint" = "latest"
```

## Plugin Management

### Installing Plugins

```bash
# Auto-install when using tool
mise use postgres@16

# Manual plugin installation
mise plugins install postgres

# Install from custom URL
mise plugins install myplugin https://github.com/user/mise-myplugin
```

### Updating Plugins

```bash
# Update all plugins
mise plugins update

# Update specific plugin
mise plugins update node

# List installed plugins
mise plugins ls
```

### Removing Plugins

```bash
# Uninstall plugin
mise plugins uninstall postgres
```

## Custom Plugins

### Using Custom Plugin Repositories

```bash
# Install from GitHub
mise plugins install custom https://github.com/org/mise-custom

# Use custom plugin
mise use custom@1.0.0
```

### Plugin Configuration

```toml
[plugins]
# Configure plugin behavior
postgres = { repo = "https://github.com/custom/mise-postgres" }
```

## Backend Priority

When a tool is available from multiple sources:

```toml
[tools]
# 1. Core backend (fastest, preferred)
node = "20.10.0"

# 2. Explicit backend selection
node = "core:20.10.0"
python = "core:3.12.0"

# 3. Plugin backend
ruby = "asdf:3.3.0"

# 4. Package manager backend
"npm:typescript" = "5.3"
```

## Best Practices

### Prefer Core Tools

```toml
# Good: Use core tools when available
[tools]
node = "20.10.0"
python = "3.12.0"

# Avoid: Explicitly using plugin backend
[tools]
node = "asdf:20.10.0"  # Unnecessary
```

### Pin Plugin Versions in CI

```bash
# In CI/CD, ensure consistent plugin versions
mise plugins install --sha 7c839f8 postgres
```

### Document Plugin Requirements

```toml
# mise.toml
[tools]
# Requires postgres plugin
postgres = "16.1"

# Core tool, no plugin needed
node = "20.10.0"
```

## Troubleshooting

### Plugin Not Found

```bash
# Error: Plugin not found
mise use unfamiliar-tool@1.0

# Solution: Install plugin first
mise plugins install unfamiliar-tool
mise use unfamiliar-tool@1.0
```

### Plugin Installation Fails

```bash
# Check plugin repository
mise plugins ls-remote | grep postgres

# Try manual installation
mise plugins install postgres

# Check plugin status
mise doctor
```

### Version Not Available

```bash
# Check available versions
mise ls-remote node

# Try different version
mise use node@20.10.0
```
