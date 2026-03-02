---
name: dockerfile-validator
description: Validates, lints, and secures Dockerfiles by running syntax checking, detecting security vulnerabilities, validating layer ordering, checking for hardcoded secrets, verifying base image tags, and analyzing build optimization. Use when validating Dockerfile syntax, checking security best practices, optimizing image builds, auditing container security, or debugging Dockerfile errors. Applies to all Dockerfile variants (Dockerfile, Dockerfile.prod, Dockerfile.dev, etc.).
---

# Dockerfile Validator

## Overview

This skill validates Dockerfiles using a **single self-contained script** (`dockerfile-validate.sh`) that handles everything: tool installation, validation, and cleanup.

**Key Features:**
- ✅ Single script execution - no dependencies on other scripts
- ✅ Auto-installs hadolint and Checkov in Python venvs if not found
- ✅ Runs all 4 validation stages (syntax, security, best practices, optimization)
- ✅ Auto-cleanup on exit using bash trap (success or failure)
- ✅ Zero configuration required

## Quick Start

**Single command to validate any Dockerfile:**

```bash
bash scripts/dockerfile-validate.sh Dockerfile
```

The script automatically checks for hadolint and Checkov, installs them temporarily in Python venvs if needed, runs all 4 validation stages, then cleans up on exit via `trap cleanup EXIT INT TERM` (ensures cleanup on normal exit, validation failure, Ctrl+C, and script errors).

## Validation Workflow

The `dockerfile-validate.sh` script runs a comprehensive 4-stage validation:

1. **Auto-Install (if needed)** — Check for hadolint and Checkov; install in Python venvs if absent; set `TEMP_INSTALL=true` to trigger cleanup on exit
2. **[1/4] Syntax Validation (hadolint)** — Dockerfile syntax checking, instruction validation, shell script validation via ShellCheck, 100+ built-in linting rules
3. **[2/4] Security Scan (Checkov)** — Security policy validation, hardcoded secret detection, port exposure checks, USER directive validation, 50+ security policies
4. **[3/4] Best Practices Validation (custom)** — Base image tag validation, non-root USER enforcement, HEALTHCHECK presence, layer efficiency, package cache cleanup, COPY ordering
5. **[4/4] Optimization Analysis (custom)** — Base image size analysis, multi-stage build opportunities, layer count, .dockerignore check, build structure recommendations
6. **Auto-Cleanup (bash trap — always runs)** — Remove temp venvs if `TEMP_INSTALL=true`

## Core Capabilities

### 1. Syntax Validation with hadolint

**Workflow:**

```bash
# Run hadolint on Dockerfile
hadolint Dockerfile

# Run with JSON output for parsing
hadolint --format json Dockerfile

# Run with specific rules ignored
hadolint --ignore DL3006 --ignore DL3008 Dockerfile

# Using Docker if not installed
docker run --rm -i hadolint/hadolint < Dockerfile
```

For the full list of DL-prefixed (hadolint) and SC-prefixed (ShellCheck) rules, see `references/docker_best_practices.md`.

**Rule Severity Levels:** `error` → `warning` → `info` → `style`

**Best Practices:**
- Run hadolint before every docker build
- Integrate into CI/CD pipelines
- Configure `.hadolint.yaml` for project-specific rules
- Address errors before warnings

### 2. Security Scanning with Checkov

**Installation (permanent):**

```bash
pip3 install checkov          # direct
brew install checkov          # macOS
```

**Workflow:**

```bash
# Scan a Dockerfile
checkov -f Dockerfile --framework dockerfile

# Scan a directory (finds all Dockerfiles)
checkov -d . --framework dockerfile

# Compact output (failures only)
checkov -f Dockerfile --framework dockerfile --compact

# Skip specific checks
checkov -f Dockerfile --framework dockerfile --skip-check CKV_DOCKER_2
```

For the full list of CKV_DOCKER_* security checks, see `references/security_checklist.md`.

**Suppressing False Positives:**

```dockerfile
# checkov:skip=CKV_DOCKER_2:Health check not applicable for this init container
FROM alpine:3.21
```

**Best Practices:**
- Run Checkov after hadolint (syntax first, then security)
- Address high-severity findings first
- Document all suppressions with clear justification
- Integrate into CI/CD pipelines

### 3. Best Practices Validation

**Custom Validation Checks:**

