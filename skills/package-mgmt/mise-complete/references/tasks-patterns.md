# Task Patterns

## Common Task Patterns

### Pre/Post Hooks

Execute tasks before and after main tasks:

```toml
[tasks.deploy]
depends = ["pre-deploy"]
run = "./scripts/deploy.sh"

[tasks.pre-deploy]
run = [
  "mise run test",
  "mise run build",
  "mise run security-check"
]

[tasks.post-deploy]
run = [
  "./scripts/notify-team.sh",
  "./scripts/smoke-test.sh"
]
```

```bash
mise run pre-deploy && mise run deploy && mise run post-deploy
```

### Conditional Execution

```bash
#!/usr/bin/env bash
# mise description="Conditional deployment"

if [ "$ENVIRONMENT" = "production" ]; then
  echo "Deploying to production..."
  ./scripts/deploy-prod.sh
else
  echo "Deploying to staging..."
  ./scripts/deploy-staging.sh
fi
```

### Environment-Specific Tasks

```toml
[tasks.deploy-staging]
env = { ENVIRONMENT = "staging" }
run = "./scripts/deploy.sh"

[tasks.deploy-production]
env = { ENVIRONMENT = "production" }
run = "./scripts/deploy.sh"
```

### Multi-Stage Builds

```toml
[tasks.build]
depends = ["build:clean", "build:compile", "build:optimize"]

[tasks."build:clean"]
run = "rm -rf dist"

[tasks."build:compile"]
run = "tsc"

[tasks."build:optimize"]
run = "esbuild dist/index.js --bundle --minify --outfile=dist/bundle.js"
```

### Parallel Task Execution

```toml
[tasks.ci-fast]
depends = ["lint", "test:unit", "type-check"]  # Run in parallel
run = "echo All checks passed"

[tasks.lint]
run = "eslint ."

[tasks."test:unit"]
run = "vitest run"

[tasks."type-check"]
run = "tsc --noEmit"
```

## Testing Patterns

### Test Pyramid

```toml
[tasks.test]
depends = ["test:unit", "test:integration", "test:e2e"]

[tasks."test:unit"]
description = "Fast unit tests"
run = "vitest run"

[tasks."test:integration"]
description = "Integration tests"
run = "vitest run --config vitest.integration.config.ts"

[tasks."test:e2e"]
description = "End-to-end tests"
depends = ["build"]
run = "playwright test"
```

### Test with Coverage

```toml
[tasks."test:coverage"]
run = [
  "vitest run --coverage",
  "echo Coverage report: ./coverage/index.html"
]
```

### Watch Mode

```toml
[tasks."test:watch"]
run = "vitest watch"

[tasks."dev:test"]
run = "vitest watch --ui"
```

## Database Patterns

### Migration Workflow

```toml
[tasks."db:migrate"]
description = "Run database migrations"
run = "knex migrate:latest"

[tasks."db:rollback"]
description = "Rollback last migration"
run = "knex migrate:rollback"

[tasks."db:reset"]
description = "Reset database"
run = [
  "knex migrate:rollback --all",
  "knex migrate:latest",
  "knex seed:run"
]

[tasks."db:seed"]
description = "Seed database"
run = "knex seed:run"
```

### Database Backup

```bash
#!/usr/bin/env bash
# mise description="Backup database"
# mise depends=["check-db-connection"]

timestamp=$(date +%Y%m%d_%H%M%S)
backup_file="backups/db_backup_${timestamp}.sql"

echo "Creating backup: $backup_file"
pg_dump $DATABASE_URL > "$backup_file"
echo "Backup complete"
```

## Deployment Patterns

### Blue-Green Deployment

```toml
[tasks."deploy:blue"]
env = { DEPLOYMENT_SLOT = "blue" }
run = "./scripts/deploy.sh"

[tasks."deploy:green"]
env = { DEPLOYMENT_SLOT = "green" }
run = "./scripts/deploy.sh"

[tasks."switch:to-blue"]
run = "./scripts/switch-traffic.sh blue"

[tasks."switch:to-green"]
run = "./scripts/switch-traffic.sh green"
```

### Canary Deployment

```bash
#!/usr/bin/env bash
# mise description="Canary deployment"
# mise depends=["build", "test"]

echo "Deploying to 10% of traffic..."
./scripts/deploy.sh --canary 10

echo "Monitoring for 5 minutes..."
sleep 300

if ./scripts/check-health.sh; then
  echo "Health check passed, deploying to 100%"
  ./scripts/deploy.sh --canary 100
else
  echo "Health check failed, rolling back"
  ./scripts/rollback.sh
  exit 1
fi
```

### Zero-Downtime Deployment

```toml
[tasks.deploy-zero-downtime]
run = [
  "./scripts/deploy-new-version.sh",
  "./scripts/health-check.sh",
  "./scripts/switch-traffic.sh",
  "./scripts/shutdown-old-version.sh"
]
```

## Release Patterns

### Semantic Versioning

