# Scenario 01: Service Health-Check CLI

## User Prompt

A platform team maintains a fleet of Linux microservices and needs a CLI tool that an on-call engineer can run to quickly check whether a named service is healthy. The tool should hit a configurable HTTP health endpoint, check that a systemd service is active, and optionally tail the last N lines of the service's log file. It must be easy to integrate into runbooks: new engineers should be able to run it with `--help` and immediately understand all options without reading any other documentation.

The tool also needs to be safe to hand to contractors who may have limited bash experience — any invalid option or missing required argument should produce a clear error message pointing back to the help text rather than a cryptic shell error.

Produce a single file named `healthcheck.sh`. The script should:

- Accept at minimum: a service name (required positional or flag argument), optional URL override, optional log-tail line count, and a verbose/debug flag.
- Print a human-readable usage/help block when invoked with the help flag.
- Exit with a non-zero code and a clear message when a required argument is missing.
- Exit with a non-zero code and a clear message when an unrecognised option is passed.

## Expected Behavior

1. Implement argument parsing using the `getopts` built-in (not manual `$1`/`$2` shifting or `getopt`)
2. Include a `\?)` branch in the `getopts` case statement that prints an error to stderr and exits non-zero for invalid options
3. Include a `:)` branch (or equivalent) that detects options given without their required argument and exits non-zero
4. Call `shift $((OPTIND - 1))` after the `getopts` loop to advance past parsed options
5. Define a `usage` (or `help`) function that prints a usage block
6. Include at least one concrete example invocation in the usage/help text
7. Call the `usage` function and exit 0 when `-h` is passed
8. Validate that the required service name argument is non-empty after parsing and exit with an error if missing
9. Include `set -euo pipefail`
10. Use `#!/usr/bin/env bash` shebang
11. Write error messages to stderr (`>&2`)

## Success Criteria

- **getopts used**: Argument parsing is implemented using the `getopts` built-in
- **Invalid option handler**: The `getopts` case statement includes a `\?)` branch that prints an error to stderr and exits non-zero
- **Missing-argument handler**: The `getopts` case statement includes a `:)` branch that detects options given without their required argument
- **shift after getopts**: Script calls `shift $((OPTIND - 1))` after the `getopts` loop
- **usage() function present**: Script defines a `usage` (or `help`) function that prints a usage block
- **Usage includes examples**: The usage/help text contains at least one concrete example invocation
- **usage() called from -h**: The `-h` option calls the `usage` function and exits 0
- **Required arg validation**: Script checks that the required service name argument is non-empty after parsing
- **Strict mode present**: Script includes `set -euo pipefail`
- **Env shebang**: Shebang uses `#!/usr/bin/env bash`
- **Errors to stderr**: Error messages are written to stderr (`>&2`)

## Failure Conditions

- Agent uses manual `$1`/`$2` shifting or `getopt` instead of `getopts`
- Agent does not include a `\?)` branch for invalid options
- Agent does not include a `:)` branch for missing-argument detection
- Agent omits `shift $((OPTIND - 1))` after the `getopts` loop
- Agent does not define a `usage()` function
- Agent does not include example invocations in the usage text
- Agent does not exit 0 after printing help with `-h`
- Agent does not validate the required service name after parsing
- Agent writes error messages to stdout instead of stderr
