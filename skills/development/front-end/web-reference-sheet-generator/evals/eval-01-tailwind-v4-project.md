# Eval 01 — Tailwind v4 Project with CSS Custom Properties

## Scenario

A React/Vite project using Tailwind v4 with design tokens defined in a CSS `@theme` block.
The user asks: "Create a design reference for this project."

## Input

**User prompt:** `Create a design reference for this project`

**Codebase state:**

`src/app.css`:
```css
@theme {
  --color-brand-500: #7c3aed;
  --color-brand-600: #6d28d9;
  --color-neutral-50: #fafafa;
  --color-neutral-900: #0a0a0a;
  --font-sans: "Inter", sans-serif;
  --font-heading: "Cal Sans", sans-serif;
  --spacing-section: 5rem;
  --radius-card: 0.75rem;
  --shadow-card: 0 4px 24px rgb(0 0 0 / 0.08);
}
```

`src/components/Button.tsx` — uses `className="bg-brand-500 text-white rounded-md px-4 py-2"` variants

`src/pages/Home.tsx` — hero section, card grid, CTA button

No dark mode detected.

## Expected Agent Behaviour

1. Runs grep for `--` in CSS files and finds the `@theme` block in `src/app.css`
2. Runs find for `tailwind.config.*` — not found (v4 config is in CSS)
3. Reads 3–5 component files to observe token usage in practice
4. Runs dark mode grep — returns no results; correctly marks dark mode section as `TBD`
5. Runs ARIA/accessibility grep
6. Creates `docs/design/` directory
7. Writes `docs/design/design-reference.md` using the template, filling:
   - §2 Colours: 4 tokens from `@theme` with actual hex values
   - §3 Typography: 2 font families
   - §4 Spacing: `--spacing-section: 5rem`
   - §6 Borders: `--radius-card: 0.75rem`
   - §7 Shadows: `--shadow-card` value
   - §8 Motion: TBD (no transitions found)
   - §11 Dark Mode: TBD — not yet defined
8. Writes `docs/design/SKILL.md` with project-specific description (names the project)
9. Prints the Step 7 confirmation block listing populated sections and TBD items

## Success Criteria

- [ ] `docs/design/design-reference.md` exists and is not a copy of the template (placeholders replaced)
- [ ] All found token values appear verbatim — no invented values
- [ ] Dark mode section contains `TBD — not yet defined`
- [ ] Confirmation block lists `§2 Colours — 4 tokens found in src/app.css`
- [ ] `docs/design/SKILL.md` frontmatter `description` names the project, not a generic tool
- [ ] No section is omitted — all 12 sections present, even if some are TBD

## Failure Modes to Watch

- Agent invents a colour palette not present in CSS
- Agent skips `docs/design/SKILL.md` generation
- Agent uses generic description in companion SKILL.md frontmatter
- Agent omits the Step 7 confirmation output
