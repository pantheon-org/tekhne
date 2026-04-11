# Scenario 03: Validate an Ansible Role with Molecule Tests

## User Prompt

A platform team has developed an Ansible role called `nginx-proxy` that sets up nginx as a reverse proxy. The role includes a Molecule test suite for local testing. A new team member is preparing to submit this role to the team's shared role library, and needs a full quality validation pass before submitting.

The role's molecule test environment uses Docker. Perform a complete validation of the role, documenting what you ran and in what order. If any environment blockers prevent molecule tests from running, document the blocker clearly but continue with all other validation steps.

Your report should give the reviewer enough information to make a go/no-go decision on accepting the role into the shared library.

Produce `validation_report.md` documenting the full validation with all steps, findings, and a final recommendation.

The following files are provided as inputs:

**File: roles/nginx-proxy/tasks/main.yml**
```yaml
---
- name: Install nginx
  ansible.builtin.apt:
    name: nginx
    state: present
    update_cache: true

- name: Create proxy configuration directory
  ansible.builtin.file:
    path: /etc/nginx/conf.d
    state: directory
    owner: root
    group: root
    mode: "0755"

- name: Deploy proxy configuration
  ansible.builtin.template:
    src: proxy.conf.j2
    dest: /etc/nginx/conf.d/proxy.conf
    owner: root
    group: root
    mode: "0644"
  notify: Reload nginx

- name: Enable and start nginx
  ansible.builtin.service:
    name: nginx
    state: started
    enabled: true
```

**File: roles/nginx-proxy/handlers/main.yml**, **defaults/main.yml**, **meta/main.yml**, and **molecule/** configuration files are also provided.

## Expected Behavior

1. Run `ansible-lint` BEFORE running molecule tests — never substitute molecule for ansible-lint
2. Not skip `ansible-lint` even if molecule is also present
3. Explicitly note that a `molecule/` directory was detected in the role
4. Run `bash scripts/test_role.sh` (or equivalent wrapper) for molecule testing
5. If Docker is unavailable, document the specific blocker and continue other validation steps rather than failing the whole validation
6. Run `yamllint` on the role files
7. Report `ansible-lint` output or finding count
8. Run `bash scripts/validate_role_security.sh` for security scanning
9. Run `bash scripts/check_fqcn.sh` to check for non-FQCN usage
10. Acknowledge that the role uses FQCN module names (`ansible.builtin.*`) which is correct
11. Conclude with a clear go/no-go recommendation for accepting the role

## Success Criteria

- **ansible-lint run before molecule**: The validation_report.md shows ansible-lint was run before molecule tests
- **molecule not a substitute for ansible-lint**: The report does NOT skip ansible-lint in favor of only molecule testing
- **molecule/ detected**: The report explicitly notes that a molecule/ directory was detected in the role
- **test_role.sh used**: The report documents running `bash scripts/test_role.sh` (or the equivalent wrapper) for molecule testing
- **Environment blocker handling**: If molecule could not run (Docker not available), the report documents the specific blocker and continues with other steps rather than failing the overall validation
- **yamllint run**: The report documents running yamllint
- **ansible-lint findings reported**: The report includes ansible-lint output or a finding count (even if zero)
- **Security scan run**: The report documents running `bash scripts/validate_role_security.sh` (not the playbook security script)
- **FQCN check run**: The report documents running `bash scripts/check_fqcn.sh` to check for non-FQCN usage
- **Role FQCN usage acknowledged**: The report acknowledges that the role uses FQCN module names (ansible.builtin.*) which is correct
- **Final recommendation**: The report ends with a clear go/no-go recommendation for accepting the role

## Failure Conditions

- Runs molecule tests before `ansible-lint`
- Skips `ansible-lint` because molecule tests are present
- Does not note detection of the `molecule/` directory
- Does not use or reference `bash scripts/test_role.sh`
- Fails the overall validation if Docker is unavailable instead of continuing
- Does not run `yamllint`
- Does not report `ansible-lint` output or a finding count
- Does not run `bash scripts/validate_role_security.sh`
- Does not run `bash scripts/check_fqcn.sh`
- Does not acknowledge that the role's FQCN usage is correct
- Report has no final go/no-go recommendation
