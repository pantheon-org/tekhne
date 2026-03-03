# Environment Variable Loading

## Activation Methods

### Automatic Shell Integration

Enable automatic environment loading when entering directories:

```bash
# Add to ~/.bashrc, ~/.zshrc, etc.
eval "$(mise activate bash)"  # or zsh, fish
```

When you `cd` into a directory with `mise.toml`, environment variables are automatically loaded.

### Manual Activation

```bash
# Load environment in current shell
eval "$(mise env)"

# Load environment for specific shell
eval "$(mise env -s zsh)"
```

### Running Commands with Environment

```bash
# Run command with mise environment
mise exec -- npm start

# Short form
mise x -- node app.js

# Run task (automatically includes environment)
mise run dev
```

### Checking Current Environment

```bash
# Show all mise-managed environment variables
mise env

# Show specific variable
mise env | grep NODE_ENV

# Show environment for specific directory
mise env -C /path/to/project
```

## Loading Order

Environment variables are loaded in this order (later overrides earlier):

1. Global config (`~/.config/mise/config.toml`)
2. Project config (`mise.toml`)
3. Local config (`mise.local.toml`)
4. Environment files (`.env`, `.env.local`)
5. Shell environment (existing variables)

### Example Loading Sequence

```toml
# ~/.config/mise/config.toml
[env]
EDITOR = "vim"
NODE_ENV = "development"

# project/mise.toml
[env]
NODE_ENV = "production"  # Overrides global
API_URL = "https://api.example.com"

# project/mise.local.toml
[env]
API_URL = "http://localhost:3000"  # Overrides project
DEBUG = "true"
```

Result:
- `EDITOR = "vim"` (from global)
- `NODE_ENV = "production"` (from project, overrides global)
- `API_URL = "http://localhost:3000"` (from local, overrides project)
- `DEBUG = "true"` (from local)

## Environment File Loading

### Basic .env Loading

```toml
[env]
_.file = ".env"
```

```bash
# .env
NODE_ENV=development
API_URL=http://localhost:3000
DATABASE_URL=postgresql://localhost/myapp
```

### Multiple File Loading

```toml
[env]
_.file = [
  ".env",              # Base configuration
  ".env.local",        # Local overrides
  ".env.{{ env.NODE_ENV }}"  # Environment-specific
]
```

Load order: `.env` → `.env.local` → `.env.development`

### Conditional Loading

```toml
[env]
NODE_ENV = "development"

# This will load .env.development
_.file = ".env.{{ env.NODE_ENV }}"
```

### Optional File Loading

```bash
# Mise ignores missing files by default
# These won't error if files don't exist
[env]
_.file = [".env", ".env.local"]
```

## Tool Environment Activation

### Python Virtual Environments

```toml
[tools]
python = "3.11"

[env]
VIRTUAL_ENV = "{{ config_root }}/.venv"
_.path = ["{{ config_root }}/.venv/bin"]
```

```bash
# Automatically activates venv when entering directory
cd my-project
which python  # Points to .venv/bin/python
```

### Node.js Environment

```toml
[tools]
node = "20"

[env]
NODE_ENV = "development"
_.path = ["{{ config_root }}/node_modules/.bin"]
```

```bash
cd my-project
which eslint  # Points to node_modules/.bin/eslint
```

### Go Environment

```toml
[tools]
go = "1.21"

[env]
GOPATH = "{{ config_root }}/.go"
_.path = ["{{ config_root }}/.go/bin"]
```

### Rust Environment

```toml
[tools]
rust = "stable"

[env]
CARGO_HOME = "{{ config_root }}/.cargo"
_.path = ["{{ config_root }}/.cargo/bin"]
```

## Replacing direnv

Mise can fully replace direnv with more features:

### direnv .envrc

```bash
# Old .envrc
export NODE_ENV=development
export API_URL=http://localhost:3000
PATH_add ./node_modules/.bin
```

### Mise Equivalent

```toml
# mise.toml
[env]
NODE_ENV = "development"
API_URL = "http://localhost:3000"
_.path = ["./node_modules/.bin"]
```

### Migration from direnv

1. Remove `.envrc` files
2. Create `mise.toml` with equivalent configuration
3. Replace `direnv allow` with automatic mise activation
4. Use `mise trust` if needed for project config

## Debugging Environment Loading

### Check Loaded Variables

```bash
# Show all mise environment variables
mise env

# Show as export statements
mise env -s bash

# Check specific variable
mise env | grep DATABASE_URL
```

### Verify Loading Order

```bash
# Show which config files are loaded
mise config ls

# Show environment from specific config
mise env -C /path/to/project
```

### Troubleshooting

```bash
# Check for syntax errors in mise.toml
mise doctor

# Validate environment configuration
mise env --json

# See which files would be loaded
mise config ls
```

## Performance Optimization

### Lazy Loading

Environment variables are loaded lazily - only when needed:

```bash
# Fast - doesn't load environment
cd project

# Loads environment only when needed
npm start  # Environment loaded here
```

### Caching

Mise caches environment calculations:

```bash
# First load - slower
cd project && mise env

# Subsequent loads - faster (cached)
cd project && mise env
```

### Clear Cache

```bash
# Clear environment cache
mise cache clear
```

## Shell-Specific Features

### Bash/Zsh Completion

```bash
# Enable completion for environment variables
eval "$(mise activate bash)"

# Tab completion works for mise-managed vars
echo $NODE_<TAB>
```

### Fish Shell

```fish
# ~/.config/fish/config.fish
mise activate fish | source
```

### Nushell

```nu
# ~/.config/nushell/config.nu
mise activate nu
```

## Common Patterns

### Development vs Production

```toml
[env]
NODE_ENV = "development"
_.file = ".env.{{ env.NODE_ENV }}"

# Loads .env.development in dev
# Loads .env.production in prod
```

### CI/CD Integration

```bash
# In CI pipeline
mise install  # Install tools
mise exec -- npm test  # Run with environment
```

### Docker Integration

```dockerfile
# Dockerfile
FROM node:20
WORKDIR /app

# Install mise
RUN curl https://mise.run | sh
ENV PATH="/root/.local/bin:$PATH"

# Copy config and install tools
COPY mise.toml .
RUN mise install

# Use mise to run application
CMD ["mise", "exec", "--", "npm", "start"]
```

## Best Practices

1. **Use Shell Integration**: Enable `mise activate` for automatic loading
2. **Prefer mise exec**: Use `mise exec` over manual env loading
3. **Check Variables**: Use `mise env` to verify loaded variables
4. **Cache Awareness**: Clear cache if environment seems stale
5. **Trust Projects**: Use `mise trust` for known projects

## Anti-Patterns

### ❌ Don't: Mix Environment Managers

```bash
# Don't mix direnv and mise
eval "$(direnv hook bash)"
eval "$(mise activate bash)"
```

### ✅ Do: Use One Manager

```bash
# Use mise exclusively
eval "$(mise activate bash)"
```

### ❌ Don't: Override Mise Variables

```bash
# Don't override mise-managed variables
export NODE_ENV=production  # Conflicts with mise
```

### ✅ Do: Configure in mise.toml

```toml
[env]
NODE_ENV = "production"
```

### ❌ Don't: Source .env Manually

```bash
# Don't manually source files
source .env
```

### ✅ Do: Let Mise Handle It

```toml
[env]
_.file = ".env"
```
