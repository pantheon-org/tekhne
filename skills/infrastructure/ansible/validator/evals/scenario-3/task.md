# Security Audit an Ansible Playbook for a Production Database Server

## Problem/Feature Description

A security team is conducting a routine pre-production audit of an Ansible playbook that configures a MySQL database server. The audit needs to identify any security issues before the playbook is applied to production systems. A previous audit at the company caught hardcoded database passwords in a different playbook, so the team is especially vigilant about credential handling.

Run a complete security-focused validation pass on the playbook and produce a security audit report. The report should be detailed enough for a security engineer who is not familiar with Ansible to understand each issue and the remediation steps.

## Output Specification

Produce `security_audit_report.md` documenting:
- Each security finding with its severity
- What is wrong and why it is a security risk
- The specific remediation for each finding
- An overall risk assessment (high/medium/low risk to proceed)

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: playbooks/configure_mysql.yml ===============
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
