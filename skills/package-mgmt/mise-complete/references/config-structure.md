# Configuration File Structure

## mise.toml Overview

The `mise.toml` file is the main configuration file for Mise, using TOML format.

## Basic Structure

```toml
# Tool versions
[tools]
node = "20"
python = "3.11"
rust = "stable"

# Environment variables
[env]
NODE_ENV = "development"
API_URL = "http://localhost:3000"

# Tasks
[tasks.build]
description = "Build the project"
run = "npm run build"

# Settings
[settings]
experimental = true
```

## Configuration Sections

### [tools]

Specifies tool versions to install and manage:

```toml
[tools]
node = "20.10.0"
python = "3.11"
terraform = "1.6"
kubectl = "latest"
```

**Supported formats:**
- Exact version: `"20.10.0"`
- Major.minor: `"20.10"`
- Major only: `"20"`
- Version prefix: `"prefix:20.10.0"`
- Latest: `"latest"`
- Path: `"path:./custom-node"`

### [env]

Defines environment variables:

```toml
[env]
NODE_ENV = "development"
DATABASE_URL = "postgresql://localhost/myapp"
_.path = ["./bin", "./node_modules/.bin"]
_.file = [".env", ".env.local"]
```

**Special keys:**
- `_.path`: Adds directories to PATH
- `_.file`: Loads environment from files
- `_.source`: Sources shell scripts

### [tasks.*]

Defines executable tasks:

```toml
[tasks.dev]
description = "Start development server"
run = "npm run dev"

[tasks.test]
description = "Run tests"
depends = ["lint"]
run = [
  "npm run test:unit",
  "npm run test:integration"
]
```

**Task properties:**
- `description`: Human-readable description
- `run`: Command(s) to execute
- `depends`: Task dependencies
- `env`: Task-specific environment
- `dir`: Working directory
- `sources`: Input files
- `outputs`: Generated files
- `alias`: Alternative names

### [settings]

Mise behavior configuration:

```toml
[settings]
experimental = true
verbose = false
jobs = 4
```

## Configuration Files

### Project Configuration

**mise.toml** (committed):
```toml
[tools]
node = "20"

[env]
NODE_ENV = "development"

[tasks.dev]
run = "npm run dev"
```

**mise.local.toml** (git-ignored):
```toml
[env]
DATABASE_URL = "postgresql://localhost/myapp_johndoe"
DEBUG = "true"
```

### Global Configuration

**~/.config/mise/config.toml**:
```toml
[settings]
experimental = true

[env]
EDITOR = "code"
_.path = ["$HOME/.local/bin"]

[tools]
# Global tools available everywhere
```

### System Configuration

**/etc/mise/config.toml**:
```toml
[env]
CORPORATE_PROXY = "http://proxy.company.com:8080"
```

## File Locations

### Search Order

Mise searches for configuration files in this order:

1. `./mise.local.toml` (current directory)
2. `./mise.toml` (current directory)
3. `../.mise.toml` (parent directory)
4. `~/.config/mise/config.toml` (global)
5. `/etc/mise/config.toml` (system)

### Explicit Paths

```bash
# Use specific config file
mise --config /path/to/config.toml run build

# Use config from directory
mise -C /path/to/project run test
```

## Tool Configuration

### Version Specification

```toml
[tools]
# Exact version
node = "20.10.0"

# Version range
python = "3.11"

# Latest
rust = "latest"

# Version prefix
ruby = "ref:main"

# Path to local installation
go = "path:$HOME/custom-go"

# System installation
java = "system"
```

### Tool-Specific Options

```toml
[tools]
node = { version = "20", install_args = "--with-intl=full" }
python = { version = "3.11", virtualenv = ".venv" }
```

### Multiple Versions

```toml
[tools]
node = ["18", "20", "21"]  # Install multiple versions
```

## Environment Configuration

### Variable Definition

```toml
[env]
# Simple values
NODE_ENV = "development"
PORT = "3000"

# Multi-line values
PRIVATE_KEY = """
-----BEGIN PRIVATE KEY-----
...
-----END PRIVATE KEY-----
"""

# Template variables
PROJECT_ROOT = "{{ config_root }}"
DATA_DIR = "{{ config_root }}/data"
```

### PATH Management

```toml
[env]
_.path = [
  "{{ config_root }}/bin",
  "{{ config_root }}/node_modules/.bin",
  "$HOME/.local/bin"
]
```

### File Loading

```toml
[env]
_.file = [
  ".env",
  ".env.local",
  ".env.{{ env.NODE_ENV }}"
]
```

### Script Sourcing

