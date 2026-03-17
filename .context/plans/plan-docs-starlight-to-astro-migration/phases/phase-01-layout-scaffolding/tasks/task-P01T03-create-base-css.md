# P01T03 — Create `base.css`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/styles/base.css` with a CSS reset, base typography defaults, and body background wired to `--tk-*` tokens.

## File to create / modify

```
docs/src/styles/base.css
```

## Implementation

```css
*, *::before, *::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

html {
  font-size: 16px;
  -webkit-text-size-adjust: 100%;
}

body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  background-color: var(--tk-bg);
  color: var(--tk-text);
  line-height: 1.6;
  min-height: 100vh;
}

a {
  color: var(--tk-link);
  text-decoration: underline;
}

a:hover {
  color: var(--tk-accent-hover);
}

img, svg {
  display: block;
  max-width: 100%;
}

code, pre {
  font-family: ui-monospace, "Cascadia Code", "Source Code Pro", Menlo, monospace;
  font-size: 0.875em;
}

pre {
  background: var(--tk-code-bg);
  border: 1px solid var(--tk-border);
  border-radius: 6px;
  overflow-x: auto;
  padding: 1rem;
}
```

## Notes

- Keep this file minimal; component-specific styles belong in component `<style>` blocks.
- Do not import `tokens.css` from here — `BaseLayout.astro` will import both in order.

## Verification

```sh
test -f docs/src/styles/base.css
grep 'var(--tk-bg)' docs/src/styles/base.css
```
