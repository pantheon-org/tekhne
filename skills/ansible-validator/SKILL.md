---
name: ansible-validator
description: Comprehensive toolkit for validating, linting, testing, and automating Ansible playbooks, roles, and collections. Use this skill when working with Ansible files (.yml, .yaml playbooks, roles, inventories), validating automation code, debugging playbook execution, performing dry-run testing with check mode, or working with custom modules and collections.
---

# Ansible Validator

## Overview

Comprehensive toolkit for validating, linting, and testing Ansible playbooks, roles, and collections. Ensures code quality through syntax validation, lint enforcement, dry-run testing, automatic molecule testing for roles, and intelligent documentation lookup for custom modules with version awareness.

**IMPORTANT BEHAVIOR:** When validating any Ansible role, this skill AUTOMATICALLY runs molecule tests if a `molecule/` directory is detected in the role. This is non-negotiable and happens without asking for user permission. If molecule tests cannot run due to environmental issues (Docker, version compatibility), the skill documents the blocker but continues with other validation steps.

## Validation Workflow

Follow this decision tree for comprehensive Ansible validation:

```
0. Tool Prerequisites Check (RECOMMENDED for first-time validation)
   └─> Run bash scripts/setup_tools.sh for diagnostics

1. Identify Ansible files in scope
   ├─> Single playbook / role / collection / multi-playbook validation

2. Syntax Validation
   ├─> Run ansible-playbook --syntax-check
   └─> Run yamllint for YAML syntax

3. Lint and Best Practices
   ├─> Run ansible-lint
   ├─> Check for deprecated modules (see references/module_alternatives.md)
   ├─> **DETECT NON-FQCN MODULE USAGE** (apt vs ansible.builtin.apt)
   │   └─> Run bash scripts/check_fqcn.sh
   │   └─> Recommend FQCN alternatives from references/module_alternatives.md
   └─> Verify role structure

4. Dry-Run Testing (check mode)
   └─> Run ansible-playbook --check --diff (if inventory available)

5. Molecule Testing (for roles) - AUTOMATIC
   ├─> Check if molecule/ directory exists in role
   ├─> If present, ALWAYS run bash scripts/test_role.sh <role-path> automatically
   └─> Report results (pass/fail/blocked) including any environmental blockers

6. Custom Module/Collection Analysis (if detected)
   ├─> Run bash scripts/extract_ansible_info_wrapper.sh <path> to extract info
   ├─> Lookup docs via Context7 MCP or WebSearch (see Custom Module Lookup below)
   └─> Provide version-specific guidance

7. Security and Best Practices Review - DUAL SCANNING REQUIRED
   ├─> Run bash scripts/validate_playbook_security.sh or validate_role_security.sh (Checkov)
   └─> **ALSO run bash scripts/scan_secrets.sh** (catches passwords, API keys, tokens Checkov may miss)

8. Reference Documentation - MANDATORY CONSULTATION
   ├─> **MUST READ** references/common_errors.md when ANY errors are detected
   ├─> **MUST READ** references/best_practices.md when warnings are detected
   ├─> **MUST READ** references/module_alternatives.md for deprecated or non-FQCN modules
   └─> **MUST READ** references/security_checklist.md when security issues found
```

**CRITICAL: Reference files are NOT optional.** When issues are detected, the corresponding reference file MUST be read and its guidance applied. Simply mentioning the reference file path is insufficient — the content must be consulted and relevant guidance extracted.

## Core Capabilities (Summary)

### 1. YAML Syntax Validation

Run `yamllint` then `ansible-playbook --syntax-check`. Fix YAML errors first, then Ansible-specific issues. See `references/common_errors.md` for solutions.

```bash
yamllint -c .yamllint .
ansible-playbook playbook.yml --syntax-check
```

### 2. Ansible Lint

Run `ansible-lint` to enforce best practices. Use `--fix` for auto-fixable issues; review before applying.

```bash
ansible-lint playbook.yml          # single playbook
ansible-lint .                     # entire directory
ansible-lint -x yaml[line-length] playbook.yml  # skip rule
ansible-lint -f pep8 playbook.yml  # parseable output
```

### 3. Security Scanning (Checkov + Secrets)

