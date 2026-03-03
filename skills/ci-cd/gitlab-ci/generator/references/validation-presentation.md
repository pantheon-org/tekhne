# Validation Presentation Requirements

When presenting the final pipeline to the user, include the following sections:

## 1. Validation Status

Provide pass/fail status with issue counts by severity:

```
✓ Validation passed
- 0 CRITICAL issues
- 0 HIGH issues
- 2 MEDIUM issues
- 3 LOW issues
- 5 SUGGESTIONS
```

## 2. MEDIUM Issues Table (if any)

| Issue | Status | Explanation |
|-------|--------|-------------|
| `image-variable-no-digest` | Acceptable | Using `python:${PYTHON_VERSION}-alpine` allows flexible version management; `PYTHON_VERSION` is internally pinned to "3.12". |
| `git-strategy-none` | Acceptable | `stop-staging` only runs kubectl commands requiring no source code. |

## 3. Suggestions Review Table (if any)

| Suggestion | Apply/Skip | Reason |
|------------|-----------|---------|
| Add caching for dependencies | Applied | Reduces build time by 40% |
| Use `needs` for parallel execution | Applied | Speeds up pipeline by running tests in parallel |
| Add retry logic for network calls | Skipped | Not applicable for this pipeline |

## 4. Usage Instructions

Provide:
- **Required CI/CD variables** with descriptions
- **Setup steps** for first-time configuration
- **Pipeline behavior** per branch/tag (when jobs trigger, manual vs automatic)

### Example Usage Instructions

**Required Variables (Settings → CI/CD → Variables):**

| Variable | Type | Description |
|----------|------|-------------|
| `DOCKER_REGISTRY_USER` | Variable (masked) | Registry username |
| `DOCKER_REGISTRY_PASSWORD` | Variable (masked) | Registry password |
| `KUBE_CONTEXT` | Variable | Kubernetes context name |

**Setup Steps:**
1. Add required variables in project settings
2. Ensure GitLab Runner has Docker executor configured
3. Configure Kubernetes agent for deployment jobs

**Pipeline Behavior:**
- **main branch:** Builds, tests, and deploys to production (manual approval)
- **develop branch:** Builds, tests, and deploys to staging (automatic)
- **feature branches:** Builds and tests only
- **tags:** Creates release artifacts and deploys to production

## Severity-Based Actions

| Severity | Action |
|----------|--------|
| **CRITICAL** | Must fix before presenting |
| **HIGH** | Must fix before presenting |
| **MEDIUM** | Fix or explain why acceptable |
| **LOW** | Acknowledge in output |
| **SUGGESTIONS** | Review and apply if beneficial |
