# Scenario 4: Resolve a Type/Directory Mismatch

## User Prompt

"I ran the index regeneration script and it printed a warning about a
type/directory mismatch. What does that mean, and can you fix it?"

## Input

**`.context/decisions/2026-06-30-cache-strategy.md`**:

```yaml
---
title: "Decision: Cache Strategy for the Read Path"
type: finding
date: 2026-06-30
status: active
tags: []
---
```

Running `regenerate-context-index.sh` produces:

```text
Generated 1 entries -> .context/index.yaml
WARNING: type/directory mismatches (entry kept, but check the source file):
  .context/decisions/2026-06-30-cache-strategy.md: type: 'finding' does not match its directory 'decisions/' (expected type: 'decision')
```

## Expected Behavior

1. Explain that the file's `type:` frontmatter field (`finding`) disagrees
   with the typology its own directory implies (`decisions/` → `decision`),
   and that this is a mistake in the file, not a new category.
2. Fix the file by changing `type: finding` to `type: decision` — do not move
   the file to a different directory, and do not edit `.context/index.yaml`
   directly.
3. Re-run `regenerate-context-index.sh` and confirm the warning is gone and
   the entry now appears under the `decisions:` section of the index.

## Success Criteria

- Correctly explains the mismatch (directory says `decision`, frontmatter
  says `finding`).
- Fixes the file's `type:` field, not the directory or the index.
- Re-regenerates the index and confirms zero warnings afterward.
- States that the entry now appears under `decisions:` in the regenerated
  index.

## Failure Conditions

- Ignores the warning or tells the user it's safe to leave as-is.
- "Fixes" it by editing `.context/index.yaml` directly instead of the source
  file.
- Moves the file into a `findings/` directory instead of correcting `type:`.
- Does not re-run the regeneration script to confirm the fix.
