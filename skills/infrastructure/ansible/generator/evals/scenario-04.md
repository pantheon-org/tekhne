# Scenario 04: Application Deployment with Rollback Capability

## User Prompt

A fintech company deploys their Java billing service to a group called `app_servers`. Deployments occasionally fail mid-way (e.g., the new JAR file is copied but the service fails to start), leaving servers in a broken state. The on-call team then spends 30 minutes manually rolling back.

The deployment process is:
1. Stop the application service
2. Back up the current JAR at `/opt/billing/billing.jar` to `/opt/billing/billing.jar.bak`
3. Copy the new JAR from the control node to `/opt/billing/billing.jar`
4. Start the service and wait for it to listen on port 8080

If any step from (3) onwards fails, the playbook should automatically restore the backup JAR and restart the service. Regardless of success or failure, the deployment status (success or failure with the error message) should be written to `/var/log/ansible-deploy.log`.

Produce a complete Ansible playbook file named `deploy_billing.yml` that implements the deployment with automatic rollback.

A starting-point stub is provided:

```yaml
---
- name: Deploy billing service
  hosts: app_servers
  become: yes

  vars:
    billing_jar_src: "files/billing.jar"
    billing_jar_dest: "/opt/billing/billing.jar"
    billing_service_port: 8080

  tasks:
    - name: stop billing service
      ansible.builtin.service:
        name: billing
        state: stopped
      ignore_errors: yes

    - name: backup current jar
      command: cp /opt/billing/billing.jar /opt/billing/billing.jar.bak

    - name: copy new jar
      copy:
        src: "{{ billing_jar_src }}"
        dest: "{{ billing_jar_dest }}"
      ignore_errors: yes

    - name: start service
      service:
        name: billing
        state: started
      ignore_errors: yes
```

## Expected Behavior

1. Remove all `ignore_errors` from the playbook and replace with `block`/`rescue`/`always` error handling
2. Wrap deployment steps (copy, start) in a `block:` section so failures are caught
3. Use a `rescue:` block to restore the backup JAR and restart the service on failure
4. Use an `always:` block to write deployment status to the log file regardless of success or failure
5. Replace `command: cp ...` with `ansible.builtin.copy` using `remote_src: true` for file backup operations
6. Use FQCN for all module references
7. Assign a `name:` field to every task
8. Use `true`/`false` for all boolean values

## Success Criteria

- **No ignore_errors present**: The output playbook contains NO instances of `ignore_errors: true` or `ignore_errors: yes`
- **block/rescue structure used**: The deployment steps are wrapped in a `block:` / `rescue:` structure that handles failures and performs rollback
- **always block for logging**: An `always:` section writes deployment status to the log file regardless of success or failure
- **No command for file copy**: Copying or backing up files does NOT use `ansible.builtin.command: cp ...` — uses `ansible.builtin.copy` with `remote_src: true` or `ansible.builtin.file` instead
- **FQCN modules used**: All module references use FQCN (no short names like `copy:`, `service:`, `command:`)
- **All tasks have name field**: Every task in the playbook has a `name:` field
- **Boolean true/false syntax**: All boolean values use `true` or `false`, not `yes` or `no`
- **Rollback restores backup**: The `rescue:` block contains a task that restores the backup JAR and starts the service

## Failure Conditions

- Any `ignore_errors: true` or `ignore_errors: yes` remains in the playbook
- Deployment steps are not wrapped in a `block:` / `rescue:` structure
- No `always:` section to write deployment status to the log
- File backup uses `ansible.builtin.command: cp ...` instead of `ansible.builtin.copy` with `remote_src: true`
- Module references use short names instead of FQCN
- Any task is missing a `name:` field
- Boolean values use `yes` or `no` instead of `true` or `false`
- The `rescue:` block does not restore the backup JAR or restart the service
