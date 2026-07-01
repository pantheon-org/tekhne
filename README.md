# Tekhne - Agent Skills Repository

A curated collection of reusable agent skills for AI assistants, designed for easy redistribution and integration.

## What are Agent Skills?

Agent skills are modular instruction packages that extend AI assistant capabilities. Each skill provides specialized
domain knowledge, workflows, and best practices that can be loaded on-demand.

## Skill Catalog

<!-- skill-catalog-stats -->
Browse all **0 skills across 0 tiles** in the [Skill Catalog](https://pantheon-org.github.io/tekhne/tiles/).

## CLI Tool

The repository includes a TypeScript CLI tool built with Cliffy and Bun for managing skills:

```bash
# Install skills locally
bun cli/index.ts install

# Audit skill quality
bun cli/index.ts audit all
bun cli/index.ts audit status
bun cli/index.ts audit summary

# Update the skill catalog docs page
bun cli/index.ts readme update

# Manage Tessl lifecycle
bun cli/index.ts tessl manage
bun cli/index.ts tessl publish-check <tiles...>
```

See [`cli/README.md`](cli/README.md) for complete documentation.

## skill-auditor (Go binary)

`tools/skill-auditor/` is a typed, testable Go binary that replaces the `evaluate.sh` shell script. It implements the same 9-dimension rubric (140 pts total) with unit tests and a structured JSON output.

```bash
# Build once
bun run build:skill-auditor

# Evaluate a single skill
skill-auditor evaluate agentic-harness/skill-quality-auditor --json --store

# Batch evaluate
skill-auditor batch infrastructure/terraform-generator ci-cd/github-actions-generator --store

# Exit 1 if any skill grades below B+
skill-auditor batch --fail-below B+ agentic-harness/skill-quality-auditor
```

The shell scripts (`evaluate.sh`, `batch-audit.sh`) remain as fallback. See `tools/skill-auditor/` for source.
