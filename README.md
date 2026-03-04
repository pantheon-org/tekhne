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

- **description**: Complete azure-pipelines toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/azure-pipelines-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [azure-pipelines-generator](skills/ci-cd/azure-pipelines/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/generator/2026-03-02/audit.json) | - |
| [azure-pipelines-validator](skills/ci-cd/azure-pipelines/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/validator/2026-03-02/audit.json) | - |

### [gitlab-ci-toolkit](skills/ci-cd/gitlab-ci)

- **description**: Complete GitLab CI/CD toolkit with generation and validation capabilities for pipelines and configurations
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/gitlab-ci-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [gitlab-ci-generator](skills/ci-cd/gitlab-ci/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/generator/2026-03-02/audit.json) | - |
| [gitlab-ci-validator](skills/ci-cd/gitlab-ci/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/validator/2026-03-02/audit.json) | - |

### [fluentbit-toolkit](skills/ci-cd/fluentbit)

- **description**: Complete fluentbit toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/fluentbit-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [fluentbit-generator](skills/ci-cd/fluentbit/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/fluentbit/generator/2026-03-02/audit.json) | - |
| [fluentbit-validator](skills/ci-cd/fluentbit/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/fluentbit/validator/2026-03-02/audit.json) | - |

### [jenkinsfile-toolkit](skills/ci-cd/jenkinsfile)

- **description**: Complete jenkinsfile toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/jenkinsfile-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [jenkinsfile-generator](skills/ci-cd/jenkinsfile/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/generator/2026-03-02/audit.json) | - |
| [jenkinsfile-validator](skills/ci-cd/jenkinsfile/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/validator/2026-03-02/audit.json) | - |

### [helm-toolkit](skills/ci-cd/helm)

- **description**: Complete helm toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/helm-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [helm-generator](skills/ci-cd/helm/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/helm/generator/2026-03-02/audit.json) | - |
| [helm-validator](skills/ci-cd/helm/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/helm/validator/2026-03-02/audit.json) | - |

### [github-actions-toolkit](skills/ci-cd/github-actions)

- **description**: Complete GitHub Actions toolkit with generation and validation capabilities for workflows, custom actions, and CI/CD configurations
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/github-actions-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [github-actions-generator](skills/ci-cd/github-actions/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/generator/2026-03-02/audit.json) | - |
| [github-actions-validator](skills/ci-cd/github-actions/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/validator/2026-03-02/audit.json) | - |

## Infrastructure (8 tiles)

Infrastructure as Code

### [terraform-toolkit](skills/infrastructure/terraform)

- **description**: Complete terraform toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/terraform-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [terraform-generator](skills/infrastructure/terraform/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terraform/generator/2026-03-02/audit.json) | - |
| [terraform-validator](skills/infrastructure/terraform/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/terraform/validator/2026-03-02/audit.json) | - |

### [dockerfile-toolkit](skills/infrastructure/dockerfile)

- **description**: Complete dockerfile toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/dockerfile-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [dockerfile-generator](skills/infrastructure/dockerfile/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/dockerfile/generator/2026-03-02/audit.json) | - |
| [dockerfile-validator](skills/infrastructure/dockerfile/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/dockerfile/validator/2026-03-02/audit.json) | - |

### [terragrunt-toolkit](skills/infrastructure/terragrunt)

- **description**: Complete terragrunt toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/terragrunt-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [terragrunt-generator](skills/infrastructure/terragrunt/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/generator/2026-03-02/audit.json) | - |
| [terragrunt-validator](skills/infrastructure/terragrunt/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/validator/2026-03-02/audit.json) | - |

### [k8s-yaml-toolkit](skills/infrastructure/k8s-yaml)

- **description**: Complete k8s-yaml toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/k8s-yaml-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [k8s-yaml-generator](skills/infrastructure/k8s-yaml/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/generator/2026-03-02/audit.json) | - |
| [k8s-yaml-validator](skills/infrastructure/k8s-yaml/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/k8s-yaml/validator/2026-03-02/audit.json) | - |

