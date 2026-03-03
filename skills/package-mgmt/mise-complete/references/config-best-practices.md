# Configuration Best Practices

## Project Setup

### Initial Configuration

```toml
# mise.toml - Start simple and grow as needed
[tools]
node = "20"

[env]
NODE_ENV = "development"

[tasks.dev]
run = "npm run dev"
```

### Documentation

```toml
# ==============================================================================
# Project: My Application
# Description: Web application with Node.js backend
# Maintained by: Engineering Team
# ==============================================================================

[tools]
# Node.js 20 LTS for long-term stability
node = "20"

[env]
# Development environment by default
# Override in mise.local.toml for personal settings
NODE_ENV = "development"
```

### README Integration

```markdown
# My Application

## Setup

1. Install Mise: `curl https://mise.run | sh`
2. Install tools: `mise install`
3. Trust config: `mise trust`
4. Start development: `mise run dev`

## Configuration

- `mise.toml` - Project configuration (committed)
- `mise.local.toml` - Personal overrides (git-ignored)
- `.env.local` - Local secrets (git-ignored)
```

## Version Control

### What to Commit

```bash
# Commit these files
mise.toml          # Shared team configuration
.mise/tasks/*      # Shared tasks
.env.example       # Environment template
README.md          # Setup documentation
```

### What to Ignore

```gitignore
# .gitignore
mise.local.toml
.env.local
.env.secrets
*.backup
```

### Example .env.example

```bash
# .env.example - Copy to .env.local and fill in values

# Database
DATABASE_URL=postgresql://localhost/myapp
DATABASE_PASSWORD=

# API Keys
API_SECRET=
JWT_SECRET=

# External Services
STRIPE_API_KEY=
SENDGRID_API_KEY=
```

## Team Collaboration

### Onboarding New Developers

```bash
# Setup script (scripts/setup.sh)
#!/usr/bin/env bash
set -e

echo "Setting up development environment..."

# Check Mise installation
if ! command -v mise &> /dev/null; then
  echo "Installing Mise..."
  curl https://mise.run | sh
fi

# Install tools
echo "Installing tools..."
mise install

# Setup environment
if [ ! -f .env.local ]; then
  echo "Creating .env.local from template..."
  cp .env.example .env.local
  echo "Please edit .env.local with your settings"
fi

# Trust configuration
mise trust

echo "Setup complete! Run 'mise run dev' to start."
```

### Communication

```toml
# Document breaking changes in mise.toml
# ==============================================================================
# BREAKING CHANGE (2024-02-11): Updated Node.js to v20
# Run: mise install
# ==============================================================================
[tools]
node = "20"
```

## Configuration Organization

### Small Projects

```toml
# Keep it simple - everything in one file
[tools]
node = "20"

[env]
NODE_ENV = "development"

[tasks.dev]
run = "npm run dev"

[tasks.test]
run = "npm test"

[tasks.build]
run = "npm run build"
```

### Medium Projects

```toml
# Group by concern with clear sections
# ==============================================================================
# Tools
# ==============================================================================
[tools]
node = "20"
python = "3.11"

# ==============================================================================
# Environment
# ==============================================================================
[env]
NODE_ENV = "development"
_.path = ["./bin"]
_.file = [".env", ".env.local"]

# ==============================================================================
# Development Tasks
# ==============================================================================
[tasks.dev]
run = "npm run dev"

[tasks.test]
run = "npm test"

# ==============================================================================
# Deployment Tasks
# ==============================================================================
[tasks.deploy]
depends = ["test", "build"]
run = "./scripts/deploy.sh"
```

### Large Projects (Monorepo)

```
project/
├── mise.toml                  # Root: shared tools and environment
├── .mise/
│   └── tasks/                # Shared complex tasks
│       ├── ci-full
│       ├── release
│       └── deploy-all
├── packages/
│   ├── api/
│   │   ├── mise.toml         # API-specific configuration
│   │   └── .mise/tasks/      # API tasks
│   ├── web/
│   │   ├── mise.toml         # Web-specific configuration
│   │   └── .mise/tasks/      # Web tasks
│   └── shared/
│       └── mise.toml         # Shared library configuration
```

## Maintenance

### Regular Updates

```bash
# Update tools
mise upgrade

# Update plugins
mise plugins update

# Check for issues
mise doctor

# Audit configuration
mise config validate
```

### Version Pinning

```toml
# Pin exact versions in production
[tools]
node = "20.10.0"  # Not "20" or "latest"
python = "3.11.5"

# Use ranges in development
[tools]
node = "20"  # Latest 20.x
python = "3.11"  # Latest 3.11.x
```

### Deprecation Management

```toml
# Document deprecations
[tools]
# DEPRECATED: Migrating from Node 18 to 20
# Remove Node 18 after 2024-03-01
node = "20"

# TODO: Upgrade Python to 3.12
python = "3.11"
```

## Security Best Practices

### Secret Management

```toml
# NEVER commit secrets in mise.toml
[env]
_.file = [".env", ".env.secrets"]

# .env.secrets (git-ignored)
API_SECRET=actual-secret-here
```

### Trust System

```bash
# Only trust configurations you understand
mise trust

# Review before trusting
cat mise.toml

# List trusted configs
mise trust list
```

### Sensitive Data Patterns

```toml
# Use template variables for non-sensitive parts
[env]
DB_HOST = "localhost"
DB_PORT = "5432"
DB_NAME = "myapp"
# DB_PASSWORD loaded from .env.secrets

DATABASE_URL = "postgresql://postgres:{{ env.DB_PASSWORD }}@{{ env.DB_HOST }}:{{ env.DB_PORT }}/{{ env.DB_NAME }}"
```

## Performance Optimization

### Tool Installation

```toml
# Only install needed tools
[tools]
node = "20"  # Required

# python = "3.11"  # Commented out if not needed
```

### Task Optimization

```toml
# Parallel tasks for independent operations
[tasks.ci-fast]
depends = ["lint", "type-check", "test:unit"]  # Run in parallel

# Sequential tasks for dependent operations
[tasks.deploy]
run = [
  "mise run build",
  "mise run test",
  "./scripts/deploy.sh"
]
```

### Environment Loading

```toml
# Load only necessary files
[env]
_.file = [".env"]  # Not 10 different files

# Use computed values instead of duplicates
API_HOST = "localhost"
API_PORT = "3000"
API_URL = "http://{{ env.API_HOST }}:{{ env.API_PORT }}"
```

## Testing Configuration

### Configuration Tests

```bash
#!/usr/bin/env bash
# scripts/test-config.sh

set -e

echo "Testing Mise configuration..."

# Test tool installation
mise install

# Test environment loading
mise env > /dev/null

# Test task definitions
mise tasks ls > /dev/null

# Test specific tasks
mise run test --dry-run

echo "Configuration tests passed!"
```

### CI/CD Integration

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Mise
        run: curl https://mise.run | sh
      
      - name: Install tools
        run: mise install
      
      - name: Run tests
        run: mise run ci
```

## Documentation Standards

### Inline Documentation

```toml
# ==============================================================================
# Tools: Runtime versions for development and production
# ==============================================================================
[tools]
# Node.js 20 LTS - primary runtime
node = "20"

# Python 3.11 - for build scripts
python = "3.11"

# ==============================================================================
# Environment: Project configuration
# ==============================================================================
[env]
# Application environment (development, staging, production)
NODE_ENV = "development"

# API endpoint (override in mise.local.toml for local development)
API_URL = "https://api.example.com"
```

### Change Log

```toml
# ==============================================================================
# CHANGELOG
# ==============================================================================
# 2024-02-11: Updated Node.js from 18 to 20
# 2024-02-01: Added Python for build scripts
# 2024-01-15: Initial Mise configuration
# ==============================================================================
```

## Anti-Patterns to Avoid

### ❌ Configuration Smells

```toml
# Too many tools
[tools]
node = "20"
python = "3.11"
ruby = "3.2"
go = "1.21"
rust = "stable"
java = "17"
# Only install what you need!

# Hardcoded paths
[env]
DATA_DIR = "/Users/john/projects/myapp/data"

# Secrets in configuration
[env]
API_SECRET = "secret-key-123"

# Duplicate logic
[tasks.test-api]
run = "cd api && npm test"
[tasks.test-web]
run = "cd web && npm test"
```

### ✅ Better Approaches

```toml
# Only necessary tools
[tools]
node = "20"

# Template paths
[env]
DATA_DIR = "{{ config_root }}/data"

# Secrets in separate files
[env]
_.file = ".env.secrets"

# Reusable patterns
[tasks.test]
run = "npm test"
# Call from root: mise run -C api test
```

## Checklist for Good Configuration

- [ ] Tools are necessary and versions are specified
- [ ] No secrets committed to version control
- [ ] Environment variables are documented
- [ ] Tasks have clear descriptions
- [ ] mise.local.toml is in .gitignore
- [ ] .env.example exists for required variables
- [ ] Configuration is validated (`mise doctor`)
- [ ] README includes setup instructions
- [ ] Breaking changes are documented
- [ ] Configuration is tested in CI/CD
