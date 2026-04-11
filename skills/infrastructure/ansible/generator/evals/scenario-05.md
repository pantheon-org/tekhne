# Scenario 05: Database Migration Playbook for Large PostgreSQL Cluster

## User Prompt

A data engineering team needs to run a database schema migration across a fleet of `postgres_servers`. The migration script at `/opt/migrations/run_migration.sh` typically takes 15 to 45 minutes per server. When the team ran a previous migration synchronously, Ansible timed out and left servers in inconsistent states.

The team also needs to manage the `pg_hba.conf` access rules file using the `community.postgresql` collection (specifically `community.postgresql.postgresql_pg_hba` module) after the migration completes to add a new application user access rule.

The playbook must:
1. Launch the migration script asynchronously (fire and forget), then poll until it completes (max 1 hour).
2. After migration succeeds, use the community.postgresql collection to update `pg_hba.conf` with a new entry allowing `app_user` from `10.0.0.0/8` via `md5` authentication.
3. Reload the PostgreSQL service if pg_hba.conf changes.

The servers run RHEL 9. Assume the necessary Python bindings are available.

Produce a complete Ansible playbook file named `run_db_migration.yml`. The playbook should include proper documentation of any collection dependencies.

## Expected Behavior

1. Set `async:` to 3600 or greater on the migration task so it runs asynchronously and does not block
2. Set `poll: 0` on the migration launch task to fire-and-forget
3. Use `ansible.builtin.async_status` with `until: job_result.finished` and `retries:` to poll for migration completion
4. Include a comment showing the `ansible-galaxy collection install community.postgresql` install command
5. Use FQCN for all module references, including the community.postgresql module
6. Assign a `name:` field to every task
7. Trigger a PostgreSQL reload via a `notify:` handler after the pg_hba.conf update
8. Avoid `ignore_errors: true` or `ignore_errors: yes`
9. Use `true`/`false` for all boolean values

## Success Criteria

- **async parameter on migration task**: The migration task uses `async:` with a value of 3600 or greater (seconds), not blocking synchronously
- **poll: 0 on launch**: The migration launch task sets `poll: 0` to fire-and-forget
- **async_status polling task**: A separate task uses `ansible.builtin.async_status` with `until: job_result.finished` and `retries:` to poll until the migration completes
- **Collection install comment**: The playbook includes a comment (at the top or near the community.postgresql task) showing the `ansible-galaxy collection install community.postgresql` command
- **FQCN modules used**: All module references use FQCN, including the community.postgresql module
- **All tasks have name field**: Every task has a `name:` field
- **Service reload via handler**: The pg_hba.conf update task uses `notify:` to trigger a PostgreSQL reload handler rather than a direct restart task
- **No ignore_errors present**: The playbook contains no `ignore_errors: true` or `ignore_errors: yes`
- **Boolean true/false syntax**: All boolean values use `true` or `false`, not `yes` or `no`

## Failure Conditions

- Migration task runs synchronously (no `async:` parameter or `async` value is too small)
- Migration launch task does not set `poll: 0`
- No polling task uses `ansible.builtin.async_status` to wait for completion
- No comment documents the collection install command
- Module references use short names instead of FQCN
- Any task is missing a `name:` field
- pg_hba.conf update triggers a direct restart instead of notifying a handler
- `ignore_errors: true` or `ignore_errors: yes` is present in the playbook
- Boolean values use `yes` or `no` instead of `true` or `false`
