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

## CI/CD (12 skills)

CI/CD pipelines & deployment automation

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [azure-pipelines-validator](skills/ci-cd/azure-pipelines/validator/SKILL.md) | Validates, lints, and security-scans Azure DevOps Pipeline configurations (azure... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/validator/2026-03-02/audit.json) | - |
| [azure-pipelines-generator](skills/ci-cd/azure-pipelines/generator/SKILL.md) | Generates production-ready Azure DevOps Pipelines (azure-pipelines.yml) followin... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/generator/2026-03-02/audit.json) | - |
| [gitlab-ci-validator](skills/ci-cd/gitlab-ci/validator/SKILL.md) | Validates .gitlab-ci.yml syntax, detects security misconfigurations in job defin... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/validator/2026-03-02/audit.json) | - |
| [gitlab-ci-generator](skills/ci-cd/gitlab-ci/generator/SKILL.md) | Creates .gitlab-ci.yml files, configures pipeline stages, defines CI jobs and ru... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/generator/2026-03-02/audit.json) | - |
| [fluentbit-validator](skills/ci-cd/fluentbit/validator/SKILL.md) | Validates syntax, checks pipeline tag connections, detects security misconfigura... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/fluentbit/validator/2026-03-02/audit.json) | - |
| [fluentbit-generator](skills/ci-cd/fluentbit/generator/SKILL.md) | Generates, validates, and optimizes Fluent Bit configurations for production use... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/fluentbit/generator/2026-03-02/audit.json) | - |
| [jenkinsfile-validator](skills/ci-cd/jenkinsfile/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Jenkinsfi... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/validator/2026-03-02/audit.json) | - |
| [jenkinsfile-generator](skills/ci-cd/jenkinsfile/generator/SKILL.md) | Generates Jenkinsfiles with stages, agents, parallel builds, post-build actions,... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/generator/2026-03-02/audit.json) | - |
| [helm-validator](skills/ci-cd/helm/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and analyzing Helm chart... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/helm/validator/2026-03-02/audit.json) | - |
| [helm-generator](skills/ci-cd/helm/generator/SKILL.md) | Comprehensive toolkit for generating best practice Helm charts and resources fol... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/helm/generator/2026-03-02/audit.json) | - |
| [github-actions-validator](skills/ci-cd/github-actions/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and testing GitHub Actions workfl... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/validator/2026-03-02/audit.json) | - |
| [github-actions-generator](skills/ci-cd/github-actions/generator/SKILL.md) | Generates production-ready GitHub Actions workflows, custom actions, and CI/CD c... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/generator/2026-03-02/audit.json) | - |

## Infrastructure (13 skills)

Infrastructure as Code

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [terraform-validator](skills/infrastructure/terraform/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Terraform... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/terraform/validator/2026-03-02/audit.json) | - |
| [terraform-generator](skills/infrastructure/terraform/generator/SKILL.md) | Generate Terraform modules, configure providers, define variables and outputs, s... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terraform/generator/2026-03-02/audit.json) | - |
| [dockerfile-validator](skills/infrastructure/dockerfile/validator/SKILL.md) | Validates, lints, and secures Dockerfiles by running syntax checking, detecting ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/dockerfile/validator/2026-03-02/audit.json) | - |
| [dockerfile-generator](skills/infrastructure/dockerfile/generator/SKILL.md) | Generates production-ready Dockerfiles with multi-stage builds, layer caching, s... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/dockerfile/generator/2026-03-02/audit.json) | - |
| [terragrunt-validator](skills/infrastructure/terragrunt/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Terragrun... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/validator/2026-03-02/audit.json) | - |
| [terragrunt-generator](skills/infrastructure/terragrunt/generator/SKILL.md) | Comprehensive toolkit for generating best-practice Terragrunt configurations (HC... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/generator/2026-03-02/audit.json) | - |
| [k8s-yaml-validator](skills/infrastructure/k8s-yaml/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and testing Kubernetes YAML resou... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/validator/2026-03-02/audit.json) | - |
| [k8s-yaml-generator](skills/infrastructure/k8s-yaml/generator/SKILL.md) | Comprehensive toolkit for generating, validating, and managing Kubernetes YAML r... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/generator/2026-03-02/audit.json) | - |
| [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator/SKILL.md) | Creates test stacks, analyzes CloudFormation events, and compares actual vs docu... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/cfn/behavior-validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-behavior-validator) |
| [cfn-template-compare](skills/infrastructure/cfn/template-compare/SKILL.md) | Compares deployed CloudFormation templates with locally synthesized CDK template... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/infrastructure/cfn/template-compare/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-template-compare) |
| [ansible-validator](skills/infrastructure/ansible/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Ansible p... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/ansible/validator/2026-03-02/audit.json) | - |
| [ansible-generator](skills/infrastructure/ansible/generator/SKILL.md) | Generates, validates, and refactors production-ready Ansible playbooks, roles, t... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/ansible/generator/2026-03-02/audit.json) | - |
| [aws-cdk-cdk-nag](skills/infrastructure/aws-cdk/cdk-nag/SKILL.md) | Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding r... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/aws-cdk/cdk-nag/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cdk-nag) |

