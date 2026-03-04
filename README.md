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

Complete GitLab CI/CD toolkit with generation and validation capabilities for pipelines and configurations · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/gitlab-ci-toolkit)

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

Complete GitHub Actions toolkit with generation and validation capabilities for workflows, custom actions, and CI/CD configurations · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/github-actions-toolkit)

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

Creates test stacks, analyzes CloudFormation events, and compares actual vs documented update behavior to validate whether resource property changes trigger replacement or in-place updates. Use when: a user wants to test if a CFN property change causes resource replacement; when investigating stack update behavior or "Update requires" documentation accuracy; when validating whether a workaround (e.g. hash-based logical IDs) is actually necessary; when questioning UpdateRequiresReplacement behavior for immutable properties; when empirical evidence is needed before an architectural decision involving CDK or CloudFormation stack updates. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-behavior-validator)

| Skill | Rating | Audit |
| --- | --- | --- |
| [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/cfn/behavior-validator/2026-03-02/audit.json) |

### [cfn-template-compare](skills/infrastructure/cfn/template-compare)

Compares deployed CloudFormation templates with locally synthesized CDK templates to detect drift, validate changes, and ensure consistency before deployment. Use when the user wants to compare CDK output with a deployed stack, check for infrastructure drift, run a pre-deployment validation, audit IAM or security changes, investigate a failing deployment, or perform a 'cdk diff'-style review. Triggered by phrases like 'compare templates', 'check for drift', 'cfn drift', 'stack comparison', 'infrastructure drift detection', 'safe to deploy', or 'what changed in my CDK stack'. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cfn-template-compare)

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

Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding rule packs, triaging findings, writing justified suppressions, integrating checks in CI/CD, or preventing insecure infrastructure patterns in CDK stacks. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/cdk-nag)

| Skill | Rating | Audit |
| --- | --- | --- |
| [cdk-nag](skills/infrastructure/aws-cdk/cdk-nag/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/aws-cdk/cdk-nag/2026-03-02/audit.json) |

## Repository Management (5 tiles)

Repository & workspace management

### [nx-biome-integration](skills/repository-mgmt/nx/biome-integration)

Integrate Biome into Nx monorepos with deterministic setup, caching, migration from ESLint and Prettier, and plugin-based inferred tasks; use when adding Biome, replacing ESLint/Prettier, tuning cache inputs, or scaling lint and format workflows across projects. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-biome-integration)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-biome-integration](skills/repository-mgmt/nx/biome-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/biome-integration/2026-03-02/audit.json) |

### [nx-bun-integration](skills/repository-mgmt/nx/bun-integration)

Integrate Bun runtime into Nx monorepos with deterministic plugin setup, executor configuration, migration from Node.js toolchains, and cache-aware build/test workflows; use when adding the nx-bun plugin, converting projects, or standardizing Bun targets across Nx workspaces. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-bun-integration)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-bun-integration](skills/repository-mgmt/nx/bun-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/bun-integration/2026-03-02/audit.json) |

### [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns)

Configure and optimize Nx monorepo workspaces with deterministic project-graph structure, boundary enforcement, cache-aware pipelines, and affected-command CI workflows; use when designing workspace architecture, tightening dependency rules, or reducing CI cost through Nx task orchestration. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-workspace-patterns)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/workspace-patterns/2026-03-02/audit.json) |

### [nx-plugin-toolkit](skills/repository-mgmt/nx)

Complete Nx plugin development toolkit: create generators, executors, and extend Nx workspaces · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-plugin-toolkit)

| Skill | Rating | Audit |
| --- | --- | --- |
| [nx-generators](skills/repository-mgmt/nx/generators/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/generators/2026-03-02/audit.json) |
| [nx-executors](skills/repository-mgmt/nx/executors/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/repository-mgmt/nx/executors/2026-03-02/audit.json) |
| [extending-nx-plugins](skills/repository-mgmt/nx/extending-plugins/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/repository-mgmt/nx/extending-plugins/2026-03-03/audit.json) |

### [nx-vite-integration](skills/repository-mgmt/nx/vite-integration)

Configure and integrate Vite in Nx monorepos for applications and libraries. Covers vite.config.ts setup, framework plugins, TypeScript path resolution, asset copying, library mode builds, and Vitest integration.  Use when: adding Vite to an Nx project, migrating from Webpack, configuring Vitest, fixing tsconfig path resolution, or setting up library mode.  Triggers: "add vite", "nx vite", "vite setup", "vite.config.ts", "vitest config", "library mode", "nxViteTsPaths", "copy assets", "vite path aliases", "migrate webpack to vite"  Examples: - user: "Add Vite to this Nx app" -> install plugin and configure vite.config.ts - user: "Vitest is failing in Nx" -> fix test config and cache/coverage paths - user: "Path aliases break in Vite" -> add nxViteTsPaths plugin - user: "Set up Vite for my Nx library" -> configure lib mode + dts + externals · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/nx-vite-integration)

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

