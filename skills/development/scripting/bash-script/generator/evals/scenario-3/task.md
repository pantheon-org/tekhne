# Nginx Access Log Analyser

## Problem/Feature Description

A web operations team needs a daily summary report from their nginx access logs. The log file can be several hundred megabytes. Each morning the on-call engineer currently runs a handful of one-liners manually to get top URLs, error rates, and request counts by status code — a tedious and error-prone process.

The team wants a single script that reads a log file and prints a tidy summary: total requests, count of each HTTP status class (2xx, 3xx, 4xx, 5xx), the top 10 requested URLs, and the top 5 client IPs by request count. The script should be fast on large files — the team runs it against logs that are 200–500 MB, so unnecessary passes over the file or processes that re-read it multiple times would add unacceptable latency to the morning routine.

## Output Specification

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
