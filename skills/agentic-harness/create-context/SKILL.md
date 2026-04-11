---
name: create-context
description: Create baseline context from .context/session/in/ folder with manifest-driven organization (run once per project). Use when bootstrapping project context, setting up .context/session/ctx/ snapshot. Triggers include "create context", "bootstrap context", "setup context", "init context".
argument-hint: "[--force to overwrite existing .context/session/ctx/]"
allowed-tools: read, write, glob, bash, askuserquestion, task
context: main
user-invocable: true
---

# Create Context

Bootstrap project context from `.context/session/in/` folder → `.context/session/ctx/` snapshot + baseline + RISEN INPUT table.

## ⚠️ AskUserQuestion Guard

**CRITICAL**: After EVERY `AskUserQuestion` call, check if answers are empty/blank. Known Claude Code bug: outside Plan Mode, AskUserQuestion silently returns empty answers without showing UI.

**If answers are empty**: DO NOT proceed with assumptions. Instead:
1. Output: "⚠️ Questions didn't display (known Claude Code bug outside Plan Mode)."
2. Present the options as a **numbered text list** and ask user to reply with their choice number.
3. WAIT for user reply before continuing.

## Folder Architecture

- **`.context/session/in/`**: Immutable bootstrap (user dumps raw docs, never modified by commands)
- **`.context/session/ctx/`**: Actionable snapshot (generated: manifest + summaries + copied files)

## Steps

### 1. Prerequisites

```bash
[ -d .context/session/in/ ] || error "No .context/session/in/ folder found. Create it and add source files first."
[ -d .context/session/ctx/ ] && [ "$1" != "--force" ] && error ".context/session/ctx/ exists. Use --force to recreate."
```

Skip security-sensitive files: `.env*`, `*credentials*`, `*secrets*`, `*token*`, `*.key`, `*.pem`, `*.crt`

### 2. Scan & Prioritize

1. Glob: `.context/session/in/**/*.{md,txt,csv,yaml,json}`
2. AskUserQuestion per file: HIGH / MEDIUM / LOW + brief description

### 3. Create .context/session/ctx/ and Generate Manifest

Write `.context/session/ctx/manifest.yaml` — see `reference.md` for schema.

### 4. Context Sizing & Copy

Token estimation: `tokens ≈ words / 0.75` (via `wc -w`)

| Priority | ≤ threshold | > threshold, ≤25K | > 25K |
|----------|-------------|-------------------|-------|
| HIGH | ≤1500 → inline | summarize directly | summarize via sub-agent |
| MEDIUM | ≤2500 → inline | summarize directly | summarize via sub-agent |
| LOW | reference only | — | — |

Copy HIGH/MEDIUM files to `.context/session/ctx/`, preserving subdirectory structure.

### 5. Summarize (if needed)

- Direct: Read + generate ~500 token summary → `.context/session/ctx/{nn}-{basename}-summary-llm.md`
- Sub-agent: Task(`summarize-for-context`) for files >25K tokens

### 6. Write Baseline

Create `.context/session/CONTEXT-baseline-llm.md` with inline content + summary refs + LOW references.
Target: ≤2000 tokens.

### 7. Output

Report: file counts, RISEN INPUT table, suggest `/save-context baseline`.

See `reference.md` for manifest schema, baseline template, and validation rules.

## Philosophy

- Context is **scoped to a session's open questions**, not a transcript of activity — include only what shapes decisions.
- **In-folder as the source of truth** — everything in `.context/session/in/` is intentional input; the manifest reflects it faithfully.
- **One manifest per session** — multiple contexts in the same directory cause ambiguity for load-context.
- **Explicit over implicit** — every constraint, assumption, and decision captured in context must state its origin.

## When to Use

- At the start of a new project or feature branch when raw source materials have been placed in `.context/session/in/` and no snapshot exists yet.
- When multiple agents need a shared, prioritized view of project documents to avoid each agent independently re-reading every file.
- When the `.context/session/ctx/` directory is missing or empty and a downstream skill (e.g., `load-context`) cannot find a manifest.
- When source files in `.context/session/in/` have been replaced or significantly updated and a fresh snapshot is required (use `--force`).
- When onboarding a new session after a project handoff and the inherited documents must be classified and summarized before work begins.

## When Not to Use

- When `.context/session/ctx/` already contains a current manifest and the source files have not changed — re-running without `--force` is a no-op and may discard existing summaries.
- When `.context/session/in/` is empty; there is nothing to scan and the skill will error.
- When the goal is to update a single document's summary rather than rebuild the full snapshot — edit the relevant file and update the manifest entry directly.
- When working in a read-only environment where writing to `.context/session/ctx/` is not permitted.
- When a live session context already exists and only needs to be saved to a named stream — use `save-context` instead.

## Anti-Patterns

- **NEVER create context without scanning `.context/session/in/`** — Skipping the input scan produces incomplete context. **Why:** Source materials in the in-folder define the scope; a context built without them is speculative and will mislead downstream agents.
- **NEVER write a manifest without all three priority sections (high/medium/low)** — Omitting a section causes validation failures in `validate-manifest.sh`. **Why:** The schema requires all three arrays to be present, even if empty, so that consumers can iterate predictably.
- **NEVER copy security-sensitive files into `.context/session/ctx/`** — Files matching `.env*`, `*credentials*`, `*secrets*`, `*token*`, `*.key`, `*.pem`, `*.crt` must be skipped. **Why:** The ctx snapshot may be committed or shared; leaking credentials through it is a serious security risk.
- **NEVER inline files that exceed the token threshold** — Files above 1500 tokens (HIGH) or 2500 tokens (MEDIUM) must be summarized, not inlined verbatim. **Why:** Over-sized baselines break the 2000-token budget and degrade agent performance on every subsequent load.
- **NEVER run the skill concurrently with `save-context` or `load-context`** — Parallel writes to `.context/session/ctx/` corrupt the manifest. **Why:** There is no locking mechanism; the last writer wins and partial manifests are silently invalid.

## Usage Examples

**Creating context at session start:**

```bash
# Drop source files into the immutable in-folder
cp design-doc.md .context/session/in/
cp requirements.csv .context/session/in/

# Invoke the skill — it scans in-folder, classifies files, writes manifest
# Output: .context/session/ctx/manifest.yaml + summaries + CONTEXT-baseline-llm.md
```

**Force-recreating context after source files change:**

```bash
# Update a source document
cp updated-spec.md .context/session/in/spec.md

# Re-run with --force to overwrite the existing ctx snapshot
# The skill will re-scan, re-prioritize, and rewrite all ctx artefacts
```

**Inspecting the generated manifest and baseline:**

```bash
# After the skill completes, review the generated artefacts
ls .context/session/ctx/
# manifest.yaml  01-design-doc-summary-llm.md  ...

# The baseline provides a single load point for all context
cat .context/session/CONTEXT-baseline-llm.md
```

## References

- [Reference](reference.md) — manifest schema, context sizing rules, baseline template, validation rules, and security skip patterns