```bash
# Check for :latest tag usage
grep -E "^FROM.*:latest" Dockerfile

# Count FROM statements (single FROM = multi-stage opportunity)
grep -c "^FROM" Dockerfile

# Ensure USER is set before CMD/ENTRYPOINT
grep "^USER" Dockerfile

# Verify HEALTHCHECK is defined for services
grep "^HEALTHCHECK" Dockerfile

# Count RUN commands (>5 suggests combination opportunity)
grep -c "^RUN" Dockerfile

# Verify cache cleanup in same RUN layer
grep "rm -rf /var/lib/apt/lists" Dockerfile
grep "--no-cache" Dockerfile  # for apk
```

**Non-Obvious Checks (project-specific):**

| Category | Non-Obvious Convention |
|----------|----------------------|
| Base Images | Prefer digest pinning (`FROM alpine@sha256:…`) over version tags for reproducible builds |
| Layer Ordering | COPY dependency manifests before source code so edits don't bust the install cache |
| Cache Cleanup | Must happen in the **same** `RUN` layer as the install; a separate `RUN rm -rf …` creates a new layer that doesn't reduce size |
| Multi-Stage | Named stages (`AS build`) allow selective `--target build` for CI debugging without exposing secrets in final image |
| Secrets | BuildKit `--mount=type=secret` avoids secrets ever appearing in any layer, including intermediate ones |

### 4. Optimization Analysis

**Optimization Categories:**

**Image Size Reduction:**
```dockerfile
# Bad: Full distro
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y curl

# Good: Minimal distro
FROM alpine:3.21
RUN apk add --no-cache curl

# Better: Multi-stage with distroless
FROM golang:1.21 AS build
WORKDIR /app
COPY . .
RUN go build -o myapp

FROM gcr.io/distroless/base-debian11
COPY --from=build /app/myapp /
ENTRYPOINT ["/myapp"]
```

**Layer Optimization:**
```dockerfile
# Bad: Separate RUN commands (creates many layers)
RUN apt-get update
RUN apt-get install -y curl
RUN apt-get install -y git

# Good: Combined RUN (single layer)
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*
```

**Build Cache Efficiency:**
```dockerfile
# Bad: Copy all, then install dependencies
COPY . /app
RUN pip install -r requirements.txt

# Good: Copy dependency file first
COPY requirements.txt /app/
RUN pip install -r requirements.txt
COPY . /app
```

### 5. .dockerignore Validation

```bash
# Check if .dockerignore exists
if [ ! -f .dockerignore ]; then
    echo "WARNING: .dockerignore file not found"
fi
```

**Common patterns to include:**
```
.git
.gitignore
.env
*.log
node_modules
Dockerfile*
docker-compose*.yml
```

## Tool Prerequisites

The validation script automatically installs tools if not found. No manual installation required.

**For permanent installations:**

```bash
# hadolint
brew install hadolint  # macOS
wget -O ~/.local/bin/hadolint https://github.com/hadolint/hadolint/releases/latest/download/hadolint-Linux-x86_64 && chmod +x ~/.local/bin/hadolint  # Linux

# Checkov
pip3 install checkov
```

**Minimum Versions:** hadolint >= 2.12.0 | Checkov: latest | Python >= 3.8

**Testing Auto-Install:**

```bash
# Force temporary installation for testing
FORCE_TEMP_INSTALL=true bash scripts/dockerfile-validate.sh Dockerfile
```

## Handling Missing Tools

When tools are not installed, the script auto-installs them temporarily. If auto-install fails:

1. **Complete available validations** — continue with custom best practices checks and note skipped stages
2. **Provide installation guidance:**
   - hadolint: `brew install hadolint` (macOS) or wget from GitHub releases (Linux)
   - Checkov: `pip3 install checkov`
3. **Offer to rerun** after installation

**Tool Priority:**
- **Required (always run):** Custom best practices validation, file existence checks
- **Recommended:** hadolint (syntax/linting), Checkov (security scanning)
- **Optional:** docker (test builds), trivy (vulnerability scanning)

## Error Troubleshooting

| Error | Solution |
|-------|----------|
| `FROM instruction must be first non-comment` | Move `ARG VERSION=18` before `FROM node:${VERSION}` |
| Unknown instruction (typo) | Check spelling: common typos are `RUNS`, `COPIES`, `FRUM` |
| Chained RUN command fails | Use `apt-get install -y package \|\| exit 1` or `set -e` |
| `COPY failed: file not found` | Check path is relative to build context; verify not excluded by `.dockerignore` |
| Hardcoded secrets detected | Use BuildKit secrets: `docker build --secret id=api_key,src=api_key.txt` instead of `ENV API_KEY=secret123` |
| Slow builds | Optimize layer caching (COPY package files first), use `.dockerignore`, enable BuildKit (`export DOCKER_BUILDKIT=1`), use multi-stage builds |

