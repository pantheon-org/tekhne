---
category: workflow
priority: CRITICAL
---

# Generation Workflow

## Step 1 — Discover design tokens

Search the codebase for CSS custom properties, theme files, and design tokens:

```bash
# Find CSS custom properties in :root / @theme blocks
grep -r "^\s*--" src/ --include="*.css" -n

# Find Tailwind config (v3: tailwind.config.js; v4: @theme in CSS)
find . -name "tailwind.config.*" -not -path "*/node_modules/*"
grep -r "@theme" src/ --include="*.css"

# Find font imports
grep -r "font-family\|@import.*font\|@font-face" src/ --include="*.css" -n
```

Collect all tokens into these categories:
- Colours (brand, semantic, surface, text, border)
- Typography (font families, weights, size scale)
- Spacing (padding, margin, gap tokens)
- Borders and radius
- Shadows and elevation
- Motion and transitions

## Step 2 — Inspect existing components

Read 3–5 representative component/page files to understand how tokens are used in practice:

```bash
# Find the most complex page files (likely have all pattern variants)
find src/pages -name "*.tsx" -o -name "*.jsx" | head -5

# Find shared layout components
find src/components -name "*.tsx" -o -name "*.jsx" | head -10
```

For each file read, note:
- Button variants used (primary, secondary, ghost, outline)
- Card/panel patterns
- Section structure patterns
- Badge/label/tag patterns
- Any gradient or special text effects

## Step 3 — Check dark mode implementation

```bash
grep -r "dark\|prefers-color-scheme\|\.dark\s" src/ --include="*.css" -n | head -20
grep -r "dark:" src/ --include="*.tsx" --include="*.jsx" | head -10
```

Note the dark mode strategy (class-based `.dark`, media query, or data attribute).

## Step 4 — Check accessibility patterns

```bash
grep -r "aria-\|role=\|focus-visible\|visually-hidden\|sr-only" src/ --include="*.tsx" -n | head -20
```

## Step 5 — Fill the structured extraction template

Before writing any markdown, populate [`skill-output.yaml`](skill-output.yaml) with
every value discovered in Steps 1–4. This acts as a structured scratchpad and ensures no
section is skipped.

Validate the populated YAML against
[`design-reference.schema.json`](design-reference.schema.json).
All `required` fields must have real values (not `TBD`) before proceeding. Fields that are
genuinely unknown may remain `TBD` but must be called out in the Step 8 report.

## Step 6 — Generate `docs/design/design-reference.md`

Create the directory if it does not exist:

```bash
mkdir -p docs/design
```

Write the reference sheet using the template at
[`references/design-reference-template.md`](design-reference-template.md),
filling every section from the values in the populated `skill-output.yaml`. Replace
`{PROJECT_NAME}` and all `{TOKEN_*}` placeholders with real values.

The template covers all 12 sections: Brand Identity, Colour Palette, Typography,
Spacing System, Layout & Grid, Borders & Radius, Shadows & Elevation, Motion & Transitions,
Component Patterns, Accessibility, Dark Mode, and Figma Sync Notes.

## Step 7 — Generate `.agents/skills/{project-slug}/SKILL.md`

After generating the reference sheet, generate a companion enforcement skill using the
template at [`references/enforcement-skill-template.md`](enforcement-skill-template.md).

The skill must live under `.agents/skills/` — **not** under `docs/`. Create the directory:

```bash
mkdir -p .agents/skills/{project-slug}
```

Use a lowercase-hyphenated slug derived from the project name (e.g. `design-reference` for
Design & Bloom). Fill all `{PROJECT_NAME}` and section-specific placeholders from what was
generated in Step 6. In Step 3 of the enforcement skill template, paste the 6–8 most
important project-specific rules extracted from the reference sheet.

## Step 8 — Confirm output

After writing both files, report to the user:

```
Generated:
  docs/design/design-reference.md              — 12-section design reference sheet
  .agents/skills/{project-slug}/SKILL.md       — enforcement skill for {PROJECT_NAME}

Sections populated from codebase:
  §2 Colours     — {n} tokens found in {file}
  §3 Typography  — {n} font roles, {n} scale entries
  §4 Spacing     — {n} tokens
  §9 Components  — {n} patterns extracted from {files}

TBD items requiring user input:
  §1 Brand Identity — personality, tagline not found in code
  (list any others)

To keep in sync: update docs/design/design-reference.md whenever
a token in {token file path} changes.
```
