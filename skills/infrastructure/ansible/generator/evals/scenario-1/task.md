# Database User and Credentials Setup Playbook

## Problem/Feature Description

A SaaS company is setting up automated database provisioning for their PostgreSQL clusters. The operations team currently stores database passwords in a shared spreadsheet and manually creates database users over SSH — a compliance nightmare discovered during a recent internal audit.

The team needs an Ansible playbook that provisions application database users and sets their passwords on a group of `postgres_servers`. The playbook must create three database users: `app_readonly` (password from variable `pg_readonly_password`), `app_writer` (password from variable `pg_writer_password`), and `app_admin` (password from variable `pg_admin_password`). Passwords must never appear in playbook run logs. The playbook should also deploy a `pg_hba.conf` file from a template with mode `0640`.

The team plans to store the password variables in an encrypted `group_vars/postgres_servers/vault.yml` file. The main inventory `group_vars/postgres_servers/vars.yml` file should not contain any secret values.

## Output Specification

Produce the following files:
- `setup_postgres_users.yml` — the main playbook
- `group_vars/postgres_servers/vars.yml` — non-secret variables (database names, connection settings)
- `group_vars/postgres_servers/vault.yml` — a template/example showing which variables this file should define (use placeholder values like `!vault |` references or clearly labelled placeholder strings to show the structure without real secrets)
- `templates/pg_hba.conf.j2` — a basic pg_hba.conf Jinja2 template
