# Scenario 01: Token Extraction from a Legacy Site

## User Prompt

You are helping a design agency port the visual identity of a legacy marketing site into a React project. You have already run the initial CSS custom property extraction and received the following output, which was saved to `.context/artifacts/legacy-brand-co/2026-03-13/tokens-css-vars.json`:

```json
{
  "colors": {},
  "computed": {
    "fontFamily": "Arial, sans-serif",
    "fontSize": "16px",
    "lineHeight": "24px",
    "backgroundColor": "rgb(255, 255, 255)",
    "color": "rgb(30, 30, 30)"
  }
}
```

You have also received a second extraction of key elements, saved to `.context/artifacts/legacy-brand-co/2026-03-13/tokens-computed.json`:

```json
{
  "body": { "bg": "rgb(255, 255, 255)", "color": "rgb(30, 30, 30)", "font": "Arial, sans-serif", "fontSize": "16px", "fontWeight": "400", "borderRadius": "0px", "border": "none", "boxShadow": "none", "padding": "0px 0px" },
  "btn": { "bg": "rgb(220, 53, 69)", "color": "rgb(255, 255, 255)", "font": "Arial, sans-serif", "fontSize": "14px", "fontWeight": "700", "borderRadius": "4px", "border": "none", "boxShadow": "none", "padding": "10px 20px" },
  "nav": { "bg": "rgb(30, 30, 30)", "color": "rgb(255, 255, 255)", "font": "Arial, sans-serif", "fontSize": "14px", "fontWeight": "600", "borderRadius": "0px", "border": "none", "boxShadow": "0 2px 4px rgba(0,0,0,0.15)", "padding": "16px 24px" },
  "card": { "bg": "rgb(245, 245, 245)", "color": "rgb(30, 30, 30)", "font": "Arial, sans-serif", "fontSize": "14px", "fontWeight": "400", "borderRadius": "8px", "border": "1px solid rgb(220, 220, 220)", "boxShadow": "0 1px 3px rgba(0,0,0,0.08)", "padding": "24px 24px" }
}
```

Based on this data, produce:

1. `.context/artifacts/legacy-brand-co/2026-03-13/theme/colours.md` — a colour documentation file mapping the extracted RGB values to semantic tokens
2. `.context/artifacts/legacy-brand-co/2026-03-13/theme/overview.md` — a brief summary including the source URL (`https://legacy-brand.co`), extraction date, extraction method, and any important notes about what was discovered

## Expected Behavior

1. Note in `overview.md` that the source site uses hardcoded values and has no CSS custom property design token system
2. Identify the extraction method and note it explicitly in `overview.md`
3. Create a `colours.md` table with Token, Hex, HSL, and Role columns (or equivalent)
4. Convert extracted `rgb()` values to hex format (e.g., `rgb(255,255,255)` → `#FFFFFF`)
5. Convert at least one extracted `rgb()` value to HSL channel format
6. Assign the CTA button background colour (`rgb(220, 53, 69)`) to `--primary`
7. Assign the page background (`rgb(255, 255, 255)`) to `--background`
8. Assign the body text colour (`rgb(30, 30, 30)`) to `--foreground`
9. Assign the card background (`rgb(245, 245, 245)`) to `--muted` or a custom token like `--surface`
10. Place both output files inside `.context/artifacts/legacy-brand-co/2026-03-13/theme/`

## Success Criteria

- **No-token-system note**: `overview.md` contains a note indicating that the source site uses hardcoded values and has no CSS custom property design token system
- **Extraction method noted**: `overview.md` explicitly identifies the extraction method used
- **Colour table format**: `colours.md` contains a markdown table with Token, Hex, HSL, and Role columns
- **RGB to Hex conversion**: `colours.md` converts the extracted `rgb()` values to hex format
- **RGB to HSL conversion**: `colours.md` converts at least one extracted `rgb()` value to HSL channel format
- **Primary token assigned**: `colours.md` assigns `rgb(220, 53, 69)` (CTA button) to `--primary`
- **Background token assigned**: `colours.md` assigns `rgb(255, 255, 255)` (page background) to `--background`
- **Foreground token assigned**: `colours.md` assigns `rgb(30, 30, 30)` (body text) to `--foreground`
- **Muted token assigned**: `colours.md` assigns `rgb(245, 245, 245)` (card background) to `--muted` or `--surface`
- **Correct artifacts path**: Both output files are placed inside `.context/artifacts/legacy-brand-co/2026-03-13/theme/`

## Failure Conditions

- Agent places output files in `docs/` or the project root instead of the artifact directory
- Agent does not note the absence of a CSS custom property system in `overview.md`
- Agent omits the colour table format (Token, Hex, HSL, Role columns)
- Agent does not convert `rgb()` values to hex in `colours.md`
- Agent assigns the CTA button colour to `--accent` instead of `--primary`
- Agent invents token values not derived from the provided extraction data
