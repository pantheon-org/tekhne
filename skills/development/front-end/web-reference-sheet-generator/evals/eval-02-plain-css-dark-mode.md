# Eval 02 — Plain CSS Project with Dark Mode

## Scenario

A vanilla HTML/CSS project (no framework) with a comprehensive dark mode via `prefers-color-scheme`.
The user asks: "Document the design tokens for this site."

## Input

**User prompt:** `Document the design tokens for this site`

**Codebase state:**

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

## Expected Agent Behaviour

1. Finds `styles/tokens.css` via grep for `--` — 20+ tokens discovered
2. Detects `prefers-color-scheme: dark` — correctly identifies media-query dark mode strategy
3. Reads button and card components — extracts button variants, card structure
4. Finds `role="article"` and `aria-labelledby` — populates §10 Accessibility
5. Creates `docs/design/` and writes both output files
6. §11 Dark Mode correctly states strategy as "media query (`prefers-color-scheme: dark`)" and lists overridden tokens
7. §8 Motion populated with `--transition-fast` and `--transition-base`
8. Confirmation lists `--space-1` through `--space-16` as spacing tokens

## Success Criteria

- [ ] §11 Dark Mode names the strategy as `prefers-color-scheme` media query (not class-based)
- [ ] Dark mode section lists the 5 overridden tokens with their dark values
- [ ] §8 Motion contains both transition tokens with correct values
- [ ] §4 Spacing lists all 5 space tokens
- [ ] §10 Accessibility notes `role="article"` usage found in components
- [ ] Companion SKILL.md generated with project-specific description

## Failure Modes to Watch

- Agent conflates class-based dark mode with media query dark mode
- Agent lists only light-mode colour values, ignoring the `@media` block
- Agent marks §8 Motion as TBD despite tokens being present
