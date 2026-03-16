# Database Backup Script

## Problem/Feature Description

A small SaaS company runs a PostgreSQL database on a Linux VM and currently backs it up with an ad-hoc one-liner that a developer types manually each night. There is no cleanup of old backups, no error reporting, and the process has already failed silently twice this quarter — once because `pg_dump` was missing on the server and once because the destination disk was full. The ops team needs a proper backup script they can hand to a junior admin and schedule in cron.

The script should dump the database to a timestamped `.sql.gz` file in a configurable output directory, enforce a retention policy (keep only the N most recent backups), and exit cleanly on any failure. The ops team will run it as a low-privilege user; the script must never leave partial or corrupt backup files behind if it fails midway.

## Output Specification

Produce a single executable file named `backup_db.sh`. The script should:

- Accept the database name, output directory, and number of backups to retain as command-line parameters.
- Compress the dump with gzip.
- Remove backup files older than the retention count after a successful dump.
- Print status messages so an operator watching cron output can understand what happened.

Do not include any hardcoded database credentials; the script should expect them to be available in the environment (e.g. `PGPASSWORD`).
