---
name: jenkinsfile-validator
description: Comprehensive toolkit for validating, linting, testing, and automating Jenkinsfile pipelines (both Declarative and Scripted). Use this skill when working with Jenkins pipeline files, validating pipeline syntax, checking best practices, debugging pipeline issues, or working with custom plugins.
---

# Jenkinsfile Validator Skill

Comprehensive toolkit for validating, linting, and testing Jenkinsfile pipelines (both Declarative and Scripted). This skill applies when working with Jenkins pipeline files, validating pipeline syntax, checking best practices, debugging pipeline issues, or working with custom plugins that require documentation lookup.

## Validation Capabilities

**Declarative**: Required sections, directive placement, parallel execution, credential management, combined shell commands.
**Scripted**: Groovy syntax, node blocks, try-catch-finally, NonCPS annotation usage, variable scoping.
**Both types**: Hardcoded credential detection, controller-heavy operations (JsonSlurper, HttpRequest), variable declarations, plugin-specific step validation.
**Shared Library** — `vars/*.groovy`: call() method, NonCPS annotation correctness, CPS compatibility, camelCase naming, documentation comments. `src/**/*.groovy`: package declaration, class-filename match, Serializable implementation, wildcard import warnings, static method CPS compatibility.

See [references/validation_rules.md](references/validation_rules.md) for detailed rules, error reporting format, and examples.

## Pipeline Type Detection

Auto-detected: Declarative (`pipeline {`), Scripted (`node` block or Groovy outside pipeline block). Clarification is requested only if ambiguous.

## Validation Command Reference

### Full Validation (Recommended)

```bash
# Run complete validation (syntax + security + best practices)
bash scripts/validate_jenkinsfile.sh Jenkinsfile
```

Auto-detects pipeline type, validates syntax, scans for hardcoded credentials, checks best practices, and produces a unified summary.

### Command Options

```bash
# Full validation (default)
bash scripts/validate_jenkinsfile.sh Jenkinsfile

# Syntax validation only (fastest)
bash scripts/validate_jenkinsfile.sh --syntax-only Jenkinsfile

# Security audit only
bash scripts/validate_jenkinsfile.sh --security-only Jenkinsfile

# Best practices check only
bash scripts/validate_jenkinsfile.sh --best-practices Jenkinsfile

# Skip security checks
bash scripts/validate_jenkinsfile.sh --no-security Jenkinsfile

# Skip best practices
bash scripts/validate_jenkinsfile.sh --no-best-practices Jenkinsfile

# Strict mode (fail on warnings)
bash scripts/validate_jenkinsfile.sh --strict Jenkinsfile
```

### Script Architecture

The validation system uses a modular script architecture:

```
scripts/
├── validate_jenkinsfile.sh      # Main orchestrator (USE THIS)
│   ├── Auto-detects pipeline type
│   ├── Runs syntax validation
│   ├── Runs security scan
│   ├── Runs best practices check
│   └── Produces unified summary
│
├── validate_declarative.sh      # Declarative syntax validator
├── validate_scripted.sh         # Scripted syntax validator
├── common_validation.sh         # Shared functions + security scan
├── best_practices.sh            # 15-point best practices scorer
└── validate_shared_library.sh   # Shared library validator
```

### Shared Library Validation

Validate Jenkins Shared Library files using `validate_shared_library.sh`:

```bash
# Validate a single vars file
bash scripts/validate_shared_library.sh vars/myStep.groovy

# Validate entire shared library directory
bash scripts/validate_shared_library.sh /path/to/shared-library
```

## Plugin Documentation Lookup

**Important**: Plugin documentation lookup is Claude's responsibility (not automated in scripts). After running validation, Claude should identify unknown plugins and look them up.

### When to Look Up Plugin Documentation

Look up documentation when you encounter:
- Steps not in `references/common_plugins.md` (e.g., `customDeploy`, `sendToDatadog`, `grafanaNotify`)
- Plugin-specific configuration (e.g., `nexusArtifactUploader`, `sonarQubeScanner`)
- User questions about plugin parameters or best practices

### Plugin Lookup Workflow (Claude's Responsibility)

1. **Identify Unknown Plugin Step** - Review Jenkinsfile for unrecognized steps
2. **Check Local Reference First** - Read: references/common_plugins.md
3. **Use Context7 MCP** (if not in local reference)
   - mcp__context7__resolve-library-id with query: "jenkinsci \<plugin-name\>-plugin"
   - mcp__context7__get-library-docs for usage examples and parameters
4. **Web Search Fallback** (if Context7 has no results)
   - WebSearch: "Jenkins \<plugin-name\> plugin documentation"
   - Official source: <https://plugins.jenkins.io/>
5. **Provide Usage Guidance**
   - Required vs optional parameters
   - Best practices for the plugin
   - Security considerations

See [references/common_plugins.md](references/common_plugins.md) for documentation on commonly used plugins.

## Reference Documentation

- [Declarative Syntax](references/declarative_syntax.md): Complete Declarative Pipeline syntax reference
- [Scripted Syntax](references/scripted_syntax.md): Complete Scripted Pipeline syntax reference
- [Best Practices](references/best_practices.md): Jenkins pipeline best practices and patterns
- [Common Plugins](references/common_plugins.md): Documentation for frequently used Jenkins plugins
- [Validation Rules](references/validation_rules.md): Detailed validation rules, error reporting format, and examples
- [Troubleshooting](references/troubleshooting.md): Common issues, debug mode, and limitations

## Claude's Workflow

When a user provides a Jenkinsfile for validation:

1. **Run validation** using the main script:
   ```bash
   bash scripts/validate_jenkinsfile.sh <path-to-jenkinsfile>
   ```

2. **Optionally read the Jenkinsfile** using the Read tool if you need to:
   - Understand the pipeline structure before validation
   - Provide context-specific advice
   - Identify specific plugins being used

3. **Look up unknown plugins** after validation:
   - Review validation output for unrecognized step names
   - Check `references/common_plugins.md` first
   - If not found, use Context7 MCP: `mcp__context7__resolve-library-id` with query "jenkinsci \<plugin-name\>"
   - If still not found, use WebSearch: "Jenkins \<plugin-name\> plugin documentation"
   - Provide usage guidance based on documentation

4. **Report results** with line numbers, severity, and actionable suggestions

5. **Provide inline fix suggestions** when errors are found (include corrected code snippets directly in the response)

## Tools Available

This skill uses standard shell scripts for validation. No special tools or external dependencies are required beyond:
- bash (for running validation scripts)
- Basic Unix utilities (grep, awk, sed)

## External References

- Official Jenkins Pipeline Syntax: <https://www.jenkins.io/doc/book/pipeline/syntax/>
- Pipeline Development Tools: <https://www.jenkins.io/doc/book/pipeline/development/>
- Pipeline Best Practices: <https://www.jenkins.io/doc/book/pipeline/pipeline-best-practices/>
- Jenkins Plugins: <https://plugins.jenkins.io/>
