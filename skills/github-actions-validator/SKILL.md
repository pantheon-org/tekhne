---
name: github-actions-validator
description: Comprehensive toolkit for validating, linting, and testing GitHub Actions workflow files, custom local actions, and public actions. Use this skill when working with GitHub Actions YAML files (.github/workflows/*.yml), validating workflow syntax, testing workflow execution with act, or debugging workflow issues.
---

# GitHub Actions Validator

## Overview

Validate and test GitHub Actions workflows, custom actions, and public actions using industry-standard tools (actionlint and act). This skill provides comprehensive validation including syntax checking, static analysis, local workflow execution testing, and action verification with version-aware documentation lookup.

## CRITICAL: Assistant Workflow (MUST FOLLOW)

**Every validation MUST follow these steps. Skipping any step is non-compliant.**

### Step 1: Run Validation Script

```bash
cd .claude/skills/github-actions-validator
bash scripts/validate_workflow.sh <workflow-file-or-directory>
```

### Step 2: For EACH Error - Consult Reference File

When actionlint or act reports ANY error, you MUST:

1. **Read the appropriate reference file** (see mapping below)
2. **Find the matching error pattern**
3. **Extract the fix/solution**

### Step 3: Quote the Fix to User

For each error, provide:

1. **Error message** (from script output)
2. **Explanation** (from reference file)
3. **Fix code** (quoted from reference file)
4. **Corrected code** (applied to user's workflow)

### Step 4: Verify Public Actions (if present)

For any public actions (`uses: owner/action@version`):

1. **First check `references/action_versions.md`** for known actions and versions
2. **Use web search** for unknown actions: `"[action-name] [version] github action documentation"`
3. **Verify required inputs match**
4. **Check for deprecation warnings**

### Step 5: Provide Complete Summary

After all errors are addressed:
- List all fixes applied
- Note any warnings
- Recommend best practices from `references/`

### Error-to-Reference Mapping

| Error Pattern / Output Category | Reference File | Section to Consult |
|----------------------------------|---------------|--------------------|
| `runs-on:`, `runner`, `ubuntu`, `macos`, `windows` | `references/runners.md` | Runner labels |
| `cron`, `schedule` / `[SCHEDULE]` | `references/common_errors.md` | Schedule Errors |
| `${{`, `expression`, `if:` / `[EXPRESSION]` | `references/common_errors.md` | Expression Errors |
| `needs:`, `job`, `dependency` / `[SYNTAX]` | `references/common_errors.md` | Job Configuration Errors |
| `uses:`, `action`, `input` / `[ACTION]` | `references/common_errors.md` | Action Errors |
| `untrusted`, `injection`, `security` / `[SECURITY]` | `references/common_errors.md` | Script Injection section |
| `syntax`, `yaml`, `unexpected` | `references/common_errors.md` | Syntax Errors |
| `docker`, `container` / `[DOCKER]` | `references/act_usage.md` | Troubleshooting |
| `[ACT-LIMIT]`, act fails but GitHub works | `references/act_usage.md` | Limitations |
| `@v3`, `@v4`, `deprecated`, `outdated` | `references/action_versions.md` | Version table |
| `workflow_call`, `reusable`, `oidc` | `references/modern_features.md` | Relevant section |
| `glob`, `path`, `paths:`, `pattern` | `references/common_errors.md` | Path Filter Errors |
| User asks about actionlint config | `references/actionlint_usage.md` | Provide examples |
| Runner questions/errors | `references/runners.md` | Labels and availability |

## Quick Start

### Initial Setup

```bash
cd .claude/skills/github-actions-validator
bash scripts/install_tools.sh
```

This installs **act** (local workflow execution) and **actionlint** (static analysis) to `scripts/.tools/`.

### Basic Validation

```bash
# Validate a single workflow
bash scripts/validate_workflow.sh .github/workflows/ci.yml

# Validate all workflows
bash scripts/validate_workflow.sh .github/workflows/

# Lint-only (fastest)
bash scripts/validate_workflow.sh --lint-only .github/workflows/ci.yml

# Test-only with act (requires Docker)
bash scripts/validate_workflow.sh --test-only .github/workflows/
```

## Core Validation Workflow

Run the three stages in order:

1. **Static analysis** — catches syntax errors and common issues first:
   ```bash
   bash scripts/validate_workflow.sh --lint-only .github/workflows/ci.yml
   ```
   *actionlint checks:* YAML syntax, schema compliance, expression syntax, runner labels, action inputs/outputs, job dependencies, CRON syntax, glob patterns, shell scripts, security vulnerabilities.

2. **Local execution test** — after passing static analysis, test workflow execution (requires Docker):
   ```bash
   bash scripts/validate_workflow.sh --test-only .github/workflows/
   ```
   *Note:* act has limitations — see `references/act_usage.md`.

3. **Full validation** — both stages together:
   ```bash
   bash scripts/validate_workflow.sh .github/workflows/ci.yml
   ```

## Validating Resource Types

### Workflows

```bash
# Single workflow
bash scripts/validate_workflow.sh .github/workflows/ci.yml

# All workflows
bash scripts/validate_workflow.sh .github/workflows/
```

**Key validation points:** triggers, job configurations, runner labels, environment variables, secrets, conditionals, matrix strategies.

### Custom Local Actions

Create a test workflow that uses the custom action, then validate:

```bash
bash scripts/validate_workflow.sh .github/workflows/test-custom-action.yml
```

### Public Actions

When workflows use public actions (e.g., `actions/checkout@v6`):

1. Use web search to find action documentation
2. Verify required inputs and version
3. Check for deprecation warnings
4. Run validation script

**Search format:** `"[action-name] [version] github action documentation"`

## Reference Files Summary

| File | Content |
|------|---------|
| `references/act_usage.md` | Act tool usage, commands, options, limitations, troubleshooting |
| `references/actionlint_usage.md` | Actionlint validation categories, configuration, integration |
| `references/common_errors.md` | Common errors catalog with fixes |
| `references/action_versions.md` | Current action versions, deprecation timeline, SHA pinning |
| `references/modern_features.md` | Reusable workflows, SBOM, OIDC, environments, containers |
| `references/runners.md` | GitHub-hosted runners (ARM64, GPU, M2 Pro, deprecations) |

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Tools not found" | Run `bash scripts/install_tools.sh` |
| "Docker daemon not running" | Start Docker or use `--lint-only` |
| "Permission denied" | Run `chmod +x scripts/*.sh` |
| act fails but GitHub works | See `references/act_usage.md` Limitations |

### Debug Mode

```bash
actionlint -verbose .github/workflows/ci.yml  # Verbose actionlint
act -v                                         # Verbose act
act -n                                         # Dry-run (no execution)
```

## Best Practices

1. **Always validate locally first** - Catch errors before pushing
2. **Use actionlint in CI/CD** - Automate validation in pipelines
3. **Pin action versions** - Use `@v6` not `@main` for stability; SHA pinning for security
4. **Keep tools updated** - Regularly update actionlint and act
5. **Use web search for unknown actions** - Verify usage with documentation
6. **Check version compatibility** - See `references/action_versions.md`
7. **Enable shellcheck** - Catch shell script issues early
8. **Review security warnings** - Address script injection issues

## Limitations

- **act limitations**: Not all GitHub Actions features work locally
- **Docker requirement**: act requires Docker to be running
- **Network actions**: Some GitHub API actions may fail locally
- **Private actions**: Cannot validate without access
- **Runtime behavior**: Static analysis cannot catch all issues
- **File location**: act can only validate workflows in `.github/workflows/` directory; files outside (like `assets/`) can only be validated with actionlint

## Quick Examples

### Example 1: Pre-commit Validation

```bash
cd .claude/skills/github-actions-validator
bash scripts/validate_workflow.sh .github/workflows/
git add .github/workflows/ && git commit -m "Update workflows"
```

### Example 2: Debug Failing Workflow

```bash
bash scripts/validate_workflow.sh --lint-only .github/workflows/failing.yml
# Fix issues
bash scripts/validate_workflow.sh .github/workflows/failing.yml
```

## Complete Worked Example: Multi-Error Workflow

### User's Problematic Workflow

```yaml
name: Broken CI
on:
  schedule:
    - cron: '0 0 * * 8'  # ERROR 1
jobs:
  build:
    runs-on: ubuntu-lastest  # ERROR 2
    steps:
      - uses: actions/checkout@v3  # ERROR 3 (outdated)
      - run: echo ${{ github.event.issue.title }}  # ERROR 4 (security)
  deploy:
    needs: biuld  # ERROR 5 (typo)
    runs-on: ubuntu-latest
    steps:
      - run: echo "Deploying"
```

### Step 1: Run Validation

```bash
bash scripts/validate_workflow.sh --lint-only workflow.yml
```

**Output:**
```
[ERROR] invalid CRON format "0 0 * * 8"
[ERROR] label "ubuntu-lastest" is unknown
[WARN] "github.event.issue.title" is potentially untrusted
[ERROR] job "deploy" needs job "biuld" which does not exist
```

### Step 2-3: Consult References and Quote Fixes to User

Using the Error-to-Reference Mapping table above, consult the relevant reference file for each error and quote the fix to the user:

| # | Error | Reference File | Fix |
|---|-------|---------------|-----|
| 1 | `invalid CRON format "0 0 * * 8"` | `common_errors.md` - Schedule Errors | Change `8` to `0` (weekday range is 0–6) |
| 2 | `label "ubuntu-lastest" is unknown` | `common_errors.md` + `runners.md` | Change to `ubuntu-latest` |
| 3 | `checkout@v3` (outdated) | `action_versions.md` | Update to `@v6` or SHA-pinned equivalent |
| 4 | Untrusted input in `run:` | `common_errors.md` - Script Injection | Pass through environment variable |
| 5 | `needs: biuld` (typo) | `common_errors.md` - Job Configuration | Change to `needs: build` |

### Step 4: Corrected Workflow

```yaml
name: Fixed CI
on:
  schedule:
    - cron: '0 0 * * 0'  # Fixed: Sunday (valid range 0-6)
jobs:
  build:
    runs-on: ubuntu-latest  # Fixed: typo corrected
    steps:
      - uses: actions/checkout@1af3b93b6815bc44a9784bd300feb67ff0d1eeb3  # v6.0.0
      - name: Process issue
        env:
          TITLE: ${{ github.event.issue.title }}  # Fixed: sanitized via env var
        run: echo "$TITLE"
  deploy:
    needs: build  # Fixed: typo corrected
    runs-on: ubuntu-latest
    steps:
      - run: echo "Deploying"
```

### Step 5: Summary

| Error | Type | Fix Applied |
|-------|------|-------------|
| CRON `0 0 * * 8` | Schedule | Changed to `0 0 * * 0` |
| `ubuntu-lastest` | Runner | Changed to `ubuntu-latest` |
| `checkout@v3` | Outdated Action | Updated to `@v6.0.0` (SHA-pinned) |
| Direct `${{ }}` in run | Security | Wrapped in environment variable |
| `needs: biuld` | Job Dependency | Changed to `needs: build` |

**Recommendations:**
- Run `bash scripts/validate_workflow.sh --check-versions` regularly
- Use SHA pinning for all actions in production workflows
- Always pass untrusted input through environment variables

## Summary

1. **Setup**: Install tools with `install_tools.sh`
2. **Validate**: Run `validate_workflow.sh` on workflow files
3. **Fix**: Address issues using reference documentation
4. **Test**: Verify locally with act (when possible)
5. **Search**: Use web search to verify unknown actions
6. **Commit**: Push validated workflows with confidence

For detailed information, consult the appropriate reference file in `references/`.
