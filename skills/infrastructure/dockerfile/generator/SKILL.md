---
name: dockerfile-generator
description: Generates production-ready Dockerfiles with multi-stage builds, layer caching, security hardening (non-root users, minimal base images), health checks, and automatic .dockerignore creation. Validates output using devops-skills:dockerfile-validator with iterative error fixing. Use when a user wants to create, generate, write, or build a Dockerfile or docker image, containerize an app, set up a container, or needs a Dockerfile for Node.js, Python, Go, Java, or another language. Also applies when the user says 'containerize my app', 'docker-compose setup', 'container setup', or 'optimize my Docker build'.
---

# Dockerfile Generator

## Overview

Generates production-ready Dockerfiles with security, optimization, and best practices built-in: multi-stage builds, security-hardened configurations, optimized layer structures, automatic validation, and iterative error fixing.

## When to Use / Not Use

Use for creating, generating, or optimizing Dockerfiles and containerizing applications. Do **not** use for validating existing Dockerfiles (use devops-skills:dockerfile-validator), building/running containers, debugging running containers, or managing image registries.

## Dockerfile Generation Workflow

Follow this workflow when generating Dockerfiles. Adapt based on user needs:

### Stage 1: Gather Requirements

**Objective:** Understand what needs to be containerized and gather all necessary information.

**Information to Collect (use AskUserQuestion if missing or unclear):**

- Language/version, application type, framework, entry point
- Package manager, system dependencies, build-time vs runtime deps
- Port(s), environment variables, config files, health check endpoint, volume mounts
- Build/test commands, compilation needs, static asset generation
- Image size constraints, security requirements, resource constraints

### Stage 2: Framework/Library Documentation Lookup (if needed)

**Objective:** Research framework-specific containerization patterns and best practices.

**When to Perform This Stage:**
- User mentions a specific framework (Next.js, Django, FastAPI, Spring Boot, etc.)
- Application has complex build requirements
- Need guidance on framework-specific optimization

**Research Process:**

1. **Try context7 MCP first (preferred):**
   ```
   Use mcp__context7__resolve-library-id with the framework name
   Examples:
   - "next.js" for Next.js applications
   - "django" for Django applications
   - "fastapi" for FastAPI applications
   - "spring-boot" for Spring Boot applications
   - "express" for Express.js applications

   Then use mcp__context7__get-library-docs with:
   - context7CompatibleLibraryID from resolve step
   - topic: "docker deployment production build"
   - page: 1 (fetch additional pages if needed)
   ```

2. **Fallback to WebSearch if context7 fails:**
   ```
   Search query pattern:
   "<framework>" "<version>" dockerfile best practices production 2025

   Examples:
   - "Next.js 14 dockerfile best practices production 2025"
   - "FastAPI dockerfile best practices production 2025"
   - "Spring Boot 3 dockerfile best practices production 2025"
   ```

3. **Extract key information:**
   - Recommended base images
   - Build optimization techniques
   - Framework-specific environment variables
   - Production vs development configurations
   - Security considerations

### Stage 3: Generate Dockerfile

**Objective:** Create a production-ready, multi-stage Dockerfile following best practices.

**Core Principles:**

1. **Multi-Stage Builds (REQUIRED for compiled languages, RECOMMENDED for all):**
   - Separate build stage from runtime stage
   - Keep build tools out of final image
   - Copy only necessary artifacts

2. **Security Hardening (REQUIRED):**
   - Use specific version tags (NEVER use :latest)
   - Run as non-root user (create dedicated user)
   - Use minimal base images (alpine, distroless)
   - No hardcoded secrets
   - Scan base images for vulnerabilities

3. **Layer Optimization (REQUIRED):**
   - Order instructions from least to most frequently changing
   - Copy dependency files before application code
   - Combine related RUN commands with &&
   - Clean up package manager caches in same layer
   - Leverage build cache effectively

4. **Production Readiness (REQUIRED):**
   - Add HEALTHCHECK for services
   - Use exec form for ENTRYPOINT/CMD
   - Set WORKDIR to absolute paths
   - Document exposed ports with EXPOSE

