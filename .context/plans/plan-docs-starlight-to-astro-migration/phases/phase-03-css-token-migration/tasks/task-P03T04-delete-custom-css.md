# P03T04 — delete-custom-css

## Phase

Phase 03 — CSS Token Migration

## Goal

Delete the Starlight custom CSS override file (`docs/src/styles/custom.css` or equivalent) that patched `--sl-*` variables at the Starlight layer, since those overrides are now superseded by `tokens.css` and are no longer referenced by anything.

## File to create / modify

```
docs/src/styles/custom.css   (DELETE — or whichever file overrode --sl-* at Starlight layer)
```

## Implementation

```sh
# Confirm the file only contains --sl-* overrides and Starlight-specific rules
grep -v "^$\|^/\*\|^\s*\*" docs/src/styles/custom.css | head -30

# If confirmed safe:
rm docs/src/styles/custom.css
```

Also remove the import of this file from `BaseLayout.astro` or `astro.config.mjs` (`customCss` array):

```sh
# Check for references to custom.css
grep -rn "custom.css" docs/src/ docs/astro.config.mjs
# Remove each reference found
```

## Notes

- The Starlight `customCss` config option is removed with the `starlight()` integration in P02T08; if this file was listed there, it is already detached from the build.
- However, if `BaseLayout.astro` still imports it via `import styles from "./custom.css?url"`, that import must also be removed.
- If `custom.css` contains any rules that are **not** `--sl-*` overrides (e.g. markdown typography patches), extract those rules into `base.css` before deletion.
- If the file does not exist (already deleted or never created), skip the `rm` and only verify the import is absent.

## Verification

```sh
[ ! -f docs/src/styles/custom.css ] && echo "custom.css deleted ok" || echo "custom.css still exists - check if intentional"
grep -rn "custom.css" docs/src/ && echo "FAIL: custom.css still imported" || echo "import removed ok"
cd docs && bunx astro build 2>&1 | grep -E "(error|Error)" | head -10
```
