---
name: ansible-generator
description: Generates, validates, and refactors production-ready Ansible playbooks, roles, task files, and inventory configurations following current best practices. Use when the user asks to create, build, or generate Ansible automation, YAML playbooks, infrastructure as code, configuration management files, DevOps roles, or .yml files for Ansible — including requests like "create a playbook to...", "build a role for...", "generate an inventory for...", or "set up Ansible to automate...". Automatically validates all output using the devops-skills:ansible-validator skill.
---

# Ansible Generator

## Overview

Generate production-ready Ansible resources (playbooks, roles, task files, inventory files, project configs) following current best practices, naming conventions, and security standards. All generated resources are validated using the `devops-skills:ansible-validator` skill before delivery.

## Core Capabilities

### 1. Generate Playbooks

**Triggers:** "Create a playbook to...", "Build a playbook for...", "Generate playbook that..."

**Process:**
1. Clarify target hosts, required privileges, and OS
2. Read `references/best-practices.md` and `references/module-patterns.md`
3. Use `assets/templates/playbook/basic_playbook.yml` as structural reference
4. Generate following mandatory standards (see [Mandatory Standards](#mandatory-standards))
5. Validate per [Validation Workflow](#validation-workflow)

**Example structure:**
```yaml
---
# Playbook: Deploy Web Application
# Description: Deploy nginx web server with SSL
# Requirements: Ansible 2.10+, Ubuntu 20.04+
# Variables:
#   - app_port: Application port (default: 8080)
# Usage: ansible-playbook -i inventory/production deploy_web.yml

- name: Deploy and configure web server
  hosts: webservers
  become: true
  gather_facts: true
  vars:
    app_port: 8080

  pre_tasks:
    - name: Update package cache
      ansible.builtin.apt:
        update_cache: true
        cache_valid_time: 3600
      when: ansible_os_family == "Debian"

  tasks:
    - name: Ensure nginx is installed
      ansible.builtin.package:
        name: nginx
        state: present
      tags: [install, nginx]

    - name: Deploy nginx configuration
      ansible.builtin.template:
        src: templates/nginx.conf.j2
        dest: /etc/nginx/nginx.conf
        mode: '0644'
        backup: true
        validate: 'nginx -t -c %s'
      notify: Reload nginx
      tags: [configure]

  post_tasks:
    - name: Verify nginx is responding
      ansible.builtin.uri:
        url: "http://localhost:{{ app_port }}/health"
        status_code: 200
      register: health_check
      until: health_check.status == 200
      retries: 5
      delay: 10

  handlers:
    - name: Reload nginx
      ansible.builtin.service:
        name: nginx
        state: reloaded
```

---

### 2. Generate Roles

**Triggers:** "Create a role for...", "Generate a role to...", "Build role that..."

**Process:**
1. Clarify role purpose and scope
2. Copy and customize the full role structure from `assets/templates/role/`:
   - `tasks/main.yml`, `handlers/main.yml`, `templates/`, `files/`
   - `vars/main.yml`, `vars/Debian.yml`, `vars/RedHat.yml`
   - `defaults/main.yml`, `meta/main.yml`, `meta/argument_specs.yml` (Ansible 2.11+), `README.md`
3. Replace all `[PLACEHOLDERS]`: `[ROLE_NAME]`, `[role_name]`, `[PLAYBOOK_DESCRIPTION]`, `[package_name]`, `[service_name]`, `[default_port]`
4. Prefix all role variables with the role name (e.g., `nginx_port`, `nginx_worker_processes`)
5. Use `include_vars` for OS-specific variables
6. Validate per [Validation Workflow](#validation-workflow)

**`meta/argument_specs.yml`** enables automatic variable validation — always include it when generating roles (Ansible 2.11+).

---

### 3. Generate Task Files

**Triggers:** "Create tasks to...", "Generate task file for..."

**Process:**
1. Define the specific operation
2. Reference `references/module-patterns.md` for module usage
3. Generate with: verb-first task names, FQCN modules, idempotency checks, appropriate tags
4. Validate per [Validation Workflow](#validation-workflow)

**Example (database backup):**
```yaml
---
# Tasks: Database backup operations

- name: Create backup directory
  ansible.builtin.file:
    path: "{{ backup_dir }}"
    state: directory
    mode: '0755'
    owner: postgres
    group: postgres

- name: Dump PostgreSQL database
  ansible.builtin.command: >
    pg_dump -h {{ db_host }} -U {{ db_user }} -d {{ db_name }}
    -f {{ backup_dir }}/{{ db_name }}_{{ ansible_date_time.date }}.sql
  environment:
    PGPASSWORD: "{{ db_password }}"
  no_log: true
  changed_when: true

- name: Compress backup file
  ansible.builtin.archive:
    path: "{{ backup_dir }}/{{ db_name }}_{{ ansible_date_time.date }}.sql"
    dest: "{{ backup_dir }}/{{ db_name }}_{{ ansible_date_time.date }}.sql.gz"
    format: gz
    remove: true

- name: Find old backups
  ansible.builtin.find:
    paths: "{{ backup_dir }}"
    patterns: "*.sql.gz"
    age: "{{ backup_retention_days }}d"
  register: old_backups

- name: Delete old backup files
  ansible.builtin.file:
    path: "{{ item.path }}"
    state: absent
  loop: "{{ old_backups.files }}"
```

---

### 4. Generate Inventory Files

**Triggers:** "Create inventory for...", "Generate inventory file..."

**Process:**
1. Understand the infrastructure topology
2. Use `assets/templates/inventory/` as reference:
   - `hosts` — main inventory (INI for simple; YAML for complex hierarchies)
   - `group_vars/all.yml`, `group_vars/[groupname].yml`, `host_vars/[hostname].yml`
3. Organize hosts into logical groups (functional, environment, geographic) with `[group:children]` hierarchies
4. Define variables at appropriate levels: all → group → host

**Dynamic inventory (cloud):** Use provider plugins configured from `references/module-patterns.md`:
- AWS EC2: `plugin: amazon.aws.aws_ec2`
- Azure: `plugin: azure.azcollection.azure_rm`

---

### 5. Generate Project Configuration Files

**Triggers:** "Set up Ansible project", "Initialize Ansible configuration"

Use templates from `assets/templates/project/`:
- `ansible.cfg` — forks, timeout, paths
- `requirements.yml` — collections and roles dependencies
- `.ansible-lint` — lint rules

---

### 6. Handling Custom Modules and Collections

When a user mentions a non-builtin collection (e.g., `kubernetes.core`, `amazon.aws`, `community.docker`):

1. **Search for current documentation:**
   ```
   "ansible [collection.name] [module] latest documentation examples"
   ```
2. **If Context7 MCP is available:** Use `mcp__context7__resolve-library-id` then `mcp__context7__get-library-docs`
3. **Generate using discovered info:** correct FQCN, current parameters, collection install instructions

**Include installation instructions in comments:**
```yaml
# Requirements:
#   - ansible-galaxy collection install kubernetes.core:2.4.0
# or in requirements.yml:
# collections:
#   - name: kubernetes.core
#     version: "2.4.0"
```

---

## Mandatory Standards

All generated resources must follow these standards. See `references/best-practices.md` for full details and rationale.

**Key rules at a glance:**

| Standard | Correct | Incorrect |
|---|---|---|
| FQCN | `ansible.builtin.copy` | `copy` |
| Booleans | `true`/`false` | `yes`/`no` |
| RHEL packages | `ansible.builtin.dnf` | `ansible.builtin.yum` |
| Secrets | `no_log: true` | plain logging |
| File perms | `'0644'` configs, `'0600'` secrets | world-writable |

### Module Selection Priority

1. **`ansible.builtin.*`** — always first choice
2. **Official collection modules** — only when no builtin alternative exists
3. **`community.*` modules** — third choice
4. **`command`/`shell`** — last resort only

### Builtin Fallback Pattern

When validation fails due to missing collections, rewrite using builtins:

```yaml
# Preferred (requires community.postgresql):
# - community.postgresql.postgresql_db: {name: mydb, state: present}

# Builtin fallback:
- name: Check if database exists
  ansible.builtin.command:
    cmd: psql -tAc "SELECT 1 FROM pg_database WHERE datname='mydb'"
  become: true
  become_user: postgres
  register: db_check
  changed_when: false

- name: Create database
  ansible.builtin.command:
    cmd: psql -c "CREATE DATABASE mydb"
  become: true
  become_user: postgres
  when: db_check.stdout != "1"
  changed_when: true
```

---

## Common Patterns

### Multi-OS Support
```yaml
- name: Install nginx (Debian/Ubuntu)
  ansible.builtin.apt:
    name: nginx
    state: present
  when: ansible_os_family == "Debian"

- name: Install nginx (RHEL 8+)
  ansible.builtin.dnf:
    name: nginx
    state: present
  when: ansible_os_family == "RedHat"
```

### Async Long-Running Tasks
```yaml
- name: Run database migration
  ansible.builtin.command: /opt/app/migrate.sh
  async: 3600
  poll: 0
  register: migration

- name: Check migration status
  ansible.builtin.async_status:
    jid: "{{ migration.ansible_job_id }}"
  register: job_result
  until: job_result.finished
  retries: 360
  delay: 10
```

---

## Validation Workflow

**Every generated resource must be validated before presenting to the user.**

1. Generate the Ansible file
2. Invoke `devops-skills:ansible-validator`
3. If validation fails → fix errors → re-validate
4. If validation passes → present using the required output format

**Skip validation only when:** generating partial snippets, documentation examples, or when the user explicitly requests to skip.

### Required Output Format

```markdown
## Generated [Resource Type]: [Name]

**Validation Status:** ✅ All checks passed
- YAML syntax: Passed
- Ansible syntax: Passed
- Ansible lint: Passed

**Summary:**
- [What was generated and key decisions]

**Usage:**
```bash
[Exact command]
```

**Prerequisites:**
- [Required collections, system requirements]
```

---

## Resources

### References (read at generation start)

- `references/best-practices.md` — directory structures, naming conventions, security, performance, common pitfalls
- `references/module-patterns.md` — module usage patterns, copy-paste examples for all common modules

### Assets (structural templates)

- `assets/templates/playbook/basic_playbook.yml` — playbook structure reference
- `assets/templates/role/*` — role directory structure and variable conventions
- `assets/templates/inventory/*` — host grouping and group_vars/host_vars patterns
- `assets/templates/project/*` — `ansible.cfg`, `requirements.yml`, `.ansible-lint`

**Template usage:** Review structure → generate following the same pattern → replace `[PLACEHOLDERS]` → customize for requirements → remove inapplicable sections → validate.
