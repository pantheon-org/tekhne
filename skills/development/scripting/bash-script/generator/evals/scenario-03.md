# Scenario 03: Nginx Access Log Analyser

## User Prompt

A web operations team needs a daily summary report from their nginx access logs. The log file can be several hundred megabytes. Each morning the on-call engineer currently runs a handful of one-liners manually to get top URLs, error rates, and request counts by status code — a tedious and error-prone process.

The team wants a single script that reads a log file and prints a tidy summary: total requests, count of each HTTP status class (2xx, 3xx, 4xx, 5xx), the top 10 requested URLs, and the top 5 client IPs by request count. The script should be fast on large files — the team runs it against logs that are 200–500 MB, so unnecessary passes over the file or processes that re-read it multiple times would add unacceptable latency to the morning routine.

Produce a file named `log_analyser.sh`. The script should:

- Accept the log file path as a command-line argument.
- Print the summary report to stdout.
- The file path may contain spaces, so the script must handle that correctly.

The following file is provided as sample input. Extract it before beginning.

=============== FILE: inputs/access.log ===============
127.0.0.1 - - [10/Mar/2026:08:01:01 +0000] "GET /api/users HTTP/1.1" 200 512 "-" "curl/7.68.0"
10.0.0.5 - - [10/Mar/2026:08:01:02 +0000] "POST /api/orders HTTP/1.1" 201 128 "-" "python-requests/2.28"
192.168.1.10 - - [10/Mar/2026:08:01:03 +0000] "GET /api/users HTTP/1.1" 200 512 "-" "Mozilla/5.0"
127.0.0.1 - - [10/Mar/2026:08:01:04 +0000] "GET /health HTTP/1.1" 200 20 "-" "curl/7.68.0"
10.0.0.5 - - [10/Mar/2026:08:01:05 +0000] "GET /api/products HTTP/1.1" 404 64 "-" "python-requests/2.28"
192.168.1.11 - - [10/Mar/2026:08:01:06 +0000] "DELETE /api/users/42 HTTP/1.1" 403 32 "-" "Mozilla/5.0"
127.0.0.1 - - [10/Mar/2026:08:01:07 +0000] "GET /api/users HTTP/1.1" 500 256 "-" "curl/7.68.0"
10.0.0.5 - - [10/Mar/2026:08:01:08 +0000] "GET /api/users HTTP/1.1" 200 512 "-" "python-requests/2.28"
192.168.1.10 - - [10/Mar/2026:08:01:09 +0000] "GET /api/users HTTP/1.1" 302 0 "-" "Mozilla/5.0"
127.0.0.1 - - [10/Mar/2026:08:01:10 +0000] "POST /api/auth HTTP/1.1" 401 48 "-" "curl/7.68.0"

## Expected Behavior

1. Use `awk` (not `grep` or `sed` alone) to parse structured log fields (status code, URL, IP)
2. Compute status code counts (2xx, 3xx, 4xx, 5xx) in a single `awk` invocation — not separate passes
3. Do NOT use `cat file | awk` or `cat file | grep` — pass the file directly as an argument
4. Do NOT use `ls | grep` to filter files
5. Quote the log file path variable when used in commands (e.g., `awk '...' "${log_file}"`) to handle paths with spaces
6. Check that the input file exists and is readable before processing
7. Include `set -euo pipefail`
8. Use `#!/usr/bin/env bash` shebang
9. Produce output including a total request count and at least one status class breakdown when run against the sample log
10. Define a `usage()` or `help()` function invoked when the file argument is missing

## Success Criteria

- **awk for structured parsing**: Script uses `awk` to parse the structured log fields from the access log
- **Single-pass awk for multiple stats**: Status code counts are computed in a single `awk` invocation rather than separate passes over the file
- **No useless cat**: Script does NOT use `cat file | awk` or `cat file | grep` patterns
- **No ls pipe to grep**: Script does NOT use `ls | grep` to filter files
- **Quoted file path variable**: The log file path variable is quoted when used in commands to handle paths with spaces
- **validate_file or equivalent**: Script checks that the input file exists and is readable before processing
- **Strict mode present**: Script includes `set -euo pipefail`
- **Env shebang**: Shebang uses `#!/usr/bin/env bash`
- **Correct output produced**: Running against the sample log produces output including total request count and status class breakdown
- **Usage function present**: Script defines a `usage()` or `help()` function invoked when the file argument is missing

## Failure Conditions

- Agent uses `grep` or `sed` alone for parsing log fields instead of `awk`
- Agent uses separate `awk` invocations for each status class instead of a single pass
- Agent uses `cat file | awk` (useless cat anti-pattern)
- Agent leaves the log file path variable unquoted
- Agent does not validate that the input file exists before processing
- Agent does not define a `usage()` function
