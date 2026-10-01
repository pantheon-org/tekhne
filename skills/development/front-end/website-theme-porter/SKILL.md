---
name: website-theme-porter
description: "Ports the visual identity of any live website into a React plus Tailwind CSS project. Extracts colours, typography, spacing, component styles via browser automation, source inspection, curl, wget, DevTools. Writes structured documentation, applies findings as Tailwind v4 CSS tokens, then verifies by visually diffing the original site against the local build. Use when cloning a brand, replicating a design system, matching a reference site, migrating visual identity, copying a style guide, porting a theme from any live URL into a React codebase, matching a client reference design."
---

# Website Theme Porter

Port the visual identity of any live website into a React + Tailwind CSS project.
The workflow is four linear stages: **Extract → Document → Apply → Verify**.

---

## Principles

- **Document before you code**: write all theme documentation under the artifacts directory (see Artifact Storage Convention) before touching a single source file.
- **Tokens over literals**: every extracted value becomes a semantic CSS token; hardcoded colours never appear in component classes.
- **Method hierarchy**: prefer automated extraction (agent-browser) over manual inspection; document whichever method was used.
- **Verify visually**: the workflow is not complete until a diff screenshot confirms the local build matches the reference site.
- **Respect scope**: only port colours, typography, spacing, and component patterns; never copy the source site's actual stylesheet files.

## When to Use

- Replicating a visual design from a live URL into a React + Tailwind codebase
- Running a brand migration or matching a client reference design
- Porting a design system or replicating a style guide into Tailwind v4 tokens
- Verifying a local build against a reference site with a visual diff

---

## Artifact Storage Convention

All artifacts (screenshots, extracted JSON, theme docs) **must** be saved under:

```
.context/artifacts/<website-slug>/<YYYY-MM-DD>/
```

Where `<website-slug>` is a lowercase, hyphenated version of the target domain
(e.g. `designandbloom-co-uk`), and `<YYYY-MM-DD>` is the extraction date.

```bash
SLUG="designandbloom-co-uk"
DATE=$(date +%Y-%m-%d)
ARTIFACTS=".context/artifacts/${SLUG}/${DATE}"
mkdir -p "${ARTIFACTS}"
```

All screenshot paths, JSON dumps, and theme docs in the steps below use `${ARTIFACTS}/`
as their base. Never save artifacts to `docs/` or any other location.

---

## Reference Loading Schedule

Load each reference file only when its stage begins — do not pre-load all three at once.

| Stage | Load | Do NOT load |
|-------|------|-------------|
| Stage 1: Extract | `references/extraction.md` | tailwind-mapping.md, verification.md |
| Stage 2: Document | `references/tailwind-mapping.md` | extraction.md (done), verification.md |
| Stage 3: Apply | `references/tailwind-mapping.md` (if not cached) | extraction.md, verification.md |
| Stage 4: Verify | `references/verification.md` | extraction.md, tailwind-mapping.md |

---

## Stage 1: Extract

Pull raw design tokens from the target website. **Choose the method that fits your situation:**

### Capture method decision tree

```
Can you run agent-browser OR mcp-playwright in this environment?
  YES → use Method A (automated browser extraction) — most thorough
        agent-browser: run CLI commands directly
        mcp-playwright: use playwright_browser_* MCP tools
  NO ↓

Is the site's HTML/CSS source accessible (View Source, curl, wget)?
  YES → use Method B (source inspection) — good for SSR/static sites
  NO ↓

Can you manually inspect the live site in browser DevTools?
  YES → use Method C (manual inspection) — always possible, least automated
  NO → unblock agent-browser or mcp-playwright access before continuing
```

For the full command equivalents table, see [`references/extraction.md`](references/extraction.md).

### Method A — Automated browser extraction (agent-browser or mcp-playwright)

Steps below use `agent-browser` syntax. If using **mcp-playwright**, translate each command using the equivalents table above. The JS payloads are identical — only the invocation differs.

#### 1a. Navigate and screenshot

```bash
agent-browser open <TARGET_URL> && agent-browser wait --load networkidle
agent-browser screenshot --full "${ARTIFACTS}/source-full.png"
agent-browser set viewport 375 812 && agent-browser screenshot --full "${ARTIFACTS}/source-mobile.png"
agent-browser set viewport 1280 720
```

#### 1b. Extract computed styles via JS

```bash
agent-browser eval --stdin <<'JS'
JSON.stringify({
  colors: (() => {
    const s = getComputedStyle(document.documentElement);
    const props = Array.from(document.styleSheets)
      .flatMap(sheet => { try { return Array.from(sheet.cssRules); } catch { return []; } })
      .filter(r => r.selectorText === ':root')
      .flatMap(r => Array.from(r.style));
    return props.filter(p => p.startsWith('--')).reduce((acc, p) => {
      acc[p] = s.getPropertyValue(p).trim(); return acc;
    }, {});
  })(),
  computed: (() => {
    const el = document.body;
    const s = getComputedStyle(el);
    return {
      fontFamily: s.fontFamily,
      fontSize: s.fontSize,
      lineHeight: s.lineHeight,
      backgroundColor: s.backgroundColor,
      color: s.color,
    };
  })()
}, null, 2)
JS
```

