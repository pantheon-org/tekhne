# Scenario 03: Add Language Tags to Untagged Code Fences

## User Prompt

An engineering team maintains a large operations runbook. During a recent lint audit,
`markdownlint` flagged every code block that lacks a language tag (rule MD040). The
file has seven code blocks; none have language tags.

Here is the input document (`runbook.md`):

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

Produce a corrected `runbook.md` that:

1. Adds the correct language tag to every fenced code block based on the content type.
2. Does not change any other content in the document.
3. Does not add any markdownlint disable comments.

Produce the corrected file `runbook.md`.

## Expected Behavior

1. Add a language tag to every fenced code block (no untagged fences should remain)
2. Identify the correct language for each block: `bash` for shell scripts, `sql` for SQL queries, `toml` for TOML config, `yaml` for YAML config
3. Leave all document structure, headings, prose, and code content unchanged — only the fence opening lines differ
4. Add no markdownlint disable comments of any kind

## Success Criteria

- **All six code blocks have a language tag**: Every fenced code block in the output file starts with a language identifier. No bare triple-backtick fences remain.
- **Language tags are accurate for each block's content**: Shell scripts are tagged `bash`, SQL queries are tagged `sql`, TOML config is tagged `toml`, and YAML config is tagged `yaml`. No wrong or generic tags (e.g., `text`) are used where a specific tag is correct.
- **No other content changes made**: The document structure, headings, prose, and code content are identical to the input. Only the fence opening lines differ.
- **No markdownlint suppressions added**: The output contains no markdownlint disable comments of any kind.

## Failure Conditions

- Any code block is still missing a language tag
- Any code block has an inaccurate or generic tag (e.g., `text` used where `bash` or `sql` is correct)
- Document structure, prose, headings, or code content has been changed beyond adding language tags
- Any markdownlint disable comment has been added
