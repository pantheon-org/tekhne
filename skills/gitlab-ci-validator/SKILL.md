---
name: gitlab-ci-validator
description: Validates .gitlab-ci.yml syntax, detects security misconfigurations in job definitions, checks for deprecated keywords, ensures proper stage ordering, and audits pipeline configurations for best practices. Use when working with .gitlab-ci.yml files, validating GitLab CI/CD pipeline syntax, debugging configuration errors, checking for hardcoded secrets or credentials in pipeline jobs, optimizing pipeline performance with DAG or cache, or performing security audits on GitLab CI/CD configurations.
---

# GitLab CI/CD Validator

Validates, lints, tests, and secures GitLab CI/CD pipeline configurations (`.gitlab-ci.yml` files) across three layers: syntax/schema validation, best practices analysis, and security scanning.

## Core Validation Workflow

### 1. Syntax Validation (Required first)

```bash
bash scripts/validate_gitlab_ci.sh --syntax-only .gitlab-ci.yml
```

Checks: YAML structure, GitLab CI schema compliance, job definitions, stage references, dependency graphs (`needs`/`dependencies`/`extends`), include configurations (component, project, remote, local, template), circular dependency detection, and GitLab limits (500 jobs max, 255-char job names, 50 max needs, 100 max components).

**Action:** Fix all syntax errors before proceeding.

### 2. Best Practices Review (Recommended)

```bash
bash scripts/validate_gitlab_ci.sh --best-practices .gitlab-ci.yml
```

Checks: Cache usage for dependency installation, artifact expiration settings, DAG optimization with `needs`, parallel execution opportunities, Docker image version pinning, deprecated `only`/`except` → `rules` migration, missing timeouts and retries, resource group usage.

**Action:** Review suggestions and apply relevant optimizations.

### 3. Security Audit (Required)

```bash
bash scripts/validate_gitlab_ci.sh --security-only .gitlab-ci.yml
```

Checks: Hardcoded secrets and credentials, component security (version pinning, trusted sources), remote include integrity, insecure script patterns (`curl | bash`, `eval`), SSL/TLS verification bypasses, dangerous file permissions (`chmod 777`), overly broad artifact paths, variable masking, path traversal in local includes.

**Action:** Fix all critical and high-severity issues immediately.

### 4. Local Pipeline Testing (Optional)

```bash
# Install gitlab-ci-local first (requires Docker and Node.js)
bash scripts/install_tools.sh

# Test pipeline locally
gitlab-ci-local

# Or via the validator script
bash scripts/validate_gitlab_ci.sh --test-only .gitlab-ci.yml
```

Simulates local pipeline execution to test job ordering, dependencies, and environment setup. Requires Docker and `gitlab-ci-local`.

### 5. Complete Validation

```bash
# Full validation (all three layers)
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml

# Strict mode (fail on warnings)
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --strict
```

## Usage

### Validation Options

```bash
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --syntax-only
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --best-practices
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --security-only
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --no-best-practices
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --no-security
bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --strict
```

### Individual Validators

```bash
python3 scripts/validate_syntax.py .gitlab-ci.yml
python3 scripts/check_best_practices.py .gitlab-ci.yml
python3 scripts/check_security.py .gitlab-ci.yml
```

## Output Example

```
════════════════════════════════════════════════════════════════════════════════
  Validation Summary
════════════════════════════════════════════════════════════════════════════════

Syntax Validation:      PASSED
Best Practices:         WARNINGS
Security Scan:          PASSED

✓ All validation checks passed
```

## CI/CD Integration

```yaml
stages:
  - validate

validate_pipeline:
  stage: validate
  script:
    - pip3 install PyYAML
    - bash .claude/skills/gitlab-ci-validator/scripts/validate_gitlab_ci.sh .gitlab-ci.yml --strict
```

## Adding Custom Validation Rules

Add custom rules directly to the relevant script:

```python
# In check_best_practices.py
def _check_custom_rule(self):
    """Check for custom organization rule"""
    for job_name, job in self.config.items():
        if not self._is_job(job_name):
            continue
        if 'tags' not in job:
            self.issues.append(BestPracticeIssue(
                'warning',
                self._get_line(job_name),
                f"Job '{job_name}' should specify runner tags",
                'custom-missing-tags',
                "Add 'tags' to select appropriate runners"
            ))
```

- Syntax rules: `scripts/validate_syntax.py`
- Best practice rules: `scripts/check_best_practices.py`
- Security rules: `scripts/check_security.py`

## Requirements

- **Python 3.7+**
- **PyYAML**: `pip3 install PyYAML`
- **Bash**: For the orchestrator script

## Documentation & Examples

- `docs/gitlab-ci-reference.md` — Complete GitLab CI/CD YAML syntax reference
- `docs/best-practices.md` — Detailed best practices guide
- `docs/common-issues.md` — Common issues and solutions
- `docs/RULES.md` — Full validation rules catalog (syntax, best practice, security)
- `examples/basic-pipeline.gitlab-ci.yml` — Simple three-stage pipeline
- `examples/docker-build.gitlab-ci.yml` — Docker build and push workflow
- `examples/multi-stage.gitlab-ci.yml` — Multi-stage pipeline with DAG
- `examples/complex-workflow.gitlab-ci.yml` — Advanced workflow with all features
- `examples/component-pipeline.gitlab-ci.yml` — GitLab 17.0+ pipeline using CI/CD components

```bash
# Test with examples
bash scripts/validate_gitlab_ci.sh examples/basic-pipeline.gitlab-ci.yml
bash scripts/validate_gitlab_ci.sh examples/component-pipeline.gitlab-ci.yml
```

## Fetching Latest Documentation

When encountering custom GitLab features or version-specific requirements, this skill can:

1. **Use Context7 MCP** to fetch version-aware GitLab documentation
2. **Use WebSearch** to find latest GitLab CI/CD documentation
3. **Use WebFetch** to retrieve specific pages from docs.gitlab.com

---

**Note:** This skill validates GitLab CI/CD configurations but does not execute pipelines. Use GitLab's CI Lint tool or `gitlab-ci-local` for testing actual pipeline execution.
