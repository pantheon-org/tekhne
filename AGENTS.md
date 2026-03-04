# Tekhne Agent Collaboration Guide

This file defines how LLM agents should work in this repository.

## Project Purpose

- Maintain a curated collection of reusable agent skills under `skills/`.
- Keep skill content modular, discoverable, and easy to lint/validate.
- Prefer small, targeted edits over broad rewrites.

## Repository Map

- `skills/<domain>/<skill-name>/SKILL.md`: Primary entry point for a skill.
- `skills/<domain>/<tool>/{generator,validator}/SKILL.md`: Generator/validator pairs (consolidated under `<tool>/tile.json`).
- `skills/<domain>/<skill-name>/AGENTS.md`: Optional deep navigation for that skill.
- `skills/<domain>/<skill-name>/references/`: Focused reference documents.
- `skills/<domain>/<skill-name>/scripts/`: Utility scripts used by a skill.
- `skills/<domain>/<skill-name>/templates/`: Reusable templates (mostly YAML).
- `skills/<domain>/<skill-name>/schemas/`: JSON schemas for validation.
- `.context/`: Working notes and generated analysis artifacts.
- `cli/`: TypeScript CLI tool (replaced shell scripts) for skill management, auditing, and installation.

**Note:** Generator/validator pairs are consolidated at the tool level. For example, `terraform-generator` and `terraform-validator` are both referenced in `skills/infrastructure/terraform/tile.json` as a single `terraform-toolkit` tile.

## Domain Organization

Skills are organized into 12 domains for improved discoverability:

- **ci-cd/** - CI/CD pipelines (GitHub Actions, GitLab CI, Jenkins, Helm, FluentBit, Azure Pipelines)
- **infrastructure/** - IaC tools (Terraform, Ansible, K8s, Docker, CFN, AWS CDK)
- **repository-mgmt/** - Repository management (Nx workspace tools, future: Turborepo, Lerna, Git workflows)
- **development/** - Dev tooling (Bun, Biome, TypeScript, Commander.js, shell scripting)
- **agentic-harness/** - Agent framework configs (OpenCode, AGENTS.md, future: Cursor, Claude Desktop)
- **testing/** - Testing & quality (BDD, TDD, skill auditor, UI debugging)
- **software-engineering/** - Engineering principles (design patterns, architecture, SOLID, refactoring)
- **observability/** - Monitoring & debugging (PromQL, LogQL, K8s debug)
- **documentation/** - Writing & communication (Markdown, acceptance criteria, commits, plain English)
- **package-mgmt/** - Package/version management (Mise, future: npm, pip, cargo)
- **project-mgmt/** - Planning & organization (Moscow, plan splitter, context files)
- **specialized/** - Domain-specific tools (Colyseus, GitHub Copilot models, GitLab API)

See `skills/agentic-harness/skill-quality-auditor/references/skill-taxonomy.md` for detailed classification criteria and decision tree.

## Required Workflow For Agents

1. Read the relevant `SKILL.md` before editing files in that skill.
2. Load only the minimum additional files needed for the task.
3. Reuse existing templates/scripts instead of recreating content.
4. Keep changes scoped to the user request; do not refactor unrelated skills.
5. Run validation commands before finishing.

## Authoring Rules

- Use kebab-case for skill directory names.
- Keep instructions explicit, actionable, and deterministic.
- Prefer short sections and predictable headings.
- Store deep details in `references/` and keep `SKILL.md` as a navigation hub.
- Use Markdown fenced code blocks with language tags when applicable.
- Use ASCII unless a file already requires Unicode.
- Files under `skills/<skill-name>/templates/` must use YAML extensions (`.yaml` or `.yml`).
- Files under `skills/<skill-name>/schemas/` must be JSON Schema files named `*.schema.json` and include a `"$schema"` URL from `json-schema.org`.
- Files under `skills/<skill-name>/scripts/` must be executable scripts in one of the accepted languages:
  - Shell (`.sh`): `#!/usr/bin/env sh` (POSIX) or `#!/usr/bin/env bash` (with `# shell: bash` opt-in marker).
  - Python (`.py`): `#!/usr/bin/env python3`.
  - TypeScript (`.ts`): `#!/usr/bin/env bun`.
  - JavaScript (`.js`): `#!/usr/bin/env node` or `#!/usr/bin/env bun`.
- Skills must be self-contained: SKILL.md must not reference files outside the skill's own directory tree (no `../` paths, no absolute `skills/X/Y` paths, no `.context/` or `.agents/` references). Content inside fenced code blocks is exempt.

### Tessl Registry Preparation

When preparing skills for [Tessl](https://tessl.io/) submission, follow these additional guidelines:

- **Performance focus**: Write skills that provide measurable effectiveness improvements
- **Agent agnostic**: Avoid features specific to individual AI assistants
- **Quality threshold**: Target A-grade scores (>=126/140 points) using skill-quality-auditor
- **Single responsibility**: Each skill should solve one well-defined problem domain
- **Comprehensive examples**: Include practical use cases with expected outcomes
- **Cross-platform testing**: Validate functionality across multiple agents when possible
- **Clear success metrics**: Define what "effective use" looks like for evaluation

## Validation Commands

Run these from repository root:

```bash
bunx @biomejs/biome check .
bunx markdownlint-cli2 "**/*.md"
```

For artifact convention checks (`templates/`, `schemas/`, `scripts/`), use the `skill-quality-auditor` workflow documented in `skills/agentic-harness/skill-quality-auditor/SKILL.md`.

## Skill Quality Audits

All skills must be audited with `skill-quality-auditor` before publishing or committing major changes. This ensures consistent quality across the repository and identifies improvement opportunities.

### Required Audit Workflow

**Before publishing or submitting significant skill changes:**

```bash
# Run quality audit (creates .context/audits/<domain>/<skill-name>/latest/)
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store

