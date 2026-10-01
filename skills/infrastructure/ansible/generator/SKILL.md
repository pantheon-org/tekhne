---
name: ansible-generator
description: Generates, validates, and refactors production-ready Ansible playbooks, roles, task files, and inventory configurations following current best practices. Use when the user asks to create, build, or generate Ansible automation, YAML playbooks, infrastructure as code, configuration management files, DevOps roles, or .yml files for Ansible — including requests like "create a playbook to...", "build a role for...", "generate an inventory for...", or "set up Ansible to automate...". Automatically validates all output using the devops-skills:ansible-validator skill.
---

# Ansible Generator

## Overview

Generate production-ready Ansible resources (playbooks, roles, task files, inventory files, project configs) following current best practices, naming conventions, and security standards. All generated resources are validated using the `devops-skills:ansible-validator` skill before delivery.

## Philosophy

- Generate secure, idempotent automation by default: FQCN modules, named tasks, `no_log` for secrets, strict file modes.
- Start from the shipped templates and references rather than from a blank file.
- Validation is part of generation, not an optional extra.
- Prefer builtin modules; fall back to them when a collection is unavailable.

## When to Use

- The user asks to create, build or generate an Ansible playbook, role, task file or inventory.
- The user wants project configuration such as `ansible.cfg`, `requirements.yml` or `.ansible-lint`.
- An existing Ansible file needs refactoring to current standards (FQCN, `true`/`false` booleans, `dnf` on RHEL).
- A request involves a non-builtin collection and needs current module documentation looked up.

## When Not to Use

- Only validating or linting existing Ansible code without changing it (use the ansible-validator skill).
- Generating Kubernetes, Terraform or Helm resources (use the matching generator skill).
- Executing playbooks against live hosts, which this skill does not do.
- Producing documentation snippets where full validation is not wanted.

## Core Capabilities

