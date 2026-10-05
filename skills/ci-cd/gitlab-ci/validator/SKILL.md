---
name: gitlab-ci-validator
description: Validates .gitlab-ci.yml syntax, detects security misconfigurations in job definitions, checks for deprecated keywords, ensures proper stage ordering, and audits pipeline configurations for best practices. Use when working with .gitlab-ci.yml files, validating GitLab CI/CD pipeline syntax, debugging configuration errors, checking for hardcoded secrets or credentials in pipeline jobs, optimizing pipeline performance with DAG or cache, or performing security audits on GitLab CI/CD configurations.
---

# GitLab CI/CD Validator

Validates, lints, tests, and secures GitLab CI/CD pipeline configurations (`.gitlab-ci.yml` files) across three layers: syntax/schema validation, best practices analysis, and security scanning.

## Philosophy

- Validate in layer order: syntax first, then best practices, then security. A syntax error can hide later findings.
- Treat security findings as blockers and best-practice findings as guidance that tightens over time.
- Prefer fast static checks before slower local execution with `gitlab-ci-local`.
- Validate everything the pipeline pulls in, not just the entry-point file.

## When to Use

- Reviewing a `.gitlab-ci.yml` change before merging.
- Debugging a pipeline that fails at queue time with a configuration error.
- Auditing jobs for hardcoded secrets, `curl | bash` patterns or SSL bypasses.
- Checking `include:` entries, components and `needs` graphs for problems.
- Adding a validation job to a GitLab CI pipeline.

## When Not to Use

- Running the pipeline on GitLab runners: this skill checks configuration statically.
- Writing a new pipeline from scratch: use the GitLab CI generator skill instead.
- Validating Azure Pipelines or GitHub Actions files: use the matching validator skill.
- Diagnosing application test failures inside a job: the validator only reads the YAML.

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
    - bash scripts/validate_gitlab_ci.sh .gitlab-ci.yml --strict
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

- **Python 3.10+**
- **PyYAML**: `pip3 install PyYAML`
- **Bash**: For the orchestrator script

## Anti-Patterns

### NEVER skip syntax validation before running `gitlab-ci-local`

**WHY:** Local execution without pre-validation produces obscure runtime errors; resolve schema and syntax errors first to get actionable output.
**BAD:** Run `gitlab-ci-local` on an untested config and debug runtime failures.
**GOOD:** Run `bash scripts/validate_gitlab_ci.sh --syntax-only .gitlab-ci.yml` first; proceed to local testing only after syntax passes.

### NEVER ignore `only`/`except` deprecation warnings

**WHY:** Pipelines using deprecated keywords will break when GitLab removes them; treat these as errors, not warnings.
**BAD:** Leave `only: [main]` after the validator flags it as deprecated.
**GOOD:** Migrate to `rules:` syntax during the same fix session.

### NEVER validate the pipeline file without also checking all referenced `include:` targets

**WHY:** A valid base file with an invalid include template causes mysterious pipeline failures at queue time.
**BAD:** Validate only `.gitlab-ci.yml` and skip `templates/*.yml` includes.
**GOOD:** Validate all local include files as well; the validator checks `include:local` paths automatically.

### NEVER run `--strict` as the first validation step on an unfamiliar pipeline

**WHY:** Strict mode fails on warnings and produces noise that obscures real errors; establish a baseline first.
**BAD:** Run `--strict` on an inherited pipeline and skip the output because it has 50 warnings.
**GOOD:** Run without `--strict` first; fix errors and critical warnings, then enable strict mode as a CI gate.

### NEVER merge a pipeline with hardcoded secrets in job definitions

**WHY:** The security layer flags hardcoded credentials and tokens in scripts and variables (for example `[variable-hardcoded-secret]`, `[api-key]`). Committed secrets persist in history even after removal.
**BAD:** `variables: { API_KEY: "abc123" }` in `.gitlab-ci.yml`.
**GOOD:** Define the value as a masked, protected CI/CD variable in project settings and reference `$API_KEY`.

### NEVER pipe a remote download straight into a shell

**WHY:** The scan reports `[curl-pipe-bash]` and `[wget-pipe-bash]`. The job runs whatever the remote host serves at that moment.
**BAD:** `script: - curl -s https://example.com/install.sh | bash`.
**GOOD:** Use a pinned image that already contains the tool, or download, verify and then run the file.

### NEVER use unpinned images or `latest` tags

**WHY:** The best-practices layer reports `[image-latest-tag]` and `[image-no-version]`. A floating tag means the same commit can build differently on different days.
**BAD:** `image: node:latest`.
**GOOD:** `image: node:20.11-alpine`, or a digest.

### NEVER include components or project files without a pinned version

**WHY:** The validator reports `[include-component-no-version]`, `[include-component-latest-version]` and `[include-project-unpinned]`. An upstream change to the included file alters your pipeline with no change in your repository.
**BAD:** `include: - project: group/ci-templates` with no `ref`.
**GOOD:** Pin `ref:` to a tag or commit, and pin component versions to a semantic version.

### NEVER leave artifacts without an expiry

**WHY:** The best-practices layer reports `[artifact-no-expiration]`. Artifacts with no expiry accumulate and consume storage.
**BAD:** `artifacts: { paths: [dist/] }` with no `expire_in`.
**GOOD:** Add `expire_in: 1 week` (or a value that fits the job).

### NEVER disable SSL verification in job scripts

**WHY:** The security layer flags verification bypasses (`[insecure-ssl]`, `[skip-verification]`). It removes protection against tampering with downloaded content.
**BAD:** `script: - git config --global http.sslVerify false`.
**GOOD:** Fix the certificate chain or install the corporate CA in the job image.

## References

- `references/gitlab-ci-reference.md`: Complete GitLab CI/CD YAML syntax reference
- `references/best-practices.md`: Detailed best practices guide
- `references/common-issues.md`: Common issues and solutions
- `scripts/`: rule definitions live in `validate_syntax.py`, `check_best_practices.py` and `check_security.py`
- `assets/basic-pipeline.gitlab-ci.yml`: Simple three-stage pipeline
- `assets/docker-build.gitlab-ci.yml`: Docker build and push workflow
- `assets/multi-stage.gitlab-ci.yml`: Multi-stage pipeline with DAG
- `assets/complex-workflow.gitlab-ci.yml`: Advanced workflow with all features
- `assets/component-pipeline.gitlab-ci.yml`: GitLab 17.0+ pipeline using CI/CD components

```bash
# Test with examples
bash scripts/validate_gitlab_ci.sh assets/basic-pipeline.gitlab-ci.yml
bash scripts/validate_gitlab_ci.sh assets/component-pipeline.gitlab-ci.yml
```

## Fetching Latest Documentation

When encountering custom GitLab features or version-specific requirements, this skill can:

1. **Use Context7 MCP** to fetch version-aware GitLab documentation
2. **Use WebSearch** to find latest GitLab CI/CD documentation
3. **Use WebFetch** to retrieve specific pages from docs.gitlab.com

---

**Note:** This skill validates GitLab CI/CD configurations but does not execute pipelines. Use GitLab's CI Lint tool or `gitlab-ci-local` for testing actual pipeline execution.