## Repository Management (7 skills)

Repository & workspace management

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [nx-biome-integration](skills/repository-mgmt/nx/biome-integration/SKILL.md) | Integrate Biome into Nx monorepos with deterministic setup, caching, migration f... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/biome-integration/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-biome-integration) |
| [nx-bun-integration](skills/repository-mgmt/nx/bun-integration/SKILL.md) | Integrate Bun runtime into Nx monorepos with deterministic plugin setup, executo... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/bun-integration/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-bun-integration) |
| [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns/SKILL.md) | Configure and optimize Nx monorepo workspaces with deterministic project-graph s... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/workspace-patterns/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-workspace-patterns) |
| [nx-executors](skills/repository-mgmt/nx/executors/SKILL.md) | Create and operate custom Nx executors in TypeScript monorepos with deterministi... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/executors/2026-03-02/audit.json) | - |
| [nx-extending-plugins](skills/repository-mgmt/nx/extending-plugins/SKILL.md) | Creates Nx plugins, builds custom generators, configures inferred tasks, and wri... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/repository-mgmt/nx/extending-plugins/2026-03-03/audit.json) | - |
| [nx-generators](skills/repository-mgmt/nx/generators/SKILL.md) | Create Nx generators for TypeScript monorepos with deterministic Tree API usage,... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/generators/2026-03-02/audit.json) | - |
| [nx-vite-integration](skills/repository-mgmt/nx/vite-integration/SKILL.md) | Configure and integrate Vite in Nx monorepos for applications and libraries. Cov... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/repository-mgmt/nx/vite-integration/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-vite-integration) |

## Development (8 skills)

Development tooling

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [scripting-makefile-validator](skills/development/scripting/makefile/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and optimizing Makefiles. Use whe... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/scripting/makefile/validator/2026-03-02/audit.json) | - |
| [scripting-makefile-generator](skills/development/scripting/makefile/generator/SKILL.md) | Generate GNU Make build systems that define build targets, configure dependencie... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/scripting/makefile/generator/2026-03-02/audit.json) | - |
| [scripting-bash-script-validator](skills/development/scripting/bash-script/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and optimizing bash and shell scr... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/validator/2026-03-02/audit.json) | - |
| [scripting-bash-script-generator](skills/development/scripting/bash-script/generator/SKILL.md) | Creates bash scripts with argument parsing, error handling, logging, and input v... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/generator/2026-03-02/audit.json) | - |
| [commanderjs](skills/development/commanderjs/SKILL.md) | Complete Commander.js CLI framework guidance covering command structure, options... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/commanderjs/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/commanderjs) |
| [bun-development](skills/development/bun-development/SKILL.md) | Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package managemen... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/bun-development/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bun-development) |
| [biome-complete](skills/development/biome-complete/SKILL.md) | Complete Biome toolchain guidance for real repository workflows. Use when users ... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/development/biome-complete/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/biome-complete) |
| [typescript-advanced](skills/development/typescript-advanced/SKILL.md) | Comprehensive TypeScript guidance covering compiler configuration, advanced type... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/typescript-advanced/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/typescript-advanced) |

## Agentic Harness (4 skills)

Agent framework configurations

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor/SKILL.md) | Audit and improve skill collections with 8-dimension scoring framework (Knowledg... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/agentic-harness/skill-quality-auditor/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/skill-quality-auditor) |
| [agents-md](skills/agentic-harness/agents-md/SKILL.md) | Create and maintain AGENTS.md documentation for simple projects and complex mono... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/agents-md/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/agents-md) |
| [tessl-publish-public](skills/agentic-harness/tessl/publish-public/SKILL.md) | Ensure Tessl tiles meet all requirements for public registry publication with co... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-03](.context/audits/agentic-harness/tessl/publish-public/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/tessl-publish-public) |
| [opencode](skills/agentic-harness/opencode/SKILL.md) | Configure OpenCode via opencode.json and AGENTS.md with deterministic provider s... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/opencode/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/opencode-config) |