# Review results
cat .context/audits/<domain>/<skill-name>/latest/analysis.md
cat .context/audits/<domain>/<skill-name>/latest/remediation-plan.md
```

### Quality Gates

Skills are evaluated on 9 dimensions (140 total points) and assigned grades:

| Grade | Score Range | Status | Action |
|-------|-------------|--------|--------|
| **A** | >=126/140 (90%) | Publication-ready | Publish immediately |
| **B+** | 119-125/140 (85-89%) | Near-ready | Optional improvements, can publish |
| **B** | 112-118/140 (80-84%) | Solid foundation | Review remediation plan, improve if time allows |
| **C+/C** | <112/140 (<80%) | Needs improvement | **BLOCK publishing** until remediated |

**Target for all skills:** B-grade minimum (112/140) before publishing to Tessl registry.

### Nine Quality Dimensions

1. **Knowledge Delta (D1):** How much new, useful information does the skill provide?
2. **Mindset & Procedures (D2):** Does it establish proper mental models and workflows?
3. **Anti-Pattern Quality (D3):** Are common mistakes documented with alternatives?
4. **Specification Compliance (D4):** Does it follow Agent Skills specification format?
5. **Progressive Disclosure (D5):** Is information architecture clear and layered?
6. **Freedom Calibration (D6):** Are boundaries and constraints properly defined?
7. **Pattern Recognition (D7):** Are examples and use cases well-structured?
8. **Practical Usability (D8):** Is the skill actionable and immediately useful?
9. **Eval Validation (D9):** Has the skill been validated at runtime via tessl eval scenarios?

### Prohibited Practices

**DO NOT:**
- Publish skills without running `skill-quality-auditor` evaluation
- Skip remediation for skills scoring <112/140 (B-grade threshold)
- Rely solely on `tessl skill review` scores as quality validation
- Bypass audit because "it looks good enough"
- Commit major skill changes without re-auditing

**Rationale:** Between 2025-2026, 63 skills were published using only `tessl skill review`, resulting in:
- Average score: 98.3/120 (82%) - below B+ threshold (scores predate D9 addition; re-audit on 140-point scale pending)
- 37% of skills in C+/C range (needs significant improvement)
- Critical weaknesses: Anti-Pattern Quality (D3: 68%), Progressive Disclosure (D5: 73%)
- 40-60 hours of remediation work required to lift quality to acceptable levels

### Common Weak Dimensions

Repository-wide audit (2026-03) identified three dimensions that consistently need attention:

**D3 Anti-Pattern Quality (68% avg)** - CRITICAL
- Add "Common Mistakes" or "Anti-Patterns" section to every skill
- Document 3-5 anti-patterns with explanations of why they fail
- Provide correct alternatives for each anti-pattern

**D5 Progressive Disclosure (73% avg)** - HIGH PRIORITY
- Structure information: Quick Start → Detailed Guide → Advanced Topics
- Use clear heading hierarchy (H2 → H3 → H4)
- Implement "What → Why → How" organization

**D2 Mindset & Procedures (74% avg)** - HIGH PRIORITY
- Establish mental models before diving into procedures
- Explain the "why" behind recommendations
- Provide decision frameworks, not just step-by-step instructions

### Audit Automation

Check audit status across all skills using the CLI tool:

```bash
# Audit a single skill
bun cli/index.ts audit skill <domain>/<skill-name>

# Audit all skills
bun cli/index.ts audit all