**Language-Specific Templates:**

For detailed templates and examples, see:
- **Node.js/Next.js**: `references/language_specific_guides.md` - Node.js section
- **Python/FastAPI**: `references/language_specific_guides.md` - Python section
- **Go**: `references/language_specific_guides.md` - Go section
- **Java/Spring Boot**: `references/language_specific_guides.md` - Java section

**Quick Template Selection:**
- Node.js: Use for JavaScript/TypeScript applications (Express, Next.js, etc.)
- Python: Use for Python applications (FastAPI, Django, Flask)
- Go: Use for Go applications (minimal images with distroless)
- Java: Use for Spring Boot, Quarkus, or other Java frameworks
- Generic: Adapt patterns from references for other languages

**Always Include:**
1. Syntax directive: `# syntax=docker/dockerfile:1`
2. Multi-stage build (build + production stages)
3. Non-root user creation and usage
4. HEALTHCHECK for services (if applicable)
5. Proper WORKDIR settings
6. EXPOSE for documented ports
7. Clean package manager caches
8. exec form for CMD/ENTRYPOINT

### Stage 4: Generate .dockerignore

**Objective:** Create comprehensive .dockerignore to reduce build context and prevent secret leaks.

**Always create .dockerignore with generated Dockerfile.**

**Standard .dockerignore Template:**

```
# Git
.git
.gitignore
.gitattributes

# CI/CD
.github
.gitlab-ci.yml
.travis.yml
.circleci

# Documentation
README.md
CHANGELOG.md
CONTRIBUTING.md
LICENSE
*.md
docs/

# Docker
Dockerfile*
docker-compose*.yml
.dockerignore

# Environment
.env
.env.*
*.local

# Logs
logs/
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Dependencies (language-specific - add as needed)
node_modules/
__pycache__/
*.pyc
*.pyo
*.pyd
.Python
venv/
.venv/
target/
*.class

# IDE
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store

# Testing
coverage/
.coverage
*.cover
.pytest_cache/
.tox/
test-results/

# Build artifacts
dist/
build/
*.egg-info/
```

**Customize based on language:**
- Node.js: Add `node_modules/`, `npm-debug.log`, `yarn-error.log`
- Python: Add `__pycache__/`, `*.pyc`, `.venv/`, `.pytest_cache/`
- Go: Add `vendor/`, `*.exe`, `*.test`
- Java: Add `target/`, `*.class`, `*.jar` (except final artifact)

### Stage 5: Validate with devops-skills:dockerfile-validator

**Objective:** Ensure generated Dockerfile follows best practices and has no issues.

**REQUIRED: Always validate after generation.**

**Validation Process:**

1. **Invoke devops-skills:dockerfile-validator skill:**
   ```
   Use the Skill tool to invoke devops-skills:dockerfile-validator
   This will run:
   - hadolint (syntax and best practices)
   - Checkov (security scanning)
   - Custom validation (layer optimization, etc.)
   ```

2. **Parse validation results:**
   - Categorize issues by severity (error, warning, info)
   - Identify actionable fixes
   - Prioritize security issues

3. **Expected validation output:**
   ```
   [1/4] Syntax Validation (hadolint)
   [2/4] Security Scan (Checkov)
   [3/4] Best Practices Validation
   [4/4] Optimization Analysis
   ```

### Stage 6: Iterate on Validation Errors

**Objective:** Fix any validation errors and re-validate.

**REQUIRED: Iterate at least ONCE if validation finds errors.**

**Iteration Process:**

1. **If validation finds errors:**
   - Analyze each error
   - Apply fixes to Dockerfile
   - Re-run validation
   - Repeat until clean OR maximum 3 iterations

2. **If validation finds warnings:**
   - Assess if warnings are acceptable
   - Apply fixes for critical warnings
   - Document suppressed warnings with justification

