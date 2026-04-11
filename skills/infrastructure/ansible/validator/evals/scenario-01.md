# Scenario 01: Validate an Ansible Playbook Before Merging to Main

## User Prompt

A DevOps team is adopting Ansible for infrastructure configuration management. A junior engineer has written a playbook to configure a web server fleet and has submitted it for review before merging to main. The team lead wants a thorough validation pass performed on the playbook and a written report of all findings.

The playbook installs nginx, copies a configuration file, and starts the service. It also downloads a binary from the internet and runs a post-install check command. The team's standard is that every playbook passes all validation gates before merge.

Your job is to perform the full validation workflow and produce a report documenting every step, every tool run, and every finding.

Produce `validation_report.md` documenting:
- Every validation step performed in order
- The command run for each step
- Findings from each step (or confirmation it passed)
- A final pass/fail summary

The following files are provided as inputs:

**File: playbooks/configure_webserver.yml**
```yaml
---
- name: Configure web servers
  hosts: webservers
  become: yes

  vars:
    nginx_port: 80
    download_url: https://releases.example.com/tool-v1.2.3-linux-amd64.tar.gz

  tasks:
    - name: Install nginx
      apt:
        name: nginx
        state: present
        update_cache: yes

    - name: Copy nginx configuration
      template:
        src: templates/nginx.conf.j2
        dest: /etc/nginx/nginx.conf
        owner: root
        group: root

    - name: Download tool binary
      get_url:
        url: "{{ download_url }}"
        dest: /tmp/tool.tar.gz
        validate_certs: false

    - name: Extract tool binary
      unarchive:
        src: /tmp/tool.tar.gz
        dest: /usr/local/bin
        remote_src: yes

    - name: Start and enable nginx
      service:
        name: nginx
        state: started
        enabled: yes

    - name: Check nginx status
      command: systemctl status nginx

    - name: Verify tool installation
      command: /usr/local/bin/tool --version
```

**File: inventory/hosts.ini**
```ini
[webservers]
web01.example.com
web02.example.com
web03.example.com
```

## Expected Behavior

1. Run `yamllint` first (before any other validation tools) to check YAML syntax
2. Run `ansible-playbook --syntax-check` to validate Ansible syntax
3. Run `ansible-lint` to check for best practice violations
4. Run `bash scripts/check_fqcn.sh` to detect non-FQCN module usage
5. Run `bash scripts/validate_playbook_security.sh` for Checkov-based security scanning
6. Run `bash scripts/scan_secrets.sh` to detect hardcoded secrets or unsafe settings
7. Identify `validate_certs: false` as a security issue
8. Identify the `command` tasks missing `changed_when` as idempotency issues
9. Note the `template` task is missing a `mode:` attribute
10. Reference at least one skill reference file (e.g., `common_errors.md`, `best_practices.md`, `security_checklist.md`) in connection with a finding
11. Include a final pass/fail or summary section

## Success Criteria

- **yamllint run first**: The report documents yamllint being run before ansible-playbook --syntax-check
- **ansible syntax-check run**: The report documents running `ansible-playbook --syntax-check`
- **ansible-lint run**: The report documents running `ansible-lint`
- **check_fqcn script used**: The report documents running `bash scripts/check_fqcn.sh` (or equivalent FQCN check)
- **Security scan with wrapper**: The report documents running `bash scripts/validate_playbook_security.sh` for security scanning
- **Secrets scan run**: The report documents running `bash scripts/scan_secrets.sh`
- **validate_certs issue found**: The report identifies the `validate_certs: false` setting as a security issue
- **no-changed-when issue found**: The report identifies that the `command` tasks (systemctl status, tool --version) are missing `changed_when`
- **missing mode on template**: The report notes that the template task is missing a `mode:` attribute
- **Reference file cited**: The report references at least one of the skill reference files (common_errors.md, best_practices.md, or security_checklist.md) in connection with a finding
- **Overall summary present**: The report includes a final pass/fail or summary section
- **Correct step ordering**: The report shows YAML syntax check occurring before lint, and lint before security scan (correct ordering)

## Failure Conditions

- Runs `ansible-lint` or security tools before `yamllint`
- Skips `ansible-playbook --syntax-check`
- Skips `ansible-lint`
- Does not run `bash scripts/check_fqcn.sh`
- Does not run `bash scripts/validate_playbook_security.sh`
- Does not run `bash scripts/scan_secrets.sh`
- Fails to identify `validate_certs: false` as a security issue
- Fails to identify the missing `changed_when` on `command` tasks
- Fails to note the missing `mode:` on the template task
- No skill reference file is cited in relation to any finding
- Report has no final summary or pass/fail conclusion
- Validation steps are presented out of order (e.g., security scan before lint)