Save JSON output to `${ARTIFACTS}/tokens-css-vars.json`.

**If the `colors` object is empty (site uses no CSS custom properties):**
The site styles everything with hardcoded values. Switch to computed-only extraction:

```bash
agent-browser eval --stdin <<'JS'
const els = { body: document.body, h1: document.querySelector('h1'),
  btn: document.querySelector('button,[class*=btn]'),
  nav: document.querySelector('nav,header'),
  card: document.querySelector('[class*=card],article') };
const out = {};
for (const [name, el] of Object.entries(els)) {
  if (!el) continue;
  const s = getComputedStyle(el);
  out[name] = { bg: s.backgroundColor, color: s.color, font: s.fontFamily,
    fontSize: s.fontSize, fontWeight: s.fontWeight, borderRadius: s.borderRadius,
    border: s.border, boxShadow: s.boxShadow, padding: `${s.paddingTop} ${s.paddingRight}` };
}
JSON.stringify(out, null, 2)
JS
```

Save to `${ARTIFACTS}/tokens-computed.json`. Manually assign tokens from these values in Stage 2.
Note in `${ARTIFACTS}/theme/overview.md`: "source uses hardcoded values — no design token system detected".

**If the target is a SPA (React, Next.js, Vue, etc.):**
Content may not be rendered on first paint. Add a scroll-and-wait step before extracting:

```bash
agent-browser eval 'window.scrollTo(0, document.body.scrollHeight)'
agent-browser wait --load networkidle
agent-browser wait --timeout 2000
```

#### 1c. Collect typography, spacing, and key element styles

```bash
agent-browser eval --stdin <<'JS'
JSON.stringify(
  ['h1','h2','h3','h4','p','a','button'].reduce((acc, tag) => {
    const el = document.querySelector(tag);
    if (!el) return acc;
    const s = getComputedStyle(el);
    acc[tag] = {
      fontFamily: s.fontFamily,
      fontSize: s.fontSize,
      fontWeight: s.fontWeight,
      lineHeight: s.lineHeight,
      color: s.color,
      letterSpacing: s.letterSpacing,
    };
    return acc;
  }, {}),
null, 2)
JS
```

Save JSON output to `${ARTIFACTS}/tokens-typography.json`.

#### 1d. Capture screenshots of key sections

```bash
agent-browser screenshot --full "${ARTIFACTS}/source-annotated.png"
```

See `references/extraction.md` for the full JS extraction toolkit, colour parsing
helpers, nav/card/footer extractors, and page-by-page capture patterns.

---

### Method B — Source inspection (curl / wget / View Source)

Use when `agent-browser` is unavailable or blocked. Fetch the raw HTML and linked
stylesheets, then grep for CSS custom properties and font declarations manually.

```bash
curl -sL <TARGET_URL> -o "${ARTIFACTS}/source.html"
grep -oP '(?<=href=")[^"]*\.css[^"]*' "${ARTIFACTS}/source.html"
curl -sL <CSS_URL> -o "${ARTIFACTS}/styles-main.css"
grep -oP '--[\w-]+\s*:\s*[^;]+' "${ARTIFACTS}/styles-main.css" > "${ARTIFACTS}/tokens-raw.txt"
grep -E '@font-face|fonts\.googleapis' "${ARTIFACTS}/styles-main.css" > "${ARTIFACTS}/fonts-raw.txt"
```

Parse `${ARTIFACTS}/tokens-raw.txt` manually and map values to the token table in Stage 2.
Note in `${ARTIFACTS}/theme/overview.md`: "extraction method: source inspection (curl)".

---

### Method C — Manual DevTools inspection

Use as a fallback or to supplement other methods.

1. Open the target site in Chrome/Firefox DevTools
2. Elements → select `<html>` → Computed tab → filter for `--` → copy all custom properties to `${ARTIFACTS}/tokens-manual.txt`
3. Network tab → filter `.css` → copy stylesheet contents
4. Console: run JS snippets from `references/extraction.md` → paste output to `${ARTIFACTS}/tokens-computed.json`
5. Take screenshots of key sections (hero, nav, footer, mobile) and save under `${ARTIFACTS}/`

Note in `${ARTIFACTS}/theme/overview.md`: "extraction method: manual DevTools inspection".

---

## Stage 2: Document

Write the findings into `${ARTIFACTS}/theme/` before touching any code.

