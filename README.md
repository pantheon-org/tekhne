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

### [azure-pipelines-toolkit](skills/ci-cd/azure-pipelines)

Complete azure-pipelines toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/azure-pipelines-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [azure-pipelines-generator](skills/ci-cd/azure-pipelines/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/generator/2026-03-02/audit.json) |
| [azure-pipelines-validator](skills/ci-cd/azure-pipelines/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/validator/2026-03-02/audit.json) |

### [gitlab-ci-toolkit](skills/ci-cd/gitlab-ci)

Complete GitLab CI/CD toolkit with generation and validation capabilities for pi... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/gitlab-ci-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [gitlab-ci-generator](skills/ci-cd/gitlab-ci/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/generator/2026-03-02/audit.json) |
| [gitlab-ci-validator](skills/ci-cd/gitlab-ci/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/validator/2026-03-02/audit.json) |

### [fluentbit-toolkit](skills/ci-cd/fluentbit)

Complete fluentbit toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/fluentbit-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [fluentbit-generator](skills/ci-cd/fluentbit/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/fluentbit/generator/2026-03-02/audit.json) |
| [fluentbit-validator](skills/ci-cd/fluentbit/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/fluentbit/validator/2026-03-02/audit.json) |

### [jenkinsfile-toolkit](skills/ci-cd/jenkinsfile)

Complete jenkinsfile toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/jenkinsfile-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [jenkinsfile-generator](skills/ci-cd/jenkinsfile/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/generator/2026-03-02/audit.json) |
| [jenkinsfile-validator](skills/ci-cd/jenkinsfile/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/validator/2026-03-02/audit.json) |

### [helm-toolkit](skills/ci-cd/helm)

Complete helm toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/helm-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [helm-generator](skills/ci-cd/helm/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/helm/generator/2026-03-02/audit.json) |
| [helm-validator](skills/ci-cd/helm/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/helm/validator/2026-03-02/audit.json) |

### [github-actions-toolkit](skills/ci-cd/github-actions)

Complete GitHub Actions toolkit with generation and validation capabilities for ... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/github-actions-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [github-actions-generator](skills/ci-cd/github-actions/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/generator/2026-03-02/audit.json) |
| [github-actions-validator](skills/ci-cd/github-actions/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/validator/2026-03-02/audit.json) |

## Infrastructure (8 tiles)

Infrastructure as Code

### [terraform-toolkit](skills/infrastructure/terraform)

Complete terraform toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/terraform-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [terraform-generator](skills/infrastructure/terraform/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terraform/generator/2026-03-02/audit.json) |
| [terraform-validator](skills/infrastructure/terraform/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/terraform/validator/2026-03-02/audit.json) |

### [dockerfile-toolkit](skills/infrastructure/dockerfile)

Complete dockerfile toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/dockerfile-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [dockerfile-generator](skills/infrastructure/dockerfile/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/dockerfile/generator/2026-03-02/audit.json) |
| [dockerfile-validator](skills/infrastructure/dockerfile/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/dockerfile/validator/2026-03-02/audit.json) |

### [terragrunt-toolkit](skills/infrastructure/terragrunt)

Complete terragrunt toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/terragrunt-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [terragrunt-generator](skills/infrastructure/terragrunt/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/generator/2026-03-02/audit.json) |
| [terragrunt-validator](skills/infrastructure/terragrunt/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/validator/2026-03-02/audit.json) |

### [k8s-yaml-toolkit](skills/infrastructure/k8s-yaml)

Complete k8s-yaml toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/k8s-yaml-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [k8s-yaml-generator](skills/infrastructure/k8s-yaml/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/generator/2026-03-02/audit.json) |
| [k8s-yaml-validator](skills/infrastructure/k8s-yaml/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/validator/2026-03-02/audit.json) |

### [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator)

Creates test stacks, analyzes CloudFormation events, and compares actual vs docu... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-behavior-validator)

| Skill | Rating | Audit |
| --- | --- | --- |
| [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/cfn/behavior-validator/2026-03-02/audit.json) |

### [cfn-template-compare](skills/infrastructure/cfn/template-compare)

Compares deployed CloudFormation templates with locally synthesized CDK template... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-template-compare)

