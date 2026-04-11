# Scenario 03: Generate a Brand Reference Sheet for a Sparse, Early-Stage Project

## User Prompt

You are working in a new project with almost no design tokens defined yet.

## Codebase state

`src/index.css`:
```css
body {
  font-family: sans-serif;
  color: #333;
  background: #fff;
}

.btn {
  background: #0070f3;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 4px;
}
```

No CSS custom properties (`--`). No Tailwind config. No font imports. No `aria-` attributes. No dark mode. No transitions defined. No component library.

Make a brand reference sheet for this project.

## Expected Behavior

1. Produce `docs/design/design-reference.md` despite the sparse codebase — do not abort, refuse, or ask the user to add tokens first
2. Use only the colours actually found (`#0070f3`, `#333`, `#fff`) — do not invent additional palette colours from the single button colour
3. Do not expand the single button colour into a full colour palette
4. Mark §3 Typography as `TBD — not yet defined` (no font imports found)
5. Mark §4 Spacing as `TBD — not yet defined` (no spacing tokens found)
6. Mark §7 Shadows as `TBD — not yet defined`
7. Mark §8 Motion as `TBD — not yet defined`
8. Mark §11 Dark Mode as `TBD — not yet defined`
9. Include all 12 sections — no section heading missing from the output
10. List each TBD section explicitly in the Step 8 confirmation block
11. Generate the companion SKILL.md under `.agents/skills/` — do not skip due to sparse data
12. Ensure no section heading is empty — every section has either real content or `TBD — not yet defined`

## Success Criteria

- **Does not abort**: Agent produced `docs/design/design-reference.md` despite sparse codebase
- **No invented tokens**: No invented colour tokens — only `#0070f3`, `#333`, and `#fff` appear as colour references in the output
- **No palette expansion**: Agent did not generate a full colour palette from the single `#0070f3` button colour
- **Typography TBD**: §3 Typography contains `TBD — not yet defined`
- **Spacing TBD**: §4 Spacing contains `TBD — not yet defined`
- **Shadows TBD**: §7 Shadows contains `TBD — not yet defined`
- **Motion TBD**: §8 Motion contains `TBD — not yet defined`
- **Dark mode TBD**: §11 Dark Mode contains `TBD — not yet defined`
- **All 12 sections present**: All 12 sections are present — no section heading missing from the output
- **TBD sections listed in confirmation**: Step 8 confirmation block explicitly lists each TBD section
- **Companion SKILL.md not skipped**: Companion SKILL.md generated under `.agents/skills/` — not skipped due to sparse data
- **Companion SKILL.md specificity**: Companion SKILL.md description names the specific project
- **No empty sections**: No empty section headings — every section has either real content or `TBD — not yet defined`

## Failure Conditions

- Agent refuses to produce output, asks the user to add tokens first, or aborts due to sparse data
- Agent invents colour tokens beyond `#0070f3`, `#333`, and `#fff`
- Agent generates a full colour palette (e.g., shades, tints) from the single button colour
- Agent leaves §3 Typography, §4 Spacing, §7 Shadows, §8 Motion, or §11 Dark Mode populated with invented values instead of TBD
- Agent omits any of the 12 required sections
- Agent skips the companion SKILL.md because the codebase is sparse
- Agent leaves any section heading with no content (neither real values nor TBD)