```bash
mkdir -p "${ARTIFACTS}/theme"
```

### Required output files

| File | Contents |
|------|----------|
| `${ARTIFACTS}/theme/colours.md` | All extracted colours with hex/HSL, semantic role, usage context |
| `${ARTIFACTS}/theme/typography.md` | Font families, size scale, weight scale, line heights |
| `${ARTIFACTS}/theme/spacing.md` | Padding/margin patterns, gap values, container widths |
| `${ARTIFACTS}/theme/components.md` | Button, card, nav, footer styles — border-radius, shadows, borders |
| `${ARTIFACTS}/theme/overview.md` | Summary, source URL, extraction date, method used, key decisions |

### Colour documentation format

```markdown
## Colours

| Token | Hex | HSL | Role |
|-------|-----|-----|------|
| --primary | #0D9488 | 174 90% 30% | CTA buttons, links |
| --background | #FFFFFF | 0 0% 100% | Page background |
| --foreground | #111827 | 221 39% 11% | Body text |
```

### Typography documentation format

```markdown
## Typography

**Primary font**: Inter, sans-serif  
**Heading font**: same (or specify if different)

| Element | Size | Weight | Line Height |
|---------|------|--------|-------------|
| h1 | 2.25rem | 700 | 1.2 |
| h2 | 1.875rem | 600 | 1.3 |
| body | 1rem | 400 | 1.6 |
```

Use the **Token Decision Tree** in `references/tailwind-mapping.md` to assign ambiguous colours.

---

## Stage 3: Apply

Translate the documented theme into Tailwind CSS variables and component styles.

### 3a. Update `src/index.css` (Tailwind v4 pattern)

```css
@import "tailwindcss";

/* Semantic colour tokens */
:root {
  --background: hsl(0 0% 100%);
  --foreground: hsl(221 39% 11%);
  --primary: hsl(174 90% 30%);
  --primary-foreground: hsl(0 0% 100%);
  --muted: hsl(210 40% 96%);
  --muted-foreground: hsl(215 16% 47%);
  --border: hsl(214 32% 91%);
  --radius: 0.5rem;
}

/* Map to Tailwind utilities */
@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-border: var(--border);
  --font-sans: "Inter", ui-sans-serif, system-ui, sans-serif;
}

@layer base {
  body {
    background-color: var(--background);
    color: var(--foreground);
    font-family: var(--font-sans);
  }
}
```

### 3b. Add font to `index.html`

```html
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet" />
```

### 3c. Update component classes

Replace hardcoded colours/fonts with semantic Tailwind tokens:

```tsx
// Before
<button className="bg-[#0D9488] text-white px-4 py-2 rounded">

// After
<button className="bg-primary text-primary-foreground px-4 py-2 rounded-[var(--radius)]">
```

Work page-by-page, starting with shared layout components (Header, Footer) then pages.

---

## Stage 4: Verify

Compare the original site visually against the local build.

### 4a. Start local dev server

```bash
bun run dev  # or: npm run dev / vite
```

### 4b. Visual diff

**agent-browser** produces a pixel-diff image and mismatch percentage:
```bash
agent-browser diff url <TARGET_URL> http://localhost:5173 --screenshot "${ARTIFACTS}/diff-homepage.png"
```

**mcp-playwright** — navigate to each URL and take screenshots without a `filename`; images are returned inline into context so the agent can inspect and describe differences directly:
```
playwright_browser_navigate(url: "<TARGET_URL>")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # source — inline

playwright_browser_navigate(url: "http://localhost:5173")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # local — inline
```

### 4c. Deployed verification (if Vercel / other hosting)

```bash
# agent-browser
agent-browser diff url <TARGET_URL> https://<your-preview>.vercel.app --screenshot "${ARTIFACTS}/diff-deployed.png"

# mcp-playwright: repeat the navigate + take_screenshot (no filename) pattern above against the preview URL
```

See `references/verification.md` for the full diff interpretation guide, threshold
table, and symptom→fix patterns for all common mismatches.

---

## When Not to Use

- **Login-walled or paywalled sites** — extraction requires an authenticated session; the browser cannot reach the styled content without credentials.
- **Canvas-rendered or WebGL UIs** — apps that paint entirely to a `<canvas>` element have no CSS to extract; `getComputedStyle` returns nothing useful.
- **Sites with heavily obfuscated class names (CSS Modules / CSS-in-JS hashes)** — class names like `_3xKy9` are generated at build time and change on every deploy; do not attempt to port them. Extract computed values only via Method A JS snippets.
- **Designs you do not have permission to replicate** — confirm you have the right to port the visual identity before starting.

---

## What to Extract

Prioritise in this order:

