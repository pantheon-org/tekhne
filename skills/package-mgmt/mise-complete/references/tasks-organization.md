# Task Organization

## Task Structure

### Inline Tasks (mise.toml)

Use for simple, frequently-used tasks:

```toml
[tasks.dev]
description = "Start development server"
run = "npm run dev"

[tasks.build]
description = "Build for production"
run = "npm run build"

[tasks.test]
description = "Run all tests"
run = "npm test"
```

### File-Based Tasks (.mise/tasks/)

Use for complex tasks with multiple steps:

```
.mise/
└── tasks/
    ├── deploy
    ├── release
    ├── db-migrate
    └── ci-full
```

Example `.mise/tasks/deploy`:

```bash
#!/usr/bin/env bash
# mise description="Deploy application"
# mise depends=["build", "test"]

set -e

echo "Deploying to $ENVIRONMENT..."
./scripts/deploy.sh
```

## Task Naming Conventions

### Namespace by Purpose

```toml
# Development tasks
[tasks.dev]
[tasks."dev:api"]
[tasks."dev:web"]

# Testing tasks
[tasks.test]
[tasks."test:unit"]
[tasks."test:integration"]
[tasks."test:e2e"]

# Build tasks
[tasks.build]
[tasks."build:api"]
[tasks."build:web"]

# Deployment tasks
[tasks.deploy]
[tasks."deploy:staging"]
[tasks."deploy:production"]
```

### Common Task Names

- `dev` / `start`: Start development environment
- `build`: Build for production
- `test`: Run tests
- `lint`: Check code quality
- `format`: Format code
- `clean`: Remove build artifacts
- `install` / `setup`: Install dependencies
- `deploy`: Deploy application
- `release`: Create a release
- `ci`: Run CI pipeline locally

## Task Composition

### Dependency Graphs

```toml
[tasks.ci]
depends = ["lint", "test", "build"]

[tasks.lint]
run = "eslint ."

[tasks.test]
depends = ["lint"]
run = "npm test"

[tasks.build]
depends = ["test"]
run = "npm run build"

[tasks.deploy]
depends = ["ci"]
run = "./deploy.sh"
```

Execution order: `lint` → `test` → `build` → `deploy`

### Reusable Task Components

```toml
[tasks.check-deps]
run = "npm outdated"

[tasks.check-security]
run = "npm audit"

[tasks.check-all]
depends = ["check-deps", "check-security"]
description = "Run all checks"
```

## Monorepo Organization

### Root-Level Tasks

```toml
# Root mise.toml
[tasks.install-all]
run = "npm install --workspaces"

[tasks.build-all]
run = '''
for pkg in packages/*; do
  mise run -C "$pkg" build
done
'''

[tasks.test-all]
run = '''
for pkg in packages/*; do
  mise run -C "$pkg" test
done
'''
```

### Package-Level Tasks

```toml
# packages/api/mise.toml
[tasks.dev]
run = "tsx watch src/index.ts"

[tasks.test]
run = "vitest"

[tasks.build]
run = "tsup"

# packages/web/mise.toml
[tasks.dev]
run = "vite"

[tasks.test]
run = "vitest"

[tasks.build]
run = "vite build"
```

### Cross-Package Dependencies

```toml
# packages/web/mise.toml
[tasks.dev]
depends = ["build-api"]
run = "vite"

[tasks.build-api]
run = "mise run -C ../api build"
```

## Task Categories

### Development Workflow

```toml
[tasks.setup]
description = "Initial project setup"
run = [
  "npm install",
  "cp .env.example .env",
  "mise run db:migrate"
]

[tasks.dev]
description = "Start development"
depends = ["setup"]
run = "npm run dev"

[tasks.format]
description = "Format code"
run = "prettier --write ."

[tasks.lint]
description = "Lint code"
run = "eslint . --fix"
```

### Testing Workflow

```toml
[tasks.test]
description = "Run all tests"
depends = ["test:unit", "test:integration"]

[tasks."test:unit"]
run = "vitest run"

[tasks."test:integration"]
run = "vitest run --config vitest.integration.config.ts"

[tasks."test:e2e"]
run = "playwright test"

[tasks."test:watch"]
run = "vitest watch"

[tasks."test:coverage"]
run = "vitest run --coverage"
```

### Build & Release Workflow

```toml
[tasks.clean]
run = "rm -rf dist coverage .turbo"

[tasks.build]
depends = ["clean", "test"]
run = "npm run build"

[tasks.version]
run = "npm version patch"

[tasks.publish]
depends = ["build"]
run = "npm publish"

[tasks.release]
depends = ["test", "build", "version", "publish"]
description = "Full release workflow"
```

### CI/CD Workflow

```toml
[tasks.ci]
description = "Run full CI pipeline"
depends = ["lint", "test", "build"]

[tasks."ci:quick"]
description = "Quick CI check"
depends = ["lint", "test:unit"]

[tasks.deploy]
depends = ["ci"]
run = "./scripts/deploy.sh"
```

## Best Practices

1. **Group Related Tasks**: Use namespaces (e.g., `test:unit`, `test:integration`)
2. **Consistent Naming**: Follow project conventions
3. **Clear Descriptions**: Every task should have a description
4. **Logical Dependencies**: Use `depends` to ensure prerequisites
5. **Atomic Tasks**: Each task should have a single, clear purpose
6. **Composable**: Build complex workflows from simple tasks

## File Organization

### Recommended Structure

```
project/
├── mise.toml                 # Common tasks (dev, test, build)
├── .mise/
│   ├── config.toml          # Mise configuration
│   └── tasks/
│       ├── deploy           # Deployment tasks
│       ├── release          # Release automation
│       ├── db-migrate       # Database tasks
│       └── utils/
│           ├── check-deps   # Utility scripts
│           └── cleanup
└── packages/
    ├── api/
    │   └── mise.toml        # API-specific tasks
    └── web/
        └── mise.toml        # Web-specific tasks
```

## Anti-Patterns

### ❌ Don't: Duplicate Task Logic

```toml
[tasks.test-api]
run = "cd packages/api && npm test"

[tasks.test-web]
run = "cd packages/web && npm test"
```

### ✅ Do: Create Reusable Tasks

```toml
[tasks.test]
run = "npm test"

# In packages/api/mise.toml and packages/web/mise.toml
[tasks.test]
run = "npm test"
```

### ❌ Don't: Hardcode Paths

```toml
[tasks.deploy]
run = "/Users/john/projects/myapp/deploy.sh"
```

### ✅ Do: Use Relative Paths

```toml
[tasks.deploy]
run = "./scripts/deploy.sh"
```

### ❌ Don't: Ignore Exit Codes

```toml
[tasks.ci]
run = "npm run lint; npm test; npm run build"
```

### ✅ Do: Use Proper Exit Code Handling

```toml
[tasks.ci]
run = [
  "npm run lint",
  "npm test",
  "npm run build"
]
```