| Skill | Rating | Audit |
| --- | --- | --- |
| [cfn-template-compare](skills/infrastructure/cfn/template-compare/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/infrastructure/cfn/template-compare/2026-03-03/audit.json) |

### [ansible-toolkit](skills/infrastructure/ansible)

Complete ansible toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/ansible-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [ansible-generator](skills/infrastructure/ansible/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/ansible/generator/2026-03-02/audit.json) |
| [ansible-validator](skills/infrastructure/ansible/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/ansible/validator/2026-03-02/audit.json) |

### [cdk-nag](skills/infrastructure/aws-cdk/cdk-nag)

Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding r... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cdk-nag)

| Skill | Rating | Audit |
| --- | --- | --- |
| [cdk-nag](skills/infrastructure/aws-cdk/cdk-nag/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/aws-cdk/cdk-nag/2026-03-02/audit.json) |

## Repository Management (5 tiles)

Repository & workspace management

### [nx-biome-integration](skills/repository-mgmt/nx/biome-integration)

Integrate Biome into Nx monorepos with deterministic setup, caching, migration f... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-biome-integration)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-biome-integration](skills/repository-mgmt/nx/biome-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/biome-integration/2026-03-02/audit.json) |

### [nx-bun-integration](skills/repository-mgmt/nx/bun-integration)

Integrate Bun runtime into Nx monorepos with deterministic plugin setup, executo... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-bun-integration)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-bun-integration](skills/repository-mgmt/nx/bun-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/bun-integration/2026-03-02/audit.json) |

### [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns)

Configure and optimize Nx monorepo workspaces with deterministic project-graph s... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-workspace-patterns)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/workspace-patterns/2026-03-02/audit.json) |

### [nx-plugin-toolkit](skills/repository-mgmt/nx)

Complete Nx plugin development toolkit: create generators, executors, and extend... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-plugin-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-generators](skills/repository-mgmt/nx/generators/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/generators/2026-03-02/audit.json) |
| [nx-executors](skills/repository-mgmt/nx/executors/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/executors/2026-03-02/audit.json) |
| [extending-nx-plugins](skills/repository-mgmt/nx/extending-plugins/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/repository-mgmt/nx/extending-plugins/2026-03-03/audit.json) |

### [nx-vite-integration](skills/repository-mgmt/nx/vite-integration)

Configure and integrate Vite in Nx monorepos for applications and libraries. Cov... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-vite-integration)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-vite-integration](skills/repository-mgmt/nx/vite-integration/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/repository-mgmt/nx/vite-integration/2026-03-02/audit.json) |

## Development (6 tiles)

Development tooling

### [makefile-toolkit](skills/development/scripting/makefile)

Complete makefile toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/makefile-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [makefile-generator](skills/development/scripting/makefile/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/scripting/makefile/generator/2026-03-02/audit.json) |
| [makefile-validator](skills/development/scripting/makefile/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/scripting/makefile/validator/2026-03-02/audit.json) |

### [bash-script-toolkit](skills/development/scripting/bash-script)

Complete bash-script toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bash-script-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [bash-script-generator](skills/development/scripting/bash-script/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/generator/2026-03-02/audit.json) |
| [bash-script-validator](skills/development/scripting/bash-script/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/validator/2026-03-02/audit.json) |

### [commanderjs](skills/development/commanderjs)

Complete Commander.js CLI framework guidance covering command structure, options... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/commanderjs)

| Skill | Rating | Audit |
| --- | --- | --- |
| [commanderjs](skills/development/commanderjs/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/commanderjs/2026-03-02/audit.json) |

### [bun-development](skills/development/bun-development)

Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package managemen... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bun-development)

| Skill | Rating | Audit |
| --- | --- | --- |
| [bun-development](skills/development/bun-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/bun-development/2026-03-02/audit.json) |

### [biome-complete](skills/development/biome-complete)

Complete Biome toolchain guidance for real repository workflows. Use when users ... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/biome-complete)

| Skill | Rating | Audit |
| --- | --- | --- |
| [biome-complete](skills/development/biome-complete/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/development/biome-complete/2026-03-02/audit.json) |

### [typescript-advanced](skills/development/typescript-advanced)

Comprehensive TypeScript guidance covering compiler configuration, advanced type... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/typescript-advanced)

