# Scenario 1: Regenerate the Index After Adding Files

## User Prompt

"I just created two new context files. Regenerate the index so they show up."

## Input

Two new files were created:

**`.context/decisions/2026-06-30-adopt-structured-logging.md`**:

```yaml
---
title: "Decision: Adopt Structured Logging"
type: decision
date: 2026-06-30
status: active
tags:
  - logging
---
```

**`.context/plans/2026-06-30-add-structured-logging.md`**:

```yaml
---
title: "Plan: Add Structured Logging"
type: plan
date: 2026-06-30
status: active
tags: []
related:
  - ../decisions/2026-06-30-adopt-structured-logging.md
---
```

## Expected Behavior

1. Run `regenerate-context-index.sh` to update `.context/index.yaml`.
2. Verify both new files appear in the regenerated index, under the `decisions:`
   and `plans:` sections respectively.
3. Confirm no files are missing required frontmatter (no stderr warnings).
4. Explain that the index is a cache and the source of truth is the
   frontmatter in each `.md` file.

## Success Criteria

- `regenerate-context-index.sh` executed.
- Both new context files present in the regenerated index, each under its
  correct typology section.
- No missing frontmatter warnings on stderr.
- Correctly states the index is regenerated and should not be edited by hand.

## Failure Conditions

- Index not regenerated.
- New files missing from the index output, or filed under the wrong typology
  section.
- Missing frontmatter warnings ignored without action.
- `.context/index.yaml` edited manually instead of regenerating.
