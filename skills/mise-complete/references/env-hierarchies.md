# Environment Variable Hierarchies

## Configuration Hierarchy

Mise uses a hierarchical configuration system where more specific configurations override more general ones.

### Configuration Levels (Priority Order)

1. **Shell Environment** (highest priority)
2. **Local Config** (`mise.local.toml`)
3. **Project Config** (`mise.toml`)
4. **Parent Directories** (recursive search)
5. **Global Config** (`~/.config/mise/config.toml`)
6. **System Config** (`/etc/mise/config.toml`)

### Hierarchy Example

```toml
# /etc/mise/config.toml (system-wide)
[env]
EDITOR = "vi"
LOG_LEVEL = "info"

# ~/.config/mise/config.toml (user global)
[env]
EDITOR = "code"  # Overrides system
GIT_EDITOR = "code --wait"

# ~/projects/myapp/mise.toml (project)
[env]
LOG_LEVEL = "debug"  # Overrides global
API_URL = "https://api.example.com"

# ~/projects/myapp/mise.local.toml (local)
[env]
API_URL = "http://localhost:3000"  # Overrides project
DEBUG = "true"
```

**Result:**
- `EDITOR = "code"` (from global, overrides system)
- `GIT_EDITOR = "code --wait"` (from global)
- `LOG_LEVEL = "debug"` (from project, overrides global)
- `API_URL = "http://localhost:3000"` (from local, overrides project)
- `DEBUG = "true"` (from local)

## Global Configuration

### User-Level Global Config

Location: `~/.config/mise/config.toml`

```toml
# Global settings for all projects
[env]
EDITOR = "code"
BROWSER = "firefox"
GIT_EDITOR = "code --wait"
LANG = "en_US.UTF-8"

# Global PATH additions
[env]
_.path = ["$HOME/.local/bin", "$HOME/bin"]
```

Use for:
- Personal editor/tool preferences
- User-specific paths
- Common environment variables across all projects

### System-Level Global Config

Location: `/etc/mise/config.toml`

```toml
# System-wide settings (requires root)
[env]
CORPORATE_PROXY = "http://proxy.company.com:8080"
COMPANY_REGISTRY = "https://registry.company.com"
```

Use for:
- Organization-wide settings
- Corporate proxies
- Compliance requirements

## Project Configuration

### Main Project Config

Location: `<project>/mise.toml`

```toml
# Committed to version control
[tools]
node = "20"
python = "3.11"

[env]
NODE_ENV = "development"
API_URL = "https://api.example.com"
DATABASE_URL = "postgresql://localhost/myapp_dev"

[env]
_.file = [".env", ".env.local"]
```

Use for:
- Tool versions
- Project-specific variables
- Shared team configuration

### Local Project Override

Location: `<project>/mise.local.toml`

```toml
# Git-ignored, developer-specific
[env]
DATABASE_URL = "postgresql://localhost/myapp_johndoe"
DEBUG = "true"
LOG_LEVEL = "trace"
```

Use for:
- Developer-specific overrides
- Local database connections
- Debug settings
- Sensitive data

## Directory Traversal

Mise searches up the directory tree for configuration files:

```
/home/user/projects/myapp/services/api/
  ├── mise.toml (api service config)
  └── src/
      └── (working directory)

/home/user/projects/myapp/
  └── mise.toml (root project config)

/home/user/
  └── .config/mise/config.toml (global config)
```

Working in `/home/user/projects/myapp/services/api/src/`:

1. Loads `/home/user/.config/mise/config.toml` (global)
2. Loads `/home/user/projects/myapp/mise.toml` (root project)
3. Loads `/home/user/projects/myapp/services/api/mise.toml` (service)
4. Merges with later configs overriding earlier ones

## Multi-Environment Setup

### Environment-Specific Files

```toml
# mise.toml
[env]
NODE_ENV = "development"
_.file = ".env.{{ env.NODE_ENV }}"
```

```bash
# .env.development
API_URL=http://localhost:3000
DATABASE_URL=postgresql://localhost/myapp_dev
DEBUG=true

# .env.staging
API_URL=https://staging-api.example.com
DATABASE_URL=postgresql://staging-db/myapp
DEBUG=false

# .env.production
API_URL=https://api.example.com
DATABASE_URL=postgresql://prod-db/myapp
DEBUG=false
```

### Switching Environments

```bash
# Development (default)
cd project
echo $API_URL  # http://localhost:3000

# Staging
NODE_ENV=staging mise env
# or
echo 'NODE_ENV = "staging"' > mise.local.toml

# Production
NODE_ENV=production mise exec -- node app.js
```

## Workspace Configuration

### Monorepo Structure

```
myapp/
├── mise.toml                    # Root config
├── packages/
│   ├── api/
│   │   └── mise.toml           # API-specific config
│   ├── web/
│   │   └── mise.toml           # Web-specific config
│   └── shared/
│       └── mise.toml           # Shared config
```

