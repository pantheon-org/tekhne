# Tekhne - Agent Skills Repository

A curated collection of reusable agent skills for AI assistants, designed for easy redistribution and integration.

## What are Agent Skills?

Agent skills are modular instruction packages that extend AI assistant capabilities. Each skill provides specialized
domain knowledge, workflows, and best practices that can be loaded on-demand.

## Available Skills

## CI/CD (12 skills)

CI/CD pipelines & deployment automation

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [azure-pipelines-generator](skills/ci-cd/azure-pipelines/generator/SKILL.md) | Generates production-ready Azure DevOps Pipelines (azure-pipelines.yml) follo... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/azure-pipelines-generator) |
| [azure-pipelines-validator](skills/ci-cd/azure-pipelines/validator/SKILL.md) | Validates, lints, and security-scans Azure DevOps Pipeline configurations (az... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/azure-pipelines-validator) |
| [fluentbit-generator](skills/ci-cd/fluentbit/generator/SKILL.md) | Generates, validates, and optimizes Fluent Bit configurations for production ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/fluentbit/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/fluentbit-generator) |
| [fluentbit-validator](skills/ci-cd/fluentbit/validator/SKILL.md) | Validates syntax, checks pipeline tag connections, detects security misconfig... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/fluentbit/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/fluentbit-validator) |
| [github-actions-generator](skills/ci-cd/github-actions/generator/SKILL.md) | Generates production-ready GitHub Actions workflows, custom actions, and CI/C... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/github-actions-generator) |
| [github-actions-validator](skills/ci-cd/github-actions/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and testing GitHub Actions wor... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/github-actions-validator) |
| [gitlab-ci-generator](skills/ci-cd/gitlab-ci/generator/SKILL.md) | Creates .gitlab-ci.yml files, configures pipeline stages, defines CI jobs and... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/gitlab-ci-generator) |
| [gitlab-ci-validator](skills/ci-cd/gitlab-ci/validator/SKILL.md) | Validates .gitlab-ci.yml syntax, detects security misconfigurations in job de... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/gitlab-ci-validator) |
| [helm-generator](skills/ci-cd/helm/generator/SKILL.md) | Comprehensive toolkit for generating best practice Helm charts and resources ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/helm/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/helm-generator) |
| [helm-validator](skills/ci-cd/helm/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and analyzing Helm ch... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/helm/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/helm-validator) |
| [jenkinsfile-generator](skills/ci-cd/jenkinsfile/generator/SKILL.md) | Generates Jenkinsfiles with stages, agents, parallel builds, post-build actio... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/jenkinsfile-generator) |
| [jenkinsfile-validator](skills/ci-cd/jenkinsfile/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Jenkin... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/jenkinsfile-validator) |

## Infrastructure (13 skills)

Infrastructure as Code

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [ansible-generator](skills/infrastructure/ansible/generator/SKILL.md) | Generates, validates, and refactors production-ready Ansible playbooks, roles... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/ansible/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/ansible-generator) |
| [ansible-validator](skills/infrastructure/ansible/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Ansibl... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/ansible/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/ansible-validator) |
| [aws-cdk-cdk-nag](skills/infrastructure/aws-cdk/cdk-nag/SKILL.md) | Enforce AWS CDK security and compliance controls with cdk-nag. Use when addin... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/aws-cdk/cdk-nag/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/cdk-nag) |
| [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator/SKILL.md) | Creates test stacks, analyzes CloudFormation events, and compares actual vs d... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/cfn/behavior-validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/cfn-behavior-validator) |
| [cfn-template-compare](skills/infrastructure/cfn/template-compare/SKILL.md) | Compares deployed CloudFormation templates with locally synthesized CDK templ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/infrastructure/cfn/template-compare/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/cfn-template-compare) |
| [dockerfile-generator](skills/infrastructure/dockerfile/generator/SKILL.md) | Generates production-ready Dockerfiles with multi-stage builds, layer caching... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/dockerfile/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/dockerfile-generator) |
| [dockerfile-validator](skills/infrastructure/dockerfile/validator/SKILL.md) | Validates, lints, and secures Dockerfiles by running syntax checking, detecti... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/dockerfile/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/dockerfile-validator) |
| [k8s-yaml-generator](skills/infrastructure/k8s-yaml/generator/SKILL.md) | Comprehensive toolkit for generating, validating, and managing Kubernetes YAM... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/k8s-yaml-generator) |
| [k8s-yaml-validator](skills/infrastructure/k8s-yaml/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and testing Kubernetes YAML re... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/k8s-yaml-validator) |
| [terraform-generator](skills/infrastructure/terraform/generator/SKILL.md) | Generate Terraform modules, configure providers, define variables and outputs... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terraform/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/terraform-generator) |
| [terraform-validator](skills/infrastructure/terraform/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Terraf... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/terraform/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/terraform-validator) |
| [terragrunt-generator](skills/infrastructure/terragrunt/generator/SKILL.md) | Comprehensive toolkit for generating best-practice Terragrunt configurations ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/terragrunt-generator) |
| [terragrunt-validator](skills/infrastructure/terragrunt/validator/SKILL.md) | Comprehensive toolkit for validating, linting, testing, and automating Terrag... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/terragrunt-validator) |