### [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator)

- **description**: Creates test stacks, analyzes CloudFormation events, and compares actual vs documented update behavior to validate whether resource property changes trigger replacement or in-place updates. Use when: a user wants to test if a CFN property change causes resource replacement; when investigating stack update behavior or "Update requires" documentation accuracy; when validating whether a workaround (e.g. hash-based logical IDs) is actually necessary; when questioning UpdateRequiresReplacement behavior for immutable properties; when empirical evidence is needed before an architectural decision involving CDK or CloudFormation stack updates.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-behavior-validator)
- **version**: 0.1.2

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/cfn/behavior-validator/2026-03-02/audit.json) | 5 |

### [cfn-template-compare](skills/infrastructure/cfn/template-compare)

- **description**: Compares deployed CloudFormation templates with locally synthesized CDK templates to detect drift, validate changes, and ensure consistency before deployment. Use when the user wants to compare CDK output with a deployed stack, check for infrastructure drift, run a pre-deployment validation, audit IAM or security changes, investigate a failing deployment, or perform a 'cdk diff'-style review. Triggered by phrases like 'compare templates', 'check for drift', 'cfn drift', 'stack comparison', 'infrastructure drift detection', 'safe to deploy', or 'what changed in my CDK stack'.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-template-compare)
- **version**: 0.2.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [cfn-template-compare](skills/infrastructure/cfn/template-compare/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/infrastructure/cfn/template-compare/2026-03-03/audit.json) | 5 |

### [ansible-toolkit](skills/infrastructure/ansible)

- **description**: Complete ansible toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/ansible-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [ansible-generator](skills/infrastructure/ansible/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/ansible/generator/2026-03-02/audit.json) | - |
| [ansible-validator](skills/infrastructure/ansible/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/ansible/validator/2026-03-02/audit.json) | - |

### [cdk-nag](skills/infrastructure/aws-cdk/cdk-nag)

- **description**: Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding rule packs, triaging findings, writing justified suppressions, integrating checks in CI/CD, or preventing insecure infrastructure patterns in CDK stacks.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cdk-nag)
- **version**: 0.1.2

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [cdk-nag](skills/infrastructure/aws-cdk/cdk-nag/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/aws-cdk/cdk-nag/2026-03-02/audit.json) | - |

## Repository Management (5 tiles)

Repository & workspace management

### [nx-biome-integration](skills/repository-mgmt/nx/biome-integration)

- **description**: Integrate Biome into Nx monorepos with deterministic setup, caching, migration from ESLint and Prettier, and plugin-based inferred tasks; use when adding Biome, replacing ESLint/Prettier, tuning cache inputs, or scaling lint and format workflows across projects.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-biome-integration)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-biome-integration](skills/repository-mgmt/nx/biome-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/biome-integration/2026-03-02/audit.json) | - |

### [nx-bun-integration](skills/repository-mgmt/nx/bun-integration)

- **description**: Integrate Bun runtime into Nx monorepos with deterministic plugin setup, executor configuration, migration from Node.js toolchains, and cache-aware build/test workflows; use when adding the nx-bun plugin, converting projects, or standardizing Bun targets across Nx workspaces.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-bun-integration)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-bun-integration](skills/repository-mgmt/nx/bun-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/bun-integration/2026-03-02/audit.json) | - |

### [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns)

- **description**: Configure and optimize Nx monorepo workspaces with deterministic project-graph structure, boundary enforcement, cache-aware pipelines, and affected-command CI workflows; use when designing workspace architecture, tightening dependency rules, or reducing CI cost through Nx task orchestration.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-workspace-patterns)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/workspace-patterns/2026-03-02/audit.json) | - |

### [nx-plugin-toolkit](skills/repository-mgmt/nx)

- **description**: Complete Nx plugin development toolkit: create generators, executors, and extend Nx workspaces
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-plugin-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-generators](skills/repository-mgmt/nx/generators/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/generators/2026-03-02/audit.json) | - |
| [nx-executors](skills/repository-mgmt/nx/executors/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/executors/2026-03-02/audit.json) | - |
| [extending-nx-plugins](skills/repository-mgmt/nx/extending-plugins/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/repository-mgmt/nx/extending-plugins/2026-03-03/audit.json) | - |

