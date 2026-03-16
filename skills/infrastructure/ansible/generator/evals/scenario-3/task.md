# Application Deployment with Rollback Capability

## Problem/Feature Description

A fintech company deploys their Java billing service to a group called `app_servers`. Deployments occasionally fail mid-way (e.g., the new JAR file is copied but the service fails to start), leaving servers in a broken state. The on-call team then spends 30 minutes manually rolling back.

The deployment process is:
1. Stop the application service
2. Back up the current JAR at `/opt/billing/billing.jar` to `/opt/billing/billing.jar.bak`
3. Copy the new JAR from the control node to `/opt/billing/billing.jar`
4. Start the service and wait for it to listen on port 8080

If any step from (3) onwards fails, the playbook should automatically restore the backup JAR and restart the service. Regardless of success or failure, the deployment status (success or failure with the error message) should be written to `/var/log/ansible-deploy.log`.

## Output Specification

Produce a complete Ansible playbook file named `deploy_billing.yml` that implements the deployment with automatic rollback.

## Input Files

The following existing playbook stub is provided as a starting point. Extract it before beginning.

=============== FILE: deploy_billing_stub.yml ===============
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
