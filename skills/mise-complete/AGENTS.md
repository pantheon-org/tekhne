# mise-complete

## Overview

Complete Mise tool management guidance covering version management, task automation, environment variables, and configuration.

## Structure

```
mise-complete/
  SKILL.md       # Main navigation hub
  AGENTS.md      # This file - complete reference guide
  references/    # Detailed reference files by category
```

## Usage

1. Read `SKILL.md` to understand when to use Mise
2. Navigate to categories based on your task
3. Load reference files on-demand
4. Start with tools, then tasks, then environment configuration

## Reference Categories

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Tool Version Management | CRITICAL | `tools-` |
| 2 | Task Automation | HIGH | `tasks-` |
| 3 | Environment Variables | HIGH | `env-` |
| 4 | Configuration | MEDIUM | `config-` |

## Available References

**Tool Version Management** (`tools-`):
- `references/tools-installation.md` - mise install, mise use commands
- `references/tools-versions.md` - .tool-versions file format
- `references/tools-plugins.md` - Available tool plugins
- `references/tools-migration.md` - Migrating from asdf, nvm, etc.

**Task Automation** (`tasks-`):
- `references/tasks-definition.md` - Defining tasks in mise.toml
- `references/tasks-dependencies.md` - Task dependencies and ordering
- `references/tasks-file-tasks.md` - File-based task triggers
- `references/tasks-parallel.md` - Parallel task execution

**Environment Variables** (`env-`):
- `references/env-management.md` - mise env, mise set commands
- `references/env-dotenv.md` - .env file integration
- `references/env-templates.md` - Environment variable templates
- `references/env-direnv.md` - Replacing direnv with mise

**Configuration** (`config-`):
- `references/config-mise-toml.md` - mise.toml structure
- `references/config-hierarchy.md` - Configuration file precedence
- `references/config-profiles.md` - Environment profiles
- `references/config-global.md` - Global vs local configuration

---

*Consolidates 3 original skills: mise-environment-management, mise-task-configuration, mise-tool-management*

*16 reference files across 4 categories*
