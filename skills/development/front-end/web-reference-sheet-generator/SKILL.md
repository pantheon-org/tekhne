---
name: web-reference-sheet-generator
description: >
  Generate a comprehensive web design reference sheet (docs/design/design-reference.md)
  and its companion enforcement skill (.agents/skills/{project-slug}/SKILL.md) for any
  website project. Extracts tokens from CSS files, validates completeness against a JSON
  schema scratchpad, inspects existing components, and produces a 12-section living
  document covering colours, typography, spacing, layout, borders, shadows, motion,
  component patterns, accessibility, dark mode, and Figma sync notes. Use when starting
  a new project, onboarding a design system, creating a Figma reference sheet, porting
  design tokens, or auditing existing styles. Triggers on: "create a design reference",
  "generate a style guide", "document the design tokens", "make a brand reference sheet",
  "create design docs", "port design tokens", "audit existing styles".
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

## Workflow

See [`references/workflow.md`](references/workflow.md) for the full 8-step generation process:

1. Discover design tokens (grep CSS for `--` custom properties, `@theme` blocks, font imports)

   ```bash
   grep -r "\-\-" src/ --include="*.css" -h | grep -E "^\s+--" | sort -u
   grep -r "@theme\|@font-face\|@import" src/ --include="*.css"
   ```
2. Inspect existing components (read 3–5 representative files)
3. Check dark mode implementation
4. Check accessibility patterns
5. Fill [`templates/skill-output.yaml`](templates/skill-output.yaml) — structured scratchpad validated against [`schemas/design-reference.schema.json`](schemas/design-reference.schema.json)
6. Generate `docs/design/design-reference.md` from the template
7. Generate `.agents/skills/{project-slug}/SKILL.md` from the enforcement skill template
8. Confirm output with a structured report listing populated sections and TBD items

---

## Design Knowledge References

### CRITICAL (required for output quality)

| File | Contents |
|---|---|
| [`references/workflow.md`](references/workflow.md) | Full 8-step generation workflow |
| [`references/design-reference-template.md`](references/design-reference-template.md) | Output template for `docs/design/design-reference.md` |
| [`references/enforcement-skill-template.md`](references/enforcement-skill-template.md) | Output template for `.agents/skills/{project-slug}/SKILL.md` |
| [`templates/skill-output.yaml`](templates/skill-output.yaml) | Structured scratchpad — populate before writing any markdown |
| [`schemas/design-reference.schema.json`](schemas/design-reference.schema.json) | JSON schema to validate the populated scratchpad |
| [`references/accessibility-guidelines.md`](references/accessibility-guidelines.md) | WCAG 2.1 AA criteria, contrast ratios, keyboard nav, ARIA, touch targets |
| [`references/typography-principles.md`](references/typography-principles.md) | Hierarchy, readability, type pairing, modular scale, typographic colour |

### HIGH (consult when relevant section has data)

| File | Contents |
|---|---|
| [`references/psychology-of-color.md`](references/psychology-of-color.md) | Colour associations, brand strategy, harmony, cultural context, WCAG contrast |
| [`references/visual-hierarchy.md`](references/visual-hierarchy.md) | The seven hierarchy variables, Z/F-patterns, component hierarchy patterns |
| [`references/grid-and-layout-theory.md`](references/grid-and-layout-theory.md) | Column grids, 8-point system, responsive strategies, proportional systems |
| [`references/whitespace-theory.md`](references/whitespace-theory.md) | Macro/micro whitespace, density, hierarchy, component spacing patterns |
| [`references/gestalt-principles.md`](references/gestalt-principles.md) | Proximity, similarity, figure-ground, closure, continuity, Prägnanz |
| [`references/elements-of-design.md`](references/elements-of-design.md) | Line, shape, form, space, texture, value, colour — the raw vocabulary |
| [`references/principles-of-design.md`](references/principles-of-design.md) | Balance, contrast, emphasis, movement, pattern, rhythm, unity, proportion, alignment |

---

## When NOT to Use

- The project has no CSS source files (generated styles only, e.g. CSS-in-JS with no extractable tokens)
- A design reference sheet already exists and only needs updating — use the companion enforcement skill's update workflow instead
- The user wants a generic design system template, not a project-specific one — redirect to a design system scaffolding tool

