# Database Migration Playbook for Large PostgreSQL Cluster

## Problem/Feature Description

A data engineering team needs to run a database schema migration across a fleet of `postgres_servers`. The migration script at `/opt/migrations/run_migration.sh` typically takes 15 to 45 minutes per server. When the team ran a previous migration synchronously, Ansible timed out and left servers in inconsistent states.

The team also needs to manage the `pg_hba.conf` access rules file using the `community.postgresql` collection (specifically `community.postgresql.postgresql_pg_hba` module) after the migration completes to add a new application user access rule.

The playbook must:
1. Launch the migration script asynchronously (fire and forget), then poll until it completes (max 1 hour).
2. After migration succeeds, use the community.postgresql collection to update `pg_hba.conf` with a new entry allowing `app_user` from `10.0.0.0/8` via `md5` authentication.
3. Reload the PostgreSQL service if pg_hba.conf changes.

The servers run RHEL 9. Assume the necessary Python bindings are available.

## Output Specification

Produce a complete Ansible playbook file named `run_db_migration.yml`. The playbook should include proper documentation of any collection dependencies.
