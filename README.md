# Tekhne - Agent Skills Repository

A curated collection of reusable agent skills for AI assistants, designed for easy redistribution and integration.

## What are Agent Skills?

Agent skills are modular instruction packages that extend AI assistant capabilities. Each skill provides specialized
domain knowledge, workflows, and best practices that can be loaded on-demand.

## Available Skills

| Skill | Description | Rating | Audit |
| --- | --- | --- | --- |
| [`acceptance-criteria`](skills/acceptance-criteria/SKILL.md) | Write effective acceptance criteria for user stories | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-02-22](.context/audits/acceptance-criteria-audit-2026-02-22.md) |
| [`agents-md`](skills/agents-md/SKILL.md) | AGENTS.md management - create, generate, and maintain AI documentation | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-22](.context/audits/agents-md-audit-2026-02-22.md) |
| [`bdd-testing`](skills/bdd-testing/SKILL.md) | Behavior-Driven Development with Gherkin/Cucumber | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/bdd-testing-audit-2026-02-23.md) |
| [`biome-complete`](skills/biome-complete/SKILL.md) | Biome toolchain: configuration, linting, formatting, ESLint/Prettier migration | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/biome-complete-audit-2026-02-23.md) |
| [`bun-development`](skills/bun-development/SKILL.md) | Bun.js runtime APIs, testing, SQLite, and security | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/bun-development-audit-2026-02-22.md) |
| [`cdk-nag`](skills/cdk-nag/SKILL.md) | AWS CDK security and compliance validation | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/cdk-nag-audit-2026-02-22.md) |
| [`cfn-behavior-validator`](skills/cfn-behavior-validator/SKILL.md) | Validate CloudFormation resource update behaviors through testing | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-02-22](.context/audits/cfn-behavior-validator-audit-2026-02-22.md) |
| [`cfn-template-compare`](skills/cfn-template-compare/SKILL.md) | Compare deployed CloudFormation templates with local CDK templates | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-22](.context/audits/cfn-template-compare-audit-2026-02-22.md) |
| [`colyseus-multiplayer`](skills/colyseus-multiplayer/SKILL.md) | Colyseus multiplayer game server framework | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/colyseus-multiplayer-audit-2026-02-23.md) |
| [`commanderjs`](skills/commanderjs/SKILL.md) | Commander.js CLI framework for building command-line tools | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-02-22](.context/audits/commanderjs-audit-2026-02-22.md) |
| [`conventional-commits`](skills/conventional-commits/SKILL.md) | Structured, semantic commit messages following spec | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/conventional-commits-audit-2026-02-22.md) |
| [`create-context-file`](skills/create-context-file/SKILL.md) | Create context files in `.context/` with unique IDs and frontmatter | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/create-context-file-audit-2026-02-22.md) |
| [`extending-nx-plugins`](skills/extending-nx-plugins/SKILL.md) | Create and manage Nx plugins: generators, tasks, migrations | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/extending-nx-plugins-audit-2026-02-22.md) |
| [`github-copilot-models`](skills/github-copilot-models/SKILL.md) | Query GitHub Copilot model availability and configurations | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/github-copilot-models-audit-2026-02-22.md) |
| [`implementation-plan-splitter`](skills/implementation-plan-splitter/SKILL.md) | Split large implementation plans into digestible structures | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-22](.context/audits/implementation-plan-splitter-audit-2026-02-22.md) |
| [`markdown-authoring`](skills/markdown-authoring/SKILL.md) | Markdown syntax, markdownlint, and documentation guidance | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/markdown-authoring-audit-2026-02-23.md) |
| [`mise-complete`](skills/mise-complete/SKILL.md) | Mise tool management for versions, tasks, and env vars | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/mise-complete-audit-2026-02-23.md) |
| [`moscow-prioritization`](skills/moscow-prioritization/SKILL.md) | Prioritize requirements using MoSCoW method | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/moscow-prioritization-audit-2026-02-23.md) |
| [`nx-biome-integration`](skills/nx-biome-integration/SKILL.md) | Integrate Biome linter/formatter into Nx monorepos | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/nx-biome-integration-audit-2026-02-22.md) |
| [`nx-bun-integration`](skills/nx-bun-integration/SKILL.md) | Integrate Bun runtime into Nx monorepos | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/nx-bun-integration-audit-2026-02-22.md) |
| [`nx-executors`](skills/nx-executors/SKILL.md) | Create and use custom Nx executors in TypeScript monorepos | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-22](.context/audits/nx-executors-audit-2026-02-22.md) |
| [`nx-generators`](skills/nx-generators/SKILL.md) | Create Nx generators to automate code scaffolding | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-22](.context/audits/nx-generators-audit-2026-02-22.md) |
| [`nx-vite-integration`](skills/nx-vite-integration/SKILL.md) | Set up and configure Vite in Nx workspaces | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-23](.context/audits/nx-vite-integration-audit-2026-02-23.md) |
| [`nx-workspace-patterns`](skills/nx-workspace-patterns/SKILL.md) | Configure and optimize Nx monorepo workspaces | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/nx-workspace-patterns-audit-2026-02-22.md) |
| [`opencode-config`](skills/opencode-config/SKILL.md) | Edit opencode.json, AGENTS.md, and configuration files | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/opencode-config-audit-2026-02-22.md) |
| [`plain-english`](skills/plain-english/SKILL.md) | Write technical content for non-technical stakeholders | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/plain-english-audit-2026-02-22.md) |
| [`skill-quality-auditor`](skills/skill-quality-auditor/SKILL.md) | Automate skill quality evaluation, duplication detection, aggregations | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-22](.context/audits/skill-quality-auditor-audit-2026-02-22.md) |
| [`software-design-principles`](skills/software-design-principles/SKILL.md) | Architecture decisions, SOLID principles, and code quality fundamentals | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-22](.context/audits/software-design-principles-audit-2026-02-22.md) |
| [`test-driven-development`](skills/test-driven-development/SKILL.md) | Master TDD with Red-Green-Refactor cycle | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-22](.context/audits/test-driven-development-audit-2026-02-22.md) |
| [`typescript-advanced`](skills/typescript-advanced/SKILL.md) | Advanced TypeScript types, compiler config, and best practices | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/typescript-advanced-audit-2026-02-22.md) |
| [`ui-debug-workflow`](skills/ui-debug-workflow/SKILL.md) | Debug UI changes with visual testing and evidence collection | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/ui-debug-workflow-audit-2026-02-23.md) |

## Installation

### For Users

Install skills from this repository using the [Agent Skills](https://agentskills.io) CLI:

```bash
# Install a specific skill to all detected agents
npx skills add pantheon-org/tekhne --skill <skill-name>

# Install a specific skill to a specific agent
npx skills add pantheon-org/tekhne --skill <skill-name> -a claude-code

# Install globally (available across all projects)
npx skills add pantheon-org/tekhne --skill <skill-name> -g

# List available skills
npx skills add pantheon-org/tekhne --list
```

### For Contributors

Sync local skills to your development harness:

```bash
# Sync all local skills to all detected agents (project scope)
npx skills add ./skills --all

# Sync to specific agent(s)
npx skills add ./skills -a claude-code -a cursor

# Sync globally for cross-project access
npx skills add ./skills --all -g

# Preview changes without applying
npx skills add ./skills --all --dry-run

# List available local skills
npx skills add ./skills --list
```

## Supported Agents

This repository follows the [Agent Skills specification](https://agentskills.io) and works with 41+ AI assistants:

| Agent          | `--agent` Flag   | Project Path         | Global Path                   |
|----------------|------------------|----------------------|-------------------------------|
| Claude Code    | `claude-code`    | `.claude/skills/`    | `~/.claude/skills/`           |
| Cursor         | `cursor`         | `.agents/skills/`    | `~/.cursor/skills/`           |
| Gemini CLI     | `gemini-cli`     | `.agents/skills/`    | `~/.gemini/skills/`           |
| Codex          | `codex`          | `.agents/skills/`    | `~/.codex/skills/`            |
| OpenCode       | `opencode`       | `.agents/skills/`    | `~/.config/opencode/skills/`  |
| Cline          | `cline`          | `.cline/skills/`     | `~/.cline/skills/`            |
| Windsurf       | `windsurf`       | `.windsurf/skills/`  | `~/.codeium/windsurf/skills/` |
| Roo Code       | `roo`            | `.roo/skills/`       | `~/.roo/skills/`              |
| GitHub Copilot | `github-copilot` | `.agents/skills/`    | `~/.copilot/skills/`          |
| Goose          | `goose`          | `.goose/skills/`     | `~/.config/goose/skills/`     |
| OpenHands      | `openhands`      | `.openhands/skills/` | `~/.openhands/skills/`        |
| Amp            | `amp`            | `.agents/skills/`    | `~/.config/agents/skills/`    |

For the complete list of supported agents, see the
[Vercel Labs Skills repository](https://github.com/vercel-labs/skills).

## Skill Structure

Each skill follows this standard structure:

```text
skills/<skill-name>/
├── SKILL.md          # Main skill definition with instructions
├── resources/        # Optional: preference for md files
├── templates/        # Optional: YAML files only (.yaml/.yml)
├── scripts/          # Optional: POSIX shell scripts only (.sh, #!/usr/bin/env sh)
├── schemas/          # Optional: JSON Schema files only (*.schema.json)
└── README.md         # Optional: additional documentation
```

Repository enforcement:

- `templates/` entries must be valid YAML (`.yaml` or `.yml`).
- `schemas/` entries must be `*.schema.json` and include a JSON Schema `"$schema"` URL from `json-schema.org`.
- `scripts/` entries must be `.sh` with `#!/usr/bin/env sh` and pass `sh -n`.
- Use the `skill-quality-auditor` skill workflow (see `skills/skill-quality-auditor/SKILL.md`) to validate and enforce
  these conventions.

### SKILL.md Format

```markdown
---
name: skill-name
description: Brief description of what the skill does
---

# Skill Title

[Detailed instructions and workflows...]
```

## Contributing

We welcome contributions! To add a new skill:

1. Fork the repository
2. Create a new directory under `skills/` with your skill name
3. Add a `SKILL.md` file following the standard format
4. Include any necessary resources in a `resources/` subdirectory
5. Submit a pull request

### Skill Guidelines

- Use clear, descriptive names (kebab-case)
- Write concise descriptions (1-2 sentences)
- Provide comprehensive instructions in SKILL.md
- Include examples where helpful
- Keep skills focused and modular

## License

MIT License - see [LICENSE](LICENSE) for details.

## Tooling References

- [shellcheck](https://www.shellcheck.net/) for shell script linting in pre-commit hooks
- [yq by mikefarah](https://github.com/mikefarah/yq) for YAML validation in pre-commit hooks
