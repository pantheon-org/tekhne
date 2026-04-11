# Scenario 02: Document Design Tokens for a Plain CSS Site with Dark Mode

## User Prompt

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

Document the design tokens for this site.

## Expected Behavior

1. Run grep for `--` custom properties and find `styles/tokens.css`
2. Identify the dark mode strategy as `prefers-color-scheme` media query (not class-based) and document it in §11 Dark Mode
3. List all 5 overridden tokens (`--color-text`, `--color-text-muted`, `--color-surface`, `--color-surface-raised`, `--color-border`) with their dark values in the dark mode section
4. Populate §8 Motion with `--transition-fast` (150ms ease) and `--transition-base` (250ms ease) — not marked TBD
5. Populate §4 Spacing with all 5 space tokens (`--space-1` through `--space-16`) with correct values
6. Note `role="article"` and `aria-labelledby` found in the card component in §10 Accessibility
7. Create `docs/design/design-reference.md`
8. Generate the companion SKILL.md under `.agents/skills/` with a project-specific description
9. Name `styles/tokens.css` as the authoritative token file
10. Populate or reference a `skill-output.yaml` scratchpad before generating the markdown output

## Success Criteria

- **Token discovery**: Agent ran grep for `--` custom properties and found `styles/tokens.css`
- **Dark mode strategy**: §11 Dark Mode correctly identifies the strategy as `prefers-color-scheme media query` (not class-based)
- **Dark mode tokens**: Dark mode section lists all 5 overridden tokens with their dark values
- **Motion tokens populated**: §8 Motion is populated with `--transition-fast` and `--transition-base` — not marked TBD
- **Spacing tokens accurate**: §4 Spacing lists all 5 space tokens with correct values
- **Accessibility noted**: §10 Accessibility notes `role="article"` and `aria-labelledby` found in card component
- **Output file created**: `docs/design/design-reference.md` was created
- **Companion SKILL.md location**: Companion SKILL.md was generated under `.agents/skills/`
- **Companion SKILL.md specificity**: Companion SKILL.md description names the specific project
- **All 12 sections present**: All 12 sections present in the output
- **Confirmation block printed**: Step 8 confirmation block printed with populated section counts
- **Authoritative token file named**: `styles/tokens.css` named as the authoritative token file
- **No invented colours**: No colour values appear in the output that are not present in `tokens.css`

## Failure Conditions

- Agent misidentifies the dark mode strategy as class-based instead of `prefers-color-scheme`
- Agent omits one or more of the 5 overridden dark mode tokens
- Agent marks §8 Motion as TBD despite transition tokens being present in the CSS
- Agent invents colour values not present in `tokens.css`
- Agent places the companion SKILL.md under `docs/` instead of `.agents/skills/`
- Agent does not read `button.html` or `card.html` before generating output
- Agent does not name `styles/tokens.css` as the authoritative token file
