# P03T03 — replace-sl-utility-classes

## Phase

Phase 03 — CSS Token Migration

## Goal

Replace every Starlight utility class (`.sl-*`, `.not-content`, etc.) used in Astro component `class=` attributes or CSS rules with either plain semantic class names or inline styles, eliminating all runtime dependency on Starlight's stylesheet.

## File to create / modify

```
docs/src/components/*.astro
docs/src/layouts/*.astro
docs/src/pages/*.astro
```

## Implementation

Identify all `.sl-*` class usages:

```sh
grep -rn --include="*.astro" -E 'class="[^"]*sl-|sl-[a-z-]+ ' docs/src/
```

Common replacements:

| Starlight class | Replacement |
|---|---|
| `sl-flex` | `style="display:flex"` or custom `.flex` in base.css |
| `sl-hidden` | `style="display:none"` or `hidden` attribute |
| `not-content` | Remove (Starlight uses this to exclude from prose styling; our base.css doesn't have `.content` scoping) |
| `sl-sidebar-item` | Replace with semantic `.nav-item` class defined in LeftNav |
| `sl-badge` | Replace with `.badge` class defined in base.css |

For each occurrence: read the component, understand the Starlight class's visual intent, apply the equivalent using our own classes or inline styles.

## Notes

- `.not-content` is a Starlight escape hatch; after removing Starlight's global styles it has no effect and can be safely deleted.
- Do not introduce new utility class proliferation — prefer adding a small named class to the component's `<style>` block.
- If a Starlight class does not have a natural equivalent, add the CSS rule to `base.css` under a semantic name.

## Verification

```sh
cd docs
grep -rn --include="*.astro" -E 'class="[^"]*\bsl-|\bsl-[a-z-]+' src/ \
  && echo "FAIL: .sl-* classes remain" || echo "ok"
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
```
