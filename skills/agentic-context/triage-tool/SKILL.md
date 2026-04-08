---
name: triage-tool
description: "Triage a context management tool or library into a structured reference summary for the agentic-context research repo. Use when given a GitHub URL, npm package, PyPI package, CLI tool, MCP server, or library name to assess context management relevance. Creates references/{slug}.md with architecture overview and scope assessment; adds REVIEWED.md entry and REFERENCE_INDEX.md row. Triggers: triage tool, add tool, analyse tool, context window tool, prompt compression library, token budgeting cli, context injection framework, session lifecycle tool, MCP context server, context daemon, context sandbox, context vault, rolling summary tool, context compressor, eviction policy tool, context pruning library, token counter, sliding window manager."
allowed-tools: Read, Write, Edit, Bash, WebFetch
---

# Triage Tool

Add a new tool or library to the agentic-context research repo as a structured reference summary.

## When to Use

- User provides a GitHub URL, npm/PyPI package name, or tool name
- User says "triage this tool", "add this tool", "analyse this tool/library"
- Evaluating whether a tool belongs in the repo

## When Not to Use

- The tool has already been triaged (check `REVIEWED.md` first)
- The tool is clearly out of scope (not related to active context window management)
- User wants a full deep-dive analysis — use `triage-tool` first, then promote to `ANALYSIS-<name>.md`

## Mindset

Triage is a quality gate, not a catalogue entry. The goal is a reproducible, honest assessment.

- **Scope before summary**: confirm the tool touches the active context window layer before investing in a full write-up. A well-reasoned out-of-scope ruling is as valuable as a full summary.
- **Evidence first**: quote README and docs; never infer capabilities not described by the author.
- **Triage-then-promote**: every tool enters via `REVIEWED.md`; promotion to `ANALYSIS-*.md` and vendoring both require explicit user confirmation — never automatic.

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

Assign one or more tags from this controlled list:

| Tag | Meaning |
|---|---|
| `compression` | Reduces token count (summarisation, pruning, compaction) |
| `tiered-loading` | Priority-based injection (L0/L1/L2, lazy vs eager) |
| `token-budgeting` | Hard caps, soft priorities, eviction policies |
| `injection` | How content enters the context window |
| `cli` | Operated via command-line interface |
| `daemon` | Runs as a background process |
| `mcp-server` | Exposes capabilities via MCP protocol |
| `session-lifecycle` | Manages wake/sleep/checkpoint across sessions |
| `retrieval` | RAG / retrieval-augmented context feeding |

### 4. Assess scope fit

Before writing the summary, confirm the tool is in scope for this repo:

- **In scope**: tools that manage what's in the active context window (compression, tiered loading, token budgeting, injection, session continuity).
- **Borderline**: tools focused on long-term memory storage — note the overlap and assess the injection/context layer specifically.
- **Out of scope**: pure vector DBs, embedding services, or tools with no context-window-layer involvement.

If borderline or out of scope, note this clearly in the summary and REVIEWED.md disposition.

### 5. Fill in the reference summary

Create `references/<slug>.md` using `templates/REFERENCE-tool.md`. Populate every section:

- **TL;DR**: 3–8 bullets capturing what the tool does and why it matters for context management.
- **What's novel**: one paragraph — what does this do that adjacent tools do not?
- **Architecture overview**: describe context representation, injection mechanism, compression/summarisation, eviction/overflow handling, session lifecycle. Skip sections that are not applicable (mark `N/A`).
- **Deployment model**: runtime, language, dependencies, storage.
- **Self-reported metrics**: quote numbers with source — always mark `(as reported)`.
- **Open questions**: gaps, risks, unverified claims, missing benchmarks.

Keep language precise. Do not pad. Mark all unverified claims.

### 6. Decide on vendoring

Ask the user whether to vendor the repo into `tools/`:

