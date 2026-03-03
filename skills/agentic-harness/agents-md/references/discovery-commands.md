# Discovery Commands Reference

Comprehensive commands for analyzing repository structure and detecting actual technologies in use.

## Phase 1: Initial Project Scan

### Find Existing Documentation
```bash
# Find existing AGENTS.md files
find . -name "AGENTS.md" -o -name "AI-DOCS.md" 2>/dev/null

# Find local skills to reference
find .claude/skills -name "SKILL.md" 2>/dev/null
find .agents/skills -name "SKILL.md" 2>/dev/null
ls plugins/*/skills/*/SKILL.md 2>/dev/null
```

### Project Structure Discovery
```bash
# Analyze project structure
find . -name "package.json" -not -path "./node_modules/*" 2>/dev/null
find . -name "pyproject.toml" -o -name "setup.py" -o -name "requirements.txt" 2>/dev/null
find . -name "go.mod" -o -name "go.sum" 2>/dev/null
find . -name "Cargo.toml" -o -name "Cargo.lock" 2>/dev/null

# Check for monorepo tools
ls -la pnpm-workspace.yaml lerna.json turbo.json nx.json 2>/dev/null
grep -r "workspaces" package.json 2>/dev/null
```

## Phase 2: Technology Stack Detection

### Programming Languages (Run ALL that apply)
```bash
# JavaScript/TypeScript
find . -name "*.js" -o -name "*.ts" -o -name "*.jsx" -o -name "*.tsx" | head -5

# Python
find . -name "*.py" | head -5

# Go
find . -name "*.go" | head -5

# Rust
find . -name "*.rs" | head -5

# PHP
find . -name "*.php" | head -5

# Java/Kotlin
find . -name "*.java" -o -name "*.kt" | head -5
```

### Frontend Frameworks (Check package.json or equivalent)
```bash
# JavaScript frameworks
grep -r "react\|vue\|angular\|svelte\|solid" package.json 2>/dev/null | head -5
grep -r "next\|nuxt\|gatsby\|remix\|astro" package.json 2>/dev/null | head -5

# Build tools
grep -r "vite\|webpack\|rollup\|parcel\|esbuild" package.json 2>/dev/null | head -3
```

### Backend Frameworks
```bash
# Node.js backends
grep -r "express\|fastify\|koa\|nest\|hapi\|adonis" package.json 2>/dev/null | head -3

# Python backends
grep -r "django\|flask\|fastapi\|tornado" pyproject.toml setup.py requirements.txt 2>/dev/null | head -3

# Go backends  
grep -r "gin\|echo\|fiber\|gorilla" go.mod 2>/dev/null | head -3

# Other languages
grep -r "rails\|sinatra" Gemfile 2>/dev/null
grep -r "laravel\|symfony" composer.json 2>/dev/null
```

### Database Technology Detection
```bash
# Database types
grep -r "postgres\|mysql\|sqlite\|mongo\|redis\|cassandra" package.json pyproject.toml go.mod Cargo.toml 2>/dev/null | head -5

# ORMs and query builders
grep -r "prisma\|drizzle\|typeorm\|sequelize\|mongoose\|knex" package.json 2>/dev/null | head -3
grep -r "sqlalchemy\|django.db\|peewee" pyproject.toml requirements.txt 2>/dev/null | head -3
grep -r "gorm\|sqlx" go.mod 2>/dev/null | head -3
```

### Testing Frameworks
```bash
# JavaScript testing
grep -r "jest\|vitest\|mocha\|jasmine\|ava" package.json 2>/dev/null | head -3
grep -r "playwright\|cypress\|puppeteer\|selenium" package.json 2>/dev/null | head -3

# Python testing
grep -r "pytest\|unittest\|nose" pyproject.toml setup.py requirements.txt 2>/dev/null | head -3

# Other languages
ls -la *_test.go 2>/dev/null | head -3
find . -name "*_test.rs" 2>/dev/null | head -3
```

## Phase 3: File Structure Analysis

### Configuration Files Discovery
```bash
# Build and development configs
find . -maxdepth 2 -name "*.config.*" -o -name ".*rc*" 2>/dev/null | head -10

# Environment and settings
ls -la .env* docker-compose.* Dockerfile* Makefile 2>/dev/null

# CI/CD configs
ls -la .github/workflows/ .gitlab-ci.yml .circleci/ 2>/dev/null
```

### Styling and Asset Management
```bash
# Styling approaches
find . -name "*.css" -o -name "*.scss" -o -name "*.sass" -o -name "*.less" | head -5
find . -name "*.styled.*" -o -name "*emotion*" | head -5
grep -r "tailwind\|styled-components\|emotion" package.json 2>/dev/null

# Static assets
ls -la public/ static/ assets/ 2>/dev/null
```

## Phase 4: Package and Service Organization

### Multi-Package Structure
```bash
# Common monorepo patterns
ls -la apps/ packages/ services/ workers/ libs/ 2>/dev/null

# Service-specific directories
ls -la api/ web/ mobile/ desktop/ backend/ frontend/ 2>/dev/null

# Specialized directories
ls -la components/ utils/ shared/ common/ core/ 2>/dev/null
```

### Development Environment Setup
```bash
# Node.js setup
ls -la package-lock.json yarn.lock pnpm-lock.yaml bun.lockb 2>/dev/null

# Python environment
ls -la poetry.lock Pipfile.lock conda.yml environment.yml 2>/dev/null

# Other environments
ls -la .ruby-version .nvmrc .python-version 2>/dev/null
```

## Analysis Report Template

After running discovery commands, structure findings as:

### 1. Repository Classification
- **Type**: Simple project / Multi-package / Monorepo  
- **Complexity**: [Number of major directories/packages]
- **Team Structure**: Single developer / Small team / Large organization

### 2. Technology Stack Summary
- **Primary Language(s)**: [Detected from file extensions and configs]
- **Frontend**: [Framework + build tool detected]
- **Backend**: [Framework + language detected] 
- **Database**: [Type + ORM detected]
- **Testing**: [Framework(s) detected]

### 3. Build and Development Tools
- **Package Manager**: [npm/yarn/pnpm/poetry/cargo/go modules]
- **Build System**: [Specific tool detected]
- **Monorepo Tool**: [If applicable - Nx/Turborepo/Lerna]
- **CI/CD**: [If detected - GitHub Actions/GitLab CI]

### 4. File Organization Patterns
- **Source Code**: [Location of main application code]
- **Tests**: [Test file patterns and locations found]
- **Configuration**: [Config file patterns discovered]
- **Documentation**: [Existing docs structure]

### 5. Existing Skills and Documentation
- **Local Skills**: [List discovered skills with paths]
- **Documentation**: [README, wiki, docs/ folder status]
- **API Docs**: [OpenAPI, GraphQL schema, etc.]

### 6. Recommended AGENTS.md Structure
Based on analysis:
- **Approach**: Simple single file / Hierarchical structure
- **Sub-files needed**: [List directories requiring own AGENTS.md]
- **Critical patterns**: [Key conventions to document]