# Tekhne - Agent Skills Repository

A curated collection of reusable agent skills for AI assistants, designed for easy redistribution and integration.

## What are Agent Skills?

Agent skills are modular instruction packages that extend AI assistant capabilities. Each skill provides specialized
domain knowledge, workflows, and best practices that can be loaded on-demand.

## Available Skills

| Skill | Description | Rating | Audit |
| --- | --- | --- | --- |
| [`acceptance-criteria`](skills/acceptance-criteria/SKILL.md) | Write clear, testable acceptance criteria for user stories and feature delive... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/acceptance-criteria/2026-02-23/audit.json) |
| [`agents-md`](skills/agents-md/SKILL.md) | Create and maintain AGENTS.md documentation for simple projects and complex m... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/agents-md/2026-02-23/audit.json) |
| [`bdd-testing`](skills/bdd-testing/SKILL.md) | Write and maintain Behavior-Driven Development tests with Gherkin and Cucumbe... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/bdd-testing/2026-02-23/audit.json) |
| [`biome-complete`](skills/biome-complete/SKILL.md) | Complete Biome toolchain guidance for real repository workflows. Use when use... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/biome-complete/2026-02-23/audit.json) |
| [`bun-development`](skills/bun-development/SKILL.md) | Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package manage... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/bun-development/2026-02-23/audit.json) |
| [`cdk-nag`](skills/cdk-nag/SKILL.md) | Enforce AWS CDK security and compliance controls with cdk-nag. Use when addin... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/cdk-nag/2026-02-23/audit.json) |
| [`cfn-behavior-validator`](skills/cfn-behavior-validator/SKILL.md) | Validate CloudFormation resource update behaviors through repeatable experime... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/cfn-behavior-validator/2026-02-23/audit.json) |
| [`cfn-template-compare`](skills/cfn-template-compare/SKILL.md) | Compare deployed CloudFormation templates with locally synthesized CDK templa... | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-22](.context/audits/cfn-template-compare/2026-02-22/audit.json) |
| [`colyseus-multiplayer`](skills/colyseus-multiplayer/SKILL.md) | Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use wh... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/colyseus-multiplayer/2026-02-23/audit.json) |
| [`commanderjs`](skills/commanderjs/SKILL.md) | Complete Commander.js CLI framework guidance covering command structure, opti... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-02-22](.context/audits/commanderjs/2026-02-22/audit.json) |
| [`conventional-commits`](skills/conventional-commits/SKILL.md) | Skill for creating structured, semantic commit messages following the Convent... | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/conventional-commits/2026-02-22/audit.json) |
| [`create-context-file`](skills/create-context-file/SKILL.md) | Create context files (plans, justifications, scratches) in .context/ director... | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/create-context-file/2026-02-22/audit.json) |
| [`extending-nx-plugins`](skills/extending-nx-plugins/SKILL.md) | Comprehensive guide for creating and managing Nx plugins including generators... | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/extending-nx-plugins/2026-02-22/audit.json) |
| [`github-copilot-models`](skills/github-copilot-models/SKILL.md) | Query and display available GitHub Copilot AI models with their capabilities,... | ![C](https://img.shields.io/badge/Rating-C%20-red) | [2026-02-22](.context/audits/github-copilot-models/2026-02-22/audit.json) |
| [`implementation-plan-splitter`](skills/implementation-plan-splitter/SKILL.md) | Split large implementation plan documents into digestible, hierarchical struc... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-22](.context/audits/implementation-plan-splitter/2026-02-22/audit.json) |
| [`journal-entry-creator`](skills/journal-entry-creator/SKILL.md) | Create structured journal entries with YAML frontmatter, template-based secti... | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-24](.context/audits/journal-entry-creator/2026-02-24/audit.json) |
| [`markdown-authoring`](skills/markdown-authoring/SKILL.md) | Author high-quality Markdown documentation with deterministic structure, lint... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/markdown-authoring/2026-02-23/audit.json) |
| [`mise-complete`](skills/mise-complete/SKILL.md) | Configure and operate Mise for deterministic developer environments. Use when... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/mise-complete/2026-02-23/audit.json) |
| [`moscow-prioritization`](skills/moscow-prioritization/SKILL.md) | Prioritize product requirements with the MoSCoW framework in a deterministic ... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/moscow-prioritization/2026-02-23/audit.json) |
| [`nx-biome-integration`](skills/nx-biome-integration/SKILL.md) | Integrate Biome into Nx monorepos with deterministic setup, caching, migratio... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/nx-biome-integration/2026-02-23/audit.json) |
| [`nx-bun-integration`](skills/nx-bun-integration/SKILL.md) | Integrate Bun runtime into Nx monorepos with deterministic plugin setup, exec... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/nx-bun-integration/2026-02-23/audit.json) |
| [`nx-executors`](skills/nx-executors/SKILL.md) | Create and operate custom Nx executors in TypeScript monorepos with determini... | ![A+](https://img.shields.io/badge/Rating-A+-brightgreen) | [2026-02-23](.context/audits/nx-executors/2026-02-23/audit.json) |
| [`nx-generators`](skills/nx-generators/SKILL.md) | Create Nx generators for TypeScript monorepos with deterministic Tree API usa... | ![A+](https://img.shields.io/badge/Rating-A+-brightgreen) | [2026-02-23](.context/audits/nx-generators/2026-02-23/audit.json) |
| [`nx-vite-integration`](skills/nx-vite-integration/SKILL.md) | Configure and integrate Vite in Nx monorepos for applications and libraries. ... | ![B](https://img.shields.io/badge/Rating-B%20-yellow) | [2026-02-23](.context/audits/nx-vite-integration/2026-02-23/audit.json) |
| [`nx-workspace-patterns`](skills/nx-workspace-patterns/SKILL.md) | Configure and optimize Nx monorepo workspaces with deterministic project-grap... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/nx-workspace-patterns/2026-02-23/audit.json) |
| [`opencode-config`](skills/opencode-config/SKILL.md) | Configure OpenCode via opencode.json and AGENTS.md with deterministic provide... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/opencode-config/2026-02-23/audit.json) |
| [`plain-english`](skills/plain-english/SKILL.md) | Write technical content in plain English for non-technical stakeholders by tr... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/plain-english/2026-02-23/audit.json) |
| [`skill-quality-auditor`](skills/skill-quality-auditor/SKILL.md) | Audit and improve skill collections with an 8-dimension scoring framework, du... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/skill-quality-auditor/2026-02-23/audit.json) |
| [`software-design-principles`](skills/software-design-principles/SKILL.md) | Apply software design principles across architecture and implementation using... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/software-design-principles/2026-02-23/audit.json) |
| [`test-driven-development`](skills/test-driven-development/SKILL.md) | Master Test-Driven Development with deterministic red-green-refactor workflow... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/test-driven-development/2026-02-23/audit.json) |
| [`typescript-advanced`](skills/typescript-advanced/SKILL.md) | Comprehensive TypeScript guidance covering compiler configuration, advanced t... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-02-23](.context/audits/typescript-advanced/2026-02-23/audit.json) |
| [`ui-debug-workflow`](skills/ui-debug-workflow/SKILL.md) | Debug UI changes with a repeatable evidence-first workflow. Use when validati... | ![A](https://img.shields.io/badge/Rating-A%20-green) | [2026-02-23](.context/audits/ui-debug-workflow/2026-02-23/audit.json) |

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

### Publishing Skills to Tessl Registry

To publish a skill to the tessl registry:

```bash
# Import the skill into your workspace
bun run tessl:import -- skills/<skill-name> --workspace pantheon-ai

# Lint the skill for errors
bun run tessl:lint -- skills/<skill-name>

# Publish to the registry
bun run tessl:publish -- skills/<skill-name>
```

### Improving a skill

Prompt your agent with the following:

```text
Find the next remediation plan under .context/plans and following the same steps:

    1. implement all remediation changes
    2. run validation/audit
    3. update README/report artifacts
    4. set remediation plan status: `completed`
    5. commit
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Tooling References

- [shellcheck](https://www.shellcheck.net/) for shell script linting in pre-commit hooks
- [yq by mikefarah](https://github.com/mikefarah/yq) for YAML validation in pre-commit hooks