## Repository Management (7 skills)

Repository & workspace management

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [nx-biome-integration](skills/repository-mgmt/nx/biome-integration/SKILL.md) | Integrate Biome into Nx monorepos with deterministic setup, caching, migratio... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/biome-integration/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-biome-integration) |
| [nx-bun-integration](skills/repository-mgmt/nx/bun-integration/SKILL.md) | Integrate Bun runtime into Nx monorepos with deterministic plugin setup, exec... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/bun-integration/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-bun-integration) |
| [nx-executors](skills/repository-mgmt/nx/executors/SKILL.md) | Create and operate custom Nx executors in TypeScript monorepos with determini... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/executors/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-executors) |
| [nx-extending-plugins](skills/repository-mgmt/nx/extending-plugins/SKILL.md) | Creates Nx plugins, builds custom generators, configures inferred tasks, and ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/repository-mgmt/nx/extending-plugins/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/extending-nx-plugins) |
| [nx-generators](skills/repository-mgmt/nx/generators/SKILL.md) | Create Nx generators for TypeScript monorepos with deterministic Tree API usa... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/generators/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-generators) |
| [nx-vite-integration](skills/repository-mgmt/nx/vite-integration/SKILL.md) | Configure and integrate Vite in Nx monorepos for applications and libraries. ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/repository-mgmt/nx/vite-integration/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-vite-integration) |
| [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns/SKILL.md) | Configure and optimize Nx monorepo workspaces with deterministic project-grap... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/workspace-patterns/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/nx-workspace-patterns) |

## Development (8 skills)

Development tooling

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [biome-complete](skills/development/biome-complete/SKILL.md) | Complete Biome toolchain guidance for real repository workflows. Use when use... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/development/biome-complete/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/biome-complete) |
| [bun-development](skills/development/bun-development/SKILL.md) | Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package manage... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/bun-development/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/bun-development) |
| [commanderjs](skills/development/commanderjs/SKILL.md) | Complete Commander.js CLI framework guidance covering command structure, opti... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/commanderjs/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/commanderjs) |
| [scripting-bash-script-generator](skills/development/scripting/bash-script/generator/SKILL.md) | Creates bash scripts with argument parsing, error handling, logging, and inpu... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/bash-script-generator) |
| [scripting-bash-script-validator](skills/development/scripting/bash-script/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and optimizing bash and shell ... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/bash-script-validator) |
| [scripting-makefile-generator](skills/development/scripting/makefile/generator/SKILL.md) | Generate GNU Make build systems that define build targets, configure dependen... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/scripting/makefile/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/makefile-generator) |
| [scripting-makefile-validator](skills/development/scripting/makefile/validator/SKILL.md) | Comprehensive toolkit for validating, linting, and optimizing Makefiles. Use ... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/scripting/makefile/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/makefile-validator) |
| [typescript-advanced](skills/development/typescript-advanced/SKILL.md) | Comprehensive TypeScript guidance covering compiler configuration, advanced t... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/typescript-advanced/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/typescript-advanced) |

## Agentic Harness (4 skills)

Agent framework configurations

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [agents-md](skills/agentic-harness/agents-md/SKILL.md) | Create and maintain AGENTS.md documentation for simple projects and complex m... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/agents-md/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/agents-md) |
| [opencode](skills/agentic-harness/opencode/SKILL.md) | Configure OpenCode via opencode.json and AGENTS.md with deterministic provide... | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/opencode/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/opencode-config) |
| [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor/SKILL.md) | Audit and improve skill collections with 8-dimension scoring framework (Knowl... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/agentic-harness/skill-quality-auditor/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/skill-quality-auditor) |
| [tessl-publish-public](skills/agentic-harness/tessl/publish-public/SKILL.md) | Ensure Tessl tiles meet all requirements for public registry publication with... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-03](.context/audits/agentic-harness/tessl/publish-public/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/tessl-publish-public) |

## Testing (3 skills)

