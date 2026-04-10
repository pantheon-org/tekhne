# Publication Standards and Accessibility Guidelines

Best practices for publication-quality scientific diagrams.

## Colour and Accessibility

**Colorblind-safe palettes:**
- Use Okabe-Ito palette (8 colours, safe for all colour-vision deficiencies)
- Avoid red/green combinations as the primary distinguishing factor
- ALWAYS add patterns, shapes, or labels as secondary encoding — never rely on colour alone

**Contrast ratios:**
- Minimum 4.5:1 for text on backgrounds (WCAG AA)
- Minimum 3:1 for graphical elements

## Typography

**Font recommendations:**
- Body labels: sans-serif (Arial, Helvetica, or Liberation Sans), minimum 8pt for journal
- Title labels: 10–12pt
- NEVER use decorative or script fonts in scientific figures

**Label placement:**
- Labels must not overlap arrows or element boundaries
- Abbreviations must be defined in the figure caption or a legend

## Layout and Composition

**Whitespace:**
- Allow at least 10% margin around diagram content
- Group related elements with visual proximity, not just colour

**Arrows and connectors:**
- Use arrowheads consistently: filled for activation, bar-ended for inhibition (biological conventions)
- NEVER mix arrow styles without a legend

## File Format and Resolution

**For journal submission:**
- Vector formats preferred: SVG, EPS, PDF
- Raster fallback: 300 DPI minimum at final print size
- TIFF or PNG for raster; NEVER JPEG (compression artefacts at print resolution)

**For conference/poster:**
- 150 DPI acceptable for large-format printing
- Embed all fonts in PDF export

## Figure Caption Standards

A complete caption answers: what is shown, what the abbreviations mean, what the scale or units are, and (for data plots) what error bars represent.

**Template:** `"Figure N. [One-sentence description of what is shown]. [Abbreviation definitions.] [Scale/unit notes.] [Error bar definition if applicable.]"`

## Integration with Papers

- Reference every figure in the main text before it appears
- Store all figures in `figures/` subdirectory
- Use relative paths in LaTeX/Markdown: `figures/consort-flowchart.png`
- NEVER embed figures as base64 in source files
