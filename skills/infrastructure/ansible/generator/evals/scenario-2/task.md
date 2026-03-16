# Redis Role for Multi-OS Fleet

## Problem/Feature Description

A platform team manages servers running both Ubuntu 22.04 and RHEL 9. They need a reusable Ansible role called `redis` that installs and configures Redis on both operating systems. The role should be robust enough to be shared across multiple teams and several projects.

The role should install Redis, deploy a `redis.conf` from a Jinja2 template (configuring bind address, port, and max memory), ensure the service is running and enabled, and handle the fact that RHEL 9 uses `dnf` while Ubuntu uses `apt`. The Redis port defaults to 6379 and max memory defaults to `256mb`, but both should be overridable.

The team wants the role to be well-structured so new team members can easily understand and customise it without reading the entire codebase.

## Output Specification

Produce a complete Ansible role directory structure for the `redis` role. Include at minimum:
- `tasks/main.yml`
- `handlers/main.yml`
- `defaults/main.yml`
- `vars/main.yml`
- `vars/Debian.yml`
- `vars/RedHat.yml`
- `templates/redis.conf.j2`
- `meta/main.yml`
- `README.md`

Also produce a short example playbook `example_playbook.yml` that demonstrates how to use the role.
