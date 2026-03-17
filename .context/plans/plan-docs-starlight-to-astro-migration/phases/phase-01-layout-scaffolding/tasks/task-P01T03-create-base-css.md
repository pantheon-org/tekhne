# P01T03 — Create `src/styles/base.css`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/styles/base.css` with a CSS reset, base typography, and body
background wired to `--tk-*` tokens.

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
  background-color: var(--tk-bg);
  color: var(--tk-text);
  font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont,
    "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  line-height: 1.6;
  min-height: 100vh;
}

a {
  color: var(--tk-link);
  text-decoration: none;
}

a:hover {
  color: var(--tk-link-hover);
  text-decoration: underline;
}

code, pre {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace;
}

img, svg {
  display: block;
  max-width: 100%;
}
```

## Notes

- This file contains no component-specific rules — those live in component `<style>` blocks.
- Imported by `BaseLayout.astro` after `tokens.css`.

## Verification

```sh
test -f docs/src/styles/base.css && echo ok
```
