# Scenario 05: Migrate a Legacy Playbook to Use Fully-Qualified Collection Names

## User Prompt

An infrastructure team is upgrading their Ansible automation from Ansible 2.9 to Ansible 2.14. As part of this upgrade, they need to migrate their playbooks from using short module names to Fully Qualified Collection Names (FQCN), which became the recommended standard starting in Ansible 2.10.

The team has a playbook that uses several short module names. They need an audit of which modules need to be updated, what the correct FQCN replacements are, and a migrated version of the playbook.

Run validation on the playbook, identify all non-FQCN module usage, look up the correct replacements, and produce the migrated playbook.

Produce:
- `fqcn_audit_report.md`: A list of all non-FQCN modules found, their correct FQCN replacements, and any other lint findings
- `migrated_playbook.yml`: The updated playbook with all modules replaced with FQCNs

The following file is provided as input:

**File: playbooks/configure_monitoring.yml**
```yaml
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
        url: "..."
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
      notify: Restart prometheus

    - name: Deploy systemd unit
      copy:
        src: files/prometheus.service
        dest: /etc/systemd/system/prometheus.service
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
```

## Expected Behavior

1. Run `bash scripts/check_fqcn.sh` to detect non-FQCN module usage
2. Reference `module_alternatives.md` for finding FQCN replacements
3. Identify all short-name modules: user, file, get_url, unarchive, template, copy, service, package, systemd (at least 7 of these 9)
4. List the correct `ansible.builtin.*` FQCN for each identified module
5. Create `migrated_playbook.yml` with every module using FQCN
6. Migrate the handlers section too, replacing `service` and `systemd` with their FQCN equivalents
7. Run `ansible-lint` as part of the validation
8. Run `yamllint` as part of the validation
9. Not skip `ansible-lint` despite the playbook appearing syntactically correct

## Success Criteria

- **check_fqcn.sh script used**: The fqcn_audit_report.md documents running `bash scripts/check_fqcn.sh` to detect non-FQCN usage
- **module_alternatives.md referenced**: The report references module_alternatives.md for finding FQCN replacements
- **All non-FQCN modules identified**: The report identifies all short-name modules: user, file, get_url, unarchive, template, copy, service, package, systemd (at least 7 of these 9)
- **FQCN replacements provided**: The report lists the correct ansible.builtin.* FQCN for each identified module
- **migrated_playbook.yml created**: A migrated_playbook.yml file exists
- **All modules in migrated playbook use FQCN**: Every module in migrated_playbook.yml uses the full ansible.builtin.* prefix
- **ansible-lint run**: The report documents running ansible-lint as part of the validation
- **yamllint run**: The report documents running yamllint
- **ansible-lint not skipped**: The report does NOT skip ansible-lint despite the playbook appearing syntactically correct
- **Handlers also migrated**: The handlers section in migrated_playbook.yml also uses FQCN for service and systemd modules

## Failure Conditions

- Does not run `bash scripts/check_fqcn.sh`
- Does not reference `module_alternatives.md`
- Identifies fewer than 7 of the 9 non-FQCN modules
- Does not list the correct FQCN replacement for each identified module
- Does not produce `migrated_playbook.yml`
- `migrated_playbook.yml` still contains short module names
- Skips `ansible-lint` because the playbook looks syntactically correct
- Does not run `yamllint`
- Handlers section in `migrated_playbook.yml` still uses short names for `service` or `systemd`
