---
name: bash-script-generator
description: Creates bash scripts with argument parsing, error handling, logging, and input validation following current standards. Use when creating new bash or shell scripts, .sh files, CLI tools, scripting automation, text processing workflows (grep/awk/sed pipelines), or building production-ready command-line utilities. Trigger phrases include 'write a bash script', 'create a shell script', 'generate a .sh file', 'bash command', 'scripting', or 'automate with bash'.
---

# Bash Script Generator

## Overview

This skill generates production-ready bash scripts with best practices built-in: strict mode, error handling, logging, argument parsing, input validation, and cleanup traps. Use for system administration, text processing, API clients, automation workflows, and scheduled tasks.

## Pre-Generation Checklist (REQUIRED)

Before writing any script, complete these steps:

1. **Clarify ambiguities** — ask if any of the following are unclear:
   - Input data format (nginx log, JSON, CSV, custom?)
   - Large file handling (>100MB)?
   - Error handling preference (fail fast / continue / retry)?
   - Security context (sensitive data, elevated privileges)?
   - Portability needs (bash-specific vs POSIX sh)?
   - Output format (human-readable, JSON, CSV)?

2. **Explain your approach** — before writing code, briefly describe:
   - Script architecture and main functions
   - Tool selections (grep/awk/sed) with rationale from `references/text-processing-guide.md`
   - Key design decisions and customization points

3. **Use the template for standard scripts** (CLI tools, automation scripts):
   ```bash
   bash scripts/generate_script_template.sh standard output-script.sh
   ```
   Then customize for your specific use case.

## Generation Workflow

### Stage 1 – Understand Requirements
Determine: purpose, input/output sources, bash vs POSIX sh, argument needs, error handling strategy, performance constraints, security requirements. Use AskUserQuestion for anything unclear.

### Stage 2 – Architecture Planning
- Select functions, config management, logging strategy, error handling approach
- Tool selection: **grep** (pattern matching/filtering), **awk** (structured data, calculations), **sed** (substitutions, stream editing), **find** (filesystem), **curl/wget** (HTTP)
- Plan `set -euo pipefail`, error functions, and `trap` cleanup

### Stage 3 – Script Structure

```bash
#!/usr/bin/env bash
#
# Script Name: script-name.sh
# Description: Brief description
# Created: YYYY-MM-DD
#

set -euo pipefail
IFS=$'\n\t'

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly SCRIPT_NAME="$(basename "${BASH_SOURCE[0]}")"

cleanup() {
    local exit_code=$?
    # Remove temp files, release locks, etc.
    exit "${exit_code}"
}
trap cleanup EXIT ERR INT TERM
```

### Stage 4 – Core Functions

Full implementations are in `assets/templates/standard-template.sh`. Key signatures to include:

**Logging (one line per level):**
```bash
log_info()  { echo "[INFO]  $(date '+%Y-%m-%d %H:%M:%S') - $*" >&2; }
log_warn()  { echo "[WARN]  $(date '+%Y-%m-%d %H:%M:%S') - $*" >&2; }
log_error() { echo "[ERROR] $(date '+%Y-%m-%d %H:%M:%S') - $*" >&2; }
log_fatal() { echo "[FATAL] $(date '+%Y-%m-%d %H:%M:%S') - $*" >&2; exit 1; }
# Add log_debug with LOG_LEVEL guard from template when DEBUG support is needed
```

**Error handling:**
```bash
die()           { log_error "$@"; exit 1; }
check_command() { command -v "$1" &>/dev/null || die "Required command '$1' not found."; }
validate_file() { [[ -f "$1" ]] || die "File not found: $1"; [[ -r "$1" ]] || die "File not readable: $1"; }
```

**Argument parsing (getopts):**
```bash
usage() {
    cat << EOF
Usage: ${SCRIPT_NAME} [OPTIONS] [ARGUMENTS]

Options:
    -h          Show this help and exit
    -v          Enable verbose output
    -f FILE     Input file path
    -o FILE     Output file path
    -d          Enable debug logging

Examples:
    ${SCRIPT_NAME} -f input.txt -o output.txt
EOF
}

parse_args() {
    while getopts ":hvf:o:d" opt; do
        case ${opt} in
            h) usage; exit 0 ;;
            v) VERBOSE=true ;;
            f) INPUT_FILE="${OPTARG}" ;;
            o) OUTPUT_FILE="${OPTARG}" ;;
            d) LOG_LEVEL="DEBUG" ;;
            \?) echo "Invalid option: -${OPTARG}" >&2; usage; exit 1 ;;
            :)  echo "Option -${OPTARG} requires an argument" >&2; usage; exit 1 ;;
        esac
    done
    shift $((OPTIND - 1))
}
```

### Stage 5 – Business Logic

- **Text processing**: use `references/text-processing-guide.md` for grep/awk/sed selection
- **System administration**: include prerequisite validation, backup, rollback, progress indicators
- **API clients**: include HTTP error handling, retry logic, authentication, response parsing