### [nx-vite-integration](skills/repository-mgmt/nx/vite-integration)

- **description**: Configure and integrate Vite in Nx monorepos for applications and libraries. Covers vite.config.ts setup, framework plugins, TypeScript path resolution, asset copying, library mode builds, and Vitest integration.  Use when: adding Vite to an Nx project, migrating from Webpack, configuring Vitest, fixing tsconfig path resolution, or setting up library mode.  Triggers: "add vite", "nx vite", "vite setup", "vite.config.ts", "vitest config", "library mode", "nxViteTsPaths", "copy assets", "vite path aliases", "migrate webpack to vite"  Examples: - user: "Add Vite to this Nx app" -> install plugin and configure vite.config.ts - user: "Vitest is failing in Nx" -> fix test config and cache/coverage paths - user: "Path aliases break in Vite" -> add nxViteTsPaths plugin - user: "Set up Vite for my Nx library" -> configure lib mode + dts + externals
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-vite-integration)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-vite-integration](skills/repository-mgmt/nx/vite-integration/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/repository-mgmt/nx/vite-integration/2026-03-02/audit.json) | - |

## Development (6 tiles)

Development tooling

### [makefile-toolkit](skills/development/scripting/makefile)

- **description**: Complete makefile toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/makefile-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [makefile-generator](skills/development/scripting/makefile/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/scripting/makefile/generator/2026-03-02/audit.json) | - |
| [makefile-validator](skills/development/scripting/makefile/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/scripting/makefile/validator/2026-03-02/audit.json) | - |

### [bash-script-toolkit](skills/development/scripting/bash-script)

- **description**: Complete bash-script toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bash-script-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [bash-script-generator](skills/development/scripting/bash-script/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/generator/2026-03-02/audit.json) | - |
| [bash-script-validator](skills/development/scripting/bash-script/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/validator/2026-03-02/audit.json) | - |

### [commanderjs](skills/development/commanderjs)

- **description**: Complete Commander.js CLI framework guidance covering command structure, options, arguments, subcommands, action handlers, version management, and TypeScript integration. Use when: building CLI tools, parsing command-line arguments, implementing subcommands, handling options/flags, creating interactive CLIs, or migrating from other CLI frameworks.  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands, action handlers, version, help text, TypeScript, yargs, meow, program, parseAsync, opts, args, variadic, required options, default values, custom help, error handling
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/commanderjs)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [commanderjs](skills/development/commanderjs/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/commanderjs/2026-03-02/audit.json) | - |

### [bun-development](skills/development/bun-development)

- **description**: Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package management, testing, SQLite, and security; use proactively when setting up Bun projects, replacing Node.js APIs with Bun-native APIs, writing bun test suites, implementing Bun.serve services, using bun:sqlite with prepared statements, configuring workspaces and lockfiles, hardening shell and SQL boundaries, or optimizing Bun performance and migration workflows.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bun-development)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [bun-development](skills/development/bun-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/bun-development/2026-03-02/audit.json) | - |

### [biome-complete](skills/development/biome-complete)

- **description**: Complete Biome toolchain guidance for real repository workflows. Use when users ask to configure biome.json, run lint or format commands, migrate from ESLint or Prettier, tune rule severity, fix formatter drift, or replace mixed ESLint+Prettier pipelines with Biome-only workflows.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/biome-complete)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [biome-complete](skills/development/biome-complete/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/development/biome-complete/2026-03-02/audit.json) | - |

### [typescript-advanced](skills/development/typescript-advanced)

- **description**: Comprehensive TypeScript guidance covering compiler configuration, advanced types, utility types, type guards, strict mode workflows, and documentation patterns; use when configuring tsconfig, designing complex generics, making illegal states unrepresentable, fixing type errors, or writing testable and maintainable type-safe APIs.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/typescript-advanced)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [typescript-advanced](skills/development/typescript-advanced/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/typescript-advanced/2026-03-02/audit.json) | - |

