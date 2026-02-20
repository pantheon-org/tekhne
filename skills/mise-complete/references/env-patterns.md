# Environment Variable Patterns

## Common Patterns

### Multi-Environment Setup

Support multiple environments (development, staging, production):

```toml
# mise.toml
[env]
NODE_ENV = "development"
_.file = ".env.{{ env.NODE_ENV }}"
```

```bash
# .env.development
API_URL=http://localhost:3000
DATABASE_URL=postgresql://localhost/myapp_dev
LOG_LEVEL=debug
DEBUG=true

# .env.staging
API_URL=https://staging-api.example.com
DATABASE_URL=postgresql://staging-db/myapp
LOG_LEVEL=info
DEBUG=false

# .env.production
API_URL=https://api.example.com
DATABASE_URL=postgresql://prod-db/myapp
LOG_LEVEL=warn
DEBUG=false
ENABLE_CACHE=true
```

**Usage:**

```bash
# Development (default)
mise run dev

# Staging
NODE_ENV=staging mise run deploy

# Production
NODE_ENV=production mise run deploy
```

### Database Configuration

Flexible database connection setup:

```toml
[env]
DB_HOST = "localhost"
DB_PORT = "5432"
DB_NAME = "myapp_dev"
DB_USER = "postgres"
DB_PASSWORD = ""  # Set in .env.local

# Computed connection string
DATABASE_URL = "postgresql://{{ env.DB_USER }}:{{ env.DB_PASSWORD }}@{{ env.DB_HOST }}:{{ env.DB_PORT }}/{{ env.DB_NAME }}"
```

**Override locally:**

```toml
# mise.local.toml
[env]
DB_NAME = "myapp_johndoe"
DB_PASSWORD = "secret123"
```

### Service Configuration

Configure multiple services with consistent patterns:

```toml
[env]
# API Service
API_HOST = "localhost"
API_PORT = "3000"
API_PROTOCOL = "http"
API_URL = "{{ env.API_PROTOCOL }}://{{ env.API_HOST }}:{{ env.API_PORT }}"

# Web Service
WEB_HOST = "localhost"
WEB_PORT = "8080"
WEB_PROTOCOL = "http"
WEB_URL = "{{ env.WEB_PROTOCOL }}://{{ env.WEB_HOST }}:{{ env.WEB_PORT }}"

# Database
DB_HOST = "localhost"
DB_PORT = "5432"
DATABASE_URL = "postgresql://{{ env.DB_HOST }}:{{ env.DB_PORT }}/myapp"
```

### Feature Flags

Manage feature toggles through environment variables:

```toml
[env]
# Feature flags
FEATURE_NEW_UI = "true"
FEATURE_BETA_API = "false"
FEATURE_ANALYTICS = "true"
FEATURE_EXPERIMENTAL_SEARCH = "false"

# Feature-specific config
ANALYTICS_KEY = "UA-123456-1"
BETA_API_ENDPOINT = "https://beta-api.example.com"
```

**Toggle features per environment:**

```bash
# .env.development
FEATURE_NEW_UI=true
FEATURE_BETA_API=true  # Enable in dev

# .env.production
FEATURE_NEW_UI=true
FEATURE_BETA_API=false  # Disable in prod
```

### Microservices Architecture

Configure multiple services in a monorepo:

```toml
# Root mise.toml
[env]
PROJECT_ROOT = "{{ config_root }}"
NODE_ENV = "development"

# Service registry
AUTH_SERVICE_URL = "http://localhost:3001"
USER_SERVICE_URL = "http://localhost:3002"
ORDER_SERVICE_URL = "http://localhost:3003"
PAYMENT_SERVICE_URL = "http://localhost:3004"

# Shared configuration
REDIS_URL = "redis://localhost:6379"
RABBITMQ_URL = "amqp://localhost:5672"
```

**Per-service configuration:**