### Stage 6 – Main Function

```bash
main() {
    parse_args "$@"  # From Stage 4
    check_command "grep"
    check_command "awk"
    [[ -n "${INPUT_FILE:-}" ]] || die "Input file not specified. Use -f option."
    validate_file "${INPUT_FILE}"
    log_info "Starting processing..."  # From Stage 4
    # Main logic here
    log_info "Processing completed successfully"
}

main "$@"
```

### Stage 7 – Documentation

```bash
#######################################
# Brief description of function
# Globals:
#   VARIABLE_NAME
# Arguments:
#   $1 - Description
# Outputs:
#   Writes results to stdout
# Returns:
#   0 if successful, non-zero on error
#######################################
```

### Stage 8 – Validate

**After generating any script, invoke `devops-skills:bash-script-validator`:**
1. Generate the script
2. Run validator; review syntax, ShellCheck, security, performance results
3. Fix issues and re-validate until all checks pass
4. Provide Post-Generation Summary (see below)

## Key Best Practices

**Security:**
- Always quote variables: `"${var}"` not `$var`
- Validate inputs: `[[ "${val}" =~ ^[a-zA-Z0-9/_.-]+$ ]] || die "Invalid"`
- Never `eval` user input; use `case` statements instead

**Performance:**
- Avoid useless `cat`: `grep pattern file` not `cat file | grep pattern`
- Single-pass awk: `awk '/ERROR/{e++} /WARN/{w++} END{print e,w}' log`

**Maintainability:**
- Functions follow single responsibility
- Use `readonly` for constants
- Meaningful variable names; comments for complex logic

**Portability (POSIX sh):**
- Avoid bash arrays, `[[ ]]`, `$BASH_SOURCE`; test with `sh -n script.sh`

## Common Script Patterns

See `references/script-patterns.md` for full templates including text processing and parallel batch processing. Quick reference for simple CLI tools:

**Pattern 1 – Simple CLI tool:**
```bash
#!/usr/bin/env bash
set -euo pipefail

# For production scripts, use the full logging and argument parsing
# functions from Stage 4 above. This minimal example demonstrates structure:

usage() { cat << EOF
Usage: ${0##*/} [OPTIONS] FILE
    -h  Show help
    -v  Verbose
    -o  Output file
EOF
}

main() {
    local verbose=false output_file="" input_file=""
    while getopts ":hvo:" opt; do
        case ${opt} in
            h) usage; exit 0 ;; v) verbose=true ;; o) output_file="${OPTARG}" ;;
            *) echo "Invalid option: -${OPTARG}" >&2; usage; exit 1 ;;
        esac
    done
    shift $((OPTIND - 1))
    input_file="${1:-}"
    [[ -n "${input_file}" ]] || { echo "Error: FILE required" >&2; usage; exit 1; }
    [[ -f "${input_file}" ]] || { echo "Error: File not found: ${input_file}" >&2; exit 1; }
    if [[ -n "${output_file}" ]]; then
        process_file "${input_file}" > "${output_file}"
    else
        process_file "${input_file}"
    fi
}

process_file() { local file="$1"; cat "${file}"; }
main "$@"
```

For **text processing** (grep/awk/sed pipelines) and **parallel batch processing** patterns, see `references/script-patterns.md`.

## Post-Generation Summary (REQUIRED)

After every script, provide:

```
## Generated Script Summary

**File:** path/to/script.sh

**Architecture:** [main functions and purposes]

**Tool Selection:**
- grep: [why used]
- awk: [why used]
- sed: [why used / not needed]

**Key Features:** [list]

**Customization Points:** [variables/functions to modify]

**Usage:**
  ./script.sh --help
  ./script.sh -v input.log

**Validation Status:** ✅ Passed ShellCheck / ❌ Issues found (fixing...)

**Documentation References:**
- references/text-processing-guide.md (tool selection)
- references/script-patterns.md (argument parsing)
```

## References

**Internal references (offline, load as context):**
- `references/bash-scripting-guide.md` — strict mode, functions, arrays, parameter expansion
- `references/script-patterns.md` — argument parsing, logging, retry logic, lock files, parallel processing
- `references/text-processing-guide.md` — grep/awk/sed selection, pipelines, large-file optimization
- `references/generation-best-practices.md` — naming, documentation, testing, security, portability
- `assets/templates/standard-template.sh` — production-ready template with all components
- `examples/log-analyzer.sh` — grep/awk/sed usage demonstration
- `scripts/generate_script_template.sh` — template generator tool

**Official documentation:**
- [GNU Bash Manual](https://www.gnu.org/software/bash/manual/bash.html)
- [ShellCheck](https://www.shellcheck.net/)
- [Google Shell Style Guide](https://google.github.io/styleguide/shellguide.html)

---

*All generated scripts are automatically validated using `devops-skills:bash-script-validator` to ensure correct syntax, ShellCheck compliance, security, and performance.*