| Skill | Rating | Audit |
| --- | --- | --- |
| [typescript-advanced](skills/development/typescript-advanced/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/typescript-advanced/2026-03-02/audit.json) |

## Agentic Harness (4 tiles)

Agent framework configurations

### [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor)

Audit and improve skill collections with a 9-dimension scoring framework (Knowle... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/skill-quality-auditor)

| Skill | Rating | Audit |
| --- | --- | --- |
| [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-04](.context/audits/agentic-harness/skill-quality-auditor/2026-03-04/audit.json) |

### [agents-md](skills/agentic-harness/agents-md)

Create and maintain AGENTS.md documentation for simple projects and complex mono... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/agents-md)

| Skill | Rating | Audit |
| --- | --- | --- |
| [agents-md](skills/agentic-harness/agents-md/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/agents-md/2026-03-02/audit.json) |

### [tessl-publish-public](skills/agentic-harness/tessl/publish-public)

Ensure Tessl tiles meet all requirements for public registry publishing with com... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/tessl-publish-public)

| Skill | Rating | Audit |
| --- | --- | --- |
| [tessl-publish-public](skills/agentic-harness/tessl/publish-public/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-03](.context/audits/agentic-harness/tessl/publish-public/2026-03-03/audit.json) |

### [opencode-config](skills/agentic-harness/opencode)

Configure OpenCode via opencode.json and AGENTS.md with deterministic provider s... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/opencode-config)

| Skill | Rating | Audit |
| --- | --- | --- |
| [opencode-config](skills/agentic-harness/opencode/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/opencode/2026-03-02/audit.json) |

## Testing (3 tiles)

Testing methodologies & quality

### [test-driven-development](skills/testing/test-driven-development)

Master Test-Driven Development with deterministic red-green-refactor workflows, ... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/test-driven-development)

| Skill | Rating | Audit |
| --- | --- | --- |
| [test-driven-development](skills/testing/test-driven-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/test-driven-development/2026-03-02/audit.json) |

### [bdd-testing](skills/testing/bdd-testing)

Write and maintain Behavior-Driven Development tests with Gherkin and Cucumber. ... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bdd-testing)

| Skill | Rating | Audit |
| --- | --- | --- |
| [bdd-testing](skills/testing/bdd-testing/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/bdd-testing/2026-03-02/audit.json) |

### [ui-debug-workflow](skills/testing/ui-debug-workflow)

Debug UI changes with a repeatable evidence-first workflow. Use when validating ... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/ui-debug-workflow)

| Skill | Rating | Audit |
| --- | --- | --- |
| [ui-debug-workflow](skills/testing/ui-debug-workflow/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/ui-debug-workflow/2026-03-02/audit.json) |

## Software Engineering (1 tile)

Software engineering principles

### [software-design-principles](skills/software-engineering/software-design-principles)

Apply software design principles across architecture and implementation using de... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/software-design-principles)

| Skill | Rating | Audit |
| --- | --- | --- |
| [software-design-principles](skills/software-engineering/software-design-principles/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/software-engineering/software-design-principles/2026-03-02/audit.json) |

## Observability (3 tiles)

Monitoring, logging & debugging

### [logql-generator](skills/observability/logql-generator)

Generate label matchers, line filters, log aggregations, and metric queries in L... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/logql-generator)

| Skill | Rating | Audit |
| --- | --- | --- |
| [logql-generator](skills/observability/logql-generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/logql-generator/2026-03-02/audit.json) |

### [k8s-debug](skills/observability/k8s-debug)

Comprehensive Kubernetes debugging and troubleshooting toolkit. Use this skill w... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/k8s-debug)

| Skill | Rating | Audit |
| --- | --- | --- |
| [k8s-debug](skills/observability/k8s-debug/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/k8s-debug/2026-03-02/audit.json) |

### [promql-toolkit](skills/observability/promql)

Complete PromQL toolkit with generation and validation capabilities · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/promql-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [promql-generator](skills/observability/promql/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/promql/generator/2026-03-02/audit.json) |
| [promql-validator](skills/observability/promql/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/observability/promql/validator/2026-03-02/audit.json) |

## Documentation (5 tiles)

Writing & communication