## Agentic Harness (4 tiles)

Agent framework configurations

### [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor)

- **description**: Audit and improve skill collections with a 9-dimension scoring framework (Knowledge Delta, Mindset, Anti-Patterns, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability, Eval Validation), duplication detection, remediation planning, baseline comparison, and CI quality gates; use when evaluating skill quality, generating remediation plans, detecting duplicates, validating artifact conventions, or enforcing publication thresholds.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/skill-quality-auditor)
- **version**: 0.1.4

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-04](.context/audits/agentic-harness/skill-quality-auditor/2026-03-04/audit.json) | 5 |

### [agents-md](skills/agentic-harness/agents-md)

- **description**: Create and maintain AGENTS.md documentation for simple projects and complex monorepos with deterministic discovery, scoped instruction files, and low-token navigation patterns; use when generating AGENTS.md, updating agent docs, or standardizing AI-facing project guidance.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/agents-md)
- **version**: 0.1.3

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [agents-md](skills/agentic-harness/agents-md/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/agents-md/2026-03-02/audit.json) | 5 |

### [tessl-publish-public](skills/agentic-harness/tessl/publish-public)

- **description**: Ensure Tessl tiles meet all requirements for public registry publishing with comprehensive validation, quality gates, and evaluation scenarios. Use when preparing skills for public Tessl release, validating tile.json configuration, creating evaluation scenarios, enforcing quality thresholds, or checking agent-agnostic compliance. Keywords: tessl, tile, publishing, public-registry, validation, quality-gates, tile.json, evaluation-scenarios, skill-publishing
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/tessl-publish-public)
- **version**: 1.0.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [tessl-publish-public](skills/agentic-harness/tessl/publish-public/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-03](.context/audits/agentic-harness/tessl/publish-public/2026-03-03/audit.json) | 9 |

### [opencode-config](skills/agentic-harness/opencode)

- **description**: Configure OpenCode via opencode.json and AGENTS.md with deterministic provider setup, model selection, permission policies, formatter behavior, and environment variable handling; use when editing opencode configuration, setting model/provider defaults, tightening agent permissions, or troubleshooting OpenCode config behavior.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/opencode-config)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [opencode-config](skills/agentic-harness/opencode/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/opencode/2026-03-02/audit.json) | - |

## Testing (3 tiles)

Testing methodologies & quality

### [test-driven-development](skills/testing/test-driven-development)

- **description**: Master Test-Driven Development with deterministic red-green-refactor workflows, test-first feature delivery, bug reproduction through failing tests, behavior-focused assertions, and refactoring safety; use when implementing new functions, changing APIs, fixing regressions, or restructuring code under test.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/test-driven-development)
- **version**: 0.2.4

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [test-driven-development](skills/testing/test-driven-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/test-driven-development/2026-03-02/audit.json) | 5 |

### [bdd-testing](skills/testing/bdd-testing)

- **description**: Write and maintain Behavior-Driven Development tests with Gherkin and Cucumber. Use when defining acceptance scenarios, writing feature files, implementing step definitions, running Three Amigos sessions, or diagnosing BDD test quality issues. Keywords: bdd, gherkin, cucumber, given when then, feature files, step definitions, acceptance criteria, three amigos, example mapping.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bdd-testing)
- **version**: 0.2.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [bdd-testing](skills/testing/bdd-testing/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/bdd-testing/2026-03-02/audit.json) | 8 |

### [ui-debug-workflow](skills/testing/ui-debug-workflow)

- **description**: Debug UI changes with a repeatable evidence-first workflow. Use when validating visual regressions, reproducing frontend bugs, comparing baseline vs changed behavior, collecting screenshots/DOM/logs, or producing stakeholder-ready UI debug reports. Keywords: ui bug, visual regression, browser devtools, playwright, screenshot evidence, dom snapshot, frontend debugging.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/ui-debug-workflow)
- **version**: 0.1.2

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [ui-debug-workflow](skills/testing/ui-debug-workflow/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/ui-debug-workflow/2026-03-02/audit.json) | - |

