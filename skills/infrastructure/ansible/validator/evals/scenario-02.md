# Scenario 02: Fix Idempotency and Dry-Run Issues in a Playbook

## User Prompt

A reliability engineering team has a playbook that configures application servers. The team recently adopted a policy requiring all playbooks to pass a dry-run check before any production deployment. During code review, a senior engineer flagged that the dry-run results were not giving enough visibility into what would actually change, and that some tasks were always reporting as "changed" even during repeated runs on already-configured hosts.

You have been asked to:
1. Audit the playbook for idempotency issues and document the findings
2. Produce a fixed version of the playbook that resolves the identified issues
3. Document the dry-run command that should be used going forward

Produce:
- `audit_report.md`: Findings from the ansible-lint pass, with explanations of each issue
- `fixed_playbook.yml`: A corrected version of the playbook
- The audit_report.md should also include the exact dry-run command the team should use

The following files are provided as inputs:

**File: playbooks/configure_app.yml**
```yaml
---
- name: Configure application servers
  hosts: appservers
  become: yes

  tasks:
    - name: Create application directory
      file:
        path: /opt/myapp
        state: directory
        owner: appuser
        group: appuser

    - name: Check if config exists
      command: test -f /opt/myapp/config.yml

    - name: Get current app version
      command: /opt/myapp/bin/app --version
      register: current_version

    - name: Check disk space
      shell: df -h /opt/myapp | tail -1 | awk '{print $5}'
      register: disk_usage

    - name: Apply database migrations
      command: /opt/myapp/bin/app migrate
      when: current_version.rc == 0

    - name: Reload app configuration
      command: /opt/myapp/bin/app reload-config
      register: reload_result

    - name: Verify app is running
      command: pgrep -x app
```

**File: inventory/appservers.ini**
```ini
[appservers]
app01.internal
app02.internal
```

## Expected Behavior

1. Identify at least 4 of the 6 command/shell tasks as having no-changed-when issues
2. Explain why missing `changed_when` breaks idempotency (always reports changed, breaks handler triggering or change auditing)
3. In `fixed_playbook.yml`, assign `changed_when: false` to read-only tasks (check if config exists, get version, check disk, verify running, pgrep)
4. In `fixed_playbook.yml`, assign an explicit `changed_when` condition (not just `false`) to state-changing tasks (migrate, reload-config)
5. Document the dry-run command as `ansible-playbook --check --diff ...` — both flags required
6. Not show `--check` without `--diff`
7. Note that the file task creating `/opt/myapp` is missing a `mode:` attribute
8. Add a `mode:` attribute to the file task in `fixed_playbook.yml`
9. Document running `ansible-lint` as part of the audit process
10. Document running `yamllint`

## Success Criteria

- **All no-changed-when tasks identified**: The audit_report.md identifies at least 4 of the 6 command/shell tasks as having no-changed-when issues
- **Why it matters explained**: The report explains why missing changed_when breaks idempotency (always reports changed, breaks handler triggering or change auditing)
- **Read-only tasks get changed_when: false**: In fixed_playbook.yml, tasks that only read state (check if config exists, get version, check disk, verify running, pgrep) have `changed_when: false`
- **State-changing tasks get explicit changed_when**: In fixed_playbook.yml, tasks that may actually change state (migrate, reload-config) have an explicit `changed_when` condition rather than just `changed_when: false`
- **Dry-run uses --check --diff**: The audit_report.md shows the dry-run command as `ansible-playbook --check --diff ...` (both flags present)
- **No --check without --diff**: The report does NOT show `--check` used without `--diff` (the anti-pattern is avoided)
- **missing mode on file task noted**: The audit identifies that the file task creating /opt/myapp is missing a `mode:` attribute
- **mode added in fixed playbook**: The fixed_playbook.yml adds a `mode:` attribute to the file task
- **ansible-lint run documented**: The audit_report.md documents running ansible-lint as part of the audit process
- **yamllint run documented**: The audit_report.md documents running yamllint

## Failure Conditions

- Fewer than 4 command/shell tasks identified as missing `changed_when`
- Does not explain why missing `changed_when` causes idempotency problems
- Read-only tasks in fixed playbook are not given `changed_when: false`
- State-changing tasks in fixed playbook are given `changed_when: false` instead of a meaningful condition
- Dry-run command shows only `--check` without `--diff`
- Dry-run command shows `--check --diff` but the report also uses `--check` alone elsewhere
- Missing `mode:` on the file task is not identified
- Fixed playbook does not add `mode:` to the file task
- `ansible-lint` is not documented in the audit process
- `yamllint` is not documented in the audit process
