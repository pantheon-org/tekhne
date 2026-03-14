# Token Extraction from a Legacy Site

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
