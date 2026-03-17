# P03T02 — replace-sl-variables

## Phase

Phase 03 — CSS Token Migration

## Goal

Replace every `--sl-*` CSS custom property reference in `docs/src/` with its `--tk-*` counterpart identified in the P03T01 audit, so no Starlight design token names remain in the codebase.

## File to create / modify

```
docs/src/styles/tokens.css       (may need additions)
docs/src/styles/base.css         (may need updates)
docs/src/components/*.astro      (inline <style> blocks)
docs/src/layouts/*.astro         (inline <style> blocks)
```

## Implementation

Use the audit from P03T01 to drive targeted `sed` replacements:

```sh
# Run from docs/src/ — adjust mappings from the audit file
# Core colour tokens
find . -name "*.css" -o -name "*.astro" | xargs sed -i '' \
  -e 's/var(--sl-color-bg)/var(--tk-bg)/g' \
  -e 's/var(--sl-color-text)/var(--tk-text)/g' \
  -e 's/var(--sl-color-accent-high)/var(--tk-accent-high)/g' \
  -e 's/var(--sl-color-accent)/var(--tk-accent)/g' \
  -e 's/var(--sl-color-white)/var(--tk-text)/g' \
  -e 's/var(--sl-color-black)/var(--tk-bg)/g'

# Typography tokens
find . -name "*.css" -o -name "*.astro" | xargs sed -i '' \
  -e 's/var(--sl-font)/var(--tk-font-sans)/g' \
  -e 's/var(--sl-font-mono)/var(--tk-font-mono)/g'
```

Add any `--tk-*` variables missing from `tokens.css` before running replacements.

## Notes

- Run the replacements **after** verifying every `--sl-*` token in the audit maps to a defined `--tk-*` token. A missing mapping will produce invisible or broken styles.
- `sed -i ''` is BSD/macOS syntax; on Linux use `sed -i`.
- After running, re-audit with the same grep from P03T01 to confirm zero `--sl-*` references remain.
- Do not replace `--sl-*` occurrences inside `.md`/`.mdx` content files (skill docs) — those are rendered markdown, not applied CSS.

## Verification

```sh
cd docs
grep -rn --include="*.css" --include="*.astro" -E "var\(--sl-" src/ \
  && echo "FAIL: --sl- variables remain" || echo "ok"
bunx astro build 2>&1 | grep -E "(error|Error)" | head -10
```
