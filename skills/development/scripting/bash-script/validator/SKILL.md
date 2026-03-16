---
name: bash-script-validator
description: Comprehensive toolkit for validating, linting, and optimizing bash and shell scripts. Use this skill when working with shell scripts (.sh, .bash), validating script syntax, detecting unquoted variables, checking POSIX compliance, identifying unsafe command substitutions, validating shebang lines, finding security vulnerabilities, or debugging shell script problems.
---

# Bash Script Validator

## Validation & Response

### Run the Validator

```bash
bash scripts/validate.sh <script-path>
```

### Required Workflow

```
1. Run: bash scripts/validate.sh <script-path>
2. Read the validation output and identify all issues
3. Read references/common-mistakes.md for fix patterns
4. Read references/shellcheck-reference.md for SC error explanations (if needed)
5. For EACH issue found:
   a. Show the problematic code
   b. Explain the issue (referencing documentation)
   c. Provide the corrected code
   d. Explain why the fix improves the script
```

### Example Output

```
========================================
BASH/SHELL SCRIPT VALIDATOR
========================================
File: myscript.sh
Detected Shell: bash

[SYNTAX CHECK]
✓ No syntax errors found (bash -n)

[SHELLCHECK]
myscript.sh:15:5: warning: Quote to prevent word splitting [SC2086]
myscript.sh:23:9: error: Use || exit to handle cd failure [SC2164]

[CUSTOM CHECKS]
⚠ Potential command injection: eval with variable found
  Line 42: eval $user_input

ℹ Useless use of cat detected
  Line 18: cat file.txt | grep pattern

========================================
VALIDATION SUMMARY
========================================
Errors:   2
Warnings: 3
Info:     1
```

### Response Format Template

```markdown
## Validation Results

Found X errors, Y warnings, Z info issues.

### Issue 1: Unquoted Variable (Line 25)

**Problem:**
\`\`\`bash
if [ ! -f $file ]; then  # Word splitting risk
\`\`\`

**Reference:** See `common-mistakes.md` section "1. Unquoted Variables"

**Fix:**
\`\`\`bash
if [ ! -f "$file" ]; then  # Properly quoted
\`\`\`

**Why:** Unquoted variables undergo word splitting and glob expansion,
causing unexpected behavior with filenames containing spaces or special characters.
```

## Example Scripts

Located in `assets/` directory:

- **good-bash.sh** — Well-written bash script demonstrating best practices
- **bad-bash.sh** — Poorly-written bash script with common mistakes
- **good-shell.sh** — POSIX-compliant sh script
- **bad-shell.sh** — sh script with bashisms and errors

## Installation Requirements

### Required
- bash or sh (for running scripts)

### ShellCheck Installation Options

**Option 1:** System-wide (Recommended)
```bash
brew install shellcheck        # macOS
apt-get install shellcheck     # Ubuntu/Debian
dnf install shellcheck         # Fedora
```

**Option 2:** Automatic via Wrapper (Python required)
```bash
./scripts/shellcheck_wrapper.sh --cache script.sh
# Clears cache: ./scripts/shellcheck_wrapper.sh --clear-cache
```

**Option 3:** Manual Python install
```bash
pip3 install shellcheck-py
```

The validator works without ShellCheck but provides enhanced validation when available.

## Technical Details & Directory Structure

### Automatic Shell Detection

- `#!/bin/bash`, `#!/usr/bin/env bash` → bash
- `#!/bin/sh`, `#!/usr/bin/sh` → POSIX sh
- `#!/bin/zsh` → zsh / `#!/bin/ksh` → ksh / `#!/bin/dash` → dash

### Exit Codes

- **0**: No issues found — **1**: Warnings found — **2**: Errors found

### Validation Logic

1. File existence and readability check
2. Shebang detection → determines shell type
3. Syntax validation → `bash -n` or `sh -n`
4. ShellCheck validation → if installed, with appropriate shell dialect
5. Custom security checks → unsafe `eval`, command injection, `rm -rf`, unquoted variables
6. Custom portability checks → bashisms in sh scripts
7. Summary generation → color-coded report with counts

### Layout

```
bash-script-validator/
├── scripts/validate.sh
├── references/         # bash, shell, shellcheck, common-mistakes, grep, awk, sed, regex
└── assets/             # good-bash.sh, bad-bash.sh, good-shell.sh, bad-shell.sh
```

## References

### Core References

- **bash-reference.md** — Bash-specific features, parameter expansion, arrays, control structures, functions, best practices, common pitfalls
- **shell-reference.md** — POSIX sh compliance, portable constructs, differences from bash, POSIX utilities
- **shellcheck-reference.md** — ShellCheck error codes explained, severity levels, configuration options, CI/CD integration
- **common-mistakes.md** — 25+ common shell scripting mistakes with real-world examples and correct solutions

### Tool References

- **grep-reference.md** — BRE/ERE patterns, common grep patterns, performance tips
- **awk-reference.md** — Field processing, pattern matching, arrays, log analysis, CSV/text processing
- **sed-reference.md** — Substitution patterns, address ranges, in-place editing, common one-liners
- **regex-reference.md** — BRE vs ERE, POSIX character classes, metacharacters, common patterns
- [GNU Bash Manual](https://www.gnu.org/software/bash/manual/)
- [POSIX Shell Specification](https://pubs.opengroup.org/onlinepubs/9699919799/)
- [ShellCheck](https://www.shellcheck.net/)
- [GNU grep](https://www.gnu.org/software/grep/manual/) / [GNU awk](https://www.gnu.org/software/gawk/manual/) / [GNU sed](https://www.gnu.org/software/sed/manual/)
