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

## Anti-Patterns

### NEVER rely only on ShellCheck for validation

- **WHY**: ShellCheck catches syntax and safety issues statically but does not execute the script; runtime errors like missing files, wrong permissions, and incorrect exit codes only surface during actual execution with real inputs.
- **BAD**: Pass ShellCheck with zero warnings and ship the script without running it against representative inputs in a real or sandboxed environment.
- **GOOD**: Run ShellCheck first (fast, catches most issues), then test in a container or sandboxed environment with representative inputs to catch runtime failures.

### NEVER disable ShellCheck rules globally with a file-level directive

- **WHY**: A global `# shellcheck disable=SCxxxx` at the top of a file defeats the purpose of linting and silently hides real issues in code added later, long after the original suppression rationale is forgotten.
- **BAD**: `# shellcheck disable=SC2086` at the top of the file to silence all quoting warnings across every line in the script.
- **GOOD**: Add per-line suppressions only where the behavior is intentional and documented: `# shellcheck disable=SC2086 # word splitting intentional here`.

### NEVER treat POSIX sh and bash scripts identically

- **WHY**: POSIX sh does not support arrays, `[[ ]]`, `$(())` arithmetic, or many bash extensions; scripts marked `#!/bin/sh` will fail with bash-only syntax on some systems (Alpine Linux, minimal containers, many CI runners).
- **BAD**: Use `#!/bin/sh` as the shebang but write bash-specific syntax like `declare -A` or `[[ -n $var ]]` — the script will fail silently or with cryptic errors on non-bash sh implementations.
- **GOOD**: Use `#!/usr/bin/env bash` for scripts that require bash features; use `#!/bin/sh` only for scripts that are tested with strict POSIX compliance using `shellcheck --shell=sh`.

### NEVER use `exit 0` or `|| true` to suppress error propagation

- **WHY**: Explicitly returning 0 from a function or subshell that encountered an error hides failures from the caller, breaks `set -e` propagation, and makes debugging silent failures much harder.
- **BAD**: `validate() { run_check || true; return 0; }` — the function always succeeds even when `run_check` fails, so callers cannot detect the failure.
- **GOOD**: Return meaningful exit codes and let `set -e` propagate failures: `validate() { run_check; }` — if `run_check` fails, `validate` fails, and the caller can act on the exit code.

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
