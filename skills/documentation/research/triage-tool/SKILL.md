---
name: triage-tool
description: "Triage a tool or library into a structured reference summary for the research repo. Use when given a GitHub URL, npm package, PyPI package, or CLI tool name. Creates references/{slug}.md with architecture overview and scope assessment; adds REVIEWED.md entry and REFERENCE_INDEX.md row. Triggers: triage tool, add tool, analyse tool, GitHub project, npm package, PyPI package, library, framework, CLI tool, MCP server, open source project."
allowed-tools: Read, Write, Edit, Bash, WebFetch
---

# Triage Tool

Add a new tool or library to the research repo as a structured reference summary.

## When to Use

- User provides a GitHub URL, npm/PyPI package name, or tool name
- User says "triage this tool", "add this tool", "analyse this tool/library"
- Evaluating whether a tool belongs in the repo

## When Not to Use

- The tool has already been triaged (check `REVIEWED.md` first)
- The tool is clearly out of scope (not relevant to the research domain)
- User wants a full deep-dive analysis — use `triage-tool` first, then promote to `ANALYSIS-<name>.md`

## Mindset

Triage is a quality gate, not a catalogue entry. The goal is a reproducible, honest assessment.

1. **Scope before summary**: confirm the tool is relevant to the research domain before investing in a full write-up. A well-reasoned out-of-scope ruling is as valuable as a full summary.
2. **Evidence first**: quote README and docs; never infer capabilities not described by the author.
3. **Triage-then-promote**: every tool enters via `REVIEWED.md`; promotion to `ANALYSIS-*.md` and vendoring both require explicit user confirmation — never automatic.

## Workflow

### 1. Resolve the source

- Fetch the repo README and any linked docs/architecture pages.
- Record: tool name, author/org, license, language, latest version/commit, primary URL.
- Derive a stable slug: `<author-or-org>-<tool-name>` (e.g. `versatly-clawvault`, `press-longchat`).
  For well-known single-name tools, use just the tool name (e.g. `context-mode`, `memv`).

### 2. Check for duplicates

- Search `REVIEWED.md` and `references/REFERENCE_INDEX.md` for the slug or repo URL.
- If already present, report it and stop.

### 3. Classify the tool

Assign one or more tags from the controlled list. See [classification guide](references/classification-guide.md) for the full taxonomy and scope assessment criteria.

### 4. Assess scope fit

Confirm the tool is relevant to the research domain. See [classification guide](references/classification-guide.md) for in-scope / borderline / out-of-scope criteria. Note any borderline ruling in REVIEWED.md.

### 5. Fill in the reference summary

Create `references/<slug>.md` using `templates/REFERENCE-tool.md`. Populate every section. Frontmatter example:

```yaml
title: "MemGPT"
author: "Packer et al."
date: 2023-10-12
type: reference
tags: [tool, library]
source: "https://github.com/cpacker/MemGPT"
```

- **TL;DR**: 3–8 bullets capturing what the tool does and why it matters.
- **What's novel**: one paragraph — what does this do that adjacent tools do not?
- **Architecture overview**: describe the core design and how the tool works. Skip sections that are not applicable (mark `N/A`).
- **Deployment model**: runtime, language, dependencies, storage.
- **Self-reported metrics**: quote numbers with source — always mark `(as reported)`.
- **Open questions**: gaps, risks, unverified claims, missing benchmarks.

Keep language precise. Do not pad. Mark all unverified claims.

### 6. Decide on vendoring

Ask whether to vendor the repo. Clone as a git submodule into `tools/<repo-name>/` (record the pinned commit in `local_clone`) for tools warranting code-level inspection, or link-only for large or peripheral tools. Do not vendor without confirmation.

### 7. Update REVIEWED.md

Add a row to the summary table at the top (reverse-chronological):

```
| <today's date> | <slug> | tool | pending | <one-line description> |
```

### 8. Update REFERENCE_INDEX.md

Add a row under the most relevant category table.

### 9. Report and offer next step

Summarise what was created. Ask the user whether to promote to `ANALYSIS-<slug>.md`, vendor into `tools/`, keep as pending, or skip with reasoning.

## Quick Commands

```bash
# Check for duplicate before starting
grep -i "<slug-or-repo-url>" REVIEWED.md references/REFERENCE_INDEX.md

# Fetch tool README
curl -s "https://raw.githubusercontent.com/<owner>/<repo>/main/README.md"

# Create reference file from template
cp templates/REFERENCE-tool.md references/<slug>.md

# Validate the completed file
./scripts/validate-reference-tool.sh references/<slug>.md
./scripts/validate-analysis-tool.sh ANALYSIS-<slug>.md

# Vendor as submodule (only after explicit user confirmation)
git submodule add <repo-url> tools/<repo-name>

# | YYYY-MM-DD | <slug> | tool | pending | <one-line description> |
```

## Anti-Patterns

### NEVER treat self-reported benchmarks as verified

**WHY:** Tool benchmarks are often run on curated inputs under favourable conditions.

**BAD** `"Achieves 40% latency reduction."` → **GOOD** `"Reports 40% latency reduction (as reported, README)."`

### NEVER skip the duplicate check

**WHY:** Re-triaging creates conflicting entries and wastes effort.

**BAD** Create new file without checking REVIEWED.md. → **GOOD** Run `grep -i "<slug>" REVIEWED.md references/REFERENCE_INDEX.md` first.

### NEVER vendor without user confirmation

**WHY:** Vendoring changes git state and submodule config — a deliberate action, not automatic.

**BAD** Run `git submodule add` during triage. → **GOOD** Offer vendoring as a follow-up after the summary is written.

### NEVER promote without user confirmation

**WHY:** Promotion to ANALYSIS-*.md is a quality gate, not automatic.

**BAD** Create ANALYSIS-*.md as part of triage. → **GOOD** Triage to REVIEWED.md, then ask the user.

## References

- [classification guide](references/classification-guide.md) — tag taxonomy and scope assessment criteria (in/borderline/out); customise for your research domain
- **Reference artifacts**: [YAML template](assets/templates/REFERENCE-tool.yaml) · [schema](assets/schemas/reference-tool.schema.json) · [validator](scripts/validate-reference-tool.sh)
- **Analysis artifacts**: [YAML template](assets/templates/ANALYSIS-tool.yaml) · [schema](assets/schemas/analysis-tool.schema.json) · [validator](scripts/validate-analysis-tool.sh)