# Check audit status and compliance
bun cli/index.ts audit status

# Generate comprehensive summary report
bun cli/index.ts audit summary
```

### Integration with Tessl

Both tools serve complementary purposes:

- **skill-quality-auditor**: Internal quality improvement, dimensional guidance, remediation plans
- **tessl skill review**: Registry publication preparation, metadata validation, optimization

**Use both independently.** Do not skip skill-quality-auditor because Tessl scores look high. Phase 1 analysis confirmed no correlation between the two scoring systems.

## Skill Management with Tessl

The repository includes a TypeScript CLI tool at `cli/` that handles the complete tessl lifecycle:

```bash
# Process all skills (import, lint, review, publish)
bun cli/index.ts tessl manage

# Process a specific skill
bun cli/index.ts tessl manage <domain>/<skill-name>

# Use different workspace
bun cli/index.ts tessl manage --workspace=my-org

# Pre-publish validation
bun cli/index.ts tessl publish-check <tile-path>
```

The CLI automatically:

1. **Imports** skills without `tile.json` using `tessl skill import`
2. **Lints and reviews** skills with `tile.json` using `tessl skill lint` and `tessl skill review`
3. **Publishes** skills that pass validation (if not already published)

### Manual Skill Review and Publishing Workflow

When manually reviewing and publishing individual skills to the public Tessl registry:

```bash
# For consolidated tiles (generator/validator pairs, nx-plugin-toolkit):
# Review individual skills by pointing to their directories

# Example: Terraform toolkit
tessl skill review skills/infrastructure/terraform/generator
tessl skill review skills/infrastructure/terraform/validator

# Example: NX plugin toolkit (learning sequence)
tessl skill review skills/repository-mgmt/nx/generators
tessl skill review skills/repository-mgmt/nx/executors
tessl skill review skills/repository-mgmt/nx/extending-plugins

# Or review directly by SKILL.md path
tessl skill review skills/infrastructure/terraform/generator/SKILL.md

# For standalone skills:
tessl skill review skills/<domain>/<skill-name>

# Optimize if score < 90% (critical step!)
tessl skill review skills/<domain>/<skill-name>/generator --optimize

# Publish consolidated tiles at the tile.json level
tessl skill publish skills/infrastructure/terraform --public
```

**Key insights**:
- Always use `--optimize` flag for skills scoring below 90%. This can dramatically improve scores (observed 85% → 99% improvements) by applying Tessl's automatic optimization engine.
- The `--skill` flag only works with remote GitHub URLs, not local paths. For local consolidated tiles, point to the specific skill directory.
- Tile operations (lint, publish) operate on the tile.json directory level.

See `cli/README.md` for detailed usage information.

## Git Hooks

- Pre-commit uses `lefthook` and runs:
- Biome checks on staged JS/TS/JSON files (with `--write`).
- markdownlint on staged `.md` files.
- YAML parse validation with `yq` when available.
- Skill artifact conventions for `templates/`, `schemas/`, and `scripts/`.

Do not bypass hooks unless explicitly requested by the user.

## Multi-Agent Development

This repository follows the [Agent Skills specification](https://agentskills.io) for cross-harness compatibility.

### Installing Skills for Local Development

Contributors can install local skills to their development environment using the CLI tool:

```bash
# Install to local .agents/skills (default for opencode)
bun cli/index.ts install

# Install to specific agents
bun cli/index.ts install -a opencode -a cursor -a gemini

# Install globally for cross-project access (~/.config/<agent>/skills)
bun cli/index.ts install --global

# Preview what would be installed
bun cli/index.ts install --dry-run
```

Skills are symlinked with namespaced names to avoid collisions: `domain--category--skill-name`

**Alternative:** Use the cross-platform `npx skills` tool:
```bash
# Sync all skills to all detected agents
npx skills add ./skills --all

# Sync to specific agents
npx skills add ./skills -a claude-code -a cursor -a gemini-cli

# Sync globally for cross-project access
npx skills add ./skills --all -g
```

### Cross-Agent Compatibility

When authoring skills for multiple agents:

- Use standard `SKILL.md` frontmatter format (compatible with all harnesses)
- The `allowed-tools` field is agent-specific; check support before using
- Avoid agent-specific features in shared skills
- Test with `npx skills add ./skills --dry-run` before committing

### Supported Harnesses

See README.md for the full list of 41+ supported agents via `npx skills`.

## Safety Constraints

- Never delete or rename existing skills unless explicitly asked.
- Never rewrite generated reports in `.context/` unless part of the task.
- If you detect unrelated dirty changes, avoid reverting them and continue safely.
- If repository conventions conflict, prefer explicit user instructions.
