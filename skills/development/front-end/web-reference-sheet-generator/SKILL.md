---
name: web-reference-sheet-generator
description: "Generate a project-specific web design reference sheet (docs/design/design-reference.md) and companion enforcement skill for any website codebase. Extracts CSS custom properties, validates against a JSON schema scratchpad, inspects components, and produces a 12-section document covering colours, typography, spacing, layout, borders, shadows, motion, accessibility, dark mode, and Figma sync notes. Use when starting a new project, onboarding a design system, creating a Figma reference sheet, porting design tokens, or auditing existing styles. Triggers on: create a design reference, generate a style guide, document the design tokens, make a brand reference sheet, port design tokens, audit existing styles."
metadata:
  author: design-and-bloom
  version: "1.1.0"
  argument-hint: "[output-path]"
---

# Web Design Reference Sheet Generator

Generate a project-specific design reference sheet and companion enforcement skill
from any website codebase, mirroring the structure of the
[Figma Web Design Reference Sheet](https://www.figma.com/community/file/1318219306256490255/web-design-reference-sheet).

---

## Mindset

**Document what exists, not what should exist.**

Every invented token is technical debt dressed as documentation — developers trust the
reference sheet and will use the wrong values in production. When data is missing, write
`TBD — not yet defined`. A reference sheet with honest gaps is more useful than one with
plausible fictions.

---

## When to Use

Apply this skill when:

1. Starting a new project and needing a living design token reference
2. Onboarding a design system from an existing codebase
3. Creating a Figma-compatible reference for designers and developers
4. Auditing existing styles to identify undocumented or orphaned tokens
5. Porting design tokens between frameworks

---

## Workflow

See [`references/workflow.md`](references/workflow.md) for the full 8-step generation process:

1. Discover design tokens

   ```bash
   grep -r "\-\-" src/ --include="*.css" -h | grep -E "^\s+--" | sort -u
   grep -r "@theme\|@font-face\|@import" src/ --include="*.css"
   ```

2. Inspect existing components (read 3–5 representative files)

3. Check dark mode and accessibility patterns

4. Fill [`templates/skill-output.yaml`](templates/skill-output.yaml) — structured scratchpad validated against [`schemas/design-reference.schema.json`](schemas/design-reference.schema.json)

5. Generate `docs/design/design-reference.md` from the template

   ```bash
   # Review populated scratchpad before generating final docs
   cat templates/skill-output.yaml | grep -E "TBD|null" && echo "Gaps found — fill before generating"
   ```

6. Generate the enforcement skill from the enforcement skill template

   ```bash
   bun run generate --template enforcement-skill --output ./{project-slug}/SKILL.md
   ```

7. Confirm output with a structured report listing populated sections and TBD items

   ```bash
   bun run validate --schema schemas/design-reference.schema.json --input templates/skill-output.yaml
   ```

---

## When NOT to Use

- The project has no CSS source files (generated styles only, e.g. CSS-in-JS with no extractable tokens)
- A design reference sheet already exists and only needs updating — use the companion enforcement skill's update workflow instead
- The user wants a generic design system template, not a project-specific one

---

## Anti-Patterns

### NEVER invent or infer design tokens that don't exist in source code

- **WHY:** Invented values create false documentation. Developers trust the reference sheet and may use wrong tokens, causing design inconsistencies that are hard to trace.

| BAD | GOOD |
|-----|------|
| Infer `--space-4: 1rem` because it seems reasonable | Document only values found in CSS; mark missing sections as `TBD — not yet defined` |
| Expand one brand colour into a full 9-shade palette | Document only the hardcoded hex values found; note they are not formalised tokens |

### NEVER generate the reference sheet before reading actual CSS/config files

- **WHY:** Even well-known frameworks have project-level overrides. Reading source files is the only way to capture what is real.

  ```bash
  # BAD: assuming defaults
  # GOOD: always verify
  grep -r "screens" tailwind.config.ts
  ```

### NEVER leave sections empty without a TBD marker

- **WHY:** Blank sections appear complete; `TBD — not yet defined` is an actionable signal that makes gaps impossible to miss.

| BAD | GOOD |
|-----|------|
| `## Dark Mode` followed by empty content | `## Dark Mode` followed by `TBD — not yet defined` |

### NEVER use non-copy-pasteable code examples in the reference sheet

- **WHY:** A reference sheet that requires editing before use wastes developer time and erodes trust.

  ```css
  /* BAD: placeholder that doesn't exist */
  /* GOOD: actual token from source */
  --color-brand-primary: #1a2b3c;
  ```

### NEVER use a generic description in the companion enforcement skill

- **WHY:** A generic description means the skill doesn't activate correctly for the project. A common pitfall is copying the parent skill description verbatim.
- **BAD** — `description: Enforce design tokens for any web project`
- **GOOD** — `description: Enforce design tokens for the Acme Corp marketing site`

---

## References

- [`references/workflow.md`](references/workflow.md) — Full 8-step generation workflow
- [`references/design-reference-template.md`](references/design-reference-template.md) — Output template for `docs/design/design-reference.md`
- [`references/enforcement-skill-template.md`](references/enforcement-skill-template.md) — Output template for the companion enforcement skill
- [`templates/skill-output.yaml`](templates/skill-output.yaml) — Structured scratchpad; populate before writing any markdown
- [`schemas/design-reference.schema.json`](schemas/design-reference.schema.json) — JSON schema for scratchpad validation
- [`references/accessibility-guidelines.md`](references/accessibility-guidelines.md) — WCAG 2.1 AA criteria, contrast ratios, keyboard nav, ARIA
- [`references/typography-principles.md`](references/typography-principles.md) — Hierarchy, readability, type pairing, modular scale
- [`references/psychology-of-color.md`](references/psychology-of-color.md) — Colour associations, brand strategy, harmony, WCAG contrast
- [`references/visual-hierarchy.md`](references/visual-hierarchy.md) — The seven hierarchy variables, Z/F-patterns
- [`references/grid-and-layout-theory.md`](references/grid-and-layout-theory.md) — Column grids, 8-point system, responsive strategies
- [`references/whitespace-theory.md`](references/whitespace-theory.md) — Macro/micro whitespace, density, hierarchy
- [`references/gestalt-principles.md`](references/gestalt-principles.md) — Proximity, similarity, figure-ground, closure, continuity