- **Vendor**: clone the repo as a git submodule into `tools/<repo-name>/` and record the pinned commit in the reference frontmatter `local_clone` field. Do this for tools that warrant code-level inspection.
- **Reference only**: link to the upstream repo without vendoring. Do this for tools that are large, frequently updated, or clearly peripheral.

Do not vendor without confirmation.

### 7. Update REVIEWED.md

Add a row to the summary table at the top (reverse-chronological):

```
| <today's date> | <slug> | tool | pending | <one-line description> |
```

Add a detailed section below the table:

```markdown
## <slug> — <Tool name>

- **Repo**: <URL>
- **Author**: <author/org>
- **Version**: <vX.Y.Z / commit>
- **Language**: <language>
- **Tags**: <tags>
- **Summary**: <2–3 sentences>
- **Scope fit**: in scope / borderline / out of scope — <reason>
- **Disposition**: pending — awaiting user decision on promotion
```

### 8. Update REFERENCE_INDEX.md

Add a row under the most relevant category table.

### 9. Report and offer next step

Summarise what was created. Then ask:

> This tool is now in `REVIEWED.md` as **pending**. Would you like to:
> - **Promote** it to a standalone `ANALYSIS-<slug>.md` deep dive?
> - **Vendor** the repo into `tools/` for code-level inspection?
> - **Keep** it in REVIEWED.md for now?
> - **Skip** it (mark as not promoted with reasoning)?

## Quick Commands

```bash
# Check for duplicate before starting
grep -i "<slug-or-repo-url>" REVIEWED.md references/REFERENCE_INDEX.md

# Fetch tool README
curl -s "https://raw.githubusercontent.com/<owner>/<repo>/main/README.md"

# Create reference file from template
cp templates/REFERENCE-tool.md references/<slug>.md

# Validate the completed reference file
sh /path/to/skill/scripts/validate-reference-tool.sh references/<slug>.md

# Vendor as submodule (only after explicit user confirmation)
git submodule add <repo-url> tools/<repo-name>

# Add triage entry to REVIEWED.md (reverse-chronological)
# | YYYY-MM-DD | <slug> | tool | pending | <one-line description> |
```

## Anti-Patterns

### NEVER treat self-reported benchmarks as verified

**WHY:** Tool benchmarks are often run on curated inputs under favourable conditions.

**BAD:** "Achieves 60% token reduction."
**GOOD:** "Reports 60% token reduction on their internal benchmark (as reported, README)."

### NEVER skip the duplicate check

**WHY:** Re-triaging creates conflicting entries and wastes effort.

**BAD:** Creating a new references file without checking REVIEWED.md.
**GOOD:** `grep -i "<slug>" REVIEWED.md references/REFERENCE_INDEX.md` first.

### NEVER vendor without user confirmation

**WHY:** Vendoring changes git state and submodule config — a deliberate action, not automatic.

**BAD:** Running `git submodule add` during triage.
**GOOD:** Offer vendoring as a follow-up option after the summary is written.

### NEVER promote without user confirmation

**WHY:** Promotion to ANALYSIS-*.md is a quality gate, not automatic.

**BAD:** Creating ANALYSIS-*.md as part of triage.
**GOOD:** Triage to REVIEWED.md, then ask the user.

## References

- [REFERENCE-tool template](references/reference-tool-template.md) — frontmatter fields, section structure, and body conventions; use every time you create a new `references/<slug>.md` in the consuming project
- [ANALYSIS-tool template](references/analysis-tool-template.md) — 3-stage deep-dive structure and frontmatter; use when promoting a tool to a standalone ANALYSIS-*.md
- [REFERENCE-tool YAML template](assets/templates/REFERENCE-tool.yaml) — machine-readable description of the expected output structure (fields, sections, constraints)
- [reference-tool.schema.json](assets/schemas/reference-tool.schema.json) — JSON Schema for validating `references/<slug>.md` frontmatter
- [validate-reference-tool.sh](scripts/validate-reference-tool.sh) — run after creating a reference to verify frontmatter, required sections, TL;DR bullet count, and unfilled placeholders