3. **Common fixes:**
   - Add version tags to base images
   - Add USER directive before CMD
   - Add HEALTHCHECK for services
   - Combine RUN commands
   - Clean up package caches
   - Use COPY instead of ADD

**Example iteration:**
```
Iteration 1:
- Error: DL3006 - Missing version tag
- Fix: Change FROM node:alpine to FROM node:20-alpine
- Re-validate

Iteration 2:
- Warning: DL3059 - Multiple consecutive RUN commands
- Fix: Combine RUN commands with &&
- Re-validate

Iteration 3:
- All checks passed ✓
```

### Stage 7: Final Review and Recommendations

**Objective:** Provide comprehensive summary and next steps.

**Deliverables:**

1. **Generated Files:**
   - Dockerfile (validated and optimized)
   - .dockerignore (comprehensive)

2. **Validation Summary:**
   - All validation results
   - Any remaining warnings (with justification)
   - Security scan results

3. **Usage Instructions:**
   ```bash
   # Build the image
   docker build -t myapp:1.0 .

   # Run the container
   docker run -p 3000:3000 myapp:1.0

   # Test health check (if applicable)
   curl http://localhost:3000/health
   ```

4. **Optimization Metrics (REQUIRED - provide explicit estimates):**

   Always include a summary like this:
   ```
   ## Optimization Metrics

   | Metric | Estimate |
   |--------|----------|
   | Image Size | ~150MB (vs ~500MB without multi-stage, 70% reduction) |
   | Build Cache | Layer caching enabled for dependencies |
   | Security | Non-root user, minimal base image, no secrets |
   ```

   **Language-specific size estimates:**
   - **Node.js**: ~50-150MB with Alpine (vs ~1GB with full node image)
   - **Python**: ~150-250MB with slim (vs ~900MB with full python image)
   - **Go**: ~5-20MB with distroless/scratch (vs ~800MB with full golang image)
   - **Java**: ~200-350MB with JRE (vs ~500MB+ with JDK)

5. **Next Steps (REQUIRED - always include as bulleted list):**

   Always provide explicit next steps:
   ```
   ## Next Steps

   - [ ] Test the build locally: `docker build -t myapp:1.0 .`
   - [ ] Run and verify the container works as expected
   - [ ] Update CI/CD pipeline to use the new Dockerfile
   - [ ] Consider BuildKit cache mounts for faster builds (see Modern Docker Features)
   - [ ] Set up automated vulnerability scanning with `docker scout` or `trivy`
   - [ ] Add to container registry and deploy
   ```

## Generation Scripts (Optional Reference)

The `scripts/` directory contains standalone bash scripts for manual Dockerfile generation outside of this skill:

- `generate_nodejs.sh` - CLI tool for Node.js Dockerfiles
- `generate_python.sh` - CLI tool for Python Dockerfiles
- `generate_golang.sh` - CLI tool for Go Dockerfiles
- `generate_java.sh` - CLI tool for Java Dockerfiles
- `generate_dockerignore.sh` - CLI tool for .dockerignore generation

**Purpose:** These scripts are reference implementations and manual tools for users who want to generate Dockerfiles via command line without using Claude Code. They demonstrate the same best practices embedded in this skill.

**When using this skill:** Claude generates Dockerfiles directly using the templates and patterns documented in this skill.md, rather than invoking these scripts. The templates in this document are the authoritative source.

**Script usage example:**
```bash
# Manual Dockerfile generation
cd .claude/skills/dockerfile-generator/scripts
./generate_nodejs.sh --version 20 --port 3000 --output Dockerfile
```

## Best Practices Reference

For detailed best practices, see the reference documentation:
- **Security**: `references/security_best_practices.md` - Non-root users, minimal images, secrets management
- **Optimization**: `references/optimization_patterns.md` - Layer caching, multi-stage builds, BuildKit features
- **Multi-stage builds**: `references/multistage_builds.md` - Comprehensive multi-stage patterns

