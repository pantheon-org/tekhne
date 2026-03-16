# Migrate a Legacy Playbook to Use Fully-Qualified Collection Names

## Problem/Feature Description

An infrastructure team is upgrading their Ansible automation from Ansible 2.9 to Ansible 2.14. As part of this upgrade, they need to migrate their playbooks from using short module names to Fully Qualified Collection Names (FQCN), which became the recommended standard starting in Ansible 2.10.

The team has a playbook that uses several short module names. They need an audit of which modules need to be updated, what the correct FQCN replacements are, and a migrated version of the playbook.

Run validation on the playbook, identify all non-FQCN module usage, look up the correct replacements, and produce the migrated playbook.

## Output Specification

Produce:
- `fqcn_audit_report.md`: A list of all non-FQCN modules found, their correct FQCN replacements, and any other lint findings
- `migrated_playbook.yml`: The updated playbook with all modules replaced with FQCNs

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: playbooks/configure_monitoring.yml ===============
---
- name: Configure monitoring agents
  hosts: all
  become: yes

  tasks:
    - name: Create monitoring user
      user:
        name: prometheus
        system: yes
        shell: /bin/false
        create_home: no

    - name: Create directories
      file:
        path: "{{ item }}"
        state: directory
        owner: prometheus
        group: prometheus
        mode: "0755"
      loop:
        - /opt/prometheus
        - /etc/prometheus
        - /var/lib/prometheus

    - name: Download prometheus archive
      get_url:
        url: "https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz"
        dest: /tmp/prometheus.tar.gz
        checksum: "sha256:1234567890abcdef"

    - name: Extract prometheus
      unarchive:
        src: /tmp/prometheus.tar.gz
        dest: /opt/prometheus
        remote_src: yes
        extra_opts: ["--strip-components=1"]

    - name: Deploy prometheus configuration
      template:
        src: prometheus.yml.j2
        dest: /etc/prometheus/prometheus.yml
        owner: prometheus
        group: prometheus
        mode: "0644"
      notify: Restart prometheus

    - name: Deploy systemd unit
      copy:
        src: files/prometheus.service
        dest: /etc/systemd/system/prometheus.service
        owner: root
        group: root
        mode: "0644"
      notify:
        - Reload systemd
        - Restart prometheus

    - name: Enable and start prometheus
      service:
        name: prometheus
        state: started
        enabled: yes

    - name: Install node_exporter
      package:
        name: prometheus-node-exporter
        state: present

  handlers:
    - name: Reload systemd
      systemd:
        daemon_reload: yes

    - name: Restart prometheus
      service:
        name: prometheus
        state: restarted