```bash
#!/usr/bin/env bash
# mise description="Release new version"
# mise depends=["test", "build"]

VERSION_TYPE=${1:-patch}  # major, minor, or patch

echo "Creating $VERSION_TYPE release..."
npm version $VERSION_TYPE
git push --follow-tags
npm publish
```

### Changelog Generation

```toml
[tasks.changelog]
run = '''
echo "# Changelog" > CHANGELOG.md
git log --pretty=format:"- %s (%h)" >> CHANGELOG.md
'''
```

### Release Workflow

```toml
[tasks.release]
depends = ["release:prepare", "release:publish", "release:announce"]

[tasks."release:prepare"]
run = [
  "mise run test",
  "mise run build",
  "mise run changelog",
  "git add .",
  "git commit -m 'chore: prepare release'"
]

[tasks."release:publish"]
run = [
  "npm version patch",
  "git push --follow-tags",
  "npm publish"
]

[tasks."release:announce"]
run = "./scripts/announce-release.sh"
```

## CI/CD Patterns

### Local CI Simulation

```toml
[tasks."ci:local"]
description = "Simulate CI pipeline locally"
run = [
  "mise run clean",
  "mise run install",
  "mise run lint",
  "mise run type-check",
  "mise run test",
  "mise run build"
]
```

### Fast Feedback Loop

```toml
[tasks."ci:quick"]
description = "Quick checks (< 1 min)"
depends = ["lint", "type-check", "test:unit"]

[tasks."ci:full"]
description = "Full CI pipeline (~ 5 min)"
depends = ["ci:quick", "test:integration", "test:e2e", "build"]
```

### Artifact Generation

```bash
#!/usr/bin/env bash
# mise description="Generate CI artifacts"
# mise depends=["test", "build"]

mkdir -p artifacts

# Copy build output
cp -r dist artifacts/

# Copy test results
cp -r coverage artifacts/

# Create tarball
tar -czf artifacts/app-${CI_COMMIT_SHA}.tar.gz dist/

echo "Artifacts ready in ./artifacts/"
```

## Monitoring & Health Checks

### Health Check Pattern

```bash
#!/usr/bin/env bash
# mise description="Health check"

response=$(curl -sf http://localhost:3000/health)

if [ $? -eq 0 ]; then
  echo "✓ Health check passed"
  exit 0
else
  echo "✗ Health check failed"
  exit 1
fi
```

### Smoke Tests

```toml
[tasks."smoke:staging"]
env = { API_URL = "https://staging.example.com" }
run = "./scripts/smoke-tests.sh"

[tasks."smoke:production"]
env = { API_URL = "https://api.example.com" }
run = "./scripts/smoke-tests.sh"
```

## Development Workflow Patterns

### Clean Slate

```toml
[tasks.reset]
description = "Reset project to clean state"
run = [
  "rm -rf node_modules dist coverage .turbo",
  "npm install",
  "mise run db:reset",
  "mise run build"
]
```

### Quick Start

```toml
[tasks.start]
description = "Quick start (assumes deps installed)"
depends = ["db:migrate"]
run = "npm run dev"

[tasks.setup]
description = "Full setup from scratch"
run = [
  "npm install",
  "cp .env.example .env",
  "mise run db:reset",
  "mise run start"
]
```

### Development with Live Reload

```toml
[tasks.dev]
description = "Development with live reload"
env = { NODE_ENV = "development" }
run = "tsx watch src/index.ts"

[tasks."dev:debug"]
description = "Development with debugger"
env = { NODE_ENV = "development", DEBUG = "*" }
run = "tsx watch --inspect src/index.ts"
```

## Best Practices

1. **Idempotent Tasks**: Tasks should be safe to run multiple times
2. **Fast Feedback**: Prioritize quick checks before slow ones
3. **Clear Output**: Provide informative messages about progress
4. **Error Handling**: Use proper exit codes and error messages
5. **Documentation**: Include descriptions for all tasks
6. **Composability**: Build complex workflows from simple tasks
7. **Environment Awareness**: Use environment variables for configuration

## Anti-Patterns

### ❌ Don't: Chain Tasks with Semicolons

```toml
[tasks.ci]
run = "npm run lint; npm test; npm run build"
```

**Why**: Errors are ignored, tasks continue even on failure

### ✅ Do: Use Array or Dependencies

```toml
[tasks.ci]
run = [
  "npm run lint",
  "npm test",
  "npm run build"
]
```

### ❌ Don't: Hardcode Environment Values

```toml
[tasks.deploy]
run = "deploy.sh --env production --region us-east-1"
```

### ✅ Do: Use Environment Variables

```toml
[tasks.deploy]
env = { ENV = "production", REGION = "us-east-1" }
run = "deploy.sh --env $ENV --region $REGION"
```

### ❌ Don't: Duplicate Logic

```toml
[tasks.test-api]
run = "cd api && npm test"

[tasks.test-web]
run = "cd web && npm test"
```

### ✅ Do: Use Monorepo Tasks

```toml
[tasks.test]
run = "npm test"

# Call from root: mise run -C api test
```
