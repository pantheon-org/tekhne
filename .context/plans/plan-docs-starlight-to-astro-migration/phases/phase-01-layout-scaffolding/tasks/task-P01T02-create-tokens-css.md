# P01T02 — Create `src/styles/tokens.css`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/styles/tokens.css` defining `--tk-*` CSS custom properties for
both dark and light themes, mirroring the current zinc-950/green dark palette.

## File to create / modify

```
docs/src/styles/tokens.css
```

## Implementation

```css
/* Dark theme (default) */
[data-theme="dark"] {
  --tk-bg: #09090b;           /* zinc-950 */
  --tk-bg-surface: #18181b;   /* zinc-900 */
  --tk-bg-hover: #27272a;     /* zinc-800 */
  --tk-border: #3f3f46;       /* zinc-700 */
  --tk-text: #f4f4f5;         /* zinc-100 */
  --tk-text-muted: #a1a1aa;   /* zinc-400 */
  --tk-accent: #22c55e;       /* green-500 */
  --tk-accent-hover: #16a34a; /* green-600 */
  --tk-code-bg: #18181b;
  --tk-link: #4ade80;         /* green-400 */
  --tk-link-hover: #86efac;   /* green-300 */
}

/* Light theme */
[data-theme="light"] {
  --tk-bg: #ffffff;
  --tk-bg-surface: #f4f4f5;   /* zinc-100 */
  --tk-bg-hover: #e4e4e7;     /* zinc-200 */
  --tk-border: #d4d4d8;       /* zinc-300 */
  --tk-text: #18181b;         /* zinc-900 */
  --tk-text-muted: #71717a;   /* zinc-500 */
  --tk-accent: #16a34a;       /* green-600 */
  --tk-accent-hover: #15803d; /* green-700 */
  --tk-code-bg: #f4f4f5;
  --tk-link: #16a34a;
  --tk-link-hover: #15803d;
}
```

## Notes

- Token names use `--tk-` prefix to avoid collision with Starlight `--sl-*` tokens.
- Exact colour values should be verified against the current site's rendered output before Phase 2 cutover.
- Phase 3 will map remaining `--sl-*` usages in custom components to these tokens.

## Verification

```sh
grep -c "\-\-tk-" docs/src/styles/tokens.css
```
