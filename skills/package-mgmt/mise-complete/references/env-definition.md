# Environment Variable Definition

## Overview

Mise manages environment variables through the `[env]` section in `mise.toml`, providing automatic loading when entering project directories.

## Basic Definition

```toml
[env]
NODE_ENV = "development"
DEBUG = "true"
API_URL = "http://localhost:3000"
DATABASE_URL = "postgresql://localhost/myapp_dev"
```

## Variable Types

### String Values

```toml
[env]
APP_NAME = "My Application"
VERSION = "1.0.0"
```

### Numeric Values

```toml
[env]
PORT = "3000"
MAX_CONNECTIONS = "100"
TIMEOUT_MS = "30000"
```

### Boolean Values

```toml
[env]
ENABLE_CACHE = "true"
DEBUG_MODE = "false"
PRODUCTION = "false"
```

### Multi-line Values

```toml
[env]
PRIVATE_KEY = """
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA...
-----END RSA PRIVATE KEY-----
"""
```

## Template Variables

Use template syntax to reference other variables and built-in values:

```toml
[env]
PROJECT_ROOT = "{{ config_root }}"
DATA_DIR = "{{ config_root }}/data"
LOG_FILE = "{{ config_root }}/logs/app.log"
```

### Built-in Template Variables

- `{{ config_root }}`: Directory containing `mise.toml`
- `{{ cwd }}`: Current working directory

### Variable Interpolation

```toml
[env]
API_URL = "http://localhost:3000"
WEB_URL = "http://localhost:8080"
CALLBACK_URL = "{{ env.API_URL }}/callback"
```

## PATH Management

### Adding to PATH

```toml
[env]
_.path = [
  "{{ config_root }}/bin",
  "{{ config_root }}/scripts",
  "{{ config_root }}/node_modules/.bin"
]
```

### PATH Order

Paths are prepended in the order listed:

```toml
[env]
_.path = [
  "./bin",           # Highest priority
  "./scripts",
  "$HOME/.local/bin" # Lowest priority
]
```

## Environment File Loading

### Loading from .env Files

```toml
[env]
_.file = ".env"
```

### Multiple Environment Files

```toml
[env]
_.file = [
  ".env",
  ".env.local",
  ".env.{{ env.NODE_ENV }}"
]
```

Files are loaded in order; later files override earlier ones.

### Conditional File Loading

```bash
# .env.development
DATABASE_URL=postgresql://localhost/myapp_dev
DEBUG=true

# .env.production
DATABASE_URL=postgresql://prod-server/myapp
DEBUG=false
```

```toml
[env]
NODE_ENV = "development"
_.file = ".env.{{ env.NODE_ENV }}"
```

## Tool-Specific Environments

### Python Virtual Environment

```toml
[tools]
python = "3.11"

[env]
VIRTUAL_ENV = "{{ config_root }}/.venv"
_.path = ["{{ config_root }}/.venv/bin"]
```

### Node.js Environment

```toml
[tools]
node = "20"

[env]
NODE_ENV = "development"
NODE_OPTIONS = "--max-old-space-size=4096"
_.path = ["{{ config_root }}/node_modules/.bin"]
```

### Go Environment

```toml
[tools]
go = "1.21"

[env]
GOPATH = "{{ config_root }}/.go"
GOBIN = "{{ config_root }}/.go/bin"
_.path = ["{{ config_root }}/.go/bin"]
```

### Rust Environment

```toml
[tools]
rust = "stable"

[env]
CARGO_HOME = "{{ config_root }}/.cargo"
RUSTUP_HOME = "{{ config_root }}/.rustup"
_.path = ["{{ config_root }}/.cargo/bin"]
```

## Environment Scopes

### Project-Level (mise.toml)

```toml
# mise.toml
[env]
PROJECT_NAME = "my-app"
API_URL = "http://localhost:3000"
```

### Global (~/.config/mise/config.toml)

```toml
# ~/.config/mise/config.toml
[env]
EDITOR = "code"
BROWSER = "firefox"
```

### Local Override (mise.local.toml)

```toml
# mise.local.toml (git-ignored)
[env]
DATABASE_URL = "postgresql://localhost/myapp_johndoe"
DEBUG = "true"
```

Priority: `mise.local.toml` > `mise.toml` > global config

## Sensitive Data Handling

### Separate Secret Files

```toml
# mise.toml (committed)
[env]
NODE_ENV = "development"
_.file = [".env", ".env.local"]

# .env (committed)
API_URL=http://localhost:3000
PORT=3000

# .env.local (git-ignored)
API_SECRET=super-secret-key
DATABASE_PASSWORD=secret123
```

### .gitignore Configuration

```gitignore
.env.local
mise.local.toml
*.secret
```

## Mise Environment Variables

### Built-in Variables

Automatically set by Mise:

- `MISE_CONFIG_ROOT`: Directory containing mise.toml
- `MISE_PROJECT_ROOT`: Project root directory
- `MISE_DATA_DIR`: Mise data directory (~/.local/share/mise)
- `MISE_CACHE_DIR`: Mise cache directory
- `MISE_INSTALL_PATH`: Installation path for tools

### Using Built-in Variables

```toml
[env]
APP_ROOT = "{{ env.MISE_PROJECT_ROOT }}"
CACHE_DIR = "{{ env.MISE_CACHE_DIR }}/my-app"
```

## Common Patterns

### Database Configuration

```toml
[env]
DB_HOST = "localhost"
DB_PORT = "5432"
DB_NAME = "myapp_dev"
DB_USER = "postgres"
DATABASE_URL = "postgresql://{{ env.DB_USER }}@{{ env.DB_HOST }}:{{ env.DB_PORT }}/{{ env.DB_NAME }}"
```

### API Configuration

```toml
[env]
API_HOST = "localhost"
API_PORT = "3000"
API_PROTOCOL = "http"
API_URL = "{{ env.API_PROTOCOL }}://{{ env.API_HOST }}:{{ env.API_PORT }}"
```

### Feature Flags

```toml
[env]
FEATURE_NEW_UI = "true"
FEATURE_BETA_API = "false"
FEATURE_ANALYTICS = "true"
```

## Best Practices

1. **Separate Public and Private**: Commit public config, ignore secrets
2. **Use Descriptive Names**: `DATABASE_URL` not `DB_URL`
3. **Document Variables**: Add comments explaining purpose
4. **Use Templates**: Leverage `{{ config_root }}` for paths
5. **Environment-Specific Files**: Use `.env.development`, `.env.production`
6. **Version Control**: Commit `.env.example`, ignore `.env.local`

## Anti-Patterns

### ❌ Don't: Commit Secrets

```toml
[env]
API_SECRET = "my-secret-key-123"  # DON'T DO THIS
```

### ✅ Do: Use .env.local

```toml
# mise.toml
[env]
_.file = [".env", ".env.local"]

# .env.local (git-ignored)
API_SECRET=my-secret-key-123
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

### ❌ Don't: Duplicate Global Config

```toml
# In every project's mise.toml
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