```toml
# services/auth/mise.toml
[env]
SERVICE_NAME = "auth"
PORT = "3001"
DATABASE_URL = "postgresql://localhost/auth_db"

# services/user/mise.toml
[env]
SERVICE_NAME = "user"
PORT = "3002"
DATABASE_URL = "postgresql://localhost/user_db"
AUTH_SERVICE_URL = "{{ env.AUTH_SERVICE_URL }}"
```

### CI/CD Integration

Environment setup for continuous integration:

```toml
[env]
# Detect CI environment
CI = "{{ env.CI | default 'false' }}"
CI_PIPELINE_ID = "{{ env.CI_PIPELINE_ID | default '' }}"

# CI-specific behavior
LOG_FORMAT = "{{ if eq .env.CI 'true' }}json{{ else }}pretty{{ end }}"
TEST_TIMEOUT = "{{ if eq .env.CI 'true' }}60000{{ else }}5000{{ end }}"
```

**CI pipeline:**

```bash
# .gitlab-ci.yml or .github/workflows/ci.yml
script:
  - mise install
  - mise exec -- npm test
  - mise exec -- npm run build
```

### Docker Integration

Environment configuration for containers:

```toml
[env]
# Detect Docker environment
IS_DOCKER = "{{ env.IS_DOCKER | default 'false' }}"

# Docker-specific paths
DATA_DIR = "{{ if eq .env.IS_DOCKER 'true' }}/data{{ else }}{{ config_root }}/data{{ end }}"
LOG_DIR = "{{ if eq .env.IS_DOCKER 'true' }}/var/log{{ else }}{{ config_root }}/logs{{ end }}"

# Container networking
API_HOST = "{{ if eq .env.IS_DOCKER 'true' }}0.0.0.0{{ else }}localhost{{ end }}"
```

**Dockerfile:**

```dockerfile
FROM node:20
WORKDIR /app

RUN curl https://mise.run | sh
ENV PATH="/root/.local/bin:$PATH"
ENV IS_DOCKER=true

COPY mise.toml .
RUN mise install

CMD ["mise", "exec", "--", "npm", "start"]
```

### Development Tools Integration

Configure editors and development tools:

```toml
[env]
# Editor settings
EDITOR = "code"
VISUAL = "code --wait"

# Git configuration
GIT_EDITOR = "code --wait"
GIT_MERGE_TOOL = "code"

# Language server settings
TSSERVER_LOG_FILE = "{{ config_root }}/logs/tsserver.log"
RUST_ANALYZER_LOG = "{{ config_root }}/logs/rust-analyzer.log"

# Debugging
DEBUG_PORT = "9229"
NODE_OPTIONS = "--inspect={{ env.DEBUG_PORT }}"
```

### Secret Management

Secure sensitive data handling:

```toml
# mise.toml (committed)
[env]
# Public configuration
NODE_ENV = "development"
API_URL = "http://localhost:3000"

# Load secrets from separate files
[env]
_.file = [
  ".env",           # Public config
  ".env.local",     # Local overrides
  ".env.secrets"    # Secrets (git-ignored)
]
```

```bash
# .env (committed)
NODE_ENV=development
PORT=3000

# .env.secrets (git-ignored)
API_SECRET=super-secret-key
JWT_SECRET=jwt-secret-123
DATABASE_PASSWORD=db-password
AWS_SECRET_ACCESS_KEY=aws-secret

# .env.example (committed)
# Copy to .env.secrets and fill in values
API_SECRET=
JWT_SECRET=
DATABASE_PASSWORD=
AWS_SECRET_ACCESS_KEY=
```

### Logging Configuration

Flexible logging setup:

```toml
[env]
# Log level
LOG_LEVEL = "info"

# Log format
LOG_FORMAT = "json"  # or "pretty"

# Log destinations
LOG_FILE = "{{ config_root }}/logs/app.log"
ERROR_LOG_FILE = "{{ config_root }}/logs/error.log"

# Service-specific logging
AUTH_LOG_LEVEL = "debug"
DB_LOG_LEVEL = "warn"
```

**Environment-specific:**

```bash
# .env.development
LOG_LEVEL=debug
LOG_FORMAT=pretty

# .env.production
LOG_LEVEL=info
LOG_FORMAT=json
```

