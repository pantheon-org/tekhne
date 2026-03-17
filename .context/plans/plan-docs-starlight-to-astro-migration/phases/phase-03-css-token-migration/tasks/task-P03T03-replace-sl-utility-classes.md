# P03T03 — Replace Starlight utility classes

## Phase

Phase 03 — CSS Token Migration

## Goal

Replace all `sl-hidden`, `lg:sl-block`, and other Starlight utility class
references with plain CSS media queries so that no Starlight-specific class names
remain in the source.

## File to create / modify

```
src/components/*.astro   (any that use Starlight utility classes)
src/styles/custom.css    (if utility classes referenced here)
```

## Implementation

Common replacements:

| Starlight class | Replacement |
|---|---|
| `sl-hidden` | `display: none` inline or dedicate a `.visually-hidden` utility |
| `lg:sl-block` | `@media (min-width: 72rem) { display: block; }` in `<style>` |
| `lg:sl-flex` | `@media (min-width: 72rem) { display: flex; }` |
| `sl-sr-only` | `.sr-only { position: absolute; width: 1px; … }` in `base.css` |

Steps:

1. Find all usages:

```sh
grep -rn "sl-hidden\|sl-block\|lg:sl-\|sl-sr-only\|sl-flex" src/ \
  --include="*.astro" --include="*.css"
```

2. For each hit, replace the class with an equivalent `<style>` rule or an
   inline style, following the table above.

3. If `sr-only` (screen-reader-only) utilities are needed in more than one
   component, add a shared `.sr-only` rule to `base.css` instead of duplicating
   it per component.

## Notes

- Breakpoint for Starlight's `lg:` prefix is `72rem` — confirm from Starlight
  source before replacing.

## Verification

```sh
grep -rn "sl-hidden\|lg:sl-\|sl-sr-only" src/ --include="*.astro" --include="*.css" \
  && echo "FAIL: utility classes remain" && exit 1 \
  || echo "ok"
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
```
