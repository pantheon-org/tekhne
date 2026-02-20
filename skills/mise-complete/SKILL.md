---
name: mise-complete
description: |-
  Complete Mise tool management guidance covering version management, task automation, environment variables, and configuration. Use when: installing tools with mise, defining tasks, managing environment variables, or configuring mise.toml for development environments.
  
  Keywords: mise, rtx, asdf, tool versions, mise.toml, .tool-versions, tasks, environment variables, mise use, mise install, mise tasks, mise env, mise set
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
---

# Mise Complete Tool Management

Comprehensive Mise guidance covering tool version management, task automation, environment variables, and hierarchical configuration.

## When to Apply

Use this skill when:
- Installing and managing development tool versions
- Defining project tasks and workflows
- Managing environment variables per project
- Configuring mise.toml for teams
- Migrating from asdf, nvm, rbenv, pyenv
- Setting up development environments

## Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Tool Version Management | CRITICAL | `tools-` |
| 2 | Task Automation | HIGH | `tasks-` |
| 3 | Environment Variables | HIGH | `env-` |
| 4 | Configuration | MEDIUM | `config-` |

## How to Use

Read individual reference files for detailed guidance:

```
references/tools-installation.md
references/tasks-definition.md
references/env-management.md
```

Each reference file contains:
- Mise command usage and examples
- Configuration file formats
- Best practices and patterns
- Migration guides
- Common workflows

## References

- https://mise.jdx.dev/
- https://mise.jdx.dev/configuration.html
- https://mise.jdx.dev/tasks/
- https://mise.jdx.dev/environments.html
