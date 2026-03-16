# Web Server Provisioning Playbook

## Problem/Feature Description

A DevOps team at a mid-sized e-commerce company needs to automate the provisioning of their Nginx web server fleet. The servers run Ubuntu 22.04. Currently, engineers SSH into boxes manually and run commands from a wiki page — a process that is error-prone and takes 30 minutes per host.

The team wants a single Ansible playbook that can be run against a group called `webservers` to bring a fresh Ubuntu host into a fully configured state. The playbook should: install Nginx, deploy a configuration file from a template (the Nginx config listens on port 80 and proxies to an upstream at `{{ nginx_upstream_host }}:{{ nginx_upstream_port }}`), ensure the Nginx service is running and enabled at boot, and verify the service responds on port 80 after configuration.

The playbook will be committed to version control and run by multiple team members — correctness and repeatability are more important than raw speed.

## Output Specification

Produce a complete Ansible playbook file named `provision_webservers.yml` and a Jinja2 template file at `templates/nginx.conf.j2`. The playbook should follow production quality standards for Ansible.
