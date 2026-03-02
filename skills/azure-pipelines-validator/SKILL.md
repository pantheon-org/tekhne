---
name: azure-pipelines-validator
description: Comprehensive toolkit for validating, linting, and securing Azure DevOps Pipeline configurations.
---

# Azure Pipelines Validator

Comprehensive toolkit for validating, linting, testing, and securing Azure DevOps Pipeline configurations (azure-pipelines.yml, azure-pipelines.yaml files). Use this skill when working with Azure Pipelines, validating pipeline syntax, debugging configuration issues, implementing best practices, or performing security audits.

## When to Use This Skill

Use the **azure-pipelines-validator** skill in the following scenarios:

- ✅ Working with `azure-pipelines.yml` or `azure-pipelines.yaml` files
- ✅ Validating Azure Pipelines YAML syntax and structure
- ✅ Debugging pipeline configuration errors
- ✅ Implementing Azure Pipelines best practices
- ✅ Performing security audits on pipeline configurations
- ✅ Checking for hardcoded secrets or credentials
- ✅ Optimizing pipeline performance (caching, parallelization)
- ✅ Ensuring compliance with security standards
- ✅ Code review of Azure DevOps CI/CD configurations
- ✅ Migrating or refactoring pipeline configurations

## Features

### 0. YAML Linting (Optional)

- ✅ YAML formatting validation with yamllint
- ✅ Indentation checking (2-space standard)
- ✅ Line length validation
- ✅ Trailing spaces detection
- ✅ Custom Azure Pipelines YAML rules
- ✅ Automatic venv management (no manual install required)

### 1. Syntax Validation

- ✅ YAML syntax checking
- ✅ Azure Pipelines schema validation
- ✅ Required fields verification
- ✅ Stages/Jobs/Steps hierarchy validation
- ✅ Task format validation (TaskName@version)
- ✅ Pool/agent specification validation
- ✅ Deployment job strategy validation
- ✅ Trigger and PR configuration validation
- ✅ Resource definitions validation
- ✅ Variable and parameter declarations
- ✅ Dependency validation (dependsOn)

### 2. Best Practices Checking

- ✅ displayName usage for readability
- ✅ Task version pinning (specific @N not @0)
- ✅ Pool vmImage specific versions (not 'latest')
- ✅ Cache usage for package managers
- ✅ Timeout configuration for long-running jobs
- ✅ Deployment job conditions
- ✅ Artifact retention settings
- ✅ Parallel execution opportunities
- ✅ Template usage recommendations
- ✅ Variable group organization
- ✅ Deployment strategy best practices

### 3. Security Scanning

- ✅ Hardcoded secrets and credentials detection
- ✅ API keys and tokens in variables
- ✅ Task version security
- ✅ Container image security (:latest tags)
- ✅ Dangerous script patterns (curl | bash, eval)
- ✅ Service connection security
- ✅ Secret exposure in logs
- ✅ Checkout security settings
- ✅ Variable security (isSecret flag)
- ✅ Azure credential hardcoding
- ✅ SSL/TLS verification bypasses

## Usage

### Basic Validation

To validate an Azure Pipelines configuration file:

```bash
bash .claude/skills/azure-pipelines-validator/scripts/validate_azure_pipelines.sh <file-path>
```

**Example:**
```bash
bash .claude/skills/azure-pipelines-validator/scripts/validate_azure_pipelines.sh azure-pipelines.yml
```

This runs all four validation layers:
0. YAML lint (yamllint) - optional, auto-installed in venv if needed
1. Syntax validation
2. Best practices check
3. Security scan

### Validation Options

```bash
# Run only syntax validation
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --syntax-only

# Run only best practices check
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --best-practices

# Run only security scan
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --security-only

# Skip YAML linting (yamllint)
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --skip-yaml-lint

# Skip best practices check
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --no-best-practices

# Skip security scan
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --no-security

# Strict mode (fail on warnings)
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --strict
```

### Individual Validators

You can also run individual validation scripts:

```bash
# Syntax validation
python3 scripts/validate_syntax.py azure-pipelines.yml

# Best practices check
python3 scripts/check_best_practices.py azure-pipelines.yml

# Security scan
python3 scripts/check_security.py azure-pipelines.yml
```

## Output Example

```
════════════════════════════════════════════════════════════════════════════════
  Azure Pipelines Validator
════════════════════════════════════════════════════════════════════════════════

File: azure-pipelines.yml

[1/3] Running syntax validation...

✓ Syntax validation passed

[2/3] Running best practices check...

SUGGESTIONS (2):
──────────────────────────────────────────────────────────────────────────────
  INFO: Line 15: Job 'BuildJob' should have displayName for better readability [missing-displayname]
  💡 Suggestion: Add 'displayName: "Your Job Description"' to job 'BuildJob'

  WARNING: Line 25: Task 'Npm@1' in job 'BuildJob' could benefit from caching [missing-cache]
  💡 Suggestion: Add Cache@2 task to cache dependencies and speed up builds

ℹ  Best practices check completed with suggestions

[3/3] Running security scan...

MEDIUM SEVERITY (1):
──────────────────────────────────────────────────────────────────────────────
  MEDIUM: Line 8: Container 'linux' uses ':latest' tag [container-latest-tag]
  🔒 Remediation: Pin container images to specific versions or SHA digests

✓ Security scan passed

════════════════════════════════════════════════════════════════════════════════
  Validation Summary
════════════════════════════════════════════════════════════════════════════════

Syntax Validation:      PASSED
Best Practices:         WARNINGS
Security Scan:          PASSED

════════════════════════════════════════════════════════════════════════════════

✓ All validation checks passed
```

