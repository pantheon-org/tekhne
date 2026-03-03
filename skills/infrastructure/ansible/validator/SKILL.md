---
name: ansible-validator
description: Comprehensive toolkit for validating, linting, testing, and automating Ansible playbooks, roles, and collections. Use this skill when working with Ansible files (.yml, .yaml playbooks, roles, inventories), validating automation code, debugging playbook execution, performing dry-run testing with check mode, or working with custom modules and collections.
---

# Ansible Validator

## Overview

Comprehensive toolkit for validating, linting, and testing Ansible playbooks, roles, and collections. Ensures code quality through syntax validation, lint enforcement, dry-run testing, automatic molecule testing for roles, and intelligent documentation lookup for custom modules with version awareness.

**Key Behavior:** Molecule tests run automatically for roles with `molecule/` directories. If blocked by environment issues, document the blocker and continue with other validation steps.

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
   ├─> Check for deprecated modules (references/module_alternatives.md)
   ├─> Detect non-FQCN usage: bash scripts/check_fqcn.sh
   └─> Verify role structure

4. Dry-Run Testing (check mode)
   └─> Run ansible-playbook --check --diff (if inventory available)

5. Molecule Testing (for roles)
   ├─> Check if molecule/ directory exists
   ├─> If present, run bash scripts/test_role.sh <role-path> automatically
   └─> Report results (pass/fail/blocked) with any environmental blockers

6. Custom Module/Collection Analysis (if detected)
   ├─> Run bash scripts/extract_ansible_info_wrapper.sh <path> to extract info
   ├─> Lookup docs via Context7 MCP or WebSearch (see Custom Module Lookup below)
   └─> Provide version-specific guidance

6. Security and Best Practices Review
   ├─> Run bash scripts/validate_playbook_security.sh or validate_role_security.sh (Checkov)
   └─> Run bash scripts/scan_secrets.sh (detects hardcoded credentials)

7. Reference Documentation
   ├─> Read references/common_errors.md when errors detected
   ├─> Read references/best_practices.md when warnings detected
   ├─> Read references/module_alternatives.md for deprecated/non-FQCN modules
   └─> Read references/security_checklist.md when security issues found
```

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

### 3. Security Scanning

Run both Checkov and secrets scanning:

```bash
bash scripts/validate_playbook_security.sh playbook.yml
bash scripts/validate_role_security.sh roles/webserver/
bash scripts/scan_secrets.sh <playbook.yml|role-dir|directory>
```

Checkov covers SSL/TLS, HTTPS enforcement, package GPG verification. Secrets scan catches hardcoded credentials. Key checks: `CKV_ANSIBLE_1-6`, `CKV2_ANSIBLE_1-6`. Full policy: <https://www.checkov.io/5.Policy%20Index/ansible.html>

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

### 5. Molecule Testing

When `molecule/` is detected, `bash scripts/test_role.sh <role-path>` runs automatically.

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

## Integration with Other Skills

- **k8s-yaml-validator** — when Ansible manages Kubernetes resources
- **terraform-validator** — when Ansible and Terraform are used together
- **k8s-debug** — for debugging infrastructure managed by Ansible

## Notes

- Validation order: YAML syntax → Ansible syntax → Lint → Security scan → Secrets scan → Check mode → Molecule tests
- Use Ansible Vault for all sensitive data; never commit unencrypted secrets
- Pin collection versions in `requirements.yml`; test before upgrading