## Software Engineering (1 tile)

Software engineering principles

### [software-design-principles](skills/software-engineering/software-design-principles)

- **description**: Apply software design principles across architecture and implementation using deterministic decision workflows, SOLID checks, structural patterns, and anti-pattern detection; use when reviewing designs, refactoring modules, or resolving maintainability and coupling risks.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/software-design-principles)
- **version**: 0.1.4

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [software-design-principles](skills/software-engineering/software-design-principles/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/software-engineering/software-design-principles/2026-03-02/audit.json) | 7 |

## Observability (3 tiles)

Monitoring, logging & debugging

### [logql-generator](skills/observability/logql-generator)

- **description**: Generate label matchers, line filters, log aggregations, and metric queries in LogQL (Loki Query Language) following current standards and conventions. Use this skill when creating new LogQL queries, implementing log analysis dashboards, alerting rules, or troubleshooting with Loki.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/logql-generator)
- **version**: 0.1.4

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [logql-generator](skills/observability/logql-generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/logql-generator/2026-03-02/audit.json) | - |

### [k8s-debug](skills/observability/k8s-debug)

- **description**: Comprehensive Kubernetes debugging and troubleshooting toolkit. Use this skill when diagnosing Kubernetes cluster issues, debugging failing pods, investigating network connectivity problems, analyzing resource usage, troubleshooting deployments, or performing cluster health checks.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/k8s-debug)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [k8s-debug](skills/observability/k8s-debug/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/k8s-debug/2026-03-02/audit.json) | - |

### [promql-toolkit](skills/observability/promql)

- **description**: Complete PromQL toolkit with generation and validation capabilities
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/promql-toolkit)
- **version**: 0.1.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [promql-generator](skills/observability/promql/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/promql/generator/2026-03-02/audit.json) | - |
| [promql-validator](skills/observability/promql/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/observability/promql/validator/2026-03-02/audit.json) | - |

## Documentation (5 tiles)

Writing & communication

### [markdown-authoring](skills/documentation/markdown-authoring)

- **description**: Author high-quality Markdown documentation with deterministic structure, lint compliance, and CI integration. Use when writing README files, creating docs pages, fixing markdownlint failures, defining style rules, or wiring markdown checks into pre-commit and pipelines. Keywords: markdown, markdownlint, readme, docs, headings, lists, code fences, links, images, lint config, ci, documentation style.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/markdown-authoring)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [markdown-authoring](skills/documentation/markdown-authoring/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/markdown-authoring/2026-03-02/audit.json) | - |

### [plain-english](skills/documentation/plain-english)

- **description**: Write technical content in plain English for non-technical stakeholders by translating jargon into business language, surfacing decisions and impact early, and producing actionable recommendations with clear ownership and timeline.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/plain-english)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [plain-english](skills/documentation/plain-english/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/plain-english/2026-03-02/audit.json) | - |

### [journal-entry-creator](skills/documentation/journal-entry-creator)

- **description**: Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]', 'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports four entry types: general journal entries, troubleshooting sessions, learning notes, and article summaries. Keywords: journal, documentation, troubleshooting, learning, article-summary, YAML frontmatter, template schemas, validation.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/journal-entry-creator)
- **version**: 0.2.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [journal-entry-creator](skills/documentation/journal-entry-creator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/documentation/journal-entry-creator/2026-03-03/audit.json) | 5 |

### [acceptance-criteria](skills/documentation/acceptance-criteria)

- **description**: Write clear, testable acceptance criteria for user stories and feature delivery; use when defining done conditions, creating measurable requirements, applying INVEST checks, documenting negative scenarios, and aligning product, engineering, and QA on expected outcomes.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/acceptance-criteria)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [acceptance-criteria](skills/documentation/acceptance-criteria/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/acceptance-criteria/2026-03-02/audit.json) | 5 |

### [conventional-commits](skills/documentation/conventional-commits)