> **All capabilities follow the same validation loop:** generate → invoke `devops-skills:ansible-validator` → fix errors → re-validate → present output. See [Validation Workflow](#validation-workflow) for full details.

### 1. Generate Playbooks

**Process:**
1. Clarify hosts, privileges, OS
2. Read `references/best-practices.md` and `references/module-patterns.md`
3. Use `assets/templates/playbook/basic_playbook.yml` as structural reference
4. Generate following mandatory standards (see [Mandatory Standards](#mandatory-standards))

**Example structure:**
```yaml
---
# Playbook: <title>
# Description: <what it does>
# Requirements: Ansible 2.10+, <OS>
# Variables:
#   - <var_name>: <description> (default: <value>)
# Usage: ansible-playbook -i inventory/<env> <playbook>.yml

- name: <Verb phrase describing the play>
  hosts: <group>
  become: true
  gather_facts: true
  vars:
    app_port: 8080

  pre_tasks:
    - name: <Setup steps>
      # ...

  tasks:
    - name: <Verb-first task name>
      ansible.builtin.<module>:
        # parameters
      tags: [<tag1>, <tag2>]

  post_tasks:
    - name: <Verification steps>
      # ...

  handlers:
    - name: <Handler name>
      ansible.builtin.service:
        name: <service>
        state: reloaded
```

---

### 2. Generate Roles

**Process:**
1. Clarify role purpose and scope
2. Copy and customize the full role structure from `assets/templates/role/`:
   - `tasks/main.yml`, `handlers/main.yml`, `templates/`, `files/`
   - `vars/main.yml`, `vars/Debian.yml`, `vars/RedHat.yml`
   - `defaults/main.yml`, `meta/main.yml`, `meta/argument_specs.yml` (Ansible 2.11+), `README.md`
3. Replace all `[PLACEHOLDERS]`: `[ROLE_NAME]`, `[role_name]`, `[PLAYBOOK_DESCRIPTION]`, `[package_name]`, `[service_name]`, `[default_port]`
4. Prefix all role variables with the role name (e.g., `nginx_port`, `nginx_worker_processes`)
5. Use `include_vars` for OS-specific variables

**`meta/argument_specs.yml`** enables automatic variable validation (Ansible 2.11+).

---

### 3. Generate Task Files

**Process:**
1. Define the operation
2. Reference `references/module-patterns.md` for module usage
3. Generate with: verb-first names, FQCN modules, idempotency checks, tags

See `assets/templates/` for full task file examples (e.g., database backup, user management).

---

### 4. Generate Inventory Files

**Process:**
1. Understand infrastructure topology
2. Use `assets/templates/inventory/` as reference:
   - `hosts` — main inventory (INI for simple; YAML for complex hierarchies)
   - `group_vars/all.yml`, `group_vars/[groupname].yml`, `host_vars/[hostname].yml`
3. Organize hosts into logical groups (functional, environment, geographic)
4. Define variables at appropriate levels: all → group → host

**Dynamic inventory (cloud):** Use provider plugins configured from `references/module-patterns.md`:
- AWS EC2: `plugin: amazon.aws.aws_ec2`
- Azure: `plugin: azure.azcollection.azure_rm`

---

### 5. Generate Project Configuration Files

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

````markdown
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
````

---

## Anti-Patterns

### NEVER use `gather_facts: true` by default for large inventories

- **WHY:** Fact gathering adds 2-5 seconds per host at connection time; for playbooks targeting hundreds of hosts this significantly increases total runtime for plays that do not need facts.
- **BAD:** Relying on the default `gather_facts` behaviour in every play, including utility plays that never reference `ansible_*` variables.
- **GOOD:** Set `gather_facts: false` globally in `ansible.cfg` and enable it per-play only when facts are actually needed (conditionals, templates using `ansible_os_family`, etc.).

### NEVER store secrets in `group_vars/` plaintext files

- **WHY:** Any plaintext password or API key committed to `group_vars/` is permanently exposed in source control history, even after deletion.
- **BAD:** `ansible_become_password: mypassword` in `group_vars/all.yml` committed to the repository.
- **GOOD:** Use Ansible Vault (`ansible-vault encrypt_string`) or an external secrets manager (HashiCorp Vault, AWS Secrets Manager) and reference values via lookup plugins.

### NEVER use the `shell` or `command` module when a dedicated module exists

- **WHY:** `shell` and `command` bypass idempotency guarantees, built-in error handling, and change detection that dedicated modules provide; they also resist linting and security scanning.
- **BAD:** `ansible.builtin.shell: pip install requests` instead of using the `pip` module.
- **GOOD:** `ansible.builtin.pip: name: requests state: present`: use the purpose-built module so Ansible can detect and report actual state changes.

### NEVER write tasks without `name:` fields

- **WHY:** Unnamed tasks produce unreadable playbook output and make debugging nearly impossible when a play contains many tasks; they also fail `ansible-lint` name rules.
- **BAD:** `- apt: name=nginx state=present` with no `name:` field.
- **GOOD:** Always prefix every task with a descriptive `name:`, e.g., `- name: Install nginx web server`.

### NEVER use `ignore_errors: true` as a general exception handler

- **WHY:** `ignore_errors: true` silently swallows all failures and lets the playbook continue in a potentially broken state, masking errors that affect downstream tasks.
- **BAD:** `ignore_errors: true` on a package installation task where failure means the service cannot start.
- **GOOD:** Use `failed_when` with specific conditions to define expected failure states, or use `block/rescue/always` for structured error handling with recovery logic.

### NEVER use short module names

- **WHY:** The Mandatory Standards require fully qualified collection names so each task is unambiguous and passes lint.
- **BAD:** `copy:` or `apt:` as the module key.
- **GOOD:** `ansible.builtin.copy:` and `ansible.builtin.apt:`.

### NEVER write `yes`/`no` booleans

- **WHY:** The Mandatory Standards require `true`/`false`, which is the form current lint rules expect.
- **BAD:** `become: yes`
- **GOOD:** `become: true`

### NEVER use `ansible.builtin.yum` for RHEL packages

- **WHY:** The Mandatory Standards specify `ansible.builtin.dnf` for RHEL, and the multi-OS pattern uses it for RHEL 8+.
- **BAD:** `ansible.builtin.yum: name=nginx state=present`
- **GOOD:** `ansible.builtin.dnf:` with `name: nginx` and `state: present`.

### NEVER leave secret-handling tasks without `no_log: true`

- **WHY:** Without it, passwords and tokens are printed in plain text in play output and logs.
- **BAD:** A task that sets a database password and logs its parameters.
- **GOOD:** Add `no_log: true` to the task and keep the value in Ansible Vault.

### NEVER create files with permissive modes

- **WHY:** The standards call for `'0644'` on configs and `'0600'` on secrets; world-writable files are a security defect.
- **BAD:** `mode: '0777'` on a template that holds credentials.
- **GOOD:** `mode: '0600'` for secrets and `mode: '0644'` for ordinary configs.

### NEVER present generated output without validating it

- **WHY:** The Validation Workflow requires every generated resource to pass the ansible-validator skill (YAML syntax, Ansible syntax, lint) before delivery.
- **BAD:** Hand over a freshly written playbook with no validation run.
- **GOOD:** Generate, validate, fix errors, re-validate, then present using the required output format.

### NEVER leave template `[PLACEHOLDERS]` in delivered roles

- **WHY:** Unreplaced placeholders such as `[ROLE_NAME]` or `[package_name]` break the role and show the template was not customised.
- **BAD:** Copy `assets/templates/role/` and deliver it with `[ROLE_NAME]` still in `tasks/main.yml`.
- **GOOD:** Replace every placeholder and prefix role variables with the role name (e.g. `nginx_port`).

## References

### References (read at generation start)

- `references/best-practices.md` — directory structures, naming conventions, security, performance, common pitfalls
- `references/module-patterns.md` — module usage patterns, copy-paste examples for all common modules

### Assets (structural templates)

- `assets/templates/playbook/basic_playbook.yml` — playbook structure reference
- `assets/templates/role/*` — role directory structure and variable conventions
- `assets/templates/inventory/*` — host grouping and group_vars/host_vars patterns
- `assets/templates/project/*` — `ansible.cfg`, `requirements.yml`, `.ansible-lint`

**Template usage:** Review structure → generate following the same pattern → replace `[PLACEHOLDERS]` → customize for requirements → remove inapplicable sections → validate.
