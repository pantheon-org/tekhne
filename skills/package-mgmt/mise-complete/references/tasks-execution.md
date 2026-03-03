# Task Execution

## Running Tasks

### Basic Execution

```bash
# Run a single task
mise run build

# Run with short alias
mise r build

# List all available tasks
mise tasks ls
mise tasks
```

### Task Dependencies

Dependencies run automatically before the main task:

```toml
[tasks.deploy]
run = "./deploy.sh"
depends = ["build", "test"]
```

```bash
# Running 'deploy' will automatically run 'build' and 'test' first
mise run deploy
```

### Sequential vs Parallel Execution

**Sequential** (default for array of commands):

```toml
[tasks.ci]
run = [
  "mise run lint",    # Runs first
  "mise run test",    # Runs second
  "mise run build"    # Runs third
]
```

**Parallel** (for independent dependencies):

```toml
[tasks.ci]
depends = ["lint", "test"]  # Run in parallel
run = "mise run build"
```

### Task Arguments

```bash
# Pass arguments after --
mise run test -- --verbose --coverage

# Multiple arguments
mise run deploy -- --env production --region us-east-1
```

## File Watching

Watch files and re-run tasks on changes:

```bash
# Watch and re-run on file changes
mise watch build

# Watch specific files
mise watch -g "src/**/*.ts" build
```

Configuration:

```toml
[tasks.dev]
run = "npm run dev"
sources = ["src/**/*.ts", "src/**/*.tsx"]
outputs = ["dist/**/*"]
```

## Task Aliases

Define shortcuts for common tasks:

```toml
[tasks.test]
alias = "t"

[tasks.deploy-production]
alias = ["deploy:prod", "dp"]
```

```bash
mise run t          # Runs test
mise run deploy:prod # Runs deploy-production
```

## Environment Variables in Tasks

### Task-Specific Environment

```toml
[tasks.test]
run = "pytest"
env = { PYTEST_ARGS = "--verbose", DEBUG = "true" }
```

### Accessing Mise Variables

```toml
[tasks.deploy]
run = "echo Deploying from {{ config_root }}"
```

Built-in variables:
- `{{ config_root }}`: Directory containing mise.toml
- `{{ cwd }}`: Current working directory

## Monorepo Task Execution

### Workspace Tasks

```toml
# Root mise.toml
[tasks.build-all]
run = "mise run -C packages/api build && mise run -C packages/web build"

# Or use a script
[tasks.build-all]
run = '''
for pkg in packages/*; do
  mise run -C "$pkg" build
done
'''
```

### Per-Package Tasks

```bash
# Run task in specific package
mise run -C packages/api test

# Run task in all packages
for pkg in packages/*; do
  mise run -C "$pkg" lint
done
```

## Task Output

### Capturing Output

```bash
# Save output to file
mise run build > build.log 2>&1

# Pipe to other commands
mise run test | grep FAILED
```

### Silent Execution

```toml
[tasks.check]
run = "curl -sf https://api.example.com/health"
```

## Error Handling

### Exit Codes

Tasks return the exit code of the last command:

```toml
[tasks.ci]
run = [
  "mise run lint",   # If this fails, task stops
  "mise run test",
  "mise run build"
]
```

### Ignore Errors

```bash
#!/usr/bin/env bash
# mise description="Cleanup (ignore errors)"

rm -rf dist || true
rm -rf coverage || true
```

### Conditional Execution

```bash
#!/usr/bin/env bash
set -e  # Exit on error

mise run test && mise run deploy || echo "Deployment cancelled"
```

## Best Practices

1. **Check Task Status**: Use `mise tasks ls` to verify task registration
2. **Test Dependencies**: Ensure dependent tasks run in correct order
3. **Use Exit Codes**: Return proper exit codes for CI/CD integration
4. **Log Verbosity**: Use environment variables to control logging
5. **Dry Run**: Implement `--dry-run` flags for destructive tasks
6. **Timeouts**: Set reasonable timeouts for long-running tasks

## Common Patterns

### Interactive vs CI Mode

```bash
#!/usr/bin/env bash
# mise description="Run tests"

if [ -t 0 ]; then
  # Interactive mode
  pytest --verbose
else
  # CI mode
  pytest --quiet --junit-xml=results.xml
fi
```

### Retry Logic

```bash
#!/usr/bin/env bash
# mise description="Deploy with retry"

max_attempts=3
attempt=1

while [ $attempt -le $max_attempts ]; do
  if ./deploy.sh; then
    exit 0
  fi
  echo "Attempt $attempt failed, retrying..."
  ((attempt++))
  sleep 5
done

exit 1
```

### Progress Reporting

```bash
#!/usr/bin/env bash
# mise description="Multi-step build"

echo "Step 1/3: Linting..."
mise run lint

echo "Step 2/3: Testing..."
mise run test

echo "Step 3/3: Building..."
mise run build

echo "✓ Build complete"
```
