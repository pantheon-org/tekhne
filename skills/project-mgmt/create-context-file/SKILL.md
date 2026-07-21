---
name: create-context-file
description: "Creates a structured, date-stamped context file filed by typology (findings, plans, guides, follow-ups, merge-requests, tickets, decisions, notes, research) with YAML frontmatter. Use when saving a finding, writing a plan, capturing a guide, recording a decision, logging a follow-up, filing a merge-request note, plus ticket write-ups."
license: MIT
metadata:
  version: "1.0.0"
  audience: agents
  workflow: capture, planning, documentation
---

# Create Context File

Persist a working artifact into the project's context directory, filed by
**typology** and named with a date prefix so it sorts chronologically and never
collides.

## When to Use

- Save an investigation write-up, plan, guide, or decision produced while working.
- Capture a follow-up, merge-request note, or ticket write-up for later.
- The user asks to "save this finding", "create a plan", or "write this up".

## When Not to Use

- Do NOT use for technical notes that belong inline in source — write a code comment.
- Do NOT use for permanent, long-lived docs — use the README or docs site.
- Do NOT use for issue tracking — use the issue tracker.
- Do NOT use for content the team consumes externally — use the docs system.

## Principles

1. **Typology is the organizing axis.** ALWAYS choose the folder by *what the
   artifact is*, not by how long it lives.
2. **One artifact, one file.** ALWAYS keep a single typology per file; NEVER
   merge a finding with a plan.
3. **Deterministic names.** The generator MUST own the filename and frontmatter;
   never hand-craft them.
4. **Curated growth.** Extend the typology set deliberately, not per task.

## Workflow

1. Choose the typology from the curated set. — **Stop if:** none fits and the
   need will not recur; reuse the closest match instead of adding a folder.
2. Inspect what already exists for that typology to avoid duplicates:

   ```bash
   ls .context/findings/
   ```

   Expected result: the existing files, so you can confirm none already covers
   this artifact.
3. Run the generator with a specific, task-tied title. — **Verify:** the printed
   path uses the expected typology plus date.
4. Fill in the body beneath the generated heading. — **Stop if:** the content
   spans two typologies; split it into separate files.

## Quick Commands

```bash
# Create a finding with tags
./scripts/create-context-file.sh --type findings --title "Auth token analysis" \
  --tags "auth,oauth"
```

Expected result: a new dated finding file, printed as the only stdout line.

```bash
# Multi-line body via heredoc
./scripts/create-context-file.sh --type plans --title "Retriever rollout" << 'EOF'
## Phase 1
...
EOF
```

Expected result: a dated plan file containing the heredoc body.

```bash
# Preview without writing
./scripts/create-context-file.sh --type merge-requests --title "PROJ-1234" --dry-run
```

Expected result: the target path plus full file body printed; nothing written.

```bash
# A deliberate one-off typology
./scripts/create-context-file.sh --type experiments --title "spike" --allow-new-type
```

Expected result: a file under a new typology folder, created knowingly.

Full option list: see the CLI reference below.

## Anti-Patterns

### NEVER create a file without checking the typology folder first

**WHY:** the same artifact captured twice fragments context and breaks search.

**BAD:** run the generator immediately for a plan that already exists.
**GOOD:** list the folder, confirm nothing covers it, then create.

**Consequence:** duplicate, drifting files that later readers cannot reconcile —
a common pitfall once a repository grows.

### NEVER use a generic slug like `notes` or `todo`

**WHY:** vague slugs collide and read as noise when sorted by date.

**BAD:** `--title "notes"` yields a `notes.md` that clashes across tasks.
**GOOD:** `--title "API refactor findings"` yields a specific, searchable name.

**Consequence:** unsearchable filenames plus silent name clashes in production
repositories.

### NEVER mix typologies in one file

**WHY:** a finding, a plan, and a follow-up have different lifecycles and readers.

**BAD:** one file holding analysis, a rollout plan, and open questions.
**GOOD:** three files, one per typology, cross-referenced if needed.

**Consequence:** no file can be retired cleanly; each keeps stale content alive.

### NEVER hand-edit the filename or frontmatter date

**WHY:** the date prefix plus ISO `date` power sorting and downstream tooling.

**BAD:** rename to drop the date, or set `date: Jul 21st`.
**GOOD:** let the generator produce both; keep the original date on later edits.

**Consequence:** broken chronological ordering plus frontmatter parse failures.

### NEVER invent a new typology for a one-off

**WHY:** ad-hoc folders erode the curated set that makes context navigable.

**BAD:** create a bespoke folder for a single throwaway note.
**GOOD:** reuse the closest typology, or pass `--allow-new-type` knowingly.

**Consequence:** folder sprawl that defeats predictable retrieval — a slow pitfall.

## References

- [Typologies](references/typologies.md) — the curated catalog, selection rule, and how to extend the set; load when choosing or adding a typology.
- [CLI reference](references/cli.md) — full generator flags, behavior, and examples; load when you need an option beyond the Quick Commands.