Complete Commander.js CLI framework guidance covering command structure, options, arguments, subcommands, action handlers, version management, and TypeScript integration. Use when: building CLI tools, parsing command-line arguments, implementing subcommands, handling options/flags, creating interactive CLIs, or migrating from other CLI frameworks.  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands, action handlers, version, help text, TypeScript, yargs, meow, program, parseAsync, opts, args, variadic, required options, default values, custom help, error handling · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/commanderjs)

| Skill | Rating | Audit |
| --- | --- | --- |
| [commanderjs](skills/development/commanderjs/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/commanderjs/2026-03-02/audit.json) |

### [bun-development](skills/development/bun-development)

Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package management, testing, SQLite, and security; use proactively when setting up Bun projects, replacing Node.js APIs with Bun-native APIs, writing bun test suites, implementing Bun.serve services, using bun:sqlite with prepared statements, configuring workspaces and lockfiles, hardening shell and SQL boundaries, or optimizing Bun performance and migration workflows. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bun-development)

| Skill | Rating | Audit |
| --- | --- | --- |
| [bun-development](skills/development/bun-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/bun-development/2026-03-02/audit.json) |

### [biome-complete](skills/development/biome-complete)

Complete Biome toolchain guidance for real repository workflows. Use when users ask to configure biome.json, run lint or format commands, migrate from ESLint or Prettier, tune rule severity, fix formatter drift, or replace mixed ESLint+Prettier pipelines with Biome-only workflows. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/biome-complete)

| Skill | Rating | Audit |
| --- | --- | --- |
| [biome-complete](skills/development/biome-complete/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/development/biome-complete/2026-03-02/audit.json) |

### [typescript-advanced](skills/development/typescript-advanced)

Comprehensive TypeScript guidance covering compiler configuration, advanced types, utility types, type guards, strict mode workflows, and documentation patterns; use when configuring tsconfig, designing complex generics, making illegal states unrepresentable, fixing type errors, or writing testable and maintainable type-safe APIs. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/typescript-advanced)

| Skill | Rating | Audit |
| --- | --- | --- |
| [typescript-advanced](skills/development/typescript-advanced/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/typescript-advanced/2026-03-02/audit.json) |

## Agentic Harness (4 tiles)

Agent framework configurations

### [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor)

Audit and improve skill collections with a 9-dimension scoring framework (Knowledge Delta, Mindset, Anti-Patterns, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability, Eval Validation), duplication detection, remediation planning, baseline comparison, and CI quality gates; use when evaluating skill quality, generating remediation plans, detecting duplicates, validating artifact conventions, or enforcing publication thresholds. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/skill-quality-auditor)

| Skill | Rating | Audit |
| --- | --- | --- |
| [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-04](.context/audits/agentic-harness/skill-quality-auditor/2026-03-04/audit.json) |

### [agents-md](skills/agentic-harness/agents-md)

Create and maintain AGENTS.md documentation for simple projects and complex monorepos with deterministic discovery, scoped instruction files, and low-token navigation patterns; use when generating AGENTS.md, updating agent docs, or standardizing AI-facing project guidance. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/agents-md)

| Skill | Rating | Audit |
| --- | --- | --- |
| [agents-md](skills/agentic-harness/agents-md/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/agents-md/2026-03-02/audit.json) |

### [tessl-publish-public](skills/agentic-harness/tessl/publish-public)

Ensure Tessl tiles meet all requirements for public registry publishing with comprehensive validation, quality gates, and evaluation scenarios. Use when preparing skills for public Tessl release, validating tile.json configuration, creating evaluation scenarios, enforcing quality thresholds, or checking agent-agnostic compliance. Keywords: tessl, tile, publishing, public-registry, validation, quality-gates, tile.json, evaluation-scenarios, skill-publishing · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/tessl-publish-public)

| Skill | Rating | Audit |
| --- | --- | --- |
| [tessl-publish-public](skills/agentic-harness/tessl/publish-public/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-03](.context/audits/agentic-harness/tessl/publish-public/2026-03-03/audit.json) |

### [opencode-config](skills/agentic-harness/opencode)

Configure OpenCode via opencode.json and AGENTS.md with deterministic provider setup, model selection, permission policies, formatter behavior, and environment variable handling; use when editing opencode configuration, setting model/provider defaults, tightening agent permissions, or troubleshooting OpenCode config behavior. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/opencode-config)

| Skill | Rating | Audit |
| --- | --- | --- |
| [opencode-config](skills/agentic-harness/opencode/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/opencode/2026-03-02/audit.json) |

## Testing (3 tiles)

