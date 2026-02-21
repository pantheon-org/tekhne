# Tekhne - Agent Skills Repository

A curated collection of reusable agent skills for AI assistants, designed for easy redistribution and integration.

## What are Agent Skills?

Agent skills are modular instruction packages that extend AI assistant capabilities. Each skill provides specialized
domain knowledge, workflows, and best practices that can be loaded on-demand.

## Available Skills

| Skill                          | Description                                                                    |
| ------------------------------ | ------------------------------------------------------------------------------ |
| `acceptance-criteria`          | Write effective acceptance criteria for user stories                           |
| `agents-md`                    | AGENTS.md management - create, generate, and maintain AI documentation         |
| `bdd-testing`                  | Behavior-Driven Development with Gherkin/Cucumber                              |
| `biome-complete`               | Biome toolchain: configuration, linting, formatting, ESLint/Prettier migration |
| `bun-development`              | Bun.js runtime APIs, testing, SQLite, and security                             |
| `cdk-nag`                      | AWS CDK security and compliance validation                                     |
| `cfn-behavior-validator`       | Validate CloudFormation resource update behaviors through testing              |
| `cfn-template-compare`         | Compare deployed CloudFormation templates with local CDK templates             |
| `colyseus-multiplayer`         | Colyseus multiplayer game server framework                                     |
| `commanderjs`                  | Commander.js CLI framework for building command-line tools                     |
| `conventional-commits`         | Structured, semantic commit messages following spec                            |
| `create-context-file`          | Create context files in `.context/` with unique IDs and frontmatter            |
| `extending-nx-plugins`         | Create and manage Nx plugins: generators, tasks, migrations                    |
| `github-copilot-models`        | Query GitHub Copilot model availability and configurations                     |
| `implementation-plan-splitter` | Split large implementation plans into digestible structures                    |
| `markdown-authoring`           | Markdown syntax, markdownlint, and documentation guidance                      |
| `mise-complete`                | Mise tool management for versions, tasks, and env vars                         |
| `moscow-prioritization`        | Prioritize requirements using MoSCoW method                                    |
| `nx-biome-integration`         | Integrate Biome linter/formatter into Nx monorepos                             |
| `nx-bun-integration`           | Integrate Bun runtime into Nx monorepos                                        |
| `nx-executors`                 | Create and use custom Nx executors in TypeScript monorepos                     |
| `nx-generators`                | Create Nx generators to automate code scaffolding                              |
| `nx-vite-integration`          | Set up and configure Vite in Nx workspaces                                     |
| `nx-workspace-patterns`        | Configure and optimize Nx monorepo workspaces                                  |
| `opencode-config`              | Edit opencode.json, AGENTS.md, and configuration files                         |
| `plain-english`                | Write technical content for non-technical stakeholders                         |
| `skill-quality-auditor`        | Automate skill quality evaluation, duplication detection, aggregations         |
| `software-design-principles`   | Architecture decisions, SOLID principles, and code quality fundamentals        |
| `test-driven-development`      | Master TDD with Red-Green-Refactor cycle                                       |
| `typescript-advanced`          | Advanced TypeScript types, compiler config, and best practices                 |
| `ui-debug-workflow`            | Debug UI changes with visual testing and evidence collection                   |

## Installation

Each skill is self-contained in its own directory under `skills/`. To use a skill:

1. Copy the skill directory to your agent's skills location:

   ```bash
   npx skills add https://github.com/pantheon-org/tekhne --skill <skill-name> 
   ```

2. The skill will be automatically available in your Harness of choice.

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
- Use the `skill-quality-auditor` skill workflow (see `skills/skill-quality-auditor/SKILL.md`) to validate and enforce these conventions.

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

- [yq by mikefarah](https://github.com/mikefarah/yq) for YAML validation in pre-commit hooks
