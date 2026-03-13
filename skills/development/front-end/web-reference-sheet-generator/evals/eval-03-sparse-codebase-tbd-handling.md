# Eval 03 — Sparse Codebase with Minimal Tokens

## Scenario

A new project with almost no design tokens defined yet. Most sections will be TBD.
Tests that the agent does not invent values and correctly marks gaps.

## Input

**User prompt:** `Make a brand reference sheet for this project`

**Codebase state:**

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

No CSS custom properties. No Tailwind config. No font imports. No `aria-` attributes.
No dark mode. No transitions defined. No component library.

## Expected Agent Behaviour

1. Runs all discovery greps — most return empty results
2. Does NOT invent a colour palette or spacing system
3. Extracts the two hardcoded hex values `#0070f3` and `#333` as "inferred tokens (not yet formalised)"
4. Marks the following sections as `TBD — not yet defined`:
   - §3 Typography (no font imports found)
   - §4 Spacing (no tokens)
   - §5 Layout (no grid defined)
   - §6 Borders (only hardcoded `border-radius: 4px`, not a token)
   - §7 Shadows (none found)
   - §8 Motion (none found)
   - §9 Component Patterns (only one rudimentary `.btn`)
   - §10 Accessibility (no ARIA found)
   - §11 Dark Mode
   - §12 Figma Sync
5. Step 7 confirmation block lists **all** TBD sections explicitly
6. Still generates both output files — does not skip or abort

## Success Criteria

- [ ] `docs/design/design-reference.md` is created despite sparse codebase
- [ ] No invented values — only `#0070f3`, `#333`, `#fff` appear as colour references
- [ ] TBD sections contain exactly `TBD — not yet defined`, not empty headings
- [ ] Step 7 confirmation explicitly lists each TBD section
- [ ] Agent does not refuse or say "not enough data" — it produces the document with gaps marked
- [ ] Companion SKILL.md still generated (even if many sections are TBD)

## Failure Modes to Watch

- Agent invents a full colour palette based on the one blue button colour
- Agent omits TBD sections instead of writing `TBD — not yet defined`
- Agent aborts and asks the user to "add more design tokens first"
- Agent writes empty section headings with no content or TBD marker