Testing methodologies & quality

### [test-driven-development](skills/testing/test-driven-development)

Master Test-Driven Development with deterministic red-green-refactor workflows, test-first feature delivery, bug reproduction through failing tests, behavior-focused assertions, and refactoring safety; use when implementing new functions, changing APIs, fixing regressions, or restructuring code under test. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/test-driven-development)

| Skill | Rating | Audit |
| --- | --- | --- |
| [test-driven-development](skills/testing/test-driven-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/test-driven-development/2026-03-02/audit.json) |

### [bdd-testing](skills/testing/bdd-testing)

Write and maintain Behavior-Driven Development tests with Gherkin and Cucumber. Use when defining acceptance scenarios, writing feature files, implementing step definitions, running Three Amigos sessions, or diagnosing BDD test quality issues. Keywords: bdd, gherkin, cucumber, given when then, feature files, step definitions, acceptance criteria, three amigos, example mapping. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/bdd-testing)

| Skill | Rating | Audit |
| --- | --- | --- |
| [bdd-testing](skills/testing/bdd-testing/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/bdd-testing/2026-03-02/audit.json) |

### [ui-debug-workflow](skills/testing/ui-debug-workflow)

Debug UI changes with a repeatable evidence-first workflow. Use when validating visual regressions, reproducing frontend bugs, comparing baseline vs changed behavior, collecting screenshots/DOM/logs, or producing stakeholder-ready UI debug reports. Keywords: ui bug, visual regression, browser devtools, playwright, screenshot evidence, dom snapshot, frontend debugging. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/ui-debug-workflow)

| Skill | Rating | Audit |
| --- | --- | --- |
| [ui-debug-workflow](skills/testing/ui-debug-workflow/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/ui-debug-workflow/2026-03-02/audit.json) |

## Software Engineering (1 tile)

Software engineering principles

### [software-design-principles](skills/software-engineering/software-design-principles)

Apply software design principles across architecture and implementation using deterministic decision workflows, SOLID checks, structural patterns, and anti-pattern detection; use when reviewing designs, refactoring modules, or resolving maintainability and coupling risks. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/software-design-principles)

| Skill | Rating | Audit |
| --- | --- | --- |
| [software-design-principles](skills/software-engineering/software-design-principles/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/software-engineering/software-design-principles/2026-03-02/audit.json) |

## Observability (3 tiles)

Monitoring, logging & debugging

### [logql-generator](skills/observability/logql-generator)

Generate label matchers, line filters, log aggregations, and metric queries in LogQL (Loki Query Language) following current standards and conventions. Use this skill when creating new LogQL queries, implementing log analysis dashboards, alerting rules, or troubleshooting with Loki. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/logql-generator)

| Skill | Rating | Audit |
| --- | --- | --- |
| [logql-generator](skills/observability/logql-generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/logql-generator/2026-03-02/audit.json) |

### [k8s-debug](skills/observability/k8s-debug)

Comprehensive Kubernetes debugging and troubleshooting toolkit. Use this skill when diagnosing Kubernetes cluster issues, debugging failing pods, investigating network connectivity problems, analyzing resource usage, troubleshooting deployments, or performing cluster health checks. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/k8s-debug)

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

Author high-quality Markdown documentation with deterministic structure, lint compliance, and CI integration. Use when writing README files, creating docs pages, fixing markdownlint failures, defining style rules, or wiring markdown checks into pre-commit and pipelines. Keywords: markdown, markdownlint, readme, docs, headings, lists, code fences, links, images, lint config, ci, documentation style. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/markdown-authoring)

| Skill | Rating | Audit |
| --- | --- | --- |
| [markdown-authoring](skills/documentation/markdown-authoring/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/markdown-authoring/2026-03-02/audit.json) |

### [plain-english](skills/documentation/plain-english)

Write technical content in plain English for non-technical stakeholders by translating jargon into business language, surfacing decisions and impact early, and producing actionable recommendations with clear ownership and timeline. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/plain-english)

| Skill | Rating | Audit |
| --- | --- | --- |
| [plain-english](skills/documentation/plain-english/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/plain-english/2026-03-02/audit.json) |

### [journal-entry-creator](skills/documentation/journal-entry-creator)

Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]', 'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports four entry types: general journal entries, troubleshooting sessions, learning notes, and article summaries. Keywords: journal, documentation, troubleshooting, learning, article-summary, YAML frontmatter, template schemas, validation. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/journal-entry-creator)

| Skill | Rating | Audit |
| --- | --- | --- |
| [journal-entry-creator](skills/documentation/journal-entry-creator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/documentation/journal-entry-creator/2026-03-03/audit.json) |

### [acceptance-criteria](skills/documentation/acceptance-criteria)