**Always run both scripts.** Checkov covers SSL/TLS, HTTPS enforcement, package GPG verification, and cloud misconfigurations. `scan_secrets.sh` covers hardcoded credentials Checkov may miss.

```bash
bash scripts/validate_playbook_security.sh playbook.yml
bash scripts/validate_role_security.sh roles/webserver/
bash scripts/scan_secrets.sh <playbook.yml|role-dir|directory>
```

Key Checkov checks: `CKV_ANSIBLE_1-6` (cert validation, GPG), `CKV2_ANSIBLE_1-6` (HTTPS, dnf, error handling). Full policy index: <https://www.checkov.io/5.Policy%20Index/ansible.html>

```yaml
# BAD
- get_url:
    url: https://example.com/file.tar.gz
    dest: /tmp/file.tar.gz
    validate_certs: false

# GOOD
- get_url:
    url: https://example.com/file.tar.gz
    dest: /tmp/file.tar.gz
    # validate_certs: true (default)
```

### 4. Dry-Run Testing (Check Mode)

```bash
ansible-playbook -i inventory playbook.yml --check --diff
ansible-playbook -i inventory playbook.yml --check -v --limit webservers
ansible-playbook -i inventory playbook.yml --check --tags deploy
```

Interpret output: `ok` = no change, `changed` = would change, `failed` = would fail (check check_mode support), `skipped` = conditional skip. Note: not all modules support check mode — use `check_mode: no` override where needed.

### 5. Molecule Testing (Automatic for Roles)

When a `molecule/` directory is detected, `bash scripts/test_role.sh <role-path>` runs automatically without prompting.

```bash
molecule init scenario --driver-name docker  # initialize
molecule test                                # full sequence
molecule test -s alternative                 # specific scenario
molecule converge && molecule login          # debug mode
```

Test sequence: dependency → lint → destroy → syntax → create → prepare → converge → idempotence → verify → destroy. Check `molecule/default/molecule.yml` for driver and platform configuration.

If molecule fails due to environment (Docker, versions), document the blocker but don't fail overall validation. If molecule fails due to role code, provide detailed debugging steps.

### 6. Custom Module and Collection Documentation Lookup

```bash
bash scripts/extract_ansible_info_wrapper.sh <playbook-or-role>
# Outputs JSON: modules, collections, versions
```

**For public collections** (community.general, ansible.posix, cisco.ios, etc.):
1. `mcp__context7__resolve-library-id` with collection name
2. `mcp__context7__get-library-docs` for specific module

**For custom/private collections** — use WebSearch:
```
"[collection-namespace].[module-name] ansible version [version] documentation"
"ansible [module-name] error: [error-message]"
```

Check version compatibility against `requirements.yml` / `galaxy.yml`; flag deprecated parameters and breaking changes.

### 7. Security and Best Practices

Key areas (see `references/security_checklist.md` and `references/best_practices.md` for full checklists):

- **Secrets:** Use Ansible Vault / environment variables; never hardcode credentials
- **Privilege escalation:** Minimize `become: yes` scope; always specify `become_user`
- **File permissions:** Always set `mode:` on file/template tasks; encrypt sensitive files
- **Command injection:** Use `quote` filter; avoid raw `{{ var }}` in shell/command modules
- **Network:** Enforce HTTPS; never disable `validate_certs`
- **Idempotency:** Prefer proper modules over `command`/`shell`; use `creates`/`removes` when needed
- **Error handling:** Use `block/rescue/always`; set `failed_when` for custom conditions

## Tool Prerequisites

```bash
ansible --version && ansible-playbook --version
ansible-lint --version
yamllint --version
molecule --version

# Install missing tools
pip install ansible ansible-lint yamllint ansible-compat
pip install molecule molecule-docker
```

**Minimum versions:** Ansible >= 2.9 (recommend >= 2.12), ansible-lint >= 6.0.0, yamllint >= 1.26.0, molecule >= 3.4.0

## Error Troubleshooting

Consult `references/common_errors.md` first. Quick reference:

| Error | Solution |
|---|---|
| Module Not Found | `ansible-galaxy collection install <ns.name>`; check `collections/requirements.yml` |
| Undefined Variable | Define in vars/defaults/group_vars; use `default()` filter |
| Template Syntax Error | Verify Jinja2 syntax; check variable types match filters |
| Connection Failed | Verify SSH config/keys; test with `ansible -m ping` |
| Permission Denied | Add `become: yes`; verify sudo config |
| Deprecated Module | Check `references/module_alternatives.md` for FQCN replacement |