## Common Validation Scenarios

### Scenario 1: Validating a New Pipeline

```bash
# Validate syntax and structure
bash scripts/validate_azure_pipelines.sh new-pipeline.yml
```

### Scenario 2: Security Audit Before Merge

```bash
# Run security scan only with strict mode
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --security-only --strict
```

### Scenario 3: Pipeline Optimization

```bash
# Check for best practices and optimization opportunities
bash scripts/validate_azure_pipelines.sh azure-pipelines.yml --best-practices
```

### Scenario 4: CI/CD Integration

```yaml
# In your Azure Pipeline
trigger:
  branches:
    include:
    - main

pool:
  vmImage: 'ubuntu-22.04'

steps:
- script: |
    pip3 install PyYAML
    bash .claude/skills/azure-pipelines-validator/scripts/validate_azure_pipelines.sh azure-pipelines.yml --strict
  displayName: 'Validate Pipeline Configuration'
```

## Integration with Claude Code

When Claude Code invokes this skill, it will:

1. **Auto-detect Azure Pipelines files** - Run the validator without arguments to auto-detect `azure-pipelines*.yml` files in the current directory (up to 3 levels deep)
2. **Run validation** when you ask to validate, check, or review Azure Pipelines configurations
3. **Provide actionable feedback** with line numbers and suggestions
4. **Stage-aware condition checking** - Recognizes when parent stages have conditions, avoiding false positives on deployment jobs
5. **Deduplicated findings** - Reports each security issue once, even if detected by multiple patterns

**Example prompts:**
- "Validate my Azure Pipeline"
- "Check this azure-pipelines.yml for security issues"
- "Review my pipeline configuration for best practices"
- "Why is my Azure Pipeline failing?"
- "Optimize my Azure DevOps pipeline"

### When to Use Context7/WebSearch for Documentation

The validation scripts provide static analysis. For **dynamic documentation lookup**, manually use these tools when you need:

- **Task version information**: "What's the latest version of AzureWebApp task?"
- **Task input parameters**: "What inputs does Docker@2 support?"
- **Feature documentation**: "How do I configure deployment environments in Azure Pipelines?"
- **Troubleshooting**: "Why does my AzureCLI@2 task fail with error X?"

**How to fetch documentation:**
```
# Use Context7 MCP for structured docs
mcp__context7__resolve-library-id("azure-pipelines")
mcp__context7__get-library-docs(context7CompatibleLibraryID, topic="deployment")

# Or use WebSearch/WebFetch for Microsoft Learn docs
WebSearch("Azure Pipelines Docker@2 task documentation 2025")
WebFetch("https://learn.microsoft.com/en-us/azure/devops/pipelines/tasks/reference/docker-v2")
```

**Note**: Documentation lookup is a manual action - the validator scripts focus on static analysis and do not automatically fetch external documentation.

## Validation Rules

### Syntax Rules

- `yaml-syntax`: Valid YAML formatting
- `yaml-invalid-root`: Root must be a dictionary
- `invalid-hierarchy`: Cannot mix stages/jobs/steps at root level
- `task-invalid-format`: Tasks must follow TaskName@version format
- `pool-invalid`: Pool must specify name or vmImage
- `stage-missing-jobs`: Stages must define jobs
- `job-missing-steps`: Regular jobs must define steps
- `deployment-missing-strategy`: Deployment jobs must define strategy
- `variable-invalid-name`: Variables should use valid naming

### Best Practice Rules

- `missing-displayname`: Stages/jobs should have displayName
- `task-version-zero`: Tasks should not use @0 version (except whitelisted tasks where @0 is the only version: GoTool, NodeTool, UsePythonVersion, KubernetesManifest, DockerCompose, HelmInstaller, HelmDeploy)
- `task-missing-version`: Tasks must specify version
- `pool-latest-image`: Avoid 'latest' in vmImage
- `missing-cache`: Package installations should use caching
- `missing-timeout`: Deployment jobs should specify timeout
- `missing-deployment-condition`: Production deployments should have conditions
- `parallel-opportunity`: Test jobs could use parallelization
- `template-opportunity`: Duplicate job patterns could use templates
- `many-inline-variables`: Consider using variable groups

### Security Rules