## Testing (3 skills)

Testing methodologies & quality

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [test-driven-development](skills/testing/test-driven-development/SKILL.md) | Guides TDD (test-driven development) with red-green-refactor workflows, test-fir... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/test-driven-development/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/test-driven-development) |
| [bdd-testing](skills/testing/bdd-testing/SKILL.md) | Write and maintain Behavior-Driven Development tests with Gherkin and Cucumber. ... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/bdd-testing/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bdd-testing) |
| [ui-debug-workflow](skills/testing/ui-debug-workflow/SKILL.md) | Debug UI changes with a repeatable evidence-first workflow. Use when validating ... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/ui-debug-workflow/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/ui-debug-workflow) |

## Software Engineering (1 skills)

Software engineering principles

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [software-design-principles](skills/software-engineering/software-design-principles/SKILL.md) | Apply SOLID principles, detect design anti-patterns, and evaluate architectural ... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/software-engineering/software-design-principles/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/software-design-principles) |

## Observability (4 skills)

Monitoring, logging & debugging

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [logql-generator](skills/observability/logql-generator/SKILL.md) | Generate label matchers, line filters, log aggregations, and metric queries in L... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/logql-generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/logql-generator) |
| [k8s-debug](skills/observability/k8s-debug/SKILL.md) | Inspect pod logs, analyze resource quotas, trace network policies, check deploym... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/k8s-debug/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/k8s-debug) |
| [promql-validator](skills/observability/promql/validator/SKILL.md) | Comprehensive toolkit for validating, optimizing, and understanding Prometheus Q... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/observability/promql/validator/2026-03-02/audit.json) | - |
| [promql-generator](skills/observability/promql/generator/SKILL.md) | Generate PromQL queries for calculating error rates, aggregating metrics across ... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/promql/generator/2026-03-02/audit.json) | - |

## Documentation (5 skills)

Writing & communication

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [markdown-authoring](skills/documentation/markdown-authoring/SKILL.md) | Author high-quality Markdown documentation with deterministic structure, lint co... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/markdown-authoring/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/markdown-authoring) |
| [plain-english](skills/documentation/plain-english/SKILL.md) | Translates technical content into plain English for non-technical stakeholders b... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/plain-english/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/plain-english) |
| [journal-entry-creator](skills/documentation/journal-entry-creator/SKILL.md) | - | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/documentation/journal-entry-creator/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/journal-entry-creator) |
| [acceptance-criteria](skills/documentation/acceptance-criteria/SKILL.md) | Write clear, testable acceptance criteria for user stories and feature delivery;... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/acceptance-criteria/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/acceptance-criteria) |
| [conventional-commits](skills/documentation/conventional-commits/SKILL.md) | Generates and formats git commit messages following the Conventional Commits spe... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/documentation/conventional-commits/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/conventional-commits) |

## Package Management (1 skills)

Package & version management

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [mise-complete](skills/package-mgmt/mise-complete/SKILL.md) | Configure and operate Mise for deterministic developer environments. Use when in... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/package-mgmt/mise-complete/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/mise-complete) |

## Project Management (3 skills)

Planning & organization

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [create-context-file](skills/project-mgmt/create-context-file/SKILL.md) | Creates structured context files (plans, justifications, scratches) in the .cont... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/project-mgmt/create-context-file/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/create-context-file) |
| [moscow-prioritization](skills/project-mgmt/moscow-prioritization/SKILL.md) | Categorize requirements into Must/Should/Could/Won't tiers, generate priority ma... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/moscow-prioritization/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/moscow-prioritization) |
| [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter/SKILL.md) | Split large implementation plan documents into digestible, hierarchical structur... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/implementation-plan-splitter/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/implementation-plan-splitter) |

## Specialized (3 skills)

Domain-specific tools

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [github-copilot-models](skills/specialized/github-copilot-models/SKILL.md) | Query and display available GitHub Copilot AI models with their capabilities, co... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/specialized/github-copilot-models/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/github-copilot-models) |
| [gitlab-api](skills/specialized/gitlab-api/SKILL.md) | Fetches and analyzes GitLab merge request (MR) comments, metadata, and review fe... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/specialized/gitlab-api/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/gitlab-api) |
| [colyseus-multiplayer](skills/specialized/colyseus-multiplayer/SKILL.md) | Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/specialized/colyseus-multiplayer/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/colyseus-multiplayer) |

## What are Agent Skills?
