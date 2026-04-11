# Scenario 03: Document Extracted Colours for a SaaS Dashboard

## User Prompt

A product team wants to port the visual theme of a SaaS dashboard into their React codebase. Stage 1 extraction is complete. The raw data has been saved at `.context/artifacts/saas-dashboard/2026-03-13/tokens-computed.json`.

Based on the extracted data below, produce `colours.md` inside `.context/artifacts/saas-dashboard/2026-03-13/theme/`.

**Extracted computed values:**

```json
{
  "body":   { "bg": "rgb(248, 250, 252)", "color": "rgb(15, 23, 42)" },
  "h1":     { "color": "rgb(15, 23, 42)" },
  "btn_primary": { "bg": "rgb(99, 102, 241)", "color": "rgb(255, 255, 255)" },
  "btn_secondary": { "bg": "rgb(241, 245, 249)", "color": "rgb(51, 65, 85)" },
  "nav":    { "bg": "rgb(15, 23, 42)", "color": "rgb(248, 250, 252)" },
  "tag":    { "bg": "rgb(224, 231, 255)", "color": "rgb(67, 56, 202)" },
  "input":  { "bg": "rgb(255, 255, 255)", "border": "rgb(203, 213, 225)" },
  "card":   { "bg": "rgb(255, 255, 255)", "border": "rgb(226, 232, 240)" },
  "sidebar": { "bg": "rgb(241, 245, 249)" }
}
```

In addition to `colours.md`, note any colour assignment decisions or tie-breaking rationale in a `decisions.md` file in the same directory — this will help the team understand how ambiguous colours were assigned.

## Expected Behavior

1. Place `colours.md` inside `.context/artifacts/saas-dashboard/2026-03-13/theme/` — not in `docs/` or project root
2. Create a `colours.md` table with Token, Hex, HSL, and Role columns
3. Assign the primary CTA button colour (`rgb(99, 102, 241)` / `#6366F1`) to `--primary`
4. Assign the secondary button background (`rgb(241, 245, 249)`) to `--secondary` or `--muted` (not `--primary` or `--accent`)
5. Assign the tag background (`rgb(224, 231, 255)`) to `--accent` (decorative/highlight role, not `--secondary`)
6. Assign the sidebar background (`rgb(241, 245, 249)`) or card bg to `--muted`
7. Assign the input border or card border colour to `--border`
8. Assign the page/body background (`rgb(248, 250, 252)`) to `--background`
9. Assign the body text colour (`rgb(15, 23, 42)`) to `--foreground`
10. Explain at least one tie-breaking decision in `decisions.md` (e.g., why sidebar is `--muted` vs `--secondary`, or why tag is `--accent` vs `--secondary`)

## Success Criteria

- **Correct output path**: `colours.md` is placed inside `.context/artifacts/saas-dashboard/2026-03-13/theme/`
- **Colour table format**: `colours.md` contains a table with Token, Hex, HSL, and Role columns
- **Primary = CTA button**: `rgb(99, 102, 241)` / `#6366F1` is assigned to `--primary`
- **Secondary = secondary button**: `rgb(241, 245, 249)` is assigned to `--secondary` or `--muted` (not `--primary` or `--accent`)
- **Accent = tag/highlight**: `rgb(224, 231, 255)` is assigned to `--accent` (not `--secondary`, since it is decorative/highlight only)
- **Muted = sidebar**: The sidebar background or card bg is assigned to `--muted`
- **Border token**: The input border or card border colour is assigned to `--border`
- **Background token**: `rgb(248, 250, 252)` is assigned to `--background`
- **Foreground token**: `rgb(15, 23, 42)` is assigned to `--foreground`
- **Tie-breaking documented**: `decisions.md` explains at least one assignment decision for an ambiguous colour

## Failure Conditions

- Agent places `colours.md` in `docs/` or the project root instead of the artifact directory
- Agent assigns the tag colour (`rgb(224, 231, 255)`) to `--secondary` instead of `--accent`
- Agent assigns the secondary button background to `--primary` instead of `--secondary` or `--muted`
- Agent omits `decisions.md` or provides no tie-breaking rationale
- Agent omits the Hex or HSL columns from the colour table
- Agent does not assign a `--border` token
