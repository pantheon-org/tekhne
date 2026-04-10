# Scenario 01: Generate Design Reference for a Tailwind v4 Project

## User Prompt

You are working in a React/Vite project that uses Tailwind v4 with design tokens defined in a CSS `@theme` block.

## Codebase state

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

No dark mode detected. No `aria-` attributes found in components.

Create a design reference for this project.

## Expected Behavior

1. Run grep for CSS custom properties (`--`) in `src/app.css` or equivalent to discover tokens
2. Check for `tailwind.config.*` file (not found is an acceptable result for v4 projects)
3. Read at least 2 component or page files before generating output
4. Create `docs/design/design-reference.md` with all 12 sections populated
5. Replace all template placeholders with actual values from the codebase — no `{TOKEN_*}` or `{PROJECT_NAME}` strings remain
6. Include all 4 colour tokens from `@theme` verbatim with their actual hex values — no invented colours
7. Mark the dark mode section as `TBD — not yet defined` (no dark mode present in codebase)
8. Generate a companion skill under `.agents/skills/` (not under `docs/`) with a project-specific description
9. Print a Step 8 confirmation block listing populated sections with token counts and TBD items
10. Populate or reference a `skill-output.yaml` scratchpad before generating the markdown output

## Success Criteria

- **Token discovery**: Agent ran grep for CSS custom properties (`--`) in `src/app.css` or equivalent
- **Tailwind config check**: Agent checked for `tailwind.config.*` file (not found is an acceptable result)
- **Component inspection**: Agent read at least 2 component or page files before generating output
- **Output file created**: `docs/design/design-reference.md` was created
- **No placeholders**: No `{TOKEN_*}` or `{PROJECT_NAME}` placeholders remain in the generated `design-reference.md`
- **Accurate colour tokens**: All 4 colour tokens from `@theme` appear verbatim with their actual hex values — no invented colours
- **Dark mode TBD**: Dark mode section contains `TBD — not yet defined`
- **All 12 sections present**: All 12 sections are present in `design-reference.md` even if some contain only TBD markers
- **Companion SKILL.md location**: Companion SKILL.md was generated under `.agents/skills/` (not under `docs/`)
- **Companion SKILL.md specificity**: Companion SKILL.md frontmatter description names the specific project, not a generic tool description
- **Confirmation block printed**: Step 8 confirmation block was printed listing populated sections with token counts
- **TBD items listed**: Step 8 confirmation block explicitly lists TBD items (at minimum: dark mode)
- **Authoritative token file named**: Reference sheet names `src/app.css` as the authoritative token file
- **No assumed breakpoints**: Agent did not assume Tailwind breakpoints from memory — checked config or CSS
- **Actual component classNames**: Component pattern uses actual `className` strings from the codebase, not invented JSX

## Failure Conditions

- Agent invents colour tokens not present in `src/app.css`
- Agent skips reading component or page files before generating output
- Agent leaves `{TOKEN_*}` or `{PROJECT_NAME}` placeholders in the output
- Agent places the companion SKILL.md under `docs/` instead of `.agents/skills/`
- Agent uses a generic description in the companion SKILL.md frontmatter instead of naming the project
- Agent marks sections as TBD without attempting to discover tokens first
- Agent does not print a Step 8 confirmation block
- Agent omits one or more of the 12 required sections from `design-reference.md`
