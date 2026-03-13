# Tile Catalog

Detailed information for all tiles and skills.

## Contents

- [CI/CD (6 tiles, 1 skill)](#cicd-6-tiles-1-skill)
  - [azure-pipelines-toolkit](#azure-pipelines-toolkit)
  - [gitlab-ci-toolkit](#gitlab-ci-toolkit)
  - [fluentbit-toolkit](#fluentbit-toolkit)
  - [jenkinsfile-toolkit](#jenkinsfile-toolkit)
  - [helm-toolkit](#helm-toolkit)
  - [github-actions-toolkit](#github-actions-toolkit)
  - [jenkinsfile-jenkins-build-monitor _(no tile)_](#jenkinsfile-jenkins-build-monitor-no-tile)
- [Infrastructure (8 tiles)](#infrastructure-8-tiles)
  - [terraform-toolkit](#terraform-toolkit)
  - [dockerfile-toolkit](#dockerfile-toolkit)
  - [terragrunt-toolkit](#terragrunt-toolkit)
  - [k8s-toolkit](#k8s-toolkit)
  - [cfn-behavior-validator](#cfn-behavior-validator)
  - [cfn-template-compare](#cfn-template-compare)
  - [ansible-toolkit](#ansible-toolkit)
  - [cdk-nag](#cdk-nag)
- [Repository Management (5 tiles)](#repository-management-5-tiles)
  - [nx-biome-integration](#nx-biome-integration)
  - [nx-bun-integration](#nx-bun-integration)
  - [nx-workspace-patterns](#nx-workspace-patterns)
  - [nx-plugin-toolkit](#nx-plugin-toolkit)
  - [nx-vite-integration](#nx-vite-integration)
- [Development (6 tiles)](#development-6-tiles)
  - [makefile-toolkit](#makefile-toolkit)
  - [bash-script-toolkit](#bash-script-toolkit)
  - [commanderjs](#commanderjs)
  - [bun-development](#bun-development)
  - [biome-complete](#biome-complete)
  - [typescript-advanced](#typescript-advanced)
- [Agentic Harness (4 tiles)](#agentic-harness-4-tiles)
  - [skill-quality-auditor](#skill-quality-auditor)
  - [agents-md](#agents-md)
  - [tessl-publish-public](#tessl-publish-public)
  - [opencode-toolkit](#opencode-toolkit)
- [Testing (3 tiles)](#testing-3-tiles)
  - [test-driven-development](#test-driven-development)
  - [bdd-testing](#bdd-testing)
  - [ui-debug-workflow](#ui-debug-workflow)
- [Software Engineering (1 tile)](#software-engineering-1-tile)
  - [design-principles](#design-principles)
- [Observability (2 tiles)](#observability-2-tiles)
  - [logql-generator](#logql-generator)
  - [promql-toolkit](#promql-toolkit)
- [Documentation (5 tiles)](#documentation-5-tiles)
  - [markdown-authoring](#markdown-authoring)
  - [plain-english](#plain-english)
  - [journal-entry-creator](#journal-entry-creator)
  - [acceptance-criteria](#acceptance-criteria)
  - [conventional-commits](#conventional-commits)
- [Package Management (1 tile)](#package-management-1-tile)
  - [mise-complete](#mise-complete)
- [Project Management (4 tiles)](#project-management-4-tiles)
  - [implementation-planner](#implementation-planner)
  - [create-context-file](#create-context-file)
  - [moscow-prioritization](#moscow-prioritization)
  - [implementation-plan-splitter](#implementation-plan-splitter)
- [Specialized (3 tiles)](#specialized-3-tiles)
  - [github-copilot-models](#github-copilot-models)
  - [gitlab-api](#gitlab-api)
  - [colyseus-multiplayer](#colyseus-multiplayer)

## CI/CD (6 tiles, 1 skill)

CI/CD pipelines & deployment automation

### [azure-pipelines-toolkit](skills/ci-cd/azure-pipelines)

Complete azure-pipelines toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [azure-pipelines-generator](skills/ci-cd/azure-pipelines/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/generator/2026-03-02/audit.json) | - |
| [azure-pipelines-validator](skills/ci-cd/azure-pipelines/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/azure-pipelines/validator/2026-03-02/audit.json) | - |

### [gitlab-ci-toolkit](skills/ci-cd/gitlab-ci)

Complete GitLab CI/CD toolkit with generation and validation capabilities for pipelines and configurations

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [gitlab-ci-generator](skills/ci-cd/gitlab-ci/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/generator/2026-03-02/audit.json) | - |
| [gitlab-ci-validator](skills/ci-cd/gitlab-ci/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/gitlab-ci/validator/2026-03-02/audit.json) | - |

### [fluentbit-toolkit](skills/ci-cd/fluentbit)

Complete fluentbit toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [fluentbit-generator](skills/ci-cd/fluentbit/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/fluentbit/generator/2026-03-02/audit.json) | - |
| [fluentbit-validator](skills/ci-cd/fluentbit/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/fluentbit/validator/2026-03-02/audit.json) | - |

### [jenkinsfile-toolkit](skills/ci-cd/jenkinsfile)

Complete jenkinsfile toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [jenkinsfile-generator](skills/ci-cd/jenkinsfile/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/generator/2026-03-02/audit.json) | - |
| [jenkinsfile-validator](skills/ci-cd/jenkinsfile/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/jenkinsfile/validator/2026-03-02/audit.json) | - |

### [helm-toolkit](skills/ci-cd/helm)

Complete helm toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [helm-generator](skills/ci-cd/helm/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/ci-cd/helm/generator/2026-03-02/audit.json) | - |
| [helm-validator](skills/ci-cd/helm/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/helm/validator/2026-03-02/audit.json) | - |

### [github-actions-toolkit](skills/ci-cd/github-actions)

Complete GitHub Actions toolkit with generation and validation capabilities for workflows, custom actions, and CI/CD configurations

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [github-actions-generator](skills/ci-cd/github-actions/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/generator/2026-03-02/audit.json) | - |
| [github-actions-validator](skills/ci-cd/github-actions/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/ci-cd/github-actions/validator/2026-03-02/audit.json) | - |

### jenkinsfile-jenkins-build-monitor _(no tile)_

-

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [jenkinsfile-jenkins-build-monitor](skills/ci-cd/jenkinsfile/jenkins-build-monitor/SKILL.md) | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | - |

## Infrastructure (8 tiles)

Infrastructure as Code

### [terraform-toolkit](skills/infrastructure/terraform)

Complete terraform toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [terraform-generator](skills/infrastructure/terraform/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terraform/generator/2026-03-02/audit.json) | - |
| [terraform-validator](skills/infrastructure/terraform/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/terraform/validator/2026-03-02/audit.json) | - |

### [dockerfile-toolkit](skills/infrastructure/dockerfile)

Complete dockerfile toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [dockerfile-generator](skills/infrastructure/dockerfile/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/dockerfile/generator/2026-03-02/audit.json) | - |
| [dockerfile-validator](skills/infrastructure/dockerfile/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/dockerfile/validator/2026-03-02/audit.json) | - |

### [terragrunt-toolkit](skills/infrastructure/terragrunt)

Complete terragrunt toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [terragrunt-generator](skills/infrastructure/terragrunt/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/generator/2026-03-02/audit.json) | - |
| [terragrunt-validator](skills/infrastructure/terragrunt/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/terragrunt/validator/2026-03-02/audit.json) | - |

### [k8s-toolkit](skills/infrastructure/k8s)

Comprehensive Kubernetes toolkit for YAML generation, validation, and cluster debugging

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [k8s-yaml-generator](skills/infrastructure/k8s/yaml-generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-06](.context/audits/infrastructure/k8s/yaml-generator/2026-03-06/audit.json) | 5 |
| [k8s-yaml-validator](skills/infrastructure/k8s/yaml-validator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-06](.context/audits/infrastructure/k8s/yaml-validator/2026-03-06/audit.json) | 5 |
| [k8s-debug](skills/infrastructure/k8s/debug/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-06](.context/audits/infrastructure/k8s/debug/2026-03-06/audit.json) | 5 |

### [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator)

Creates test stacks, analyzes CloudFormation events, and compares actual vs documented update behavior to validate whether resource property changes trigger replacement or in-place updates. Use when: a user wants to test if a CFN property change causes resource replacement; when investigating stack update behavior or "Update requires" documentation accuracy; when validating whether a workaround (e.g. hash-based logical IDs) is actually necessary; when questioning UpdateRequiresReplacement behavior for immutable properties; when empirical evidence is needed before an architectural decision involving CDK or CloudFormation stack updates.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [cfn-behavior-validator](skills/infrastructure/cfn/behavior-validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/cfn/behavior-validator/2026-03-02/audit.json) | 5 |

### [cfn-template-compare](skills/infrastructure/cfn/template-compare)

Compares deployed CloudFormation templates with locally synthesized CDK templates to detect drift, validate changes, and ensure consistency before deployment. Use when the user wants to compare CDK output with a deployed stack, check for infrastructure drift, run a pre-deployment validation, audit IAM or security changes, investigate a failing deployment, or perform a 'cdk diff'-style review. Triggered by phrases like 'compare templates', 'check for drift', 'cfn drift', 'stack comparison', 'infrastructure drift detection', 'safe to deploy', or 'what changed in my CDK stack'.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [cfn-template-compare](skills/infrastructure/cfn/template-compare/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/infrastructure/cfn/template-compare/2026-03-03/audit.json) | 5 |

### [ansible-toolkit](skills/infrastructure/ansible)

Complete ansible toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [ansible-generator](skills/infrastructure/ansible/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/infrastructure/ansible/generator/2026-03-02/audit.json) | - |
| [ansible-validator](skills/infrastructure/ansible/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/infrastructure/ansible/validator/2026-03-02/audit.json) | - |

### [cdk-nag](skills/infrastructure/aws-cdk/cdk-nag)

Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding rule packs, triaging findings, writing justified suppressions, integrating checks in CI/CD, or preventing insecure infrastructure patterns in CDK stacks.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [cdk-nag](skills/infrastructure/aws-cdk/cdk-nag/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/infrastructure/aws-cdk/cdk-nag/2026-03-02/audit.json) | - |

## Repository Management (5 tiles)

Repository & workspace management

### [nx-biome-integration](skills/repository-mgmt/nx/biome-integration)

Integrate Biome into Nx monorepos with deterministic setup, caching, migration from ESLint and Prettier, and plugin-based inferred tasks; use when adding Biome, replacing ESLint/Prettier, tuning cache inputs, or scaling lint and format workflows across projects.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-biome-integration](skills/repository-mgmt/nx/biome-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/biome-integration/2026-03-02/audit.json) | - |

### [nx-bun-integration](skills/repository-mgmt/nx/bun-integration)

Integrate Bun runtime into Nx monorepos with deterministic plugin setup, executor configuration, migration from Node.js toolchains, and cache-aware build/test workflows; use when adding the nx-bun plugin, converting projects, or standardizing Bun targets across Nx workspaces.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-bun-integration](skills/repository-mgmt/nx/bun-integration/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/bun-integration/2026-03-02/audit.json) | - |

### [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns)

Configure and optimize Nx monorepo workspaces with deterministic project-graph structure, boundary enforcement, cache-aware pipelines, and affected-command CI workflows; use when designing workspace architecture, tightening dependency rules, or reducing CI cost through Nx task orchestration.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-workspace-patterns](skills/repository-mgmt/nx/workspace-patterns/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/repository-mgmt/nx/workspace-patterns/2026-03-02/audit.json) | - |

### [nx-plugin-toolkit](skills/repository-mgmt/nx)

Complete Nx plugin development toolkit: create custom generators, executors, and extend Nx workspaces with reusable automation

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-plugin-authoring](skills/repository-mgmt/nx/nx-plugin-authoring/SKILL.md) | ![C](https://img.shields.io/badge/Rating-C-red) | [2026-03-06](.context/audits/repository-mgmt/nx/nx-plugin-authoring/2026-03-06/audit.json) | - |

### [nx-vite-integration](skills/repository-mgmt/nx/vite-integration)

Configure and integrate Vite in Nx monorepos for applications and libraries. Covers vite.config.ts setup, framework plugins, TypeScript path resolution, asset copying, library mode builds, and Vitest integration.  Use when: adding Vite to an Nx project, migrating from Webpack, configuring Vitest, fixing tsconfig path resolution, or setting up library mode.  Triggers: "add vite", "nx vite", "vite setup", "vite.config.ts", "vitest config", "library mode", "nxViteTsPaths", "copy assets", "vite path aliases", "migrate webpack to vite"  Examples: - user: "Add Vite to this Nx app" -> install plugin and configure vite.config.ts - user: "Vitest is failing in Nx" -> fix test config and cache/coverage paths - user: "Path aliases break in Vite" -> add nxViteTsPaths plugin - user: "Set up Vite for my Nx library" -> configure lib mode + dts + externals

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [nx-vite-integration](skills/repository-mgmt/nx/vite-integration/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/repository-mgmt/nx/vite-integration/2026-03-02/audit.json) | - |

## Development (6 tiles)

Development tooling

### [makefile-toolkit](skills/development/scripting/makefile)

Complete makefile toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [makefile-generator](skills/development/scripting/makefile/generator/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/scripting/makefile/generator/2026-03-02/audit.json) | - |
| [makefile-validator](skills/development/scripting/makefile/validator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/scripting/makefile/validator/2026-03-02/audit.json) | - |

### [bash-script-toolkit](skills/development/scripting/bash-script)

Complete bash-script toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [bash-script-generator](skills/development/scripting/bash-script/generator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/generator/2026-03-02/audit.json) | - |
| [bash-script-validator](skills/development/scripting/bash-script/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/development/scripting/bash-script/validator/2026-03-02/audit.json) | - |

### [commanderjs](skills/development/commanderjs)

Complete Commander.js CLI framework guidance covering command structure, options, arguments, subcommands, action handlers, version management, and TypeScript integration. Use when: building CLI tools, parsing command-line arguments, implementing subcommands, handling options/flags, creating interactive CLIs, or migrating from other CLI frameworks.  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands, action handlers, version, help text, TypeScript, yargs, meow, program, parseAsync, opts, args, variadic, required options, default values, custom help, error handling

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [commanderjs](skills/development/commanderjs/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/development/commanderjs/2026-03-02/audit.json) | - |

### [bun-development](skills/development/bun-development)

Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package management, testing, SQLite, and security; use proactively when setting up Bun projects, replacing Node.js APIs with Bun-native APIs, writing bun test suites, implementing Bun.serve services, using bun:sqlite with prepared statements, configuring workspaces and lockfiles, hardening shell and SQL boundaries, or optimizing Bun performance and migration workflows.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [bun-development](skills/development/bun-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/bun-development/2026-03-02/audit.json) | - |

### [biome-complete](skills/development/biome-complete)

Complete Biome toolchain guidance for real repository workflows. Use when users ask to configure biome.json, run lint or format commands, migrate from ESLint or Prettier, tune rule severity, fix formatter drift, or replace mixed ESLint+Prettier pipelines with Biome-only workflows.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [biome-complete](skills/development/biome-complete/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/development/biome-complete/2026-03-02/audit.json) | - |

### [typescript-advanced](skills/development/typescript-advanced)

Comprehensive TypeScript guidance covering compiler configuration, advanced types, utility types, type guards, strict mode workflows, and documentation patterns; use when configuring tsconfig, designing complex generics, making illegal states unrepresentable, fixing type errors, or writing testable and maintainable type-safe APIs.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [typescript-advanced](skills/development/typescript-advanced/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/development/typescript-advanced/2026-03-02/audit.json) | - |

## Agentic Harness (4 tiles)

Agent framework configurations

### [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor)

Audit and improve skill collections with a 9-dimension scoring framework (Knowledge Delta, Mindset, Anti-Patterns, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability, Eval Validation), duplication detection, remediation planning, baseline comparison, and CI quality gates; use when evaluating skill quality, generating remediation plans, detecting duplicates, validating artifact conventions, or enforcing publication thresholds.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [skill-quality-auditor](skills/agentic-harness/skill-quality-auditor/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-04](.context/audits/agentic-harness/skill-quality-auditor/2026-03-04/audit.json) | 5 |

### [agents-md](skills/agentic-harness/agents-md)

Create and maintain AGENTS.md documentation for simple projects and complex monorepos with deterministic discovery, scoped instruction files, and low-token navigation patterns; use when generating AGENTS.md, updating agent docs, or standardizing AI-facing project guidance.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [agents-md](skills/agentic-harness/agents-md/SKILL.md) | ![A](https://img.shields.io/badge/Rating-A-green) | [2026-03-02](.context/audits/agentic-harness/agents-md/2026-03-02/audit.json) | 5 |

### [tessl-publish-public](skills/agentic-harness/tessl/publish-public)

Ensure Tessl tiles meet all requirements for public registry publishing with comprehensive validation, quality gates, and evaluation scenarios. Use when preparing skills for public Tessl release, validating tile.json configuration, creating evaluation scenarios, enforcing quality thresholds, or checking agent-agnostic compliance. Keywords: tessl, tile, publishing, public-registry, validation, quality-gates, tile.json, evaluation-scenarios, skill-publishing

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [tessl-publish-public](skills/agentic-harness/tessl/publish-public/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-03](.context/audits/agentic-harness/tessl/publish-public/2026-03-03/audit.json) | 9 |

### [opencode-toolkit](skills/agentic-harness/opencode)

Complete toolkit for configuring and extending OpenCode: agent creation, custom slash commands, configuration management, plugin development, and SDK usage.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [opencode-design-agents](skills/agentic-harness/opencode/design-agents/SKILL.md) | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | 3 |
| [opencode-design-commands](skills/agentic-harness/opencode/design-commands/SKILL.md) | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | 3 |
| [opencode-configure](skills/agentic-harness/opencode/configure/SKILL.md) | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | 3 |
| [opencode-build-plugins](skills/agentic-harness/opencode/build-plugins/SKILL.md) | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | 3 |
| [opencode-build-tool](skills/agentic-harness/opencode/build-tool/SKILL.md) | ![?](https://img.shields.io/badge/Rating-?-lightgrey) | - | 3 |

## Testing (3 tiles)

Testing methodologies & quality

### [test-driven-development](skills/testing/test-driven-development)

Master Test-Driven Development with deterministic red-green-refactor workflows, test-first feature delivery, bug reproduction through failing tests, behavior-focused assertions, and refactoring safety; use when implementing new functions, changing APIs, fixing regressions, or restructuring code under test.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [test-driven-development](skills/testing/test-driven-development/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/test-driven-development/2026-03-02/audit.json) | 5 |

### [bdd-testing](skills/testing/bdd-testing)

Write and maintain Behavior-Driven Development tests with Gherkin and Cucumber. Use when defining acceptance scenarios, writing feature files, implementing step definitions, running Three Amigos sessions, or diagnosing BDD test quality issues. Keywords: bdd, gherkin, cucumber, given when then, feature files, step definitions, acceptance criteria, three amigos, example mapping.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [bdd-testing](skills/testing/bdd-testing/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/bdd-testing/2026-03-02/audit.json) | 8 |

### [ui-debug-workflow](skills/testing/ui-debug-workflow)

Debug UI changes with a repeatable evidence-first workflow. Use when validating visual regressions, reproducing frontend bugs, comparing baseline vs changed behavior, collecting screenshots/DOM/logs, or producing stakeholder-ready UI debug reports. Keywords: ui bug, visual regression, browser devtools, playwright, screenshot evidence, dom snapshot, frontend debugging.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [ui-debug-workflow](skills/testing/ui-debug-workflow/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/testing/ui-debug-workflow/2026-03-02/audit.json) | - |

## Software Engineering (1 tile)

Software engineering principles

### [design-principles](skills/software-engineering/design-principles)

Strategic architecture, tactical design, and testable code principles (SOLID, Clean Architecture, Design Patterns, Testable Design)

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [solid-principles](skills/software-engineering/design-principles/solid-principles/SKILL.md) | ![C](https://img.shields.io/badge/Rating-C-red) | [2026-03-04](.context/audits/software-engineering/design-principles/solid-principles/2026-03-04/audit.json) | - |
| [clean-architecture](skills/software-engineering/design-principles/clean-architecture/SKILL.md) | ![C](https://img.shields.io/badge/Rating-C-red) | [2026-03-04](.context/audits/software-engineering/design-principles/clean-architecture/2026-03-04/audit.json) | - |
| [design-patterns](skills/software-engineering/design-principles/design-patterns/SKILL.md) | ![C](https://img.shields.io/badge/Rating-C-red) | [2026-03-04](.context/audits/software-engineering/design-principles/design-patterns/2026-03-04/audit.json) | - |
| [testable-design](skills/software-engineering/design-principles/testable-design/SKILL.md) | ![C](https://img.shields.io/badge/Rating-C-red) | [2026-03-04](.context/audits/software-engineering/design-principles/testable-design/2026-03-04/audit.json) | - |

## Observability (2 tiles)

Monitoring, logging & debugging

### [logql-generator](skills/observability/logql-generator)

Generate label matchers, line filters, log aggregations, and metric queries in LogQL (Loki Query Language) following current standards and conventions. Use this skill when creating new LogQL queries, implementing log analysis dashboards, alerting rules, or troubleshooting with Loki.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [logql-generator](skills/observability/logql-generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/logql-generator/2026-03-02/audit.json) | - |

### [promql-toolkit](skills/observability/promql)

Complete PromQL toolkit with generation and validation capabilities

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [promql-generator](skills/observability/promql/generator/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/observability/promql/generator/2026-03-02/audit.json) | - |
| [promql-validator](skills/observability/promql/validator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/observability/promql/validator/2026-03-02/audit.json) | - |

## Documentation (5 tiles)

Writing & communication

### [markdown-authoring](skills/documentation/markdown-authoring)

Author high-quality Markdown documentation with deterministic structure, lint compliance, and CI integration. Use when writing README files, creating docs pages, fixing markdownlint failures, defining style rules, or wiring markdown checks into pre-commit and pipelines. Keywords: markdown, markdownlint, readme, docs, headings, lists, code fences, links, images, lint config, ci, documentation style.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [markdown-authoring](skills/documentation/markdown-authoring/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/markdown-authoring/2026-03-02/audit.json) | - |

### [plain-english](skills/documentation/plain-english)

Write technical content in plain English for non-technical stakeholders by translating jargon into business language, surfacing decisions and impact early, and producing actionable recommendations with clear ownership and timeline.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [plain-english](skills/documentation/plain-english/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/plain-english/2026-03-02/audit.json) | 8 |

### [journal-entry-creator](skills/documentation/journal-entry-creator)

Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]', 'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports four entry types: general journal entries, troubleshooting sessions, learning notes, and article summaries. Keywords: journal, documentation, troubleshooting, learning, article-summary, YAML frontmatter, template schemas, validation.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [journal-entry-creator](skills/documentation/journal-entry-creator/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/documentation/journal-entry-creator/2026-03-03/audit.json) | 5 |

### [acceptance-criteria](skills/documentation/acceptance-criteria)

Write clear, testable acceptance criteria for user stories and feature delivery; use when defining done conditions, creating measurable requirements, applying INVEST checks, documenting negative scenarios, and aligning product, engineering, and QA on expected outcomes.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [acceptance-criteria](skills/documentation/acceptance-criteria/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-02](.context/audits/documentation/acceptance-criteria/2026-03-02/audit.json) | 5 |

### [conventional-commits](skills/documentation/conventional-commits)

Skill for creating structured, semantic commit messages following the Conventional Commits specification

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [conventional-commits](skills/documentation/conventional-commits/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/documentation/conventional-commits/2026-03-02/audit.json) | - |

## Package Management (1 tile)

Package & version management

### [mise-complete](skills/package-mgmt/mise-complete)

Configure and operate Mise for deterministic developer environments. Use when installing runtime/tool versions, defining reusable tasks, managing layered environment variables, migrating from asdf/nvm/pyenv, or debugging mise.toml behavior in CI and local shells. Keywords: mise, mise.toml, tool versions, tasks, env, asdf migration, setup automation, dev environment.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [mise-complete](skills/package-mgmt/mise-complete/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/package-mgmt/mise-complete/2026-03-02/audit.json) | - |

## Project Management (4 tiles)

Planning & organization

### [implementation-planner](skills/project-mgmt/implementation-planner)

Converts a PRD or requirements document into a structured, phased implementation plan with individual phase files and granular per-task files written to .context/plans/. Also restructures existing monolithic planning documents into digestible, hierarchical directory structures. Creates a root plan index summarising all phases, a numbered phase file per phase, and a numbered task file per task inside each phase directory.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [implementation-planner](skills/project-mgmt/implementation-planner/SKILL.md) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) | [2026-03-11](.context/audits/project-mgmt/implementation-planner/2026-03-11/audit.json) | 5 |

### [create-context-file](skills/project-mgmt/create-context-file)

Create context files (plans, justifications, scratches) in .context/ directory with unique three-word IDs and frontmatter

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [create-context-file](skills/project-mgmt/create-context-file/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/project-mgmt/create-context-file/2026-03-03/audit.json) | - |

### [moscow-prioritization](skills/project-mgmt/moscow-prioritization)

Prioritize product requirements with the MoSCoW framework in a deterministic way.  Use when teams need to define MVP scope, sequence releases, resolve stakeholder conflicts,  prevent scope creep, or rebalance backlog under time, budget, or staffing constraints.  Keywords: moscow, must should could wont, requirement prioritization, backlog, mvp,  release planning, scope control, stakeholder alignment.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [moscow-prioritization](skills/project-mgmt/moscow-prioritization/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/moscow-prioritization/2026-03-02/audit.json) | - |

### [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter)

Merged into implementation-planner. Redirects to the unified skill that handles both creating new plans and restructuring existing monolithic planning docs into hierarchical directory structures.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [implementation-plan-splitter](skills/project-mgmt/implementation-plan-splitter/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/project-mgmt/implementation-plan-splitter/2026-03-02/audit.json) | - |

## Specialized (3 tiles)

Domain-specific tools

### [github-copilot-models](skills/specialized/github-copilot-models)

Query and display available GitHub Copilot AI models with their capabilities, context limits, and features. Use when: "what models are available", "show copilot models", "list github models", "check model capabilities", "switch models".  Examples: - user: "What models can I use with GitHub Copilot?" → fetch and display available models - user: "Show me models with vision support" → filter models by capability - user: "Which model has the largest context window?" → compare model specifications - user: "List all GPT-5 models" → filter by model family

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [github-copilot-models](skills/specialized/github-copilot-models/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-03](.context/audits/specialized/github-copilot-models/2026-03-03/audit.json) | - |

### [gitlab-api](skills/specialized/gitlab-api)

Retrieve and analyze GitLab merge request comments and metadata using authenticated API calls

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [gitlab-api](skills/specialized/gitlab-api/SKILL.md) | ![C+](https://img.shields.io/badge/Rating-C+-orange) | [2026-03-02](.context/audits/specialized/gitlab-api/2026-03-02/audit.json) | 5 |

### [colyseus-multiplayer](skills/specialized/colyseus-multiplayer)

Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when implementing rooms, schema state sync, client message validation, matchmaking, authentication, reconnection handling, or server-side anti-cheat constraints. Keywords: colyseus, room lifecycle, schema, multiplayer, websocket, matchmaking, onJoin, onLeave, onDrop, allowReconnection.

| Skill | Rating | Audit | Evals |
| --- | --- | --- | --- |
| [colyseus-multiplayer](skills/specialized/colyseus-multiplayer/SKILL.md) | ![B](https://img.shields.io/badge/Rating-B-yellow) | [2026-03-02](.context/audits/specialized/colyseus-multiplayer/2026-03-02/audit.json) | - |
