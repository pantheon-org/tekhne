# P03T01 — Audit `--sl-*` variable usages

## Phase

Phase 03 — CSS Token Migration

## Goal

Produce a mapping table of every `--sl-*` custom property used in the codebase
and its `--tk-*` replacement, written to
`.context/plans/plan-docs-starlight-to-astro-migration/sl-token-map.md`.

## File to create / modify

```
.context/plans/plan-docs-starlight-to-astro-migration/sl-token-map.md  (new)
```

## Implementation

1. List all `--sl-*` references:

```sh
grep -rn -- "--sl-" src/ --include="*.astro" --include="*.css" --include="*.ts" \
  | grep -oP -- "--sl-[\w-]+" | sort -u
```

2. For each unique variable, map it to the closest `--tk-*` token defined in
   `src/styles/tokens.css`.

3. Write the mapping table:

```markdown
| Starlight var | Purpose | --tk-* replacement |
|---|---|---|
| --sl-color-text | Body text | --tk-color-text |
| --sl-color-bg | Page background | --tk-color-bg |
| --sl-color-accent | Brand accent | --tk-color-accent |
```

4. Flag any variable with no `--tk-*` counterpart with a `⚠ MISSING` note —
   those tokens must be added to `tokens.css` before P03T02 can proceed.

## Notes

- This task is read-only; no source files are modified.
- The mapping table is consumed by P03T02 and P03T03.

## Verification

```sh
test -f .context/plans/plan-docs-starlight-to-astro-migration/sl-token-map.md \
  && echo "mapping table exists"
grep -c "|" .context/plans/plan-docs-starlight-to-astro-migration/sl-token-map.md \
  | xargs -I{} test {} -gt 3 && echo "table has rows"
```
