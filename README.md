# Tekhne - Agent Skills Repository

A curated collection of reusable agent skills for AI assistants, designed for easy redistribution and integration.

## What are Agent Skills?

Agent skills are modular instruction packages that extend AI assistant capabilities. Each skill provides specialized
domain knowledge, workflows, and best practices that can be loaded on-demand.

## CLI Tool

The repository includes a TypeScript CLI tool built with Commander.js and Bun for managing skills:

```bash
# Install skills locally
bun cli/index.ts install

# Audit skill quality
bun cli/index.ts audit all
bun cli/index.ts audit status
bun cli/index.ts audit summary

# Manage Tessl lifecycle
bun cli/index.ts tessl manage
bun cli/index.ts tessl publish-check <tiles...>
```

See [`cli/README.md`](cli/README.md) for complete documentation.

## Available Skills

## CI/CD (6 tiles)

CI/CD pipelines & deployment automation

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [azure-pipelines-toolkit](TILES.md#azure-pipelines-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/azure-pipelines-toolkit) | 0.1.0 |
| [gitlab-ci-toolkit](TILES.md#gitlab-ci-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/gitlab-ci-toolkit) | 0.1.0 |
| [fluentbit-toolkit](TILES.md#fluentbit-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/fluentbit-toolkit) | 0.1.0 |
| [jenkinsfile-toolkit](TILES.md#jenkinsfile-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/jenkinsfile-toolkit) | 0.1.0 |
| [helm-toolkit](TILES.md#helm-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/helm-toolkit) | 0.1.0 |
| [github-actions-toolkit](TILES.md#github-actions-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/github-actions-toolkit) | 0.1.0 |

## Infrastructure (8 tiles)

Infrastructure as Code

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [terraform-toolkit](TILES.md#terraform-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/terraform-toolkit) | 0.1.0 |
| [dockerfile-toolkit](TILES.md#dockerfile-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/dockerfile-toolkit) | 0.1.0 |
| [terragrunt-toolkit](TILES.md#terragrunt-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/terragrunt-toolkit) | 0.1.0 |
| [k8s-yaml-toolkit](TILES.md#k8s-yaml-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/k8s-yaml-toolkit) | 0.1.0 |
| [cfn-behavior-validator](TILES.md#cfn-behavior-validator) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/cfn-behavior-validator) | 0.1.2 |
| [cfn-template-compare](TILES.md#cfn-template-compare) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/cfn-template-compare) | 0.2.0 |
| [ansible-toolkit](TILES.md#ansible-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/ansible-toolkit) | 0.1.0 |
| [cdk-nag](TILES.md#cdk-nag) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/cdk-nag) | 0.1.2 |

## Repository Management (5 tiles)

Repository & workspace management

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [nx-biome-integration](TILES.md#nx-biome-integration) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-biome-integration) | 0.1.1 |
| [nx-bun-integration](TILES.md#nx-bun-integration) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-bun-integration) | 0.1.1 |
| [nx-workspace-patterns](TILES.md#nx-workspace-patterns) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-workspace-patterns) | 0.1.1 |
| [nx-plugin-toolkit](TILES.md#nx-plugin-toolkit) | 3 | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-plugin-toolkit) | 0.1.0 |
| [nx-vite-integration](TILES.md#nx-vite-integration) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-vite-integration) | 0.1.1 |

## Development (6 tiles)

Development tooling

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [makefile-toolkit](TILES.md#makefile-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/makefile-toolkit) | 0.1.0 |
| [bash-script-toolkit](TILES.md#bash-script-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/bash-script-toolkit) | 0.1.0 |
| [commanderjs](TILES.md#commanderjs) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/commanderjs) | 0.1.1 |
| [bun-development](TILES.md#bun-development) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/bun-development) | 0.1.1 |
| [biome-complete](TILES.md#biome-complete) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/biome-complete) | 0.1.1 |
| [typescript-advanced](TILES.md#typescript-advanced) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/typescript-advanced) | 0.1.1 |

## Agentic Harness (4 tiles)

Agent framework configurations

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [skill-quality-auditor](TILES.md#skill-quality-auditor) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/skill-quality-auditor) | 0.1.4 |
| [agents-md](TILES.md#agents-md) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/agents-md) | 0.1.3 |
| [tessl-publish-public](TILES.md#tessl-publish-public) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/tessl-publish-public) | 1.0.0 |
| [opencode-config](TILES.md#opencode-config) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/opencode-config) | 0.1.1 |

## Testing (3 tiles)

Testing methodologies & quality

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [test-driven-development](TILES.md#test-driven-development) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/test-driven-development) | 0.2.4 |
| [bdd-testing](TILES.md#bdd-testing) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/bdd-testing) | 0.2.0 |
| [ui-debug-workflow](TILES.md#ui-debug-workflow) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/ui-debug-workflow) | 0.1.2 |

## Software Engineering (1 tile)

Software engineering principles

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [design-principles](TILES.md#design-principles) | 4 | [Public](https://tessl.io/registry/skills/pantheon-ai/design-principles) | 1.0.0 |

## Observability (3 tiles)

Monitoring, logging & debugging

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [logql-generator](TILES.md#logql-generator) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/logql-generator) | 0.1.4 |
| [k8s-debug](TILES.md#k8s-debug) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/k8s-debug) | 0.1.1 |
| [promql-toolkit](TILES.md#promql-toolkit) | 2 | [Public](https://tessl.io/registry/skills/pantheon-ai/promql-toolkit) | 0.1.0 |

## Documentation (5 tiles)

Writing & communication

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [markdown-authoring](TILES.md#markdown-authoring) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/markdown-authoring) | 0.1.1 |
| [plain-english](TILES.md#plain-english) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/plain-english) | 0.1.1 |
| [journal-entry-creator](TILES.md#journal-entry-creator) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/journal-entry-creator) | 0.2.0 |
| [acceptance-criteria](TILES.md#acceptance-criteria) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/acceptance-criteria) | 0.1.1 |
| [conventional-commits](TILES.md#conventional-commits) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/conventional-commits) | 0.1.1 |

## Package Management (1 tile)

Package & version management

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [mise-complete](TILES.md#mise-complete) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/mise-complete) | 0.1.1 |

## Project Management (3 tiles)

Planning & organization

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [create-context-file](TILES.md#create-context-file) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/create-context-file) | 0.2.0 |
| [moscow-prioritization](TILES.md#moscow-prioritization) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/moscow-prioritization) | 0.1.1 |
| [implementation-plan-splitter](TILES.md#implementation-plan-splitter) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/implementation-plan-splitter) | 0.1.1 |

## Specialized (3 tiles)

Domain-specific tools

| Tile | Skills | Published | Version |
| --- | --- | --- | --- |
| [github-copilot-models](TILES.md#github-copilot-models) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/github-copilot-models) | 0.2.0 |
| [gitlab-api](TILES.md#gitlab-api) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/gitlab-api) | 1.3.0 |
| [colyseus-multiplayer](TILES.md#colyseus-multiplayer) | 1 | [Public](https://tessl.io/registry/skills/pantheon-ai/colyseus-multiplayer) | 0.1.1 |