Write clear, testable acceptance criteria for user stories and feature delivery; use when defining done conditions, creating measurable requirements, applying INVEST checks, documenting negative scenarios, and aligning product, engineering, and QA on expected outcomes. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/acceptance-criteria)

| Skill | Rating | Audit |
| --- | --- | --- |
| [acceptance-criteria](skills/documentation/acceptance-criteria/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/acceptance-criteria/2026-03-02/audit.json) |

### [conventional-commits](skills/documentation/conventional-commits)

Skill for creating structured, semantic commit messages following the Conventional Commits specification · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/conventional-commits)

| Skill | Rating | Audit |
| --- | --- | --- |
| [conventional-commits](skills/documentation/conventional-commits/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/documentation/conventional-commits/2026-03-02/audit.json) |

## Package Management (1 tile)

Package & version management

### [mise-complete](skills/package-mgmt/mise-complete)

Configure and operate Mise for deterministic developer environments. Use when installing runtime/tool versions, defining reusable tasks, managing layered environment variables, migrating from asdf/nvm/pyenv, or debugging mise.toml behavior in CI and local shells. Keywords: mise, mise.toml, tool versions, tasks, env, asdf migration, setup automation, dev environment. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/mise-complete)

| Skill | Rating | Audit |
| --- | --- | --- |
| [mise-complete](skills/package-mgmt/mise-complete/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/package-mgmt/mise-complete/2026-03-02/audit.json) |

## Project Management (3 tiles)

Planning & organization

### [create-context-file](skills/project-mgmt/create-context-file)

Create context files (plans, justifications, scratches) in .context/ directory with unique three-word IDs and frontmatter · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/create-context-file)

| Skill | Rating | Audit |
| --- | --- | --- |
| [create-context-file](skills/project-mgmt/create-context-file/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/project-mgmt/create-context-file/2026-03-03/audit.json) |

### [moscow-prioritization](skills/project-mgmt/moscow-prioritization)

Prioritize product requirements with the MoSCoW framework in a deterministic way.  Use when teams need to define MVP scope, sequence releases, resolve stakeholder conflicts,  prevent scope creep, or rebalance backlog under time, budget, or staffing constraints.  Keywords: moscow, must should could wont, requirement prioritization, backlog, mvp,  release planning, scope control, stakeholder alignment. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/moscow-prioritization)

| Skill | Rating | Audit |
| --- | --- | --- |
| [moscow-prioritization](skills/project-mgmt/moscow-prioritization/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/moscow-prioritization/2026-03-02/audit.json) |

### [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter)

Split large implementation plan documents into digestible, hierarchical structures with descriptive names. Use when refactoring monolithic planning docs, organizing phase documentation, or creating contributor-friendly task breakdowns. Triggers: "split this plan", "organize phases", "break down implementation docs", "create task hierarchy". · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/implementation-plan-splitter)

| Skill | Rating | Audit |
| --- | --- | --- |
| [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/implementation-plan-splitter/2026-03-02/audit.json) |

## Specialized (3 tiles)

Domain-specific tools

### [github-copilot-models](skills/specialized/github-copilot-models)

Query and display available GitHub Copilot AI models with their capabilities, context limits, and features. Use when: "what models are available", "show copilot models", "list github models", "check model capabilities", "switch models".  Examples: - user: "What models can I use with GitHub Copilot?" → fetch and display available models - user: "Show me models with vision support" → filter models by capability - user: "Which model has the largest context window?" → compare model specifications - user: "List all GPT-5 models" → filter by model family · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/github-copilot-models)

| Skill | Rating | Audit |
| --- | --- | --- |
| [github-copilot-models](skills/specialized/github-copilot-models/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/specialized/github-copilot-models/2026-03-03/audit.json) |

### [gitlab-api](skills/specialized/gitlab-api)

Retrieve and analyze GitLab merge request comments and metadata using authenticated API calls · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/gitlab-api)

| Skill | Rating | Audit |
| --- | --- | --- |
| [gitlab-api](skills/specialized/gitlab-api/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/specialized/gitlab-api/2026-03-02/audit.json) |

### [colyseus-multiplayer](skills/specialized/colyseus-multiplayer)

Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when implementing rooms, schema state sync, client message validation, matchmaking, authentication, reconnection handling, or server-side anti-cheat constraints. Keywords: colyseus, room lifecycle, schema, multiplayer, websocket, matchmaking, onJoin, onLeave, onDrop, allowReconnection. · [Public](https://tessl.io/registry/skills/pantheon-ai/pantheon-ai/colyseus-multiplayer)

| Skill | Rating | Audit |
| --- | --- | --- |
| [colyseus-multiplayer](skills/specialized/colyseus-multiplayer/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/specialized/colyseus-multiplayer/2026-03-02/audit.json) |

## What are Agent Skills?
