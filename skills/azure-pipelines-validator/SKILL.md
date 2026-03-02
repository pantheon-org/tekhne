---
name: azure-pipelines-validator
description: "Validates, lints, and security-scans Azure DevOps Pipeline configurations (azure-pipelines.yml / azure-pipelines.yaml). Use when working with ADO pipelines, YAML pipeline files, or CI/CD configurations in Azure DevOps — including validating YAML syntax and schema, detecting hardcoded secrets or credentials, checking for deprecated or unpinned task versions, enforcing best practices (caching, timeouts, display names), performing pipeline security audits, or reviewing azure-pipelines.yml before merging. Trigger terms: azure-pipelines.yml, ADO pipeline, Azure Pipelines, YAML pipeline, CI/CD validation, pipeline security scan, DevOps configuration review."
---

# Azure Pipelines Validator

Validates, lints, and security-scans Azure DevOps Pipeline configurations (`azure-pipelines.yml`, `azure-pipelines.yaml`). Runs four validation layers via a single orchestrator script.

## Basic Usage

```bash
# Full validation (all layers)
bash .claude/skills/azure-pipelines-validator/scripts/validate_azure_pipelines.sh azure-pipelines.yml
```

Layers executed in order:
0. **YAML lint** (yamllint) — formatting, indentation, trailing spaces
1. **Syntax validation** — schema, required fields, stages/jobs/steps hierarchy, task format, dependencies
2. **Best practices** — display names, task version pinning, pool image specificity, caching, timeouts
3. **Security scan** — hardcoded secrets/API keys/AWS/Azure credentials, dangerous script patterns, SSL bypasses, container `:latest` tags

### Common Options

```bash
# Targeted runs
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --syntax-only
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --best-practices
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --security-only

# Skip layers
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --skip-yaml-lint
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --no-best-practices
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --no-security

# Strict mode (fail on warnings)
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --strict
```

### Individual Scripts

```bash
python3 scripts/validate_syntax.py azure-pipelines.yml
python3 scripts/check_best_practices.py azure-pipelines.yml
python3 scripts/check_security.py azure-pipelines.yml
```

## Output and Error Recovery

```
════════════════════════════════════════════════════════════════════════════════
  Azure Pipelines Validator
════════════════════════════════════════════════════════════════════════════════

[1/3] Running syntax validation...
✓ Syntax validation passed

[2/3] Running best practices check...
SUGGESTIONS (2):
  INFO: Line 15: Job 'BuildJob' should have displayName [missing-displayname]
  💡 Add 'displayName: "Your Job Description"' to job 'BuildJob'

  WARNING: Line 25: Task 'Npm@1' could benefit from caching [missing-cache]
  💡 Add Cache@2 task to cache dependencies and speed up builds

[3/3] Running security scan...
MEDIUM SEVERITY (1):
  MEDIUM: Line 8: Container 'linux' uses ':latest' tag [container-latest-tag]
  🔒 Pin container images to specific versions or SHA digests
```

**When validation fails:**
1. Note the rule code in brackets (e.g., `[missing-displayname]`) — see `references/` for rule details.
2. Fix the flagged line and re-run the same layer (`--syntax-only`, `--security-only`, etc.) to iterate quickly.
3. Run full validation once all targeted fixes are applied to confirm no regressions.
4. For `MEDIUM`/`HIGH` security findings, do not merge until resolved; `INFO` findings are advisory.

## Common Scenarios

### New pipeline validation
```bash
bash scripts/validate_azure_pipelines.sh new-pipeline.yml
```

### Security audit before merge
```bash
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --security-only --strict
```

### Pipeline optimisation
```bash
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --best-practices
```

### CI/CD self-validation in Azure Pipelines
```yaml
steps:
- script: |
    bash .claude/skills/azure-pipelines-validator/scripts/validate_azure_pipelines.sh azure-pipelines.yml --strict
  displayName: 'Validate Pipeline Configuration'
```

## Auto-detection

Run without arguments to auto-detect `azure-pipelines*.yml` files in the current directory (up to 3 levels deep).

## Fetching Live Documentation

The validator performs **static analysis only**. For dynamic lookups (task versions, input parameters, feature docs), use:

```
# Context7 MCP
mcp__context7__resolve-library-id("azure-pipelines")
mcp__context7__get-library-docs(context7CompatibleLibraryID, topic="deployment")

# Or WebSearch / WebFetch
WebSearch("Azure Pipelines Docker@2 task documentation 2025")
WebFetch("https://learn.microsoft.com/en-us/azure/devops/pipelines/tasks/reference/docker-v2")
```

## Requirements

- **Python 3.7+**, **Bash**
- **PyYAML** and **yamllint**: auto-installed in a persistent `.venv` if not available system-wide — no manual setup required.

```bash
# Optional manual install
pip3 install PyYAML yamllint
```

## Troubleshooting

| Problem | Fix |
|---|---|
| `ModuleNotFoundError: PyYAML` | `pip3 install PyYAML` |
| `Permission denied` | `chmod +x scripts/*.sh scripts/*.py` |
| Unexpected validation errors | Check `references/azure-pipelines-reference.md` or [Microsoft Learn](https://learn.microsoft.com/en-us/azure/devops/pipelines/) |

## References and Examples

- `references/azure-pipelines-reference.md` — full YAML syntax reference and rule definitions
- `assets/examples/basic-pipeline.yml` — simple CI pipeline
- `assets/examples/docker-build.yml` — Docker build and push
- `assets/examples/deployment-pipeline.yml` — multi-environment deployment with approval gates
- `assets/examples/multi-platform.yml` — multi-platform build matrix
- `assets/examples/template-example.yml` — reusable templates

```bash
# Test with a bundled example
bash scripts/validate_azure_pipelines.sh assets/examples/basic-pipeline.yml
```

## Extending the Skill

Add custom rules to the appropriate script:

- Syntax rules → `scripts/validate_syntax.py`
- Best practice rules → `scripts/check_best_practices.py`
- Security rules → `scripts/check_security.py`

```python
# Example custom best-practice rule in check_best_practices.py
def _check_custom_rule(self):
    for job in self._get_all_jobs():
        job_name = job.get('job') or job.get('deployment')
        if 'tags' not in pool:
            self.issues.append(BestPracticeIssue(
                'warning',
                self._get_line(job_name),
                f"Job '{job_name}' should specify agent tags",
                'custom-missing-tags',
                "Add 'tags' to pool to select appropriate agents"
            ))
```

---

**Note**: This skill validates pipeline configurations but does not execute pipelines. Use Azure DevOps Pipeline validation or Azure CLI to test actual pipeline execution.
