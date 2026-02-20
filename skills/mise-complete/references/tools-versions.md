# Tool Versions File Format

Understanding and using `.tool-versions` files with Mise.

## File Format

### Basic Format

```
# .tool-versions
nodejs 20.10.0
python 3.12.0
rust 1.75.0
terraform 1.6.0
```

### Version Specifications

```
# Exact version
nodejs 20.10.0

# Latest patch version
nodejs 20.10

# Latest minor version
nodejs 20

# Latest version
nodejs latest
```

### Comments

```
# Comments start with #
nodejs 20.10.0  # End-of-line comments also supported
```

## Compatibility with asdf

Mise reads `.tool-versions` files for backward compatibility with asdf:

```bash
# Existing .tool-versions file
cat .tool-versions
# nodejs 20.10.0
# python 3.12.0

# Mise automatically uses it
mise install
mise current
```

## Migration from .tool-versions to mise.toml

### Simple Migration

```bash
# Existing .tool-versions
cat .tool-versions
# nodejs 20.10.0
# python 3.12.0

# Migrate to mise.toml
mise use node@20.10.0 python@3.12.0

# This creates mise.toml:
# [tools]
# node = "20.10.0"
# python = "3.12.0"
```

### Using Both Files

```toml
# mise.toml can coexist with .tool-versions
# Mise prioritizes mise.toml over .tool-versions

# .tool-versions
nodejs 20.10.0

# mise.toml (takes precedence)
[tools]
node = "20.11.0"  # This version is used
```

## Legacy Version Files

Mise also reads legacy version files from other version managers:

```bash
# .nvmrc for nvm
cat .nvmrc
# 20.10.0

# .ruby-version for rbenv
cat .ruby-version
# 3.3.0

# .python-version for pyenv
cat .python-version
# 3.12.0
```

## Best Practices

### Use mise.toml for New Projects

```toml
# Prefer mise.toml over .tool-versions
[tools]
node = "20.10.0"
python = "3.12.0"
```

Benefits:
- More flexible configuration options
- Environment variables in same file
- Task definitions in same file
- Tool-specific settings

### Keep .tool-versions for Compatibility

Keep `.tool-versions` if your team uses asdf:

```
# .tool-versions - for asdf users
nodejs 20.10.0
python 3.12.0
```

```toml
# mise.toml - for mise users
[tools]
node = "20.10.0"
python = "3.12.0"
```

### Document Version Choices

```
# .tool-versions
# Node.js 20 LTS for production compatibility
nodejs 20.10.0

# Python 3.12 for async improvements
python 3.12.0

# Terraform 1.6 for HCL2 features
terraform 1.6.4
```

## Anti-Patterns

### Don't Mix Formats Carelessly

```bash
# Bad: Conflicting versions
# .tool-versions
nodejs 20.10.0

# mise.toml
[tools]
node = "18.19.0"  # Confusion!
```

### Don't Use Vague Versions in CI

```
# Bad: Unpredictable CI builds
nodejs latest
python 3

# Good: Explicit versions
nodejs 20.10.0
python 3.12.1
```