### Monorepo Workspace Configuration

Manage multiple packages with shared configuration:

```toml
# Root mise.toml
[env]
WORKSPACE_ROOT = "{{ config_root }}"
NODE_ENV = "development"

# Shared dependencies
POSTGRES_URL = "postgresql://localhost/myapp"
REDIS_URL = "redis://localhost:6379"

# Package paths
PACKAGE_API = "{{ config_root }}/packages/api"
PACKAGE_WEB = "{{ config_root }}/packages/web"
PACKAGE_SHARED = "{{ config_root }}/packages/shared"
```

**Package-specific:**

```toml
# packages/api/mise.toml
[env]
PACKAGE_NAME = "api"
API_PORT = "3000"
DATABASE_URL = "{{ env.POSTGRES_URL }}"

# packages/web/mise.toml
[env]
PACKAGE_NAME = "web"
WEB_PORT = "8080"
API_URL = "http://localhost:{{ env.API_PORT }}"
```

### Testing Configuration

Environment setup for different test types:

```toml
[env]
# Test environment detection
NODE_ENV = "test"

# Test database
TEST_DATABASE_URL = "postgresql://localhost/myapp_test"

# Test Redis
TEST_REDIS_URL = "redis://localhost:6380"

# Test timeouts
UNIT_TEST_TIMEOUT = "5000"
INTEGRATION_TEST_TIMEOUT = "30000"
E2E_TEST_TIMEOUT = "60000"

# Test coverage
COVERAGE_THRESHOLD = "80"
```

**Test-specific files:**

```bash
# .env.test
NODE_ENV=test
DATABASE_URL=postgresql://localhost/myapp_test
LOG_LEVEL=error
```

### Performance Tuning

Environment variables for performance:

```toml
[env]
# Node.js performance
NODE_OPTIONS = "--max-old-space-size=4096"
UV_THREADPOOL_SIZE = "8"

# Application performance
CACHE_TTL = "3600"
MAX_CONNECTIONS = "100"
REQUEST_TIMEOUT = "30000"

# Database tuning
DB_POOL_MIN = "2"
DB_POOL_MAX = "10"
DB_IDLE_TIMEOUT = "10000"
```

## Best Practices

1. **Environment-Specific Files**: Use `.env.<environment>` pattern
2. **Computed Values**: Leverage template syntax for derived values
3. **Secret Separation**: Keep secrets in git-ignored files
4. **Example Files**: Provide `.env.example` for documentation
5. **Consistent Naming**: Use prefixes (e.g., `DB_`, `API_`, `FEATURE_`)
6. **Type Hints**: Document expected values in comments

## Anti-Patterns

### ❌ Don't: Hardcode Environment-Specific Values

```toml
[env]
API_URL = "http://localhost:3000"  # Breaks in production
```

### ✅ Do: Use Environment Files

```toml
[env]
_.file = ".env.{{ env.NODE_ENV }}"
```

### ❌ Don't: Duplicate Configuration

```toml
[env]
API_PROTOCOL = "http"
API_HOST = "localhost"
API_PORT = "3000"
API_URL = "http://localhost:3000"  # Duplication
```

### ✅ Do: Compute Derived Values

```toml
[env]
API_PROTOCOL = "http"
API_HOST = "localhost"
API_PORT = "3000"
API_URL = "{{ env.API_PROTOCOL }}://{{ env.API_HOST }}:{{ env.API_PORT }}"
```

### ❌ Don't: Mix Concerns

```toml
[env]
DATABASE_URL = "postgresql://postgres:secret123@localhost/myapp"
```

### ✅ Do: Separate Configuration

```toml
[env]
DB_HOST = "localhost"
DB_USER = "postgres"
DB_PASSWORD = ""  # Set in .env.local
DB_NAME = "myapp"
DATABASE_URL = "postgresql://{{ env.DB_USER }}:{{ env.DB_PASSWORD }}@{{ env.DB_HOST }}/{{ env.DB_NAME }}"
```
