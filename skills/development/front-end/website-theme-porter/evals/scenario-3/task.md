# Document Extracted Colours for a SaaS Dashboard

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
