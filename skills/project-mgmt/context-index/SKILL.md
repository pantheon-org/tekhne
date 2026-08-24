---
name: context-index
description: "Regenerates the project's context index (index.yaml) from the YAML frontmatter across every context file, grouping entries by typology (findings/plans/guides/follow-ups/merge-requests/tickets/decisions/notes/research), and validates that each file carries the required frontmatter fields. Use when the index is stale, context files were added, renamed, or removed, or a pre-commit gate blocks a commit because a context file is missing frontmatter. Do not use it to create new context files (use create-context-file instead), to hand-edit the index, or as a substitute for fixing frontmatter at the source."
---

# Context Index

Scan, validate, and index every context file the project has captured.

## Mindset

The index is a cache, never a ledger. The source of truth for what exists is
always the YAML frontmatter inside each context file; the generated index is
a derived summary that lets an agent answer "what plans/findings/decisions
exist?" without reading every file. Because it is derived, regeneration is
idempotent and safe to run whenever there is doubt about freshness — there is
no state to corrupt, only a file to rewrite from scratch.

A file with missing or malformed frontmatter is not a formatting nitpick: it
is invisible to every tool and agent that reads the index instead of the raw
directory tree. Treat exclusion-from-index as the real failure mode, not the
warning text that reports it. Balance that with proportionate language — flag
the gap clearly (ALWAYS regenerate after a change, NEVER hand-edit the index)
without turning every missing tag into a blocking emergency.

## Prerequisites

- One or more context files, each with frontmatter that is valid or fixable.
- The `create-context-file` skill (companion to this one) to add or repair
  frontmatter on files that lack it, using its typology set and templates.
- Shell access (`bash`, `python3`, `git`) to run this skill's scripts.

## When to Use

- After creating, renaming, or deleting a context file.
- When a pre-commit or CI gate blocks with a message like "missing YAML
  frontmatter" or "index is stale."
- When asked "what plans / findings / decisions / follow-ups exist?" — read
  the index rather than walking the directory tree by hand.
- After a bulk import, migration, or retroactive frontmatter cleanup touching
  many context files at once.

## When Not to Use

- Do not run this as a substitute for creating a missing file — create it
  first with the `create-context-file` skill, which owns the filename and
  frontmatter shape.
- Do not edit the generated index by hand — it is fully regenerated and any
  manual edit is silently overwritten on the next run.
- Do not treat a clean regeneration as proof every context file is meaningful
  or current — it only proves the frontmatter parses; stale content still
  needs a human to retire it.

## Frontmatter Schema

Every context file is expected to carry this block (the same shape
`create-context-file` writes):

```yaml
---
title: "Human-readable title"
type: finding | plan | guide | follow-up | merge-request | ticket | decision | note | research
date: YYYY-MM-DD
status: active | done
tags: []
related:
  - ../relative/path/to/related.md # omit the whole key if there is nothing related
---
```

`type:` is the **singular** form of the plural typology folder a file lives
under (`findings/` → `finding`, `follow-ups/` → `follow-up`, `research/` →
`research`). The index groups entries back into the plural form for display.
`title`, `type`, `status`, and `date` are required for a file to appear in
the index; `tags` and `related` are optional and only rendered when
non-empty.

## Procedure

1. **Check for missing or malformed frontmatter first**, running
   `scripts/check-context-frontmatter.sh` against every context file. —
   **Verify:** the script prints nothing and exits 0. If it lists files,
   those are invisible to the index until fixed.
2. **Fix anything the check flagged**, using the `create-context-file`
   typology set and the schema above — not by inventing an ad hoc shape.
3. **Validate against the full schema when precision matters** (e.g. before a
   CI gate), running `scripts/validate-context-frontmatter.sh` against the
   changed files — this also catches an out-of-enum `type` or `status` value
   that the looser check above would miss.
4. **Regenerate the index**, running `scripts/regenerate-context-index.sh`. —
   **Stop if:** it reports files excluded on stderr; go back to step 2 rather
   than treating the run as complete.
5. **Stage the regenerated index alongside any context-file change** in the
   same commit.

## Quick Commands

```bash
# Full session: check, fix, regenerate, stage
./scripts/check-context-frontmatter.sh .context/**/*.md
# (fix anything reported, using create-context-file's schema)
./scripts/regenerate-context-index.sh
git add .context/index.yaml
```

Expected result: a line reporting how many entries were generated and where
the index was written, with zero stderr warnings.

```bash
# Strict schema validation (enum + date-pattern checks) before a CI gate
./scripts/validate-context-frontmatter.sh .context/**/*.md
```

Expected result: `OK: <N> file(s) validated against schema`, or a list of
per-file errors with exit code 1.

```bash
# Enforce the date-first filename convention on known typology directories
./scripts/check-context-filenames.sh
```

Expected result: `context filenames OK`, or a list of naming violations.

```bash
# Advisory: surface active plans/follow-ups that have gone stale
./scripts/check-plan-staleness.sh
```

Expected result: a notice listing entries older than the threshold (default
60 days via `CONTEXT_STALENESS_THRESHOLD_DAYS`), or no output. Always exits 0
— this is a nudge, not a gate.

## Anti-Patterns

### NEVER edit the generated index by hand

**WHY:** it is fully regenerated by `regenerate-context-index.sh`; a manual
edit is overwritten on the very next run and gives a false sense of
correctness in the meantime.

**BAD:** adding or editing an entry directly in the generated index file.
**GOOD:** update the source file's frontmatter, then re-run
`regenerate-context-index.sh`.

**Consequence:** the index silently drifts from the files it claims to
summarize, and the next regeneration erases the "fix" without warning.

### NEVER ignore stderr warnings from `regenerate-context-index.sh`

**WHY:** files excluded from the index are invisible to any agent or tool
that reads the index instead of the raw directory — they cannot be found,
followed up on, or superseded.

**BAD:** running the script, seeing warnings, and treating stdout's success
line as the whole story.

**GOOD:** fix every warning first, then regenerate, then treat the output as
complete.

**Consequence:** an active plan or a critical finding quietly disappears from
every summary an agent produces from the index.

### NEVER rely on the index as the sole inventory of context files

**WHY:** a file with missing or malformed frontmatter is excluded, not
flagged inline — the index only reflects files that already parse.

**BAD:** answering "what context files exist?" purely from the generated
index without having run the frontmatter check first.

**GOOD:** run `check-context-frontmatter.sh`, resolve what it reports, then
treat the freshly regenerated index as reliable.

**Consequence:** a stakeholder is told a decision or follow-up doesn't exist
when it does — it is simply unindexed.

### NEVER let a file's `type:` disagree with the directory it lives in

**WHY:** the directory already declares the typology; a mismatched `type:`
value is a mistake in the file, not a new category to route around silently.

**BAD:** a file under a `plans/` directory carrying `type: finding`, quietly
folded into an `other` catch-all that most tooling never reads.

**GOOD:** fix the file's `type:` to match its directory, or move the file to
the directory that matches its real type.

**Consequence:** the regenerated index groups the entry somewhere nobody
looks, which is functionally the same as it not existing.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Technical details, parsing rules, exit codes, and CI integration | [Regeneration Reference](references/regeneration-reference.md) | Debugging index regeneration, wiring the check into a pre-commit hook, or writing a new script against the same frontmatter shape |
| Typology catalog and the plural-folder / singular-`type:` mapping | The `create-context-file` skill's typologies reference (companion skill) | Choosing or extending the typology set this skill indexes against |
