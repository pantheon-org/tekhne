---
name: vault-consolidate
description: >
  Propose or apply consolidation of episodic memories into semantic ones,
  combining related episodic captures into higher-level semantic summary notes.
  Use after a long session or when the user asks to clean up, organise,
  summarise, merge, or declutter memories, or requests memory cleanup or memory
  management. Always run --propose first so the user can review before --apply
  commits changes to the vault.
allowed-tools: "Bash"
---

# vault-consolidate

Cluster related episodic memories and synthesise them into durable semantic notes.

## Mindset

Think of `--propose` as generating a diff and `--apply` as committing the merge.
You never commit without reviewing the diff first. The human is always the final
gatekeeper — the agent's job is to surface candidates, not to decide what gets
promoted to long-term memory.

## When to use

- After a long session with many captures
- When the user asks to "consolidate", "summarise", or "clean up" memories
- Periodic maintenance (e.g. end of week)

## Workflow

Always propose before applying:

```bash
# Step 1 — generate proposals (written to vault inbox for review)
vault-cli consolidate --propose [--project <id>]

# Step 2 — open vault inbox, review, set status: approved or rejected
# (edit 00-inbox/consolidation-proposals.md in Obsidian)

# Step 3 — apply approved proposals
vault-cli consolidate --apply
```

If `--propose` outputs "No episodic memories found to consolidate", skip Step 3 —
there is nothing to apply.

## What happens

- `--propose`: clusters episodic memories by semantic similarity, calls the
  inference command to synthesise each cluster, writes proposals to
  `00-inbox/consolidation-proposals.md`
- `--apply`: reads the proposals file, writes approved ones as semantic notes,
  marks source episodics as `superseded`, logs rejected ones to audit

## Important

Source episodic memories are never deleted — only marked `superseded`.
Human edits in Obsidian are always respected and never overwritten.

## Never

- **Never run `--apply` without first running `--propose`** — doing so bypasses human review and irreversibly promotes unreviewed clusters to semantic memory
- **Never run `--apply` without confirming the user has reviewed the proposals** — always pause at Step 2 and wait for the user to confirm before proceeding
- **Never delete episodic source memories manually** — they are preserved automatically and serve as audit history
