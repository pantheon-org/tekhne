---
name: design-reference
description: >
  Enforce visual and interaction consistency for {PROJECT_NAME} by reading the
  project design reference sheet before writing or reviewing any UI code, CSS,
  or Figma decisions. Use when adding components, updating styles, reviewing pages,
  or checking that a change matches the brand. Triggers on: "does this match the
  design", "check style consistency", "update the UI", "add a component",
  "review the design", "is this on-brand".
metadata:
  author: {author}
  version: "1.0.0"
  argument-hint: <file-or-pattern>
---

# Design Reference Skill — {PROJECT_NAME}

Enforce visual and interaction consistency across code and Figma.

## Mindset

Never make a colour, spacing, typography, or component decision from memory.
Always read the reference sheet first.
The reference sheet is the contract — code is the implementation.
When in doubt, check the token; when the token is wrong, update the reference sheet first,
then the code.

## When to activate

- Before adding or editing any CSS or inline style
- Before creating a new React/HTML component
- Before reviewing a PR that touches UI files
- When asked "is this on brand?", "does this match the design?", or "check the style"
- Before making a Figma decision on colour, spacing, or type

## Workflow

### Step 1 — Read the reference sheet

Use the Read tool on `docs/design/design-reference.md`.

### Step 2 — Identify the relevant section

| If working on...               | Read section(s)         |
| ------------------------------ | ----------------------- |
| Colours / backgrounds          | §2 Colour Palette       |
| Text / headings / fonts        | §3 Typography           |
| Padding / margin / gap         | §4 Spacing System       |
| Page layout / grids            | §5 Layout & Grid        |
| Borders / rounded corners      | §6 Borders & Radius     |
| Shadows / hover effects        | §7 Shadows & Elevation  |
| Animations / transitions       | §8 Motion & Transitions |
| Buttons / badges / cards       | §9 Component Patterns   |
| `aria-*`, focus, contrast      | §10 Accessibility       |
| Dark mode                      | §11 Dark Mode           |
| Figma ↔ code sync              | §12 Figma Sync Notes    |

### Step 3 — Apply the rules

{Paste the 6–8 most important project-specific rules from the generated reference sheet here.}

### Step 4 — Output format for reviews

When reviewing files for style consistency, output findings as:

```
file:line — [RULE] description
```

Group findings by category: COLOUR, TYPOGRAPHY, SPACING, LAYOUT, COMPONENT, ACCESSIBILITY, DARK MODE, MOTION.

## Anti-patterns to catch

| Anti-pattern | Correct approach |
| ------------ | ---------------- |
| Raw hex value in CSS | Use `var(--token-name)` |
| One-off padding not from spacing scale | Use `--spacing-*` token |
| Inline `border-radius` not matching §6 | Use the documented radius value |
| Hard-coded font-family | Use `var(--font-*)` token |
| Missing `aria-hidden` on decorative icon | Add `aria-hidden="true"` |
| Hard-coded colour in dark-mode context | Use CSS custom property |
| `transition: all` | Specify only needed properties |

## Token quick-reference

{Paste the colour, font, and spacing tokens in the compact format from docs/design/design-reference.md.}