```toml
[env]
_.source = "scripts/env-setup.sh"
```

## Task Configuration

### Inline Tasks

```toml
[tasks.build]
description = "Build the project"
run = "npm run build"

[tasks.test]
description = "Run tests"
depends = ["build"]
run = [
  "npm run lint",
  "npm test"
]
```

### Task Properties

```toml
[tasks.deploy]
description = "Deploy to production"
depends = ["test", "build"]
run = "./scripts/deploy.sh"
dir = "{{ config_root }}"
env = { ENVIRONMENT = "production" }
sources = ["src/**/*", "package.json"]
outputs = ["dist/**/*"]
alias = "ship"
```

### File-Based Tasks

```toml
# Tasks can also be defined in .mise/tasks/
# mise.toml can reference them
[tasks]
# No inline definition needed if file exists
```

## Settings Configuration

### Common Settings

```toml
[settings]
# Enable experimental features
experimental = true

# Verbose output
verbose = false

# Number of parallel jobs
jobs = 4

# Disable telemetry
telemetry = false

# Automatic activation
activate_aggressive = true

# Legacy version file support
legacy_version_file = true

# Always keep downloads
always_keep_download = false

# Plugin autoupdate
plugin_autoupdate_last_check_duration = "7d"
```

### Task Settings

```toml
[settings]
task_output = "prefix"  # or "interleave"
raw = false
```

## Configuration Validation

### Syntax Check

```bash
# Validate configuration
mise doctor

# Show parsed configuration
mise config show

# List configuration files
mise config ls
```

### Common Issues

**Invalid TOML syntax:**
```toml
[env]
KEY = value  # Missing quotes
```

**Correct:**
```toml
[env]
KEY = "value"
```

## Comments

```toml
# This is a comment
[tools]
node = "20"  # Inline comment

# Multi-line comment block
# explaining the configuration
[env]
API_URL = "http://localhost:3000"
```

## Best Practices

1. **Commit mise.toml**: Share tool versions and tasks with team
2. **Ignore mise.local.toml**: Personal overrides stay local
3. **Document Sections**: Add comments explaining configuration
4. **Group Related Items**: Organize by purpose
5. **Use Templates**: Leverage `{{ config_root }}` for paths
6. **Validate Regularly**: Run `mise doctor` to check configuration

## Example Complete Configuration

```toml
# Project: My Application
# Description: Web application with Node.js and PostgreSQL

# ==============================================================================
# Tool Versions
# ==============================================================================
[tools]
node = "20.10.0"
terraform = "1.6"
kubectl = "latest"

# ==============================================================================
# Environment Variables
# ==============================================================================
[env]
# Application
NODE_ENV = "development"
PORT = "3000"
LOG_LEVEL = "debug"

# Database
DATABASE_URL = "postgresql://localhost/myapp_dev"

# Paths
PROJECT_ROOT = "{{ config_root }}"
_.path = [
  "{{ config_root }}/bin",
  "{{ config_root }}/node_modules/.bin"
]

# Load additional environment files
_.file = [".env", ".env.local"]

# ==============================================================================
# Development Tasks
# ==============================================================================
[tasks.dev]
description = "Start development server"
run = "npm run dev"

[tasks.build]
description = "Build for production"
run = "npm run build"

[tasks.test]
description = "Run all tests"
depends = ["lint"]
run = [
  "npm run test:unit",
  "npm run test:integration"
]

[tasks.lint]
description = "Lint code"
run = "eslint ."

# ==============================================================================
# Deployment Tasks
# ==============================================================================
[tasks.deploy]
description = "Deploy to production"
depends = ["test", "build"]
run = "./scripts/deploy.sh"
env = { ENVIRONMENT = "production" }

# ==============================================================================
# Settings
# ==============================================================================
[settings]
experimental = true
verbose = false
jobs = 4
```

## Anti-Patterns

### ❌ Don't: Mix Configuration Concerns

```toml
[env]
NODE_ENV = "development"
[tools]
node = "20"
[env]
PORT = "3000"
```

### ✅ Do: Group Related Configuration

```toml
[tools]
node = "20"

[env]
NODE_ENV = "development"
PORT = "3000"
```

### ❌ Don't: Hardcode Paths

```toml
[env]
DATA_DIR = "/Users/john/projects/myapp/data"
```

### ✅ Do: Use Template Variables

```toml
[env]
DATA_DIR = "{{ config_root }}/data"
```

### ❌ Don't: Commit Secrets

```toml
[env]
API_SECRET = "super-secret-key"
```

### ✅ Do: Use .env.local

```toml
[env]
_.file = [".env", ".env.local"]
```