- `hardcoded-password`: Hardcoded passwords detected
- `hardcoded-api-key`: Hardcoded API keys detected
- `hardcoded-secret`: Hardcoded secrets/tokens detected
- `hardcoded-aws-credentials`: AWS credentials hardcoded
- `hardcoded-azure-ids`: Azure subscription/tenant IDs hardcoded
- `curl-pipe-shell`: Dangerous curl | bash pattern
- `eval-command`: Eval command usage with variables
- `chmod-777`: Overly permissive file permissions
- `insecure-ssl`: SSL/TLS verification disabled
- `secret-in-logs`: Potential secret exposure in logs
- `container-latest-tag`: Container using :latest tag
- `task-no-version`: Task missing version (security risk)
- `hardcoded-service-connection`: Service connection IDs hardcoded
- `checkout-no-clean`: Checkout without clean
- `variable-not-secret`: Sensitive variable not marked as secret

## Requirements

- **Python 3.7+**
- **PyYAML** and **yamllint**: Auto-installed in venv if not available systemwide
- **Bash**: For running the orchestrator script

**No manual installation required!** The validator uses automatic venv management:
- If PyYAML or yamllint are available system-wide, they'll be used
- Otherwise, a persistent `.venv` is created and packages are auto-installed
- The venv is reused across runs for optimal performance

To manually install dependencies system-wide (optional):
```bash
pip3 install PyYAML yamllint
```

## Documentation

Comprehensive documentation is included in the `references/` directory:

- [Azure Pipelines YAML Reference](references/azure-pipelines-reference.md): Complete Azure Pipelines YAML syntax reference with examples

## Examples

Example Azure Pipelines configurations are provided in the `assets/examples/` directory:

- **`basic-pipeline.yml`**: Simple CI pipeline with build and test stages
- **`docker-build.yml`**: Docker build and push workflow
- **`deployment-pipeline.yml`**: Multi-environment deployment with approval gates
- **`multi-platform.yml`**: Multi-platform build matrix
- **`template-example.yml`**: Pipeline using reusable templates

Test the skill with examples:
```bash
bash scripts/validate_azure_pipelines.sh assets/examples/basic-pipeline.yml
```

## Fetching Latest Documentation

When encountering specific Azure Pipelines tasks, resources, or version requirements, you can manually use the following tools to get up-to-date information:

1. **Use Context7 MCP** to fetch version-aware Azure Pipelines documentation
2. **Use WebSearch** to find latest Azure DevOps documentation
3. **Use WebFetch** to retrieve specific documentation pages from learn.microsoft.com

**Note**: These tools are not automatically invoked by the validation scripts. Use them manually when you need to look up specific Azure Pipelines tasks, features, or troubleshoot validation errors.

## Extending the Skill

### Adding Custom Validation Rules

Add custom rules to the validation scripts:

1. **Syntax rules**: Edit `scripts/validate_syntax.py`
2. **Best practice rules**: Edit `scripts/check_best_practices.py`
3. **Security rules**: Edit `scripts/check_security.py`

### Custom Rule Example

```python
# In check_best_practices.py
def _check_custom_rule(self):
    """Check for custom organization rule"""
    for job in self._get_all_jobs():
        job_name = job.get('job') or job.get('deployment')

        # Your custom validation logic
        if 'tags' not in pool:
            self.issues.append(BestPracticeIssue(
                'warning',
                self._get_line(job_name),
                f"Job '{job_name}' should specify agent tags",
                'custom-missing-tags',
                "Add 'tags' to pool to select appropriate agents"
            ))
```

## Troubleshooting

### Python Module Not Found

```bash
# Install PyYAML
pip3 install PyYAML

# Or with homebrew Python
python3 -m pip install PyYAML
```

### Permission Denied

```bash
# Make scripts executable
chmod +x scripts/*.sh scripts/*.py
```

### Validation Errors

Check the documentation:

- Review `references/azure-pipelines-reference.md` for syntax reference
- Consult Azure Pipelines documentation at <https://learn.microsoft.com/en-us/azure/devops/pipelines/>

## Version History

### v1.0.0 (2025-01-24)

- Initial release
- Syntax validation with comprehensive Azure Pipelines schema checking
- Best practices validation with 10+ rules
- Security scanning with 20+ security checks
- Comprehensive documentation and examples
- Integration with Context7 for latest Azure DevOps docs

## Contributing

To improve this skill:

1. Add new validation rules to appropriate scripts
2. Update documentation with new patterns
3. Add example configurations
4. Test with real-world Azure Pipelines files

## License

This skill is part of the DevOps Skills collection.

## Support

For issues, questions, or contributions:

- Check documentation in `references/` directory
- Review examples in `assets/examples/` directory
- Consult Azure Pipelines documentation: <https://learn.microsoft.com/en-us/azure/devops/pipelines/>

---

**Remember**: This skill validates Azure Pipelines configurations but does not execute pipelines. Use Azure DevOps Pipeline validation or Azure CLI for testing actual pipeline execution.