Testing methodologies & quality

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [bdd-testing](skills/testing/bdd-testing/SKILL.md) | Write and maintain Behavior-Driven Development tests with Gherkin and Cucumbe... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/bdd-testing/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/bdd-testing) |
| [test-driven-development](skills/testing/test-driven-development/SKILL.md) | Guides TDD (test-driven development) with red-green-refactor workflows, test-... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/test-driven-development/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/test-driven-development) |
| [ui-debug-workflow](skills/testing/ui-debug-workflow/SKILL.md) | Debug UI changes with a repeatable evidence-first workflow. Use when validati... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/ui-debug-workflow/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/ui-debug-workflow) |

## Software Engineering (1 skills)

Software engineering principles

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [software-design-principles](skills/software-engineering/software-design-principles/SKILL.md) | Apply SOLID principles, detect design anti-patterns, and evaluate architectur... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/software-engineering/software-design-principles/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/software-design-principles) |

## Observability (4 skills)

Monitoring, logging & debugging

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [k8s-debug](skills/observability/k8s-debug/SKILL.md) | Inspect pod logs, analyze resource quotas, trace network policies, check depl... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/k8s-debug/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/k8s-debug) |
| [logql-generator](skills/observability/logql-generator/SKILL.md) | Generate label matchers, line filters, log aggregations, and metric queries i... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/logql-generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/logql-generator) |
| [promql-generator](skills/observability/promql/generator/SKILL.md) | Generate PromQL queries for calculating error rates, aggregating metrics acro... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/promql/generator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/promql-generator) |
| [promql-validator](skills/observability/promql/validator/SKILL.md) | Comprehensive toolkit for validating, optimizing, and understanding Prometheu... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/observability/promql/validator/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/promql-validator) |

## Documentation (5 skills)

Writing & communication

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [acceptance-criteria](skills/documentation/acceptance-criteria/SKILL.md) | Write clear, testable acceptance criteria for user stories and feature delive... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/acceptance-criteria/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/acceptance-criteria) |
| [conventional-commits](skills/documentation/conventional-commits/SKILL.md) | Generates and formats git commit messages following the Conventional Commits ... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/documentation/conventional-commits/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/conventional-commits) |
| [journal-entry-creator](skills/documentation/journal-entry-creator/SKILL.md) | Create structured journal entries with YAML frontmatter, template-based secti... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/documentation/journal-entry-creator/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/journal-entry-creator) |
| [markdown-authoring](skills/documentation/markdown-authoring/SKILL.md) | Author high-quality Markdown documentation with deterministic structure, lint... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/markdown-authoring/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/markdown-authoring) |
| [plain-english](skills/documentation/plain-english/SKILL.md) | Translates technical content into plain English for non-technical stakeholder... | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/plain-english/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/plain-english) |

## Package Management (1 skills)

Package & version management

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [mise-complete](skills/package-mgmt/mise-complete/SKILL.md) | Configure and operate Mise for deterministic developer environments. Use when... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/package-mgmt/mise-complete/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/mise-complete) |

## Project Management (3 skills)

Planning & organization

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [create-context-file](skills/project-mgmt/create-context-file/SKILL.md) | Creates structured context files (plans, justifications, scratches) in the .c... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/project-mgmt/create-context-file/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/create-context-file) |
| [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter/SKILL.md) | Split large implementation plan documents into digestible, hierarchical struc... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/implementation-plan-splitter/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/implementation-plan-splitter) |
| [moscow-prioritization](skills/project-mgmt/moscow-prioritization/SKILL.md) | Categorize requirements into Must/Should/Could/Won't tiers, generate priority... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/moscow-prioritization/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/moscow-prioritization) |

## Specialized (3 skills)

Domain-specific tools

| Skill | Description | Rating | Audit | Tessl |
| --- | --- | --- | --- | --- |
| [colyseus-multiplayer](skills/specialized/colyseus-multiplayer/SKILL.md) | Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use wh... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/specialized/colyseus-multiplayer/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/colyseus-multiplayer) |
| [github-copilot-models](skills/specialized/github-copilot-models/SKILL.md) | Query and display available GitHub Copilot AI models with their capabilities,... | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/specialized/github-copilot-models/2026-03-03/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/github-copilot-models) |
| [gitlab-api](skills/specialized/gitlab-api/SKILL.md) | Fetches and analyzes GitLab merge request (MR) comments, metadata, and review... | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/specialized/gitlab-api/2026-03-02/audit.json) | [Public](https://tessl.io/registry/skills/pantheon-ai/gitlab-api) |

## What are Agent Skills?