- **description**: Skill for creating structured, semantic commit messages following the Conventional Commits specification
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/conventional-commits)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [conventional-commits](skills/documentation/conventional-commits/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/documentation/conventional-commits/2026-03-02/audit.json) | - |

## Package Management (1 tile)

Package & version management

### [mise-complete](skills/package-mgmt/mise-complete)

- **description**: Configure and operate Mise for deterministic developer environments. Use when installing runtime/tool versions, defining reusable tasks, managing layered environment variables, migrating from asdf/nvm/pyenv, or debugging mise.toml behavior in CI and local shells. Keywords: mise, mise.toml, tool versions, tasks, env, asdf migration, setup automation, dev environment.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/mise-complete)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [mise-complete](skills/package-mgmt/mise-complete/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/package-mgmt/mise-complete/2026-03-02/audit.json) | - |

## Project Management (3 tiles)

Planning & organization

### [create-context-file](skills/project-mgmt/create-context-file)

- **description**: Create context files (plans, justifications, scratches) in .context/ directory with unique three-word IDs and frontmatter
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/create-context-file)
- **version**: 0.2.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [create-context-file](skills/project-mgmt/create-context-file/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/project-mgmt/create-context-file/2026-03-03/audit.json) | - |

### [moscow-prioritization](skills/project-mgmt/moscow-prioritization)

- **description**: Prioritize product requirements with the MoSCoW framework in a deterministic way.  Use when teams need to define MVP scope, sequence releases, resolve stakeholder conflicts,  prevent scope creep, or rebalance backlog under time, budget, or staffing constraints.  Keywords: moscow, must should could wont, requirement prioritization, backlog, mvp,  release planning, scope control, stakeholder alignment.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/moscow-prioritization)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [moscow-prioritization](skills/project-mgmt/moscow-prioritization/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/moscow-prioritization/2026-03-02/audit.json) | - |

### [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter)

- **description**: Split large implementation plan documents into digestible, hierarchical structures with descriptive names. Use when refactoring monolithic planning docs, organizing phase documentation, or creating contributor-friendly task breakdowns. Triggers: "split this plan", "organize phases", "break down implementation docs", "create task hierarchy".
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/implementation-plan-splitter)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/implementation-plan-splitter/2026-03-02/audit.json) | - |

## Specialized (3 tiles)

Domain-specific tools

### [github-copilot-models](skills/specialized/github-copilot-models)

- **description**: Query and display available GitHub Copilot AI models with their capabilities, context limits, and features. Use when: "what models are available", "show copilot models", "list github models", "check model capabilities", "switch models".  Examples: - user: "What models can I use with GitHub Copilot?" → fetch and display available models - user: "Show me models with vision support" → filter models by capability - user: "Which model has the largest context window?" → compare model specifications - user: "List all GPT-5 models" → filter by model family
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/github-copilot-models)
- **version**: 0.2.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [github-copilot-models](skills/specialized/github-copilot-models/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/specialized/github-copilot-models/2026-03-03/audit.json) | - |

### [gitlab-api](skills/specialized/gitlab-api)

- **description**: Retrieve and analyze GitLab merge request comments and metadata using authenticated API calls
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/gitlab-api)
- **version**: 1.3.0

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [gitlab-api](skills/specialized/gitlab-api/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/specialized/gitlab-api/2026-03-02/audit.json) | 5 |

### [colyseus-multiplayer](skills/specialized/colyseus-multiplayer)

- **description**: Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when implementing rooms, schema state sync, client message validation, matchmaking, authentication, reconnection handling, or server-side anti-cheat constraints. Keywords: colyseus, room lifecycle, schema, multiplayer, websocket, matchmaking, onJoin, onLeave, onDrop, allowReconnection.
- **published**: [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/colyseus-multiplayer)
- **version**: 0.1.1

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [colyseus-multiplayer](skills/specialized/colyseus-multiplayer/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/specialized/colyseus-multiplayer/2026-03-02/audit.json) | - |

## What are Agent Skills?
