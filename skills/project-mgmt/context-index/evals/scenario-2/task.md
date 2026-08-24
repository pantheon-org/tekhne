# Scenario 2: Detect and Fix Missing Frontmatter

## User Prompt

"The pre-commit hook is blocking me. It says some context files are missing
YAML frontmatter. Find them and fix them."

## Input

**`.context/guides/retry-backoff-wrapper.md`** (missing frontmatter):

```markdown
# Guide: Retry/Backoff Wrapper

## Usage
```

**`.context/tickets/2026-06-30-proj-1234-refinement.md`** (valid, do not touch):

```yaml
---
title: "Ticket: PROJ-1234 Refinement"
type: ticket
date: 2026-06-30
status: active
tags: []
---
```

**`.context/follow-ups/checkout-refactor-deferred-edge-cases.md`** (missing
frontmatter):

```markdown
# Checkout Refactor — Deferred Edge Cases
```

## Expected Behavior

1. Run `check-context-frontmatter.sh` (or `validate-context-frontmatter.sh`)
   against `.context/**/*.md` to identify files missing frontmatter.
2. Identify `retry-backoff-wrapper.md` and
   `checkout-refactor-deferred-edge-cases.md` as missing frontmatter.
3. Fix both files by adding proper frontmatter, using `type: guide` for the
   file under `guides/` and `type: follow-up` for the file under
   `follow-ups/` — the singular form matching each file's own directory.
4. After fixing, run `regenerate-context-index.sh` and confirm zero warnings.
5. Do NOT modify `proj-1234-refinement.md`, which already has valid
   frontmatter.

## Success Criteria

- A check script used to identify missing frontmatter.
- Both missing-frontmatter files correctly identified.
- Proper frontmatter added to both files with all required fields, and the
  correct singular `type:` for each file's directory.
- Index regenerated with zero warnings.
- Valid file not modified.

## Failure Conditions

- Check script not used (issues identified by reading files only).
- Only some missing-frontmatter files fixed.
- Frontmatter added with a `type:` that doesn't match the file's own
  directory (e.g. `type: finding` under `guides/`).
- Index not regenerated after fixes.
- The already-valid ticket file's frontmatter modified.
