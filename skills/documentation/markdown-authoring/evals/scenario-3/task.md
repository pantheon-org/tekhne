# Scenario 3: Add Language Tags to Untagged Code Fences

## Context

An engineering team maintains a large operations runbook. During a recent lint audit,
`markdownlint` flagged every code block that lacks a language tag (rule MD040). The
file has seven code blocks; none have language tags.

## Input Document (`runbook.md`)

````markdown
# Database Maintenance Runbook

## Backup Procedure

Run the following script to create a backup:

```
#!/usr/bin/env bash
set -euo pipefail
pg_dump mydb > backup_$(date +%Y%m%d).sql
```

Verify the backup file exists:

```
ls -lh backup_*.sql
```

## Restore Procedure

To restore from a backup:

```
psql mydb < backup_20240301.sql
```

## Configuration

The maintenance window is controlled by this config block:

```
[maintenance]
window_start = "02:00"
window_end   = "04:00"
timezone     = "UTC"
```

## Monitoring Query

Use this SQL to check replication lag:

```
SELECT
  client_addr,
  sent_lsn - replay_lsn AS lag_bytes
FROM pg_stat_replication;
```

## Alert Threshold Config

Paste this into Alertmanager:

```
route:
  receiver: pagerduty
  group_wait: 30s
  group_interval: 5m
```
````

## Task

Produce a corrected `runbook.md` that:

1. Adds the correct language tag to every fenced code block based on the content type.
2. Does not change any other content in the document.
3. Does not add any markdownlint disable comments.

## Output Specification

Produce the corrected file `runbook.md`.
