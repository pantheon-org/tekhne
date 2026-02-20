# Tekhne - Agent Skills Repository

A curated collection of reusable agent skills for AI assistants, designed for easy redistribution and integration.

## What are Agent Skills?

Agent skills are modular instruction packages that extend AI assistant capabilities. Each skill provides specialized
domain knowledge, workflows, and best practices that can be loaded on-demand.

## Available Skills

| Skill                         | Description                                                              |
| ----------------------------- | ------------------------------------------------------------------------ |
| `acceptance-criteria`         | Write effective acceptance criteria for user stories                     |
| `agents-md`                   | AGENTS.md management - create, generate, and maintain AI documentation   |
| `bdd-testing`                 | Behavior-Driven Development with Gherkin/Cucumber                        |
| `bun-development`             | Bun.js runtime APIs, testing, SQLite, and security                       |
| `cdk-nag`                     | AWS CDK security and compliance validation                               |
| `colyseus-multiplayer`        | Colyseus multiplayer game server framework                               |
| `commanderjs`                 | Commander.js CLI framework for building command-line tools               |
| `conventional-commits`        | Structured, semantic commit messages following spec                      |
| `create-context-file`         | Create context files in `.context/` with unique IDs and frontmatter      |
| `github-copilot-models`       | Query GitHub Copilot model availability and configurations               |
| `implementation-plan-splitter`| Split large implementation plans into digestible structures              |
| `markdown-authoring`          | Markdown syntax, markdownlint, and documentation guidance                |
| `mise-complete`               | Mise tool management for versions, tasks, and env vars                   |
| `moscow-prioritization`       | Prioritize requirements using MoSCoW method                              |
| `nx-biome-integration`        | Integrate Biome linter/formatter into Nx monorepos                       |
| `nx-bun-integration`          | Integrate Bun runtime into Nx monorepos                                  |
| `nx-vite-integration`         | Set up and configure Vite in Nx workspaces                               |
| `nx-workspace-patterns`       | Configure and optimize Nx monorepo workspaces                            |
| `opencode-config`             | Edit opencode.json, AGENTS.md, and configuration files                   |
| `plain-english`               | Write technical content for non-technical stakeholders                   |
| `test-driven-development`     | Master TDD with Red-Green-Refactor cycle                                 |
| `typescript-advanced`         | Advanced TypeScript types, compiler config, and best practices           |
| `ui-debug-workflow`           | Debug UI changes with visual testing and evidence collection             |

## Installation

Each skill is self-contained in its own directory under `skills/`. To use a skill:

1. Copy the skill directory to your agent's skills location:

   ```bash
   cp -r skills/<skill-name> ~/.config/opencode/skills/
   ```

2. The skill will be automatically available in your OpenCode sessions.

## Skill Structure

Each skill follows this standard structure:

```
skills/<skill-name>/
├── SKILL.md          # Main skill definition with instructions
├── resources/        # Optional: templates, scripts, references
└── README.md         # Optional: additional documentation
```

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
