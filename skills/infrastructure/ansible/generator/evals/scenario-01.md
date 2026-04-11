# Scenario 01: Web Server Provisioning Playbook

## User Prompt

A DevOps team at a mid-sized e-commerce company needs to automate the provisioning of their Nginx web server fleet. The servers run Ubuntu 22.04. Currently, engineers SSH into boxes manually and run commands from a wiki page — a process that is error-prone and takes 30 minutes per host.

The team wants a single Ansible playbook that can be run against a group called `webservers` to bring a fresh Ubuntu host into a fully configured state. The playbook should: install Nginx, deploy a configuration file from a template (the Nginx config listens on port 80 and proxies to an upstream at `{{ nginx_upstream_host }}:{{ nginx_upstream_port }}`), ensure the Nginx service is running and enabled at boot, and verify the service responds on port 80 after configuration.

The playbook will be committed to version control and run by multiple team members — correctness and repeatability are more important than raw speed.

Produce a complete Ansible playbook file named `provision_webservers.yml` and a Jinja2 template file at `templates/nginx.conf.j2`. The playbook should follow production quality standards for Ansible.

## Expected Behavior

1. Use FQCN module names (e.g., `ansible.builtin.apt`, `ansible.builtin.template`, `ansible.builtin.service`) for all module calls
2. Assign a `name:` field to every task, using verb-first naming (e.g., "Install", "Deploy", "Ensure")
3. Use `true`/`false` for all boolean values instead of `yes`/`no`
4. Quote all octal file permission strings (e.g., `'0644'`)
5. Use a `notify:` handler for Nginx restarts/reloads triggered by config deployment
6. Include explicit `state:` parameters on package installation and service tasks
7. Start the playbook with a header comment block documenting title, description, variables, and usage
8. Avoid `ansible.builtin.shell` or `ansible.builtin.command` for package installation

## Success Criteria

- **FQCN modules used**: Every module call uses its FQCN (e.g., `ansible.builtin.apt`, `ansible.builtin.template`, `ansible.builtin.service`) — no short names like `apt:` or `template:` without the namespace
- **All tasks have name field**: Every task block in the playbook and handlers has a `name:` field — no anonymous tasks
- **Verb-first task names**: Task names begin with an action verb (e.g., 'Install', 'Deploy', 'Ensure', 'Copy', 'Start')
- **Boolean true/false syntax**: Boolean values use `true` or `false` (not `yes` or `no`)
- **Quoted octal file permissions**: Any `mode:` parameter uses a quoted octal string (e.g., `'0644'`, `'0755'`) rather than an unquoted integer
- **Service restart via handler**: The Nginx configuration deployment task uses `notify:` to trigger a handler that restarts/reloads Nginx, rather than a direct `ansible.builtin.service: state: restarted` task
- **State parameter present**: The package installation task and service task both include an explicit `state:` parameter
- **Playbook header comment**: The playbook file starts with a comment block documenting title, description, requirements or variables, and usage command
- **No shell/command for package install**: Nginx installation does NOT use `ansible.builtin.shell` or `ansible.builtin.command` — uses `ansible.builtin.apt` or `ansible.builtin.package`

## Failure Conditions

- Uses short module names (e.g., `apt:`, `template:`) instead of FQCN
- Any task block is missing a `name:` field
- Task names do not begin with an action verb
- Boolean values use `yes` or `no` instead of `true` or `false`
- File permission mode values are unquoted integers
- Nginx configuration change triggers a direct restart task instead of notifying a handler
- Package installation or service task omits the `state:` parameter
- Playbook file has no header comment documenting its purpose and usage
- Nginx installation uses `ansible.builtin.shell` or `ansible.builtin.command`
