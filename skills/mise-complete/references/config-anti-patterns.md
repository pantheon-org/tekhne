# Configuration Anti-Patterns

## Common Mistakes

### Committing Secrets

**❌ NEVER do this:**

```toml
# mise.toml (committed to git)
[env]
API_SECRET = "sk_live_abc123xyz"
DATABASE_PASSWORD = "super_secret_password"
JWT_SECRET = "my-secret-key"
AWS_SECRET_ACCESS_KEY = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
```

**✅ Instead:**

```toml
# mise.toml (committed)
[env]
_.file = [".env", ".env.secrets"]

# .env.secrets (git-ignored)
API_SECRET=sk_live_abc123xyz
DATABASE_PASSWORD=super_secret_password
JWT_SECRET=my-secret-key
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY

# .gitignore
.env.secrets
mise.local.toml
```

### Hardcoding Paths

**❌ Don't:**

```toml
[env]
PROJECT_DIR = "/Users/john/projects/myapp"
DATA_DIR = "/Users/john/projects/myapp/data"
LOG_FILE = "/Users/john/projects/myapp/logs/app.log"
```

**✅ Do:**

```toml
[env]
PROJECT_DIR = "{{ config_root }}"
DATA_DIR = "{{ config_root }}/data"
LOG_FILE = "{{ config_root }}/logs/app.log"
```

### Duplicate Global Configuration

**❌ Don't repeat in every project:**

```toml
# project-1/mise.toml
[env]
EDITOR = "code"
BROWSER = "firefox"
GIT_EDITOR = "code --wait"

# project-2/mise.toml
[env]
EDITOR = "code"
BROWSER = "firefox"
GIT_EDITOR = "code --wait"
```

**✅ Use global config:**

```toml
# ~/.config/mise/config.toml
[env]
EDITOR = "code"
BROWSER = "firefox"
GIT_EDITOR = "code --wait"
```

### Mixing Version Managers

**❌ Don't mix tools:**

```bash
# .bashrc
eval "$(mise activate bash)"
eval "$(nvm load)"
eval "$(pyenv init -)"
eval "$(rbenv init -)"
```

**✅ Use Mise exclusively:**

```bash
# .bashrc
eval "$(mise activate bash)"

# mise.toml
[tools]
node = "20"
python = "3.11"
ruby = "3.2"
```

### Ignoring Exit Codes

**❌ Don't:**

```toml
[tasks.ci]
run = "npm run lint; npm test; npm run build"
# If lint fails, test and build still run
```

**✅ Do:**

```toml
[tasks.ci]
run = [
  "npm run lint",
  "npm test",
  "npm run build"
]
# Stops on first failure
```

### Hardcoding Environment Names

**❌ Don't:**

```toml
[tasks.deploy-dev]
run = "deploy.sh --env development"

[tasks.deploy-staging]
run = "deploy.sh --env staging"

[tasks.deploy-prod]
run = "deploy.sh --env production"
```

**✅ Do:**

```toml
[env]
ENVIRONMENT = "development"

[tasks.deploy]
run = "deploy.sh --env {{ env.ENVIRONMENT }}"
```

### Task Logic Duplication

**❌ Don't:**

```toml
[tasks.test-api]
run = "cd packages/api && npm test"

[tasks.test-web]
run = "cd packages/web && npm test"
```

**✅ Do:**

```toml
# Root mise.toml
[tasks.test-all]
run = '''
for pkg in packages/*; do
  mise run -C "$pkg" test
done
'''

# packages/*/mise.toml
[tasks.test]
run = "npm test"
```

### Over-Installing Tools

**❌ Don't install everything:**

```toml
[tools]
node = "20"
python = "3.11"
ruby = "3.2"
go = "1.21"
rust = "stable"
java = "17"
php = "8.2"
dotnet = "8"
```

**✅ Install only what you need:**

```toml
[tools]
node = "20"
```

### Missing Descriptions

**❌ Don't:**

```toml
[tasks.x]
run = "./scripts/complex-task.sh"
```

**✅ Do:**

```toml
[tasks.deploy]
description = "Deploy application to production"
run = "./scripts/deploy.sh"
```

### Unorganized Configuration

**❌ Don't:**

```toml
[tasks.build]
run = "npm run build"
[env]
NODE_ENV = "development"
[tools]
node = "20"
```

**✅ Do:**

```toml
[tools]
node = "20"

[env]
NODE_ENV = "development"

[tasks.build]
run = "npm run build"
```

## Configuration Smells

### Warning Signs

1. Secrets in mise.toml
2. Hardcoded paths
3. Too many tools
4. Missing descriptions
5. No .gitignore
6. Duplicate configuration
7. Complex inline tasks
8. No validation

### Quick Audit

```bash
# Check for secrets
grep -i "secret\|password\|key" mise.toml

# Check for hardcoded paths
grep "/Users\|/home" mise.toml

# Validate configuration
mise doctor
```

## Recovery

### Cleanup Checklist

- [ ] Remove secrets from git history
- [ ] Add .gitignore entries
- [ ] Create .env.example
- [ ] Move secrets to .env.secrets
- [ ] Replace hardcoded paths
- [ ] Add task descriptions
- [ ] Pin tool versions
- [ ] Run `mise doctor`

### Pre-commit Hook

```bash
#!/usr/bin/env bash
# .git/hooks/pre-commit

if git diff --cached mise.toml | grep -iE "secret|password|key.*="; then
  echo "ERROR: Possible secret in mise.toml"
  exit 1
fi
```
