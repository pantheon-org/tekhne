# Scenario 00: Database Backup Script

## User Prompt

A small SaaS company runs a PostgreSQL database on a Linux VM and currently backs it up with an ad-hoc one-liner that a developer types manually each night. There is no cleanup of old backups, no error reporting, and the process has already failed silently twice this quarter — once because `pg_dump` was missing on the server and once because the destination disk was full. The ops team needs a proper backup script they can hand to a junior admin and schedule in cron.

The script should dump the database to a timestamped `.sql.gz` file in a configurable output directory, enforce a retention policy (keep only the N most recent backups), and exit cleanly on any failure. The ops team will run it as a low-privilege user; the script must never leave partial or corrupt backup files behind if it fails midway.

Produce a single executable file named `backup_db.sh`. The script should:

- Accept the database name, output directory, and number of backups to retain as command-line parameters.
- Compress the dump with gzip.
- Remove backup files older than the retention count after a successful dump.
- Print status messages so an operator watching cron output can understand what happened.

Do not include any hardcoded database credentials; the script should expect them to be available in the environment (e.g. `PGPASSWORD`).

## Expected Behavior

1. Use `#!/usr/bin/env bash` as the shebang (not a hardcoded path like `#!/bin/bash`)
2. Set strict mode with `set -euo pipefail`
3. Set `IFS=$'\n\t'` after the strict mode line
4. Register a `trap` on EXIT (and ideally ERR, INT, TERM) that calls a cleanup function
5. In the cleanup/trap handler, remove any in-progress or partial output file so failed runs do not leave corrupt backups
6. Define a `die` (or equivalent) function that prints to stderr and exits non-zero
7. Verify that required external commands (e.g., `pg_dump`, `gzip`) exist using `command -v` or equivalent
8. Use `readonly` for at least one script-level constant
9. Write status/progress messages to stderr (`>&2`), not stdout
10. Include timestamps in log/status messages using `date`
11. Use no hardcoded passwords, connection strings, or tokens — rely on environment variables

## Success Criteria

- **Env shebang**: Shebang line uses `#!/usr/bin/env bash`
- **Strict mode set**: Script contains `set -euo pipefail` (all three flags present)
- **IFS override**: Script sets `IFS=$'\n\t'` after the strict mode line
- **Trap cleanup registered**: Script registers a `trap` on EXIT that calls a cleanup function
- **Cleanup removes partial file**: The cleanup/trap handler removes any in-progress or partial output file
- **die() or equivalent helper**: Script defines a `die` (or equivalent) function that prints to stderr and exits non-zero
- **check_command used**: Script verifies required external commands exist before use
- **readonly for constants**: Script uses `readonly` for at least one script-level constant
- **Logging to stderr**: Status/progress messages are written to stderr (`>&2`)
- **Timestamped log messages**: Log/status messages include a timestamp
- **No hardcoded credentials**: Script does NOT contain hardcoded passwords or connection strings

## Failure Conditions

- Agent uses a hardcoded shebang path (`#!/bin/bash`) instead of `#!/usr/bin/env bash`
- Agent omits `set -euo pipefail` or uses only partial flags
- Agent does not set `IFS=$'\n\t'`
- Agent does not register a `trap` for cleanup
- Agent's cleanup handler does not remove partial output files
- Agent does not define a `die` or error helper function
- Agent does not verify required commands exist before using them
- Agent writes status messages to stdout instead of stderr
- Agent hardcodes database credentials in the script
