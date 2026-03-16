# Validate an Ansible Role with Molecule Tests

## Problem/Feature Description

A platform team has developed an Ansible role called `nginx-proxy` that sets up nginx as a reverse proxy. The role includes a Molecule test suite for local testing. A new team member is preparing to submit this role to the team's shared role library, and needs a full quality validation pass before submitting.

The role's molecule test environment uses Docker. Perform a complete validation of the role, documenting what you ran and in what order. If any environment blockers prevent molecule tests from running, document the blocker clearly but continue with all other validation steps.

Your report should give the reviewer enough information to make a go/no-go decision on accepting the role into the shared library.

## Output Specification

Produce `validation_report.md` documenting the full validation with all steps, findings, and a final recommendation.

## Input Files

The following files are provided as inputs. Extract them before beginning.

=============== FILE: roles/nginx-proxy/tasks/main.yml ===============
---
- name: Install nginx
  ansible.builtin.apt:
    name: nginx
    state: present
    update_cache: true

- name: Create proxy configuration directory
  ansible.builtin.file:
    path: /etc/nginx/conf.d
    state: directory
    owner: root
    group: root
    mode: "0755"

- name: Deploy proxy configuration
  ansible.builtin.template:
    src: proxy.conf.j2
    dest: /etc/nginx/conf.d/proxy.conf
    owner: root
    group: root
    mode: "0644"
  notify: Reload nginx

- name: Enable and start nginx
  ansible.builtin.service:
    name: nginx
    state: started
    enabled: true

=============== FILE: roles/nginx-proxy/handlers/main.yml ===============
---
- name: Reload nginx
  ansible.builtin.service:
    name: nginx
    state: reloaded

=============== FILE: roles/nginx-proxy/defaults/main.yml ===============
---
upstream_host: "127.0.0.1"
upstream_port: 8080
listen_port: 80
server_name: "_"

=============== FILE: roles/nginx-proxy/meta/main.yml ===============
---
galaxy_info:
  role_name: nginx_proxy
  author: platform-team
  description: Sets up nginx as a reverse proxy
  min_ansible_version: "2.12"
  platforms:
    - name: Ubuntu
      versions:
        - jammy

=============== FILE: roles/nginx-proxy/molecule/default/molecule.yml ===============
---
dependency:
  name: galaxy
driver:
  name: docker
platforms:
  - name: instance
    image: ubuntu:22.04
    pre_build_image: true
provisioner:
  name: ansible
verifier:
  name: ansible

=============== FILE: roles/nginx-proxy/molecule/default/converge.yml ===============
---
- name: Converge
  hosts: all
  roles:
    - role: nginx-proxy
