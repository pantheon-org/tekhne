# Scenario 03: Redis Role for Multi-OS Fleet

## User Prompt

A platform team manages servers running both Ubuntu 22.04 and RHEL 9. They need a reusable Ansible role called `redis` that installs and configures Redis on both operating systems. The role should be robust enough to be shared across multiple teams and several projects.

The role should install Redis, deploy a `redis.conf` from a Jinja2 template (configuring bind address, port, and max memory), ensure the service is running and enabled, and handle the fact that RHEL 9 uses `dnf` while Ubuntu uses `apt`. The Redis port defaults to 6379 and max memory defaults to `256mb`, but both should be overridable.

The team wants the role to be well-structured so new team members can easily understand and customise it without reading the entire codebase.

Produce a complete Ansible role directory structure for the `redis` role. Include at minimum:
- `tasks/main.yml`
- `handlers/main.yml`
- `defaults/main.yml`
- `vars/main.yml`
- `vars/Debian.yml`
- `vars/RedHat.yml`
- `templates/redis.conf.j2`
- `meta/main.yml`
- `README.md`

Also produce a short example playbook `example_playbook.yml` that demonstrates how to use the role.

## Expected Behavior

1. Prefix all custom role variables with `redis_` (e.g., `redis_port`, `redis_max_memory`, `redis_bind`)
2. Use `ansible.builtin.include_vars` to load OS-specific variable files (`vars/Debian.yml`, `vars/RedHat.yml`)
3. Use `ansible.builtin.dnf` for RHEL/CentOS package installation (not `yum`) and `ansible.builtin.apt` for Debian/Ubuntu
4. Include all required role directory files: `tasks/main.yml`, `handlers/main.yml`, `defaults/main.yml`, `vars/main.yml`, `meta/main.yml`
5. Trigger Redis service restart via a handler using `notify:` on config file deployment, not a direct restart task
6. Use FQCN for all module references in task files
7. Assign a `name:` field to every task
8. Add `tags:` fields to tasks using appropriate categories (e.g., `install`, `configure`)
9. Define overridable variables (port and max memory) in `defaults/main.yml`
10. Use `true`/`false` for all boolean values

## Success Criteria

- **Role variable prefix**: All custom variables are prefixed with `redis_` (e.g., `redis_port`, `redis_max_memory`, `redis_bind`) — no unprefixed variable names like `port` or `max_memory`
- **include_vars for OS-specific**: The tasks include a step using `ansible.builtin.include_vars` (or equivalent) to load OS-specific variable files, and separate `vars/Debian.yml` and `vars/RedHat.yml` files exist
- **dnf for RHEL, apt for Debian**: RHEL/CentOS package installation uses `ansible.builtin.dnf` (not `ansible.builtin.yum`), and Debian/Ubuntu uses `ansible.builtin.apt`
- **Complete role directory files**: The output includes at least: `tasks/main.yml`, `handlers/main.yml`, `defaults/main.yml`, `vars/main.yml`, and `meta/main.yml`
- **Service restart via handler**: Config file deployment uses `notify:` to trigger a Redis restart handler rather than a direct restart task
- **FQCN modules used**: All module references in task files use FQCN
- **All tasks have name field**: Every task has a `name:` field
- **Tags on tasks**: Tasks include `tags:` fields using appropriate categories (e.g., `install`, `configure`)
- **Defaults file has overridable vars**: `defaults/main.yml` contains the redis port and max memory variables with default values (so they can be overridden by callers)
- **Boolean true/false syntax**: All boolean values use `true` or `false`, not `yes` or `no`

## Failure Conditions

- Custom variables are not prefixed with `redis_`
- No `ansible.builtin.include_vars` step for OS-specific variables
- RHEL package installation uses `ansible.builtin.yum` instead of `ansible.builtin.dnf`
- One or more required role directory files are missing
- Config file deployment uses a direct restart task instead of notifying a handler
- Module references use short names instead of FQCN
- Any task is missing a `name:` field
- Tasks have no `tags:` fields
- `defaults/main.yml` is missing redis port or max memory variables
- Boolean values use `yes` or `no` instead of `true` or `false`
