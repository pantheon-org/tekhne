---
name: makefile-validator
description: Comprehensive toolkit for validating, linting, and optimizing Makefiles. Use when working with Makefiles (Makefile, makefile, *.mk files), validating build configurations, checking for best practices, identifying security issues, or debugging Makefile problems. Concrete capabilities include detecting missing .PHONY declarations, validating tab indentation in recipes, checking variable expansion safety, identifying hardcoded credentials, and flagging missing prerequisites or syntax errors.
---

# Makefile Validator

## Validation Capabilities

- **Syntax**: GNU make `-n --dry-run` validation; catches errors with line numbers
- **Formatting**: mbake tab indentation, variable assignment consistency, trailing whitespace, line continuations
- **Best practices**: `.DELETE_ON_ERROR`, `.PHONY` declarations, `$(MAKE)` for recursive calls, `.ONESHELL` error handling
- **Security**: hardcoded credentials, unsafe variable expansion in `rm`/`sudo`/`curl`/`wget`, command injection, `.EXPORT_ALL_VARIABLES` leakage
- **Optimization**: `.NOTPARALLEL`, `.INTERMEDIATE`/`.SECONDARY`, VPATH usage, incremental build efficiency

See `references/best-practices.md`, `references/common-mistakes.md`, and `references/bake-tool.md` for detailed explanations.

## Quick Start

### Basic Validation

```bash
# Validate a Makefile
bash scripts/validate_makefile.sh Makefile
```

### Example Output

```
========================================
MAKEFILE VALIDATOR
========================================
File: Makefile

[SYNTAX CHECK (GNU make)]
✓ No syntax errors found

[MBAKE VALIDATION]
✓ mbake validation passed

[MBAKE FORMAT CHECK]
⚠ Formatting issues found
Run 'mbake format Makefile' to fix formatting issues
Or run 'mbake format --diff Makefile' to preview changes

[CUSTOM CHECKS]
⚠ No .PHONY declarations found
✗ Potential spaces instead of tabs in recipes detected
ℹ No VPATH/vpath declarations found

========================================
VALIDATION SUMMARY
========================================
Errors:   1
Warnings: 2
Info:     1
⚠ Validation PASSED with warnings
```

### Exit Codes

- **0**: No issues found
- **1**: Warnings found (passed with warnings)
- **2**: Errors found (failed validation)

## Common Validation Scenarios

### Scenario 1: Pre-commit Validation

```bash
bash .claude/skills/makefile-validator/scripts/validate_makefile.sh Makefile
```

### Scenario 2: Formatting Consistency

```bash
# Preview changes
mbake format --diff Makefile

# Apply formatting
mbake format Makefile

# Re-validate
bash .claude/skills/makefile-validator/scripts/validate_makefile.sh Makefile
```

### Scenario 3: Converting Legacy Makefiles

```bash
# 1. Validate current state
bash scripts/validate_makefile.sh legacy.mk

# 2. Fix critical errors (tabs, syntax), then apply formatting
mbake format legacy.mk

# 3. Add .PHONY declarations
mbake format --auto-insert-phony-declarations legacy.mk

# 4. Re-validate
bash scripts/validate_makefile.sh legacy.mk

# 5. Reference best-practices.md for modernization guidance
```

### Scenario 4: Security Audit

The validator automatically checks for hardcoded credentials, unsafe variable expansion in dangerous commands, and command injection vulnerabilities. Reference `references/common-mistakes.md` for detailed explanations and fixes.

## Integration with Development Workflow

For pre-commit hooks, CI/CD pipelines (e.g. GitHub Actions), and self-validating Makefile targets, use the validation script at `.claude/skills/makefile-validator/scripts/validate_makefile.sh` and match on files named `Makefile`, `makefile`, or `*.mk`. The script's exit codes (0/1/2) map cleanly to pass/warn/fail states for any automation context. See `references/bake-tool.md` for CI/CD configuration details.

## Installation Requirements

### Required

- **python3**, **pip3**: For venv and mbake installation
- **bash**: For running validation script
- **GNU make**: For syntax validation (`make -n`)

### Optional (Recommended)

- **checkmake**: Additional linting (`minphony`, `phonydeclared` rules)
  ```bash
  go install github.com/checkmake/checkmake/cmd/checkmake@latest
  ```
- **unmake**: POSIX portability checks — see [github.com/mcandre/unmake](https://github.com/mcandre/unmake)

### Automatic Installation

**mbake** is automatically installed in an isolated venv per invocation and cleaned up on exit — no manual installation required.

## Directory Structure

```
makefile-validator/
├── skill.md                    # This file
├── scripts/
│   └── validate_makefile.sh    # Main validation script
├── references/
│   ├── best-practices.md       # Makefile best practices
│   ├── common-mistakes.md      # Common Makefile mistakes
│   └── bake-tool.md            # mbake tool reference (config, CI/CD, advanced features)
└── assets/
    ├── good-makefile.mk        # Well-written example
    └── bad-makefile.mk         # Anti-patterns example
```

## Known Limitations

mbake doesn't recognize some valid GNU Make special targets (`.DELETE_ON_ERROR`, `.SUFFIXES`, `.ONESHELL`, `.POSIX`) — the validator filters these false positives and surfaces them as informational messages. The `mbake format --check` vs `mbake format` output may also differ; this is a known upstream issue. See `references/bake-tool.md` for full details including mbake configuration (`~/.bake.toml`) and format-disable comments.

## Resources

- [GNU Make Manual](https://www.gnu.org/software/make/manual/)
- [mbake GitHub](https://github.com/EbodShojaei/bake) / [PyPI](https://pypi.org/project/mbake/)
- [checkmake GitHub](https://github.com/checkmake/checkmake)
- [Makefile Best Practices](references/best-practices.md)
- [Common Makefile Mistakes](references/common-mistakes.md)
- [mbake Tool Reference](references/bake-tool.md)