### Root Configuration

```toml
# myapp/mise.toml
[tools]
node = "20"

[env]
PROJECT_ROOT = "{{ config_root }}"
NODE_ENV = "development"
```

### Package-Specific Configuration

```toml
# myapp/packages/api/mise.toml
[env]
API_PORT = "3000"
DATABASE_URL = "postgresql://localhost/myapp_api"
LOG_LEVEL = "debug"

# Inherits NODE_ENV and PROJECT_ROOT from root
```

```toml
# myapp/packages/web/mise.toml
[env]
WEB_PORT = "8080"
API_URL = "http://localhost:3000"

# Inherits NODE_ENV and PROJECT_ROOT from root
```

## Sensitive Data Handling

### Separating Public and Private Configuration

```toml
# mise.toml (committed)
[tools]
node = "20"

[env]
NODE_ENV = "development"
API_URL = "http://localhost:3000"

# Load secret files
[env]
_.file = [".env", ".env.local", ".env.secrets"]
```

```bash
# .env (committed)
NODE_ENV=development
PORT=3000

# .env.local (git-ignored)
DATABASE_URL=postgresql://localhost/myapp_johndoe

# .env.secrets (git-ignored)
API_SECRET=super-secret-key
JWT_SECRET=another-secret
```

### .gitignore Setup

```gitignore
# Ignore local overrides
mise.local.toml
.env.local
.env.secrets
*.secret
```

### Environment Variable Templates

```toml
# mise.toml (committed)
[env]
DATABASE_HOST = "localhost"
DATABASE_PORT = "5432"
DATABASE_NAME = "myapp_dev"
# DATABASE_PASSWORD loaded from .env.secrets
```

```bash
# .env.secrets (git-ignored)
DATABASE_PASSWORD=secret123
```

## Conditional Configuration

### Platform-Specific Settings

```toml
[env]
# Use different paths based on OS
JAVA_HOME = "{{ if eq .os 'darwin' }}/Library/Java/Home{{ else }}/usr/lib/jvm/default{{ end }}"
```

### Tool-Based Conditional Loading

```toml
[tools]
python = "3.11"

[env]
# Only set if Python is installed
PYTHON_PATH = "{{ config_root }}/.venv"
_.path = ["{{ config_root }}/.venv/bin"]
```

## Configuration Inspection

### View Merged Configuration

```bash
# Show final merged environment
mise env

# Show configuration sources
mise config ls

# Show configuration for specific directory
mise env -C /path/to/project
```

### Debug Configuration

```bash
# Check which files are loaded
mise doctor

# Validate configuration syntax
mise config validate

# Show configuration as JSON
mise config show --json
```

## Best Practices

1. **Global for Personal**: User preferences in `~/.config/mise/config.toml`
2. **Project for Team**: Shared settings in `mise.toml` (committed)
3. **Local for Individual**: Personal overrides in `mise.local.toml` (ignored)
4. **Separate Secrets**: Keep sensitive data in git-ignored files
5. **Document Hierarchy**: Comment which settings override others
6. **Environment Files**: Use `.env.example` to document required variables

## Common Patterns

### Three-Tier Setup

```toml
# ~/.config/mise/config.toml (personal preferences)
[env]
EDITOR = "code"

# project/mise.toml (team settings)
[env]
NODE_ENV = "development"
API_URL = "https://api.example.com"

# project/mise.local.toml (your overrides)
[env]
API_URL = "http://localhost:3000"
DEBUG = "true"
```

### Multi-Environment Pattern

```toml
# mise.toml
[env]
ENV = "development"
_.file = ".env.{{ env.ENV }}"

# Override environment
# mise.local.toml
[env]
ENV = "staging"  # Now loads .env.staging
```

## Anti-Patterns

### ❌ Don't: Commit Secrets in Any Config

```toml
# mise.toml - DON'T DO THIS
[env]
API_SECRET = "secret-key-123"
```

### ✅ Do: Use Git-Ignored Files

```toml
# mise.toml
[env]
_.file = ".env.secrets"

# .env.secrets (git-ignored)
API_SECRET=secret-key-123
```

### ❌ Don't: Duplicate Global Settings

```toml
# Don't repeat in every mise.toml
[env]
EDITOR = "code"
BROWSER = "firefox"
```

### ✅ Do: Use Global Config

```toml
# ~/.config/mise/config.toml
[env]
EDITOR = "code"
BROWSER = "firefox"
```

### ❌ Don't: Hardcode Environment Names

```toml
[env]
DATABASE_URL = "postgresql://localhost/myapp_dev"
```

### ✅ Do: Use Environment-Specific Files

```toml
[env]
_.file = ".env.{{ env.NODE_ENV }}"
```
