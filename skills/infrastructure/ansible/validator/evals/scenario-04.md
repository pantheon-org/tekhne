# Scenario 04: Security Audit an Ansible Playbook for a Production Database Server

## User Prompt

A security team is conducting a routine pre-production audit of an Ansible playbook that configures a MySQL database server. The audit needs to identify any security issues before the playbook is applied to production systems. A previous audit at the company caught hardcoded database passwords in a different playbook, so the team is especially vigilant about credential handling.

Run a complete security-focused validation pass on the playbook and produce a security audit report. The report should be detailed enough for a security engineer who is not familiar with Ansible to understand each issue and the remediation steps.

Produce `security_audit_report.md` documenting:
- Each security finding with its severity
- What is wrong and why it is a security risk
- The specific remediation for each finding
- An overall risk assessment (high/medium/low risk to proceed)

The following file is provided as input:

**File: playbooks/configure_mysql.yml**
```yaml
---
- name: Configure MySQL database server
  hosts: dbservers
  become: yes

  vars:
    mysql_root_password: "Sup3rS3cr3t!"
    mysql_app_password: "AppPass2024"
    mysql_bind_address: "0.0.0.0"
    backup_bucket: s3://company-backups

  tasks:
    - name: Install MySQL
      apt:
        name: mysql-server
        state: present
        update_cache: yes

    - name: Configure MySQL
      template:
        src: templates/my.cnf.j2
        dest: /etc/mysql/my.cnf
        owner: root
        group: root

    - name: Set root password
      mysql_user:
        name: root
        password: "{{ mysql_root_password }}"
        login_unix_socket: /var/run/mysqld/mysqld.sock
        state: present

    - name: Create application user
      mysql_user:
        name: appuser
        password: "{{ mysql_app_password }}"
        priv: "appdb.*:ALL"
        state: present

    - name: Configure backup script
      copy:
        content: |
          #!/bin/bash
          mysqldump -u root -p{{ mysql_root_password }} appdb | aws s3 cp - {{ backup_bucket }}/latest.sql
        dest: /opt/backup.sh

    - name: Download SSL certificate bundle
      get_url:
        url: https://internal-ca.example.com/ca-bundle.crt
        dest: /etc/mysql/ca-bundle.crt
        validate_certs: false

    - name: Set up MySQL SSL
      mysql_variables:
        variable: ssl_ca
        value: /etc/mysql/ca-bundle.crt
```

## Expected Behavior

1. Run `bash scripts/validate_playbook_security.sh` for Checkov scanning
2. Run `bash scripts/scan_secrets.sh` to detect hardcoded credentials
3. Identify the hardcoded `mysql_root_password` and `mysql_app_password` variables as security issues
4. Identify that the backup script embeds a password directly in a shell script as a security issue
5. Identify `validate_certs: false` in the get_url task as a security issue
6. Suggest Ansible Vault or environment variables as the correct remediation for hardcoded secrets
7. Reference `security_checklist.md` in connection with findings
8. Label each security finding with a severity level (HIGH, MEDIUM, or LOW)
9. Include an overall risk assessment (high/medium/low) at the end of the report
10. Note that the template task for my.cnf is missing a `mode:` attribute
11. Flag `mysql_bind_address: 0.0.0.0` (binding to all interfaces) as a security concern

## Success Criteria

- **Security scan wrapper used**: The report documents running `bash scripts/validate_playbook_security.sh` for Checkov scanning
- **Secrets scan run**: The report documents running `bash scripts/scan_secrets.sh` to detect hardcoded credentials
- **Hardcoded passwords found**: The report identifies the hardcoded mysql_root_password and mysql_app_password variables as security issues
- **Password in backup script found**: The report identifies that the backup script embeds a password directly in a shell script as a security issue
- **validate_certs: false found**: The report identifies the `validate_certs: false` in the get_url task as a security issue
- **Ansible Vault remediation suggested**: The report suggests Ansible Vault or environment variables as the correct remediation for hardcoded secrets
- **security_checklist.md referenced**: The report references the security_checklist.md reference file in connection with findings
- **Severity labels on findings**: Each security finding has a severity label (HIGH, MEDIUM, or LOW)
- **Overall risk assessment**: The report includes an overall risk assessment (high/medium/low) at the end
- **missing mode on template**: The report notes that the template task for my.cnf is missing a `mode:` attribute
- **mysql_bind_address: 0.0.0.0 flagged**: The report flags the mysql_bind_address set to 0.0.0.0 (binding to all interfaces) as a security concern

## Failure Conditions

- Does not run `bash scripts/validate_playbook_security.sh`
- Does not run `bash scripts/scan_secrets.sh`
- Fails to identify hardcoded mysql_root_password and mysql_app_password as security issues
- Fails to identify the password embedded in the backup script
- Fails to identify `validate_certs: false` as a security issue
- Does not suggest Ansible Vault or environment variables as remediation for hardcoded secrets
- Does not reference `security_checklist.md`
- Findings are missing severity labels
- Report has no overall risk assessment
- Fails to note the missing `mode:` on the template task
- Fails to flag `mysql_bind_address: 0.0.0.0` as a security concern