1. **Colour palette** — backgrounds, text, borders, CTAs, accents
2. **Typography** — font families (check `@font-face` or Google Fonts links), size scale, weights
3. **Spacing rhythm** — common padding/margin values (e.g. 16px, 24px, 48px grid)
4. **Border radius** — buttons, cards, inputs (flat vs rounded)
5. **Shadows** — none, subtle, card, elevated
6. **Component patterns** — button variants, card borders, nav height, footer layout

## Anti-Patterns

### NEVER wrap HSL values in `hsl()` inside `@theme inline`

**WHY:** In Tailwind v4, `--color-primary: hsl(174 90% 31%)` inside `@theme inline` double-wraps when Tailwind generates utilities, so `bg-primary` resolves to `hsl(hsl(174 90% 31%))`, an invalid value; the colour renders as transparent or black.

**BAD**:

```css
@theme inline {
  --color-primary: hsl(174 90% 31%);
}
```

**GOOD**:

```css
:root { --primary: hsl(174 90% 31%); }
@theme inline { --color-primary: var(--primary); }
```

### NEVER extract colours from `:hover` pseudo-states

**WHY:** `getComputedStyle` on a hovered element requires the pointer to be physically over it. The value may silently return the resting state, or the hover shade gets assigned as the base token so all buttons show the hover shade permanently.

**BAD**: Record the colour seen while hovering as the button's base token.

**GOOD**: Extract the resting-state colour and document hover shades separately.

### NEVER use `prefers-color-scheme` media query colours as your base tokens

**WHY:** Computed styles reflect the current OS colour scheme at extraction time. If the machine is in dark mode, the extracted background is near-black, and applying it to a light-mode project inverts the whole scheme.

**BAD**: Extract with the OS in dark mode and record the result as the base palette.

**GOOD**: Extract in light mode, or document which mode was active in `overview.md`.

### NEVER use `/tmp` for any artifact storage

**WHY:** `/tmp` is ephemeral and session-scoped, so artifacts saved there are lost when the session ends.

**BAD**:

```bash
agent-browser screenshot /tmp/source.png
```

**GOOD**:

```bash
agent-browser screenshot "${ARTIFACTS}/source-full.png"
```

### NEVER save artifacts to `docs/` or any other location

**WHY:** The storage convention keeps every site and extraction date separate and findable under one root.

**BAD**: Writing theme docs to `docs/theme.md`.

**GOOD**: Writing them under `${ARTIFACTS}/theme/`.

### NEVER copy the source site's actual CSS files or stylesheets into your project

**WHY:** Copying stylesheets ports someone else's code rather than their visual identity, and drags in selectors and rules you do not own.

**BAD**: `curl -sL <CSS_URL> -o src/vendor.css` and import it.

**GOOD**: Rebuild every value from scratch using the extracted numbers.

### NEVER use arbitrary Tailwind values (`bg-[#abc]`) for semantic colours

**WHY:** Arbitrary values bypass the token system, so there is no single source of truth for the colour.

**BAD**:

```tsx
<button className="bg-[#0D9488] text-white">
```

**GOOD**:

```tsx
<button className="bg-primary text-primary-foreground">
```

### NEVER write source code before the theme documentation exists

**WHY:** Documenting first forces every value to be extracted and named before it is used, which is what stops ad hoc literals leaking into components.

**BAD**: Editing `src/index.css` straight after the first screenshot.

**GOOD**: Write `colours.md`, `typography.md`, `spacing.md`, `components.md` and `overview.md` first, then apply.

### NEVER pre-load all three reference files at once

**WHY:** Each reference belongs to one stage; loading everything up front spends context on material the current stage does not need.

**BAD**: Read `extraction.md`, `tailwind-mapping.md` and `verification.md` before Stage 1.

**GOOD**: Load `references/extraction.md` for Stage 1, `references/tailwind-mapping.md` for Stages 2 and 3, `references/verification.md` for Stage 4.

### NEVER port obfuscated class names

**WHY:** Names like `_3xKy9` are generated at build time and change on every deploy.

**BAD**: Copy `._3xKy9` rules into the project.

**GOOD**: Extract computed values only, via the Method A JS snippets.

### NEVER declare the port complete without a visual diff

**WHY:** Only a diff screenshot confirms the local build matches the reference site.

**BAD**: Finishing after Stage 3 because the code compiles.

**GOOD**: Run Stage 4 and record the diff result in `${ARTIFACTS}/`.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Extraction toolkit | [references/extraction.md](references/extraction.md) | CRITICAL: load at the start of Stage 1 for the full JS toolkit, colour parsing helpers and page capture patterns |
| Tailwind mapping | [references/tailwind-mapping.md](references/tailwind-mapping.md) | CRITICAL: load at the start of Stage 2 for token tables, HSL converter, Token Decision Tree and the stylesheet template |
| Verification | [references/verification.md](references/verification.md) | HIGH: load at the start of Stage 4 for diff thresholds and the symptom-to-fix table |
