# Migration from Other Version Managers

Migrating to Mise from asdf, nvm, rbenv, pyenv, and other version managers.

## From asdf

### Reading Existing Configuration

```bash
# Mise automatically reads .tool-versions
cat .tool-versions
# nodejs 20.10.0
# python 3.12.0

# Install and use
mise install
mise current
```

### Full Migration

```bash
# 1. Install mise
# (see mise documentation)

# 2. Migrate configuration
mise use node@20.10.0 python@3.12.0

# 3. Remove asdf (optional)
rm -rf ~/.asdf
# Remove asdf from shell config
```

### Plugin Compatibility

Most asdf plugins work with mise:

```bash
# List asdf plugins
asdf plugin list

# Install same plugins in mise
mise plugins install postgres
mise use postgres@16.1
```

## From nvm (Node Version Manager)

### Reading .nvmrc Files

```bash
# Existing .nvmrc
cat .nvmrc
# 20.10.0

# Migrate to mise
mise use node@$(cat .nvmrc)
```

### Migration Steps

```bash
# 1. Note current nvm version
nvm current
# v20.10.0

# 2. Install same version with mise
mise use node@20.10.0

# 3. Remove nvm (optional)
rm -rf ~/.nvm
# Remove nvm from shell config
```

### Global Package Migration

```bash
# Export nvm global packages
npm list -g --depth=0 > npm-global.txt

# After switching to mise
mise use node@20.10.0
# Reinstall global packages
```

## From rbenv (Ruby Version Manager)

### Reading .ruby-version Files

```bash
# Existing .ruby-version
cat .ruby-version
# 3.3.0

# Migrate to mise
mise use ruby@$(cat .ruby-version)
```

### Migration Steps

```bash
# 1. Note current rbenv version
rbenv version
# 3.3.0

# 2. Install same version with mise
mise use ruby@3.3.0

# 3. Remove rbenv (optional)
rm -rf ~/.rbenv
# Remove rbenv from shell config
```

## From pyenv (Python Version Manager)

### Reading .python-version Files

```bash
# Existing .python-version
cat .python-version
# 3.12.0

# Migrate to mise
mise use python@$(cat .python-version)
```

### Migration Steps

```bash
# 1. Note current pyenv version
pyenv version
# 3.12.0

# 2. Install same version with mise
mise use python@3.12.0

# 3. Preserve virtual environments
# Mise can use existing venvs
[tools]
python = "3.12"

[env]
_.python.venv = { path = ".venv", create = true }

# 4. Remove pyenv (optional)
rm -rf ~/.pyenv
# Remove pyenv from shell config
```

## From sdkman (Java/JVM Version Manager)

### Migration Steps

```bash
# 1. Note current SDK versions
sdk current

# 2. Install with mise
mise use java@21 gradle@8.5 maven@3.9

# 3. Remove sdkman (optional)
rm -rf ~/.sdkman
# Remove sdkman from shell config
```

## From direnv

### Environment Variable Migration

```bash
# Old .envrc
export DATABASE_URL=postgresql://localhost/myapp
export NODE_ENV=development
```

```toml
# New mise.toml
[env]
DATABASE_URL = "postgresql://localhost/myapp"
NODE_ENV = "development"
```

### Migration Steps

```bash
# 1. Convert .envrc to mise.toml
# (manual conversion of export statements)

# 2. Mise automatically activates on directory change

# 3. Remove .envrc (optional)
rm .envrc

# 4. Keep direnv for other projects (optional)
# Mise and direnv can coexist
```

## Migration Checklist

### Pre-Migration

- [ ] List current tool versions
- [ ] Export global packages/gems/modules
- [ ] Document custom configurations
- [ ] Backup shell configuration files

### During Migration

- [ ] Install mise
- [ ] Configure shell activation
- [ ] Migrate tool versions to mise.toml
- [ ] Test tool availability
- [ ] Reinstall global packages

### Post-Migration

- [ ] Remove old version managers (optional)
- [ ] Clean up shell configuration
- [ ] Update CI/CD pipelines
- [ ] Document changes for team

## Multi-Tool Migration Example

```bash
# Before: Multiple version managers
nvm use 20.10.0
rbenv local 3.3.0
pyenv local 3.12.0

# After: Single mise configuration
```

```toml
# mise.toml
[tools]
node = "20.10.0"
ruby = "3.3.0"
python = "3.12.0"
```

## Team Migration Strategy

### Gradual Migration

```toml
# Phase 1: Add mise.toml alongside existing files
# Keep .nvmrc, .ruby-version, .python-version

[tools]
node = "20.10.0"
ruby = "3.3.0"
python = "3.12.0"

# Phase 2: Team adopts mise
# (Keep legacy files for stragglers)

# Phase 3: Remove legacy files
# Once entire team migrated
```

### CI/CD Migration

```yaml
# Before: Multiple version managers in CI
- uses: actions/setup-node@v3
  with:
    node-version: '20.10.0'
- uses: ruby/setup-ruby@v1
  with:
    ruby-version: '3.3.0'
```

```yaml
# After: Single mise setup
- uses: jdx/mise-action@v2
# Automatically uses versions from mise.toml
```

## Troubleshooting Migration

### Shell Not Activating

```bash
# Ensure mise activation in shell config
# ~/.bashrc or ~/.zshrc
eval "$(mise activate bash)"  # or zsh
```

### Tool Not Found After Migration

```bash
# Verify installation
mise current

# Reinstall if needed
mise install
```

### Global Packages Missing

```bash
# Node.js
npm install -g package-name

# Python
pip install package-name

# Ruby
gem install gem-name
```

### Version Conflicts

```bash
# Check which version manager is active
which node
# Should point to mise shim

# If conflicts exist
mise doctor
```

## Best Practices

### Keep Legacy Files During Transition

```bash
# Support both mise and nvm during migration
.nvmrc          # For nvm users
mise.toml       # For mise users
```

### Document Migration for Team

Create MIGRATION.md:

```markdown
# Migration to Mise

## Installation
1. Install mise: https://mise.jdx.dev/getting-started.html
2. Add to shell: `eval "$(mise activate bash)"`
3. Install tools: `mise install`

## Support
Contact #engineering-tools for help
```

### Test Before Removing Old Managers

```bash
# 1. Install mise and test
mise install
mise current

# 2. Verify application works
npm test
npm run build

# 3. Only then remove old version managers
```
