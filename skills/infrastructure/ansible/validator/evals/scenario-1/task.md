# Fix Idempotency and Dry-Run Issues in a Playbook

## Problem/Feature Description

A reliability engineering team has a playbook that configures application servers. The team recently adopted a policy requiring all playbooks to pass a dry-run check before any production deployment. During code review, a senior engineer flagged that the dry-run results were not giving enough visibility into what would actually change, and that some tasks were always reporting as "changed" even during repeated runs on already-configured hosts.

You have been asked to:
1. Audit the playbook for idempotency issues and document the findings
2. Produce a fixed version of the playbook that resolves the identified issues
3. Document the dry-run command that should be used going forward

## Output Specification

Produce:
- `audit_report.md`: Findings from the ansible-lint pass, with explanations of each issue
- `fixed_playbook.yml`: A corrected version of the playbook
- The audit_report.md should also include the exact dry-run command the team should use

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: playbooks/configure_app.yml ===============
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

=============== FILE: inventory/appservers.ini ===============
[appservers]
app01.internal
app02.internal
