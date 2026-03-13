# Task — Document design tokens for a plain CSS site with dark mode

You are working in a vanilla HTML/CSS project with a comprehensive design token file and dark mode via `prefers-color-scheme`.

## Codebase state

`styles/tokens.css`:
```css
:root {
  --color-primary: #1a56db;
  --color-primary-dark: #1e429f;
  --color-accent: #ff6b35;
  --color-text: #111928;
  --color-text-muted: #6b7280;
  --color-surface: #ffffff;
  --color-surface-raised: #f9fafb;
  --color-border: #e5e7eb;

  --font-body: "Source Sans 3", system-ui, sans-serif;
  --font-mono: "JetBrains Mono", monospace;

  --space-1: 0.25rem;
  --space-2: 0.5rem;
  --space-4: 1rem;
  --space-8: 2rem;
  --space-16: 4rem;

  --radius-sm: 0.25rem;
  --radius-md: 0.5rem;
  --radius-lg: 1rem;

  --shadow-sm: 0 1px 2px rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px rgb(0 0 0 / 0.07);

  --transition-fast: 150ms ease;
  --transition-base: 250ms ease;
}

@media (prefers-color-scheme: dark) {
  :root {
    --color-text: #f9fafb;
    --color-text-muted: #9ca3af;
    --color-surface: #111928;
    --color-surface-raised: #1f2937;
    --color-border: #374151;
  }
}
```

`components/button.html` — uses `class="btn btn-primary"` patterns
`components/card.html` — card with `role="article"` and `aria-labelledby`

## Your task

Document the design tokens for this site.
