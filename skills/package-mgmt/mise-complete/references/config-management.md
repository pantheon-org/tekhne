# Configuration Management

## Configuration Commands

### Viewing Configuration

```bash
# Show current configuration
mise config show

# Show as JSON
mise config show --json

# List all configuration files
mise config ls

# Show configuration from specific directory
mise config show -C /path/to/project
```

### Validating Configuration

```bash
# Validate all configuration files
mise doctor

# Check specific config file
mise config validate mise.toml

# Debug configuration issues
mise config show --debug
```

### Generating Configuration

```bash
# Generate mise.toml from existing version files
mise config generate

# Generate from .tool-versions
mise config generate .tool-versions > mise.toml

# Generate from .node-version, .python-version, etc.
mise config generate --all > mise.toml
```

## Configuration Editing

### Manual Editing

```bash
# Edit project config
$EDITOR mise.toml

# Edit global config
$EDITOR ~/.config/mise/config.toml

# Edit local overrides
$EDITOR mise.local.toml
```

### Programmatic Updates

```bash
# Add a tool
echo 'node = "20"' >> mise.toml

# Update environment variable
mise set NODE_ENV=production

# Remove tool
mise unset tools.node
```

## Configuration Templates

### Starter Template

```toml
# mise.toml
[tools]
# Add your tools here
# node = "20"
# python = "3.11"

[env]
# Add your environment variables here
# NODE_ENV = "development"

[tasks.dev]
description = "Start development"
run = "echo 'Add your dev command here'"

[settings]
experimental = false
```

### Full Template

```toml
# ==============================================================================
# Tools: Define runtime versions
# ==============================================================================
[tools]
node = "20"
python = "3.11"

# ==============================================================================
# Environment: Project environment variables
# ==============================================================================
[env]
NODE_ENV = "development"
PROJECT_ROOT = "{{ config_root }}"

# PATH additions
_.path = ["{{ config_root }}/bin"]

# Load environment files
_.file = [".env", ".env.local"]

# ==============================================================================
# Tasks: Runnable commands
# ==============================================================================
[tasks.dev]
description = "Start development server"
run = "npm run dev"

[tasks.build]
description = "Build for production"
run = "npm run build"

[tasks.test]
description = "Run tests"
run = "npm test"

# ==============================================================================
# Settings: Mise behavior configuration
# ==============================================================================
[settings]
experimental = false
verbose = false
```

## Configuration Migration

### From asdf

```bash
# Convert .tool-versions to mise.toml
mise config generate .tool-versions > mise.toml

# Or migrate directly
mise install  # Mise reads .tool-versions automatically
```

**Manual migration:**

```bash
# .tool-versions
node 20.10.0
python 3.11.0
```

```toml
# mise.toml
[tools]
node = "20.10.0"
python = "3.11.0"
```

### From direnv

```bash
# Old .envrc
export NODE_ENV=development
export API_URL=http://localhost:3000
PATH_add ./node_modules/.bin
```

```toml
# New mise.toml
[env]
NODE_ENV = "development"
API_URL = "http://localhost:3000"
_.path = ["./node_modules/.bin"]
```

### From nvm/pyenv/rbenv

```bash
# .nvmrc
20.10.0

# .python-version
3.11.0

# .ruby-version
3.2.0
```

```toml
# mise.toml
[tools]
node = "20.10.0"
python = "3.11.0"
ruby = "3.2.0"
```

## Configuration Backup

### Backup Strategy

```bash
# Backup mise configuration
cp mise.toml mise.toml.backup

# Backup global config
cp ~/.config/mise/config.toml ~/.config/mise/config.toml.backup

# Backup entire mise directory
tar -czf mise-backup.tar.gz ~/.config/mise ~/.local/share/mise
```

### Version Control

```bash
# .gitignore
mise.local.toml
.env.local
*.backup

# Commit mise.toml
git add mise.toml
git commit -m "Add mise configuration"
```

## Configuration Sharing

### Team Configuration

