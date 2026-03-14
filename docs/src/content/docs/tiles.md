---
title: Skill Catalog
description: All Tekhne tiles and skills organized by domain.
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 3
---

Detailed information for all tiles and skills organized by domain.

## CI/CD (6 tiles)

CI/CD pipelines & deployment automation

### azure-pipelines-toolkit

Complete azure-pipelines toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [azure-pipelines-generator](/tekhne/skills/ci-cd/azure-pipelines/generator/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |
| [azure-pipelines-validator](/tekhne/skills/ci-cd/azure-pipelines/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### gitlab-ci-toolkit

Complete GitLab CI/CD toolkit with generation and validation capabilities for pipelines and configurations

| Skill | Rating |
| --- | --- |
| [gitlab-ci-generator](/tekhne/skills/ci-cd/gitlab-ci/generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [gitlab-ci-validator](/tekhne/skills/ci-cd/gitlab-ci/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### fluentbit-toolkit

Complete fluentbit toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [fluentbit-generator](/tekhne/skills/ci-cd/fluentbit/generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [fluentbit-validator](/tekhne/skills/ci-cd/fluentbit/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### jenkinsfile-toolkit

Complete jenkinsfile toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [jenkinsfile-generator](/tekhne/skills/ci-cd/jenkinsfile/generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [jenkinsfile-validator](/tekhne/skills/ci-cd/jenkinsfile/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### helm-toolkit

Complete helm toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [helm-generator](/tekhne/skills/ci-cd/helm/generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [helm-validator](/tekhne/skills/ci-cd/helm/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### github-actions-toolkit

Complete GitHub Actions toolkit with generation and validation capabilities for workflows, custom actions, and CI/CD configurations

| Skill | Rating |
| --- | --- |
| [github-actions-generator](/tekhne/skills/ci-cd/github-actions/generator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |
| [github-actions-validator](/tekhne/skills/ci-cd/github-actions/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

---

## Infrastructure (8 tiles)

Infrastructure as Code

### terraform-toolkit

Complete terraform toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [terraform-generator](/tekhne/skills/infrastructure/terraform/generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [terraform-validator](/tekhne/skills/infrastructure/terraform/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### dockerfile-toolkit

Complete dockerfile toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [dockerfile-generator](/tekhne/skills/infrastructure/dockerfile/generator/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |
| [dockerfile-validator](/tekhne/skills/infrastructure/dockerfile/validator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### terragrunt-toolkit

Complete terragrunt toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [terragrunt-generator](/tekhne/skills/infrastructure/terragrunt/generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [terragrunt-validator](/tekhne/skills/infrastructure/terragrunt/validator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### k8s-toolkit

Comprehensive Kubernetes toolkit for YAML generation, validation, and cluster debugging

| Skill | Rating |
| --- | --- |
| [k8s-yaml-generator](/tekhne/skills/infrastructure/k8s/yaml-generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [k8s-yaml-validator](/tekhne/skills/infrastructure/k8s/yaml-validator/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |
| [k8s-debug](/tekhne/skills/infrastructure/k8s/debug/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |

### cfn-behavior-validator

Creates test stacks, analyzes CloudFormation events, and compares actual vs documented update behavior to validate whether resource property changes trigger replacement or in-place updates. Use when: a user wants to test if a CFN property change causes resource replacement; when investigating stack update behavior or "Update requires" documentation accuracy; when validating whether a workaround (e.g. hash-based logical IDs) is actually necessary; when questioning UpdateRequiresReplacement behavior for immutable properties; when empirical evidence is needed before an architectural decision involving CDK or CloudFormation stack updates.

| Skill | Rating |
| --- | --- |
| [cfn-behavior-validator](/tekhne/skills/infrastructure/cfn/behavior-validator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### cfn-template-compare

Compares deployed CloudFormation templates with locally synthesized CDK templates to detect drift, validate changes, and ensure consistency before deployment. Use when the user wants to compare CDK output with a deployed stack, check for infrastructure drift, run a pre-deployment validation, audit IAM or security changes, investigate a failing deployment, or perform a 'cdk diff'-style review. Triggered by phrases like 'compare templates', 'check for drift', 'cfn drift', 'stack comparison', 'infrastructure drift detection', 'safe to deploy', or 'what changed in my CDK stack'.

| Skill | Rating |
| --- | --- |
| [cfn-template-compare](/tekhne/skills/infrastructure/cfn/template-compare/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### ansible-toolkit

Complete ansible toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [ansible-generator](/tekhne/skills/infrastructure/ansible/generator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |
| [ansible-validator](/tekhne/skills/infrastructure/ansible/validator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### cdk-nag

Enforce AWS CDK security and compliance controls with cdk-nag. Use when adding rule packs, triaging findings, writing justified suppressions, integrating checks in CI/CD, or preventing insecure infrastructure patterns in CDK stacks.

| Skill | Rating |
| --- | --- |
| [cdk-nag](/tekhne/skills/infrastructure/aws-cdk/cdk-nag/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

---

## Repository Management (5 tiles)

Repository & workspace management

### nx-biome-integration

Integrate Biome into Nx monorepos with deterministic setup, caching, migration from ESLint and Prettier, and plugin-based inferred tasks; use when adding Biome, replacing ESLint/Prettier, tuning cache inputs, or scaling lint and format workflows across projects.

| Skill | Rating |
| --- | --- |
| [nx-biome-integration](/tekhne/skills/repository-mgmt/nx/biome-integration/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### nx-bun-integration

Integrate Bun runtime into Nx monorepos with deterministic plugin setup, executor configuration, migration from Node.js toolchains, and cache-aware build/test workflows; use when adding the nx-bun plugin, converting projects, or standardizing Bun targets across Nx workspaces.

| Skill | Rating |
| --- | --- |
| [nx-bun-integration](/tekhne/skills/repository-mgmt/nx/bun-integration/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### nx-workspace-patterns

Configure and optimize Nx monorepo workspaces with deterministic project-graph structure, boundary enforcement, cache-aware pipelines, and affected-command CI workflows; use when designing workspace architecture, tightening dependency rules, or reducing CI cost through Nx task orchestration.

| Skill | Rating |
| --- | --- |
| [nx-workspace-patterns](/tekhne/skills/repository-mgmt/nx/workspace-patterns/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### nx-plugin-toolkit

Complete Nx plugin development toolkit: create custom generators, executors, and extend Nx workspaces with reusable automation

| Skill | Rating |
| --- | --- |
| [nx-plugin-authoring](/tekhne/skills/repository-mgmt/nx/nx-plugin-authoring/skill/) | ![C](https://img.shields.io/badge/Rating-C-red) |

### nx-vite-integration

Configure and integrate Vite in Nx monorepos for applications and libraries. Covers vite.config.ts setup, framework plugins, TypeScript path resolution, asset copying, library mode builds, and Vitest integration.  Use when: adding Vite to an Nx project, migrating from Webpack, configuring Vitest, fixing tsconfig path resolution, or setting up library mode.  Triggers: "add vite", "nx vite", "vite setup", "vite.config.ts", "vitest config", "library mode", "nxViteTsPaths", "copy assets", "vite path aliases", "migrate webpack to vite"  Examples: - user: "Add Vite to this Nx app" -> install plugin and configure vite.config.ts - user: "Vitest is failing in Nx" -> fix test config and cache/coverage paths - user: "Path aliases break in Vite" -> add nxViteTsPaths plugin - user: "Set up Vite for my Nx library" -> configure lib mode + dts + externals

| Skill | Rating |
| --- | --- |
| [nx-vite-integration](/tekhne/skills/repository-mgmt/nx/vite-integration/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

---

## Development (6 tiles)

Development tooling

### makefile-toolkit

Complete makefile toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [makefile-generator](/tekhne/skills/development/scripting/makefile/generator/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |
| [makefile-validator](/tekhne/skills/development/scripting/makefile/validator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### bash-script-toolkit

Complete bash-script toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [bash-script-generator](/tekhne/skills/development/scripting/bash-script/generator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
| [bash-script-validator](/tekhne/skills/development/scripting/bash-script/validator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### commanderjs

Complete Commander.js CLI framework guidance covering command structure, options, arguments, subcommands, action handlers, version management, and TypeScript integration. Use when: building CLI tools, parsing command-line arguments, implementing subcommands, handling options/flags, creating interactive CLIs, or migrating from other CLI frameworks.  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands, action handlers, version, help text, TypeScript, yargs, meow, program, parseAsync, opts, args, variadic, required options, default values, custom help, error handling

| Skill | Rating |
| --- | --- |
| [commanderjs](/tekhne/skills/development/commanderjs/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### bun-development

Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package management, testing, SQLite, and security; use proactively when setting up Bun projects, replacing Node.js APIs with Bun-native APIs, writing bun test suites, implementing Bun.serve services, using bun:sqlite with prepared statements, configuring workspaces and lockfiles, hardening shell and SQL boundaries, or optimizing Bun performance and migration workflows.

| Skill | Rating |
| --- | --- |
| [bun-development](/tekhne/skills/development/bun-development/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### biome-complete

Complete Biome toolchain guidance for real repository workflows. Use when users ask to configure biome.json, run lint or format commands, migrate from ESLint or Prettier, tune rule severity, fix formatter drift, or replace mixed ESLint+Prettier pipelines with Biome-only workflows.

| Skill | Rating |
| --- | --- |
| [biome-complete](/tekhne/skills/development/biome-complete/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |

### typescript-advanced

Comprehensive TypeScript guidance covering compiler configuration, advanced types, utility types, type guards, strict mode workflows, and documentation patterns; use when configuring tsconfig, designing complex generics, making illegal states unrepresentable, fixing type errors, or writing testable and maintainable type-safe APIs.

| Skill | Rating |
| --- | --- |
| [typescript-advanced](/tekhne/skills/development/typescript-advanced/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

---

## Agentic Harness (4 tiles)

Agent framework configurations

### opencode-toolkit

Complete toolkit for configuring and extending OpenCode: agent creation, custom slash commands, configuration management, plugin development, and SDK usage.

| Skill | Rating |
| --- | --- |
| [opencode-design-agents](/tekhne/skills/agentic-harness/opencode-toolkit/design-agents/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |
| [opencode-design-commands](/tekhne/skills/agentic-harness/opencode-toolkit/design-commands/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |
| [opencode-configure](/tekhne/skills/agentic-harness/opencode-toolkit/configure/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |
| [opencode-build-plugins](/tekhne/skills/agentic-harness/opencode-toolkit/build-plugins/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |
| [opencode-build-tool](/tekhne/skills/agentic-harness/opencode-toolkit/build-tool/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |

### skill-quality-auditor

Audit and improve skill collections with a 9-dimension scoring framework (Knowledge Delta, Mindset, Anti-Patterns, Specification Compliance, Progressive Disclosure, Freedom Calibration, Pattern Recognition, Practical Usability, Eval Validation), duplication detection, remediation planning, baseline comparison, and CI quality gates; use when evaluating skill quality, generating remediation plans, detecting duplicates, validating artifact conventions, or enforcing publication thresholds.

| Skill | Rating |
| --- | --- |
| [skill-quality-auditor](/tekhne/skills/agentic-harness/skill-quality-auditor/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |

### agents-md

Create and maintain AGENTS.md documentation for simple projects and complex monorepos with deterministic discovery, scoped instruction files, and low-token navigation patterns; use when generating AGENTS.md, updating agent docs, or standardizing AI-facing project guidance.

| Skill | Rating |
| --- | --- |
| [agents-md](/tekhne/skills/agentic-harness/agents-md/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |

### tessl-publish-public

Ensure Tessl tiles meet all requirements for public registry publishing with comprehensive validation, quality gates, and evaluation scenarios. Use when preparing skills for public Tessl release, validating tile.json configuration, creating evaluation scenarios, enforcing quality thresholds, or checking agent-agnostic compliance. Keywords: tessl, tile, publishing, public-registry, validation, quality-gates, tile.json, evaluation-scenarios, skill-publishing

| Skill | Rating |
| --- | --- |
| [tessl-publish-public](/tekhne/skills/agentic-harness/tessl/publish-public/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

---

## Testing (3 tiles)

Testing methodologies & quality

### test-driven-development

Master Test-Driven Development with deterministic red-green-refactor workflows, test-first feature delivery, bug reproduction through failing tests, behavior-focused assertions, and refactoring safety; use when implementing new functions, changing APIs, fixing regressions, or restructuring code under test.

| Skill | Rating |
| --- | --- |
| [test-driven-development](/tekhne/skills/testing/test-driven-development/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### bdd-testing

Write and maintain Behavior-Driven Development tests with Gherkin and Cucumber. Use when defining acceptance scenarios, writing feature files, implementing step definitions, running Three Amigos sessions, or diagnosing BDD test quality issues. Keywords: bdd, gherkin, cucumber, given when then, feature files, step definitions, acceptance criteria, three amigos, example mapping.

| Skill | Rating |
| --- | --- |
| [bdd-testing](/tekhne/skills/testing/bdd-testing/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### ui-debug-workflow

Debug UI changes with a repeatable evidence-first workflow. Use when validating visual regressions, reproducing frontend bugs, comparing baseline vs changed behavior, collecting screenshots/DOM/logs, or producing stakeholder-ready UI debug reports. Keywords: ui bug, visual regression, browser devtools, playwright, screenshot evidence, dom snapshot, frontend debugging.

| Skill | Rating |
| --- | --- |
| [ui-debug-workflow](/tekhne/skills/testing/ui-debug-workflow/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

---

## Software Engineering (1 tile)

Software engineering principles

### design-principles

Strategic architecture, tactical design, and testable code principles (SOLID, Clean Architecture, Design Patterns, Testable Design)

| Skill | Rating |
| --- | --- |
| [solid-principles](/tekhne/skills/software-engineering/design-principles/solid-principles/skill/) | ![C](https://img.shields.io/badge/Rating-C-red) |
| [clean-architecture](/tekhne/skills/software-engineering/design-principles/clean-architecture/skill/) | ![C](https://img.shields.io/badge/Rating-C-red) |
| [design-patterns](/tekhne/skills/software-engineering/design-principles/design-patterns/skill/) | ![C](https://img.shields.io/badge/Rating-C-red) |
| [testable-design](/tekhne/skills/software-engineering/design-principles/testable-design/skill/) | ![C](https://img.shields.io/badge/Rating-C-red) |

---

## Observability (2 tiles)

Monitoring, logging & debugging

### logql-generator

Generate label matchers, line filters, log aggregations, and metric queries in LogQL (Loki Query Language) following current standards and conventions. Use this skill when creating new LogQL queries, implementing log analysis dashboards, alerting rules, or troubleshooting with Loki.

| Skill | Rating |
| --- | --- |
| [logql-generator](/tekhne/skills/observability/logql-generator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### promql-toolkit

Complete PromQL toolkit with generation and validation capabilities

| Skill | Rating |
| --- | --- |
| [promql-generator](/tekhne/skills/observability/promql/generator/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |
| [promql-validator](/tekhne/skills/observability/promql/validator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

---

## Documentation (5 tiles)

Writing & communication

### markdown-authoring

Author high-quality Markdown documentation with deterministic structure, lint compliance, and CI integration. Use when writing README files, creating docs pages, fixing markdownlint failures, defining style rules, or wiring markdown checks into pre-commit and pipelines. Keywords: markdown, markdownlint, readme, docs, headings, lists, code fences, links, images, lint config, ci, documentation style.

| Skill | Rating |
| --- | --- |
| [markdown-authoring](/tekhne/skills/documentation/markdown-authoring/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### plain-english

Write technical content in plain English for non-technical stakeholders by translating jargon into business language, surfacing decisions and impact early, and producing actionable recommendations with clear ownership and timeline.

| Skill | Rating |
| --- | --- |
| [plain-english](/tekhne/skills/documentation/plain-english/skill/) | ![A](https://img.shields.io/badge/Rating-A-green) |

### journal-entry-creator

Create structured journal entries with YAML frontmatter, template-based sections, and compliance validation. Use when user asks to 'create journal entry', 'new journal', 'document [topic]', 'journal about [topic]', or needs to create timestamped .md files in YYYY/MM/ directories. Supports four entry types: general journal entries, troubleshooting sessions, learning notes, and article summaries. Keywords: journal, documentation, troubleshooting, learning, article-summary, YAML frontmatter, template schemas, validation.

| Skill | Rating |
| --- | --- |
| [journal-entry-creator](/tekhne/skills/documentation/journal-entry-creator/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### acceptance-criteria

Write clear, testable acceptance criteria for user stories and feature delivery; use when defining done conditions, creating measurable requirements, applying INVEST checks, documenting negative scenarios, and aligning product, engineering, and QA on expected outcomes.

| Skill | Rating |
| --- | --- |
| [acceptance-criteria](/tekhne/skills/documentation/acceptance-criteria/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### conventional-commits

Skill for creating structured, semantic commit messages following the Conventional Commits specification

| Skill | Rating |
| --- | --- |
| [conventional-commits](/tekhne/skills/documentation/conventional-commits/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

---

## Package Management (1 tile)

Package & version management

### mise-complete

Configure and operate Mise for deterministic developer environments. Use when installing runtime/tool versions, defining reusable tasks, managing layered environment variables, migrating from asdf/nvm/pyenv, or debugging mise.toml behavior in CI and local shells. Keywords: mise, mise.toml, tool versions, tasks, env, asdf migration, setup automation, dev environment.

| Skill | Rating |
| --- | --- |
| [mise-complete](/tekhne/skills/package-mgmt/mise-complete/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

---

## Project Management (4 tiles)

Planning & organization

### implementation-planner

Converts a PRD or requirements document into a structured, phased implementation plan with individual phase files and granular per-task files written to .context/plans/. Also restructures existing monolithic planning documents into digestible, hierarchical directory structures. Creates a root plan index summarising all phases, a numbered phase file per phase, and a numbered task file per task inside each phase directory.

| Skill | Rating |
| --- | --- |
| [implementation-planner](/tekhne/skills/project-mgmt/implementation-planner/skill/) | ![B+](https://img.shields.io/badge/Rating-B+-yellowgreen) |

### create-context-file

Create context files (plans, justifications, scratches) in .context/ directory with unique three-word IDs and frontmatter

| Skill | Rating |
| --- | --- |
| [create-context-file](/tekhne/skills/project-mgmt/create-context-file/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### moscow-prioritization

Prioritize product requirements with the MoSCoW framework in a deterministic way.  Use when teams need to define MVP scope, sequence releases, resolve stakeholder conflicts,  prevent scope creep, or rebalance backlog under time, budget, or staffing constraints.  Keywords: moscow, must should could wont, requirement prioritization, backlog, mvp,  release planning, scope control, stakeholder alignment.

| Skill | Rating |
| --- | --- |
| [moscow-prioritization](/tekhne/skills/project-mgmt/moscow-prioritization/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### implementation-plan-splitter

Merged into implementation-planner. Redirects to the unified skill that handles both creating new plans and restructuring existing monolithic planning docs into hierarchical directory structures.

| Skill | Rating |
| --- | --- |
| [implementation-plan-splitter](/tekhne/skills/project-mgmt/implementation-plan-splitter/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

---

## Specialized (3 tiles)

Domain-specific tools

### github-copilot-models

Query and display available GitHub Copilot AI models with their capabilities, context limits, and features. Use when: "what models are available", "show copilot models", "list github models", "check model capabilities", "switch models".  Examples: - user: "What models can I use with GitHub Copilot?" → fetch and display available models - user: "Show me models with vision support" → filter models by capability - user: "Which model has the largest context window?" → compare model specifications - user: "List all GPT-5 models" → filter by model family

| Skill | Rating |
| --- | --- |
| [github-copilot-models](/tekhne/skills/specialized/github-copilot-models/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |

### gitlab-api

Retrieve and analyze GitLab merge request comments and metadata using authenticated API calls

| Skill | Rating |
| --- | --- |
| [gitlab-api](/tekhne/skills/specialized/gitlab-api/skill/) | ![C+](https://img.shields.io/badge/Rating-C+-orange) |

### colyseus-multiplayer

Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when implementing rooms, schema state sync, client message validation, matchmaking, authentication, reconnection handling, or server-side anti-cheat constraints. Keywords: colyseus, room lifecycle, schema, multiplayer, websocket, matchmaking, onJoin, onLeave, onDrop, allowReconnection.

| Skill | Rating |
| --- | --- |
| [colyseus-multiplayer](/tekhne/skills/specialized/colyseus-multiplayer/skill/) | ![B](https://img.shields.io/badge/Rating-B-yellow) |
