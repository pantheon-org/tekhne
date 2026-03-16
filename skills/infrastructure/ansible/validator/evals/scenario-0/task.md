# Validate an Ansible Playbook Before Merging to Main

## Problem/Feature Description

A DevOps team is adopting Ansible for infrastructure configuration management. A junior engineer has written a playbook to configure a web server fleet and has submitted it for review before merging to main. The team lead wants a thorough validation pass performed on the playbook and a written report of all findings.

The playbook installs nginx, copies a configuration file, and starts the service. It also downloads a binary from the internet and runs a post-install check command. The team's standard is that every playbook passes all validation gates before merge.

Your job is to perform the full validation workflow and produce a report documenting every step, every tool run, and every finding.

## Output Specification

Produce `validation_report.md` documenting:
- Every validation step performed in order
- The command run for each step
- Findings from each step (or confirmation it passed)
- A final pass/fail summary

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: playbooks/configure_webserver.yml ===============
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

=============== FILE: inventory/hosts.ini ===============
[webservers]
web01.example.com
web02.example.com
web03.example.com