```toml
# mise.toml (shared with team)
[tools]
node = "20"
python = "3.11"

[env]
NODE_ENV = "development"

[tasks.dev]
run = "npm run dev"
```

**Setup for new team members:**

```bash
git clone repo
cd repo
mise install  # Installs tools
mise trust    # Trust the configuration
mise run dev  # Start development
```

### Personal Overrides

```toml
# mise.local.toml (personal, git-ignored)
[env]
DATABASE_URL = "postgresql://localhost/myapp_johndoe"
DEBUG = "true"

[settings]
verbose = true
```

## Configuration Troubleshooting

### Common Issues

**Configuration not loading:**

```bash
# Check which configs are being used
mise config ls

# Verify syntax
mise config validate

# Check environment
mise env
```

**Tool versions not applying:**

```bash
# Check tool installation
mise list

# Verify configuration
mise config show

# Reinstall tools
mise install
```

**Environment variables not set:**

```bash
# Check environment loading
mise env

# Verify activation
eval "$(mise activate bash)"

# Debug environment
mise env --debug
```

### Debugging Commands

```bash
# Full system check
mise doctor

# Verbose output
mise --verbose config show

# Debug mode
mise --debug config show

# Trace execution
mise --trace config show
```

## Configuration Security

### Sensitive Data

```toml
# mise.toml (committed)
[env]
_.file = [".env", ".env.secrets"]

# .env.secrets (git-ignored)
API_SECRET=secret-key
DATABASE_PASSWORD=password
```

### Trust System

```bash
# Trust project configuration
mise trust

# Trust specific config file
mise trust mise.toml

# List trusted configs
mise trust list

# Revoke trust
mise trust revoke
```

### .gitignore Setup

```gitignore
# Mise local configuration
mise.local.toml

# Environment files
.env.local
.env.secrets
*.secret

# Backup files
*.backup
mise.toml.backup
```

## Configuration Best Practices

1. **Version Control mise.toml**: Share tool versions with team
2. **Ignore mise.local.toml**: Keep personal settings private
3. **Document Variables**: Comment configuration sections
4. **Validate Regularly**: Run `mise doctor` frequently
5. **Backup Before Changes**: Keep backups of working configs
6. **Use Templates**: Start with starter templates
7. **Separate Secrets**: Never commit sensitive data
8. **Trust Carefully**: Only trust configurations you understand

## Configuration Organization

### Small Projects

```
project/
├── mise.toml            # All configuration
├── mise.local.toml      # Local overrides (git-ignored)
└── .env.local           # Secrets (git-ignored)
```

### Medium Projects

```
project/
├── mise.toml            # Tools and environment
├── .mise/
│   └── tasks/           # Complex tasks
│       ├── deploy
│       ├── test
│       └── build
├── mise.local.toml      # Local overrides
└── .env.local           # Secrets
```

### Large Projects (Monorepo)

```
project/
├── mise.toml            # Root configuration
├── .mise/
│   ├── config.toml     # Additional config
│   └── tasks/          # Shared tasks
├── packages/
│   ├── api/
│   │   └── mise.toml   # API-specific config
│   └── web/
│       └── mise.toml   # Web-specific config
└── mise.local.toml     # Root overrides
```

## Configuration Anti-Patterns

### ❌ Don't: Commit Secrets

```toml
[env]
API_SECRET = "secret-key"
```

### ✅ Do: Use .env.local

```toml
[env]
_.file = ".env.local"
```

### ❌ Don't: Duplicate Configuration

```toml
# In every package's mise.toml
[tools]
node = "20"
python = "3.11"
```

### ✅ Do: Use Root Configuration

```toml
# Root mise.toml
[tools]
node = "20"
python = "3.11"

# Packages inherit from root
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

### ❌ Don't: Mix Concerns

```toml
[env]
NODE_ENV = "development"
[tools]
node = "20"
[env]
PORT = "3000"
```

### ✅ Do: Group Related Items

```toml
[tools]
node = "20"

[env]
NODE_ENV = "development"
PORT = "3000"
```
