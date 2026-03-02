---
name: jenkinsfile-validator
description: Comprehensive toolkit for validating, linting, testing, and automating Jenkinsfile pipelines (both Declarative and Scripted). Use this skill when working with Jenkins pipeline files, validating pipeline syntax, checking best practices, debugging pipeline issues, or working with custom plugins.
---

# Jenkinsfile Validator Skill

Comprehensive toolkit for validating, linting, and testing Jenkinsfile pipelines (both Declarative and Scripted). This skill applies when working with Jenkins pipeline files, validating pipeline syntax, checking best practices, debugging pipeline issues, or working with custom plugins that require documentation lookup.

## When to Use This Skill

- Validating Jenkinsfile syntax before committing to repository
- Checking Jenkins pipeline best practices compliance
- Debugging pipeline syntax errors or configuration issues
- Validating both Declarative and Scripted pipeline syntaxes
- **Validating Jenkins Shared Library files (vars/*.groovy, src/**/*.groovy)**
- Working with plugin-specific steps that need documentation
- Ensuring proper credential handling and security practices
- Checking for common anti-patterns and performance issues
- Verifying variable usage and scope

## Validation Capabilities

### Declarative Pipeline Validation

- **Syntax Structure**: Validates required sections (pipeline, agent, stages, steps)
- **Directive Validation**: Checks proper usage of environment, options, parameters, triggers, tools, when, input
- **Best Practices**: Parallel execution, credential management, combined shell commands
- **Section Placement**: Ensures directives are in correct locations

See [references/declarative_syntax.md](references/declarative_syntax.md) for complete syntax reference.

### Scripted Pipeline Validation

- **Groovy Syntax**: Validates Groovy code syntax and structure
- **Node Blocks**: Ensures proper node/agent block usage
- **Error Handling**: Checks for try-catch-finally patterns
- **Best Practices**: @NonCPS usage, agent-based operations, proper variable scoping

See [references/scripted_syntax.md](references/scripted_syntax.md) for complete syntax reference.

### Common Validations (Both Types)

- **Security**: Detects hardcoded credentials, passwords, API keys
- **Performance**: Identifies controller-heavy operations (JsonSlurper, HttpRequest on controller)
- **Variables**: Validates variable declarations and usage
- **Plugins**: Detects and validates plugin-specific steps with dynamic documentation lookup

### Shared Library Validation

- **vars/*.groovy**: Validates global variable files (callable steps)
  - call() method presence and signature
  - @NonCPS annotation correctness (no pipeline steps in @NonCPS methods)
  - CPS compatibility (closures with .each{}, .collect{}, etc.)
  - Hardcoded credentials detection
  - Controller-heavy operations (JsonSlurper, new URL(), new File())
  - Thread.sleep() vs sleep() step
  - System.getenv() vs env.VAR_NAME
  - File naming conventions (camelCase)
  - Documentation comment presence
- **src/**/*.groovy**: Validates Groovy source class files
  - Package declaration presence
  - Class naming matches filename
  - Serializable implementation (required for CPS)
  - Wildcard import warnings
  - Static method CPS compatibility

## Pipeline Type Detection

The skill automatically detects the pipeline type:
- **Declarative**: Starts with `pipeline {` block
- **Scripted**: Starts with `node` or contains Groovy code outside pipeline block
- **Ambiguous**: Will ask for clarification if uncertain

## Core Validation Workflow

Follow this workflow when validating Jenkinsfiles to catch issues early and ensure pipeline quality:

### Quick Start - Full Validation (Recommended)

```bash
# Run complete validation (syntax + security + best practices)
bash scripts/validate_jenkinsfile.sh Jenkinsfile
```

This single command:
1. Auto-detects pipeline type (Declarative/Scripted)
2. Runs syntax validation
3. Runs security scan (credential detection)
4. Runs best practices check
5. Provides a unified summary with pass/fail status

### Validation Options

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

**Key Point**: Always use `validate_jenkinsfile.sh` as the main entry point - it orchestrates all other scripts automatically.

### Shared Library Validation

Validate Jenkins Shared Library files using `validate_shared_library.sh`:

```bash
# Validate a single vars file
bash scripts/validate_shared_library.sh vars/myStep.groovy

# Validate entire shared library directory
bash scripts/validate_shared_library.sh /path/to/shared-library
```

The shared library validator checks:
- **vars/*.groovy files**: call() method, @NonCPS usage, CPS compatibility, credential handling
- **src/**/*.groovy files**: Package declaration, class naming, Serializable implementation, imports

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

## Usage Instructions

When a user provides a Jenkinsfile for validation:

1. **Run the main validation script** (recommended - handles everything automatically):
   ```bash
   bash scripts/validate_jenkinsfile.sh <path-to-jenkinsfile>
   ```
   This single command auto-detects pipeline type, runs syntax validation, security scan, and best practices check.

2. **Optionally read the Jenkinsfile** using the Read tool if you need to:
   - Understand the pipeline structure before validation
   - Provide context-specific advice
   - Identify specific plugins being used

3. **After validation, scan for unknown plugins** (Claude's responsibility):
   - Review the validation output for any unrecognized step names
   - Check `references/common_plugins.md` first for documentation
   - If not found, use Context7 MCP: `mcp__context7__resolve-library-id` with query "jenkinsci \<plugin-name\>"
   - If still not found, use WebSearch: "Jenkins \<plugin-name\> plugin documentation"
   - Provide usage guidance based on found documentation

4. **Report results** with line numbers, severity, and actionable suggestions

5. **Provide inline fix suggestions** when errors are found (do not use AskUserQuestion - include corrected code snippets directly in the response)

## Common Validation Scenarios

### Scenario 1: Validate Declarative Pipeline

```markdown
User: "Validate my Jenkinsfile"
1. Read the Jenkinsfile
2. Detect type: Declarative (starts with 'pipeline {')
3. Run: bash scripts/validate_declarative.sh Jenkinsfile
4. Run: bash scripts/best_practices.sh Jenkinsfile
5. Report results with suggestions
```

### Scenario 2: Validate with Unknown Plugin

```markdown
User: "Check this pipeline with custom plugin steps"
1. Read Jenkinsfile
2. Run validation
3. Detect unknown step (e.g., 'customDeploy')
4. Search context7 for plugin docs
5. If not found, web search "Jenkins custom deploy plugin"
6. Validate plugin usage against found documentation
7. Report results
```

### Scenario 3: Security Audit

```markdown
User: "Check for security issues in my pipeline"
1. Run: bash scripts/validate_jenkinsfile.sh --security-only Jenkinsfile
2. Report all credential/secret findings
3. Suggest withCredentials patterns
4. Reference: references/best_practices.md#credential-management
```

## Tools Available

This skill uses standard shell scripts for validation. No special tools or external dependencies are required beyond:
- bash (for running validation scripts)
- Basic Unix utilities (grep, awk, sed)

## Automatic Actions

When this skill is invoked:
1. Always validate syntax first (errors block execution)
2. Then check best practices (warnings for improvement)
3. Look up unknown plugins automatically
4. Provide actionable suggestions with every issue
5. Reference documentation files for detailed guidance

## External References

- Official Jenkins Pipeline Syntax: <https://www.jenkins.io/doc/book/pipeline/syntax/>
- Pipeline Development Tools: <https://www.jenkins.io/doc/book/pipeline/development/>
- Pipeline Best Practices: <https://www.jenkins.io/doc/book/pipeline/pipeline-best-practices/>
- Jenkins Plugins: <https://plugins.jenkins.io/>
