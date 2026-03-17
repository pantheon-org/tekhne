# P01T02 — Create `tokens.css`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/styles/tokens.css` defining both `[data-theme="dark"]` and `[data-theme="light"]` CSS custom property blocks with `--tk-*` variables that mirror the current zinc-950/green palette.

## File to create / modify

```
docs/src/styles/tokens.css
```

## Implementation

```css
/* Dark theme (default) */
[data-theme="dark"],
:root {
  --tk-bg:           #09090b;  /* zinc-950 */
  --tk-bg-surface:   #18181b;  /* zinc-900 */
  --tk-bg-subtle:    #27272a;  /* zinc-800 */
  --tk-border:       #3f3f46;  /* zinc-700 */
  --tk-text:         #fafafa;  /* zinc-50 */
  --tk-text-muted:   #a1a1aa;  /* zinc-400 */
  --tk-accent:       #4ade80;  /* green-400 */
  --tk-accent-hover: #22c55e;  /* green-500 */
  --tk-link:         #86efac;  /* green-300 */
  --tk-code-bg:      #18181b;
  --tk-shadow:       0 1px 3px rgba(0,0,0,.5);
}

/* Light theme */
[data-theme="light"] {
  --tk-bg:           #ffffff;
  --tk-bg-surface:   #f4f4f5;  /* zinc-100 */
  --tk-bg-subtle:    #e4e4e7;  /* zinc-200 */
  --tk-border:       #d4d4d8;  /* zinc-300 */
  --tk-text:         #18181b;  /* zinc-900 */
  --tk-text-muted:   #71717a;  /* zinc-500 */
  --tk-accent:       #16a34a;  /* green-600 */
  --tk-accent-hover: #15803d;  /* green-700 */
  --tk-link:         #15803d;
  --tk-code-bg:      #f4f4f5;
  --tk-shadow:       0 1px 3px rgba(0,0,0,.12);
}
```

## Notes

- Audit the current `--sl-*` values in `docs/src/styles/custom.css` and the three custom components before finalising values; adjust if the current site uses different shades.
- Tokens are intentionally minimal; add more (`--tk-radius`, `--tk-font-mono`, etc.) only when a component actually requires them.

## Verification

```sh
test -f docs/src/styles/tokens.css
grep '\-\-tk-bg' docs/src/styles/tokens.css
grep 'data-theme="light"' docs/src/styles/tokens.css
```