**Quick Security Checklist:**
- ✓ Use specific version tags (e.g., `node:20-alpine`, not `node:latest`)
- ✓ Run as non-root user
- ✓ Use minimal base images (alpine, distroless)
- ✓ Never hardcode secrets in ENV

**Quick Optimization Checklist:**
- ✓ Order layers from least to most frequently changing
- ✓ Copy dependency files before application code
- ✓ Combine related RUN commands with &&
- ✓ Clean package manager caches in same layer

## Common Patterns

For complete pattern examples, see `references/language_specific_guides.md`:
- Next.js with multi-stage builds
- FastAPI with health checks
- Go CLI tools with distroless
- Spring Boot with JRE optimization

## Modern Docker Features

For BuildX multi-platform builds, SBOM generation, and BuildKit cache mounts, see `references/modern_docker_features.md`.

## Error Handling

### Common Generation Issues

1. **Missing dependency files:**
   - Ensure package.json, requirements.txt, go.mod, pom.xml exist
   - Ask user to provide or generate template

2. **Unknown framework:**
   - Use WebSearch or context7 to research
   - Fall back to generic template
   - Ask user for specific requirements

3. **Validation failures:**
   - Apply fixes automatically
   - Iterate until clean
   - Document any suppressions

## Integration with Other Skills

This skill works well in combination with:
- **devops-skills:dockerfile-validator** - Validates generated Dockerfiles (REQUIRED)
- **k8s-generator** - Generate Kubernetes deployments for the container
- **helm-generator** - Create Helm charts with the container image

## Anti-Patterns

### NEVER use `latest` as the base image tag

- **WHY**: `FROM node:latest` resolves to a different image on each build, producing non-reproducible images and silently pulling in breaking changes or vulnerabilities.
- **BAD**: `FROM node:latest`
- **GOOD**: `FROM node:20.18-alpine3.20` (specific version and variant)

### NEVER split related package installation across multiple `RUN` layers

- **WHY**: Each `RUN` creates a new layer; separate `apt-get update` and `apt-get install` layers can leak stale package lists and cache files into the image, increasing size unnecessarily.
- **BAD**: `RUN apt-get update\nRUN apt-get install -y curl\nRUN rm -rf /var/lib/apt/lists/*`
- **GOOD**: `RUN apt-get update && apt-get install -y --no-install-recommends curl && rm -rf /var/lib/apt/lists/*`

### NEVER run containers as root

- **WHY**: Running as the default root user allows any exploited process full container access; always create a non-root user for application processes.
- **BAD**: No `USER` instruction (defaults to root)
- **GOOD**: `RUN addgroup -S appgroup && adduser -S appuser -G appgroup` followed by `USER appuser`

### NEVER copy the entire build context before installing dependencies

- **WHY**: Copying all files before `npm install` or `pip install` busts the layer cache on every code change, making builds slower than necessary.
- **BAD**: `COPY . .` followed by `RUN npm ci`
- **GOOD**: `COPY package*.json ./` then `RUN npm ci` then `COPY . .` — dependency layer is cached until package.json changes

### NEVER use `ADD` when `COPY` is sufficient

- **WHY**: `ADD` has implicit behaviors (auto-extracting tarballs, fetching URLs) that make Dockerfiles harder to reason about; `COPY` is explicit and predictable.
- **BAD**: `ADD app.tar.gz /app/`
- **GOOD**: `COPY app.tar.gz /app/` and explicitly extract if needed: `RUN tar -xzf /app/app.tar.gz -C /app/`

## Notes

- **Always use multi-stage builds** for compiled languages
- **Always create non-root user** for security
- **Always generate .dockerignore** to prevent secret leaks
- **Always validate** with devops-skills:dockerfile-validator
- **Iterate at least once** if validation finds errors
- Use alpine or distroless base images when possible
- Pin all version tags (never use :latest)
- Clean up package manager caches in same layer
- Order Dockerfile instructions from least to most frequently changing
- Use BuildKit features for advanced optimization
- Test builds locally before committing
- Keep Dockerfiles simple and maintainable
- Document any non-obvious patterns with comments