---

## Anti-Patterns

### NEVER invent or infer design tokens that don't exist in source code

- **WHY**: Invented values create false documentation. Developers trust the reference sheet and may use wrong tokens, causing design inconsistencies that are hard to trace.
- **BAD**: Inferring `--space-4: 1rem` because it seems reasonable, or expanding one brand color into a full 9-shade palette.
- **GOOD**: Document only the values explicitly found in CSS files; mark missing sections as `TBD — not yet defined`.

### NEVER generate the reference sheet before reading actual CSS/config files

- **WHY**: Even well-known frameworks have project-level overrides. Reading source files is the only way to capture what is real, not what is typical.
- **BAD**: Assuming Tailwind breakpoints are `sm/md/lg/xl` without checking `tailwind.config.ts`.
- **GOOD**: Run grep on config/CSS to confirm actual breakpoint values before documenting them.

### NEVER leave sections empty without a TBD marker

- **WHY**: Blank sections are invisible gaps that appear complete; `TBD — not yet defined` is an actionable signal that appears in the confirmation block and makes gaps impossible to miss.
- **BAD**: `## Dark Mode\n\n` (empty heading that looks like the section was intentionally left blank).
- **GOOD**: `## Dark Mode\n\nTBD — not yet defined`.

### NEVER use non-copy-pasteable code examples in the reference sheet

- **WHY**: A reference sheet that requires editing before use wastes developer time and introduces errors. Examples that don't match actual code erode trust in the document.
- **BAD**: `<Button variant="primary" />` when that component doesn't exist yet.
- **GOOD**: Actual HTML/JSX copied from source files that renders as-is.

## Rules

### Never invent design values

**WHY:** Invented values create false documentation. Developers trust the reference sheet and may use wrong tokens, causing inconsistencies that are hard to trace.

| BAD | GOOD |
|---|---|
| Agent infers a `--space-4: 1rem` token that doesn't exist in code | Agent writes `TBD — not yet defined` for the spacing section |
| Agent adds a full 9-shade colour palette from one button colour | Agent documents only the 2 hardcoded hex values found, notes they're not yet formalised tokens |

### Mark gaps as `TBD`

**WHY:** Blank sections are invisible gaps; `TBD — not yet defined` is an actionable signal to the team. It appears in the confirmation block, making gaps impossible to miss.

| BAD | GOOD |
|---|---|
| `## Dark Mode\n\n` (empty heading) | `## Dark Mode\n\nTBD — not yet defined` |
| Skipping a section entirely | Including the section header with explicit TBD marker |

### Keep patterns copy-pasteable

**WHY:** A reference sheet that requires editing before use wastes developer time and introduces errors. Code examples that don't run erode trust in the document.

| BAD | GOOD |
|---|---|
| `<Button variant="primary" />` (JSX component that doesn't exist) | Actual HTML/JSX from the codebase that renders as-is |
| Pseudo-code like `{primary button styles}` | Real className strings copied from source files |

### One token file is the master

**WHY:** Multiple token sources create ambiguity about which file to update. Naming the authoritative file makes the reference sheet a navigation aid, not just a snapshot.

**Consequence:** When developers don't know the master file, they update whichever CSS file is open — creating duplicate or conflicting tokens. Merge conflicts follow, and the reference sheet becomes unreliable. Name the single source of truth explicitly in §1 Brand Identity.

### The companion SKILL.md is always project-specific

**WHY:** A generic description means the skill doesn't activate correctly for the project. Project-specific descriptions ensure the enforcement skill fires when working on that codebase.

| BAD | GOOD |
|---|---|
| `description: Enforce design tokens for any web project` | `description: Enforce design tokens for the Acme Corp marketing site` |

### Do not generate from memory

**WHY:** Even well-known frameworks have project-level overrides. Reading the actual source files is the only way to capture what is real, not what is typical.

| BAD | GOOD |
|---|---|
| Assuming Tailwind breakpoints are `sm/md/lg/xl` without checking | Running grep on the config/CSS to confirm actual breakpoint values |