## Scripts Reference

| Script | Purpose | Usage |
|---|---|---|
| `setup_tools.sh` | Check/diagnose required tools | `bash scripts/setup_tools.sh` |
| `extract_ansible_info_wrapper.sh` | Parse playbooks/roles for module/collection info | `bash scripts/extract_ansible_info_wrapper.sh <path>` |
| `validate_playbook.sh` | Syntax + yamllint + ansible-lint | `bash scripts/validate_playbook.sh <playbook.yml>` |
| `validate_playbook_security.sh` | Checkov security scan for playbooks | `bash scripts/validate_playbook_security.sh <playbook.yml\|dir>` |
| `validate_role.sh` | Structure + syntax + lint + molecule config check | `bash scripts/validate_role.sh <role-dir>` |
| `validate_role_security.sh` | Checkov security scan for roles | `bash scripts/validate_role_security.sh <role-dir>` |
| `test_role.sh` | Molecule test wrapper (auto-installs deps) | `bash scripts/test_role.sh <role-dir> [scenario]` |
| `scan_secrets.sh` | Grep-based hardcoded secret detection | `bash scripts/scan_secrets.sh <playbook\|role-dir\|dir>` |
| `check_fqcn.sh` | Detect non-FQCN module usage | `bash scripts/check_fqcn.sh <playbook\|role-dir\|dir>` |

All validation scripts auto-install required tools in a temporary venv if not available system-wide.

## References

| File | Purpose |
|---|---|
| `references/security_checklist.md` | Security vulnerabilities checklist |
| `references/best_practices.md` | Ansible coding standards |
| `references/common_errors.md` | Common errors and solutions |
| `references/module_alternatives.md` | Deprecated module → FQCN migration guide |
| `assets/.yamllint` | Pre-configured yamllint rules |
| `assets/.ansible-lint` | Pre-configured ansible-lint configuration |
| `assets/molecule.yml.template` | Molecule configuration template |

## Workflow Examples

### Validate a Single Playbook

```
1. yamllint → ansible-playbook --syntax-check → ansible-lint
2. bash scripts/validate_playbook_security.sh playbook.yml
3. bash scripts/scan_secrets.sh playbook.yml
4. If custom modules detected, run extract_ansible_info_wrapper.sh and lookup docs
5. Consult references/ for any issues found; propose fixes
```

### Validate an Ansible Role

```
1. bash scripts/validate_role.sh ./roles/webserver/
2. bash scripts/validate_role_security.sh ./roles/webserver/
3. bash scripts/scan_secrets.sh ./roles/webserver/
4. **CRITICAL:** If molecule/ exists → AUTOMATICALLY run bash scripts/test_role.sh ./roles/webserver/
5. Consult references/ for issues; provide debugging steps for molecule failures
```

### Dry-Run for Production

```
1. Verify inventory exists
2. ansible-playbook --check --diff -i production playbook.yml
3. Highlight changed tasks, handler notifications, security concerns
4. Recommend safe/unsafe to apply
```

### Custom Collection Documentation

```
User: "community.postgresql.postgresql_db version 2.3.0 — what parameters?"
1. mcp__context7__resolve-library-id("ansible community.postgresql")
2. mcp__context7__get-library-docs for postgresql_db module
3. Fallback: WebSearch "ansible community.postgresql version 2.3.0 postgresql_db documentation"
4. Extract required vs optional params, examples, version notes
```

## Integration with Other Skills

- **k8s-yaml-validator** — when Ansible manages Kubernetes resources
- **terraform-validator** — when Ansible and Terraform are used together
- **k8s-debug** — for debugging infrastructure managed by Ansible

## Notes

- Validation order: YAML syntax → Ansible syntax → Lint → Security scan → Secrets scan → Check mode → Molecule tests
- **CRITICAL:** Role with `molecule/` directory → run molecule tests automatically, no user prompt required
- Use Ansible Vault for all sensitive data; never commit unencrypted secrets
- Pin collection versions in `requirements.yml`; test before upgrading
