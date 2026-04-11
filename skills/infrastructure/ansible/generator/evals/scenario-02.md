# Scenario 02: Database User and Credentials Setup Playbook

## User Prompt

A SaaS company is setting up automated database provisioning for their PostgreSQL clusters. The operations team currently stores database passwords in a shared spreadsheet and manually creates database users over SSH — a compliance nightmare discovered during a recent internal audit.

The team needs an Ansible playbook that provisions application database users and sets their passwords on a group of `postgres_servers`. The playbook must create three database users: `app_readonly` (password from variable `pg_readonly_password`), `app_writer` (password from variable `pg_writer_password`), and `app_admin` (password from variable `pg_admin_password`). Passwords must never appear in playbook run logs. The playbook should also deploy a `pg_hba.conf` file from a template with mode `0640`.

The team plans to store the password variables in an encrypted `group_vars/postgres_servers/vault.yml` file. The main inventory `group_vars/postgres_servers/vars.yml` file should not contain any secret values.

Produce the following files:
- `setup_postgres_users.yml` — the main playbook
- `group_vars/postgres_servers/vars.yml` — non-secret variables (database names, connection settings)
- `group_vars/postgres_servers/vault.yml` — a template/example showing which variables this file should define (use placeholder values like `!vault |` references or clearly labelled placeholder strings to show the structure without real secrets)
- `templates/pg_hba.conf.j2` — a basic pg_hba.conf Jinja2 template

## Expected Behavior

1. Apply `no_log: true` to every task that references a password variable to prevent credential leakage in logs
2. Keep all password and secret values out of the plaintext `vars.yml` file
3. Use `!vault |` prefixes or equivalent vault references in `vault.yml` to indicate encrypted values
4. Use quoted octal mode strings for file permissions (e.g., `'0640'`)
5. Use FQCN for all module references
6. Assign a `name:` field to every task
7. Use `true`/`false` for all boolean values
8. Avoid `ansible.builtin.shell` or `ansible.builtin.command` for database user creation

## Success Criteria

- **no_log on password tasks**: Every task that references a password variable (pg_readonly_password, pg_writer_password, pg_admin_password) has `no_log: true`
- **No plaintext secrets in vars.yml**: The `group_vars/postgres_servers/vars.yml` file does NOT contain any password or secret values — only non-sensitive configuration
- **Vault file uses vault references**: The `group_vars/postgres_servers/vault.yml` file uses `!vault |` prefixes or otherwise indicates vault-encrypted values (not plaintext passwords)
- **Quoted octal file permissions**: The pg_hba.conf deployment task uses a quoted octal mode string (e.g., `'0640'`)
- **FQCN modules used**: All module references use FQCN (e.g., `ansible.builtin.template`, `ansible.builtin.user` or community module FQCN)
- **All tasks have name field**: Every task in the playbook has a `name:` field
- **Boolean true/false syntax**: All boolean values use `true` or `false`, not `yes` or `no`
- **No shell/command for user creation**: Database user creation does NOT use `ansible.builtin.shell` or `ansible.builtin.command` when a dedicated module (ansible.builtin.user, community.postgresql.*) is appropriate

## Failure Conditions

- Any task referencing a password variable is missing `no_log: true`
- Plaintext password values appear in `vars.yml`
- The `vault.yml` file contains plaintext passwords instead of vault references
- File permission mode for pg_hba.conf is an unquoted integer
- Module references use short names instead of FQCN
- Any task is missing a `name:` field
- Boolean values use `yes` or `no` instead of `true` or `false`
- Database user creation uses `ansible.builtin.shell` or `ansible.builtin.command`
