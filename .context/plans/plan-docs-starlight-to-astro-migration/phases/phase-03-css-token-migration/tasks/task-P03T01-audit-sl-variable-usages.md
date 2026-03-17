# P03T01 — audit-sl-variable-usages

## Phase

Phase 03 — CSS Token Migration

## Goal

Produce a complete inventory of every `--sl-*` CSS custom property and Starlight utility class used across all CSS and Astro component files, saved to `.context/plans/plan-docs-starlight-to-astro-migration/sl-audit.txt`, so Phase 3 replacements are exhaustive.

## File to create / modify

```
.context/plans/plan-docs-starlight-to-astro-migration/sl-audit.txt  (CREATE)
```

## Implementation

```sh
# Run from repo root
grep -rn --include="*.css" --include="*.astro" \
  -E "(--sl-[a-z-]+|\.sl-[a-z-]+)" \
  docs/src/ \
  > .context/plans/plan-docs-starlight-to-astro-migration/sl-audit.txt

echo "--- UNIQUE VARIABLE NAMES ---" >> .context/plans/plan-docs-starlight-to-astro-migration/sl-audit.txt
grep -oh --include="*.css" --include="*.astro" \
  -rE "(--sl-[a-z-]+)" \
  docs/src/ \
  | sort -u >> .context/plans/plan-docs-starlight-to-astro-migration/sl-audit.txt
```

Review the output file. For each `--sl-*` variable, identify the semantic equivalent in `tokens.css` (`--tk-*`). Document the mapping in the audit file as comments.

## Notes

- This task is preparatory — it produces the mapping table that P03T02 uses to perform replacements safely.
- Common Starlight variables and their `--tk-*` equivalents:
  - `--sl-color-bg` → `--tk-bg`
  - `--sl-color-text` → `--tk-text`
  - `--sl-color-accent` → `--tk-accent`
  - `--sl-color-accent-high` → `--tk-accent-high`
  - `--sl-color-gray-*` → `--tk-gray-*` (if defined) or raw zinc values
  - `--sl-font` → `--tk-font-sans`
  - `--sl-font-mono` → `--tk-font-mono`
  - `--sl-text-*` → raw `font-size` values from base.css
- Variables not mapped in `tokens.css` must be added to `tokens.css` before P03T02 runs.

## Verification

```sh
[ -s .context/plans/plan-docs-starlight-to-astro-migration/sl-audit.txt ] && echo "audit file exists and non-empty" || echo "FAIL: audit file missing or empty"
wc -l .context/plans/plan-docs-starlight-to-astro-migration/sl-audit.txt
```