## Resources

### scripts/

#### dockerfile-validate.sh

- Single self-contained validation script
- Auto-installs hadolint and Checkov if needed
- Runs all 4 validation stages (syntax, security, best practices, optimization)
- Auto-cleanup on exit
- Usage: `bash scripts/dockerfile-validate.sh [Dockerfile]`

### assets/

| File | Purpose |
|------|---------|
| `good-example.Dockerfile` | Best practices and optimal structure |
| `bad-example.Dockerfile` | Common mistakes and anti-patterns |
| `security-issues.Dockerfile` | Intentional security vulnerabilities for testing |
| `python-optimized.Dockerfile` | Python-specific optimizations and multi-stage build |
| `golang-distroless.Dockerfile` | Minimal Go application using distroless base image |
| `.dockerignore.example` | Example .dockerignore for build context optimization |

### references/

| File | Contents |
|------|---------|
| `docker_best_practices.md` | Official Docker best practices; full hadolint DL/SC rule listings |
| `optimization_guide.md` | Layer optimization and image size reduction techniques |
| `security_checklist.md` | Container security best practices; full CKV_DOCKER_* check listings |

## Mandatory Workflow

**Follow these steps in order for every validation request:**

### 1. Pre-Validation

- **Read the Dockerfile first** — Use the Read tool to examine the Dockerfile before running validation.

### 2. Validation

- **Run the validation script** — Execute `bash scripts/dockerfile-validate.sh <Dockerfile>` to run all 4 stages.

### 3. Post-Validation

- **Summarize findings by severity:**
  - Critical (security vulnerabilities, hardcoded secrets)
  - High (missing USER, HEALTHCHECK, :latest tags)
  - Medium (layer optimization, version pinning)
  - Low (style, informational)

- **Propose specific fixes** — Use the Read tool to load appropriate reference files before proposing fixes:

  | Issue Type | Reference File |
  |------------|----------------|
  | Security issues (secrets, USER, ports) | `references/security_checklist.md` |
  | Size/performance optimization | `references/optimization_guide.md` |
  | General best practices | `references/docker_best_practices.md` |

- **Offer to apply fixes** — Ask the user if they want fixes applied, then apply if approved.

**Example interaction:**
```
User: "Validate my Dockerfile"

1. Read the Dockerfile using Read tool
2. Run: bash scripts/dockerfile-validate.sh Dockerfile
3. Review output from all 4 stages
4. Summarize findings by severity (critical → low)
5. Use Read tool to load relevant reference files:
   - Read references/security_checklist.md (if security issues found)
   - Read references/optimization_guide.md (if optimization issues found)
   - Read references/docker_best_practices.md (if best practice issues found)
6. Propose specific fixes with code examples from reference content
7. Ask: "Would you like me to apply these fixes?"
8. Apply fixes if user approves
```

For multi-Dockerfile projects: find all `Dockerfile*` files, validate each sequentially, aggregate results, and provide a unified report with project-wide improvement suggestions.

## Integration with Other Skills

- **dockerfile-generator** — Generate optimized Dockerfiles
- **k8s-yaml-validator** — Validate Kubernetes deployments that reference Docker images
- **helm-validator** — Validate Helm charts with container configurations

## Notes

- Always validate before building images
- Address security issues before optimizations
- Test builds after applying fixes
- Version pin base images for reproducibility
- Use multi-stage builds for compiled languages
- Keep production images minimal (distroless, Alpine)
- Never commit Dockerfiles with hardcoded secrets
- Document inline suppressions with clear justification
- Regularly update base images for security patches
- Integrate validation into CI/CD pipelines

## Sources

**Official Docker Documentation:**
- [Docker Best Practices](https://docs.docker.com/build/building/best-practices/)
- [Dockerfile Reference](https://docs.docker.com/reference/dockerfile/)
- [Multi-stage Builds](https://docs.docker.com/build/building/multi-stage/)

**Security Guidelines:**
- [Checkov Dockerfile Scanning](https://www.checkov.io/7.Scan%20Examples/Dockerfile.html)
- [hadolint Rules](https://github.com/hadolint/hadolint)

**Best Practices Resources:**
- [Dockerfile Best Practices 2025](https://blog.bytescrum.com/dockerfile-best-practices-2025-secure-fast-and-modern)
- [Docker Security Best Practices](https://docs.docker.com/develop/security-best-practices/)