### [markdown-authoring](skills/documentation/markdown-authoring)

Author high-quality Markdown documentation with deterministic structure, lint co... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/markdown-authoring)

| Skill | Rating | Audit |
| --- | --- | --- |
| [markdown-authoring](skills/documentation/markdown-authoring/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/markdown-authoring/2026-03-02/audit.json) |

### [plain-english](skills/documentation/plain-english)

Write technical content in plain English for non-technical stakeholders by trans... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/plain-english)

| Skill | Rating | Audit |
| --- | --- | --- |
| [plain-english](skills/documentation/plain-english/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/plain-english/2026-03-02/audit.json) |

### [journal-entry-creator](skills/documentation/journal-entry-creator)

Create structured journal entries with YAML frontmatter, template-based sections... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/journal-entry-creator)

| Skill | Rating | Audit |
| --- | --- | --- |
| [journal-entry-creator](skills/documentation/journal-entry-creator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/documentation/journal-entry-creator/2026-03-03/audit.json) |

### [acceptance-criteria](skills/documentation/acceptance-criteria)

Write clear, testable acceptance criteria for user stories and feature delivery;... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/acceptance-criteria)

| Skill | Rating | Audit |
| --- | --- | --- |
| [acceptance-criteria](skills/documentation/acceptance-criteria/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/acceptance-criteria/2026-03-02/audit.json) |

### [conventional-commits](skills/documentation/conventional-commits)

Skill for creating structured, semantic commit messages following the Convention... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/conventional-commits)

| Skill | Rating | Audit |
| --- | --- | --- |
| [conventional-commits](skills/documentation/conventional-commits/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/documentation/conventional-commits/2026-03-02/audit.json) |

## Package Management (1 tile)

Package & version management

### [mise-complete](skills/package-mgmt/mise-complete)

Configure and operate Mise for deterministic developer environments. Use when in... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/mise-complete)

| Skill | Rating | Audit |
| --- | --- | --- |
| [mise-complete](skills/package-mgmt/mise-complete/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/package-mgmt/mise-complete/2026-03-02/audit.json) |

## Project Management (3 tiles)

Planning & organization

### [create-context-file](skills/project-mgmt/create-context-file)

Create context files (plans, justifications, scratches) in .context/ directory w... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/create-context-file)

| Skill | Rating | Audit |
| --- | --- | --- |
| [create-context-file](skills/project-mgmt/create-context-file/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/project-mgmt/create-context-file/2026-03-03/audit.json) |

### [moscow-prioritization](skills/project-mgmt/moscow-prioritization)

Prioritize product requirements with the MoSCoW framework in a deterministic way... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/moscow-prioritization)

| Skill | Rating | Audit |
| --- | --- | --- |
| [moscow-prioritization](skills/project-mgmt/moscow-prioritization/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/moscow-prioritization/2026-03-02/audit.json) |

### [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter)

Split large implementation plan documents into digestible, hierarchical structur... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/implementation-plan-splitter)

| Skill | Rating | Audit |
| --- | --- | --- |
| [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/implementation-plan-splitter/2026-03-02/audit.json) |

## Specialized (3 tiles)

Domain-specific tools

### [github-copilot-models](skills/specialized/github-copilot-models)

Query and display available GitHub Copilot AI models with their capabilities, co... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/github-copilot-models)

| Skill | Rating | Audit |
| --- | --- | --- |
| [github-copilot-models](skills/specialized/github-copilot-models/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/specialized/github-copilot-models/2026-03-03/audit.json) |

### [gitlab-api](skills/specialized/gitlab-api)

Retrieve and analyze GitLab merge request comments and metadata using authentica... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/gitlab-api)

| Skill | Rating | Audit |
| --- | --- | --- |
| [gitlab-api](skills/specialized/gitlab-api/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/specialized/gitlab-api/2026-03-02/audit.json) |

### [colyseus-multiplayer](skills/specialized/colyseus-multiplayer)

Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when ... · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/colyseus-multiplayer)

| Skill | Rating | Audit |
| --- | --- | --- |
| [colyseus-multiplayer](skills/specialized/colyseus-multiplayer/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/specialized/colyseus-multiplayer/2026-03-02/audit.json) |

## What are Agent Skills?
