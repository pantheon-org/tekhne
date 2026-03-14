---
name: website-theme-porter
description: >
  Port the visual theme and styling from a live website to a React/Tailwind CSS project.
  Extracts colours, typography, spacing, and component styles — via agent-browser automation,
  manual inspection, curl/wget, or direct source reading — writes structured documentation
  and all artifacts under .context/artifacts/{website}/ with timestamps, applies findings
  as Tailwind v4 CSS tokens, then verifies by visually diffing the original site against
  the local or deployed version. Use when cloning a brand, replicating a design system,
  matching a reference site, migrating visual identity, copying a style guide, or porting
  a theme from any live URL into a React codebase.
---

# Website Theme Porter

Port the visual identity of any live website into a React + Tailwind CSS project.
The workflow is four linear stages: **Extract → Document → Apply → Verify**.

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

## When NOT to Use This Skill

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

## NEVER Do This

- **NEVER wrap HSL values in `hsl()` inside `@theme inline`.**
  In Tailwind v4, `--color-primary: hsl(174 90% 31%)` inside `@theme inline` will
  double-wrap when Tailwind generates utilities, producing broken output.
  **Consequence:** `bg-primary` resolves to `hsl(hsl(174 90% 31%))` — an invalid value; the colour renders as transparent or black.

  ```css
  /* ❌ BAD — double-wraps at utility generation time */
  @theme inline {
    --color-primary: hsl(174 90% 31%);
  }

  /* ✅ GOOD — bare channels in @theme inline; hsl() in :root */
  :root { --primary: hsl(174 90% 31%); }
  @theme inline { --color-primary: var(--primary); }
  ```

- **NEVER extract colours from `:hover` pseudo-states.**
  `getComputedStyle` on a hovered element requires the pointer to be physically over it.
  The value may silently return the resting state, giving a token that looks correct but is wrong.
  **Consequence:** Hover colours get assigned as the base token — all buttons show the hover shade permanently.

- **NEVER use `prefers-color-scheme` media query colours as your base tokens.**
  Computed styles reflect the current OS colour scheme at extraction time.
  **Consequence:** If the machine is in dark mode, extracted "background" is near-black — applying it to a light-mode project inverts the entire colour scheme.
  Always extract in light mode, or explicitly document which mode was active in `overview.md`.

- **NEVER use `/tmp` for any artifact storage.**
  `/tmp` is ephemeral and session-scoped — artifacts saved there are lost when the session ends.
  Always save to `.context/artifacts/<website-slug>/<YYYY-MM-DD>/`.

  ```bash
  # ❌ BAD
  agent-browser screenshot /tmp/source.png

  # ✅ GOOD
  agent-browser screenshot "${ARTIFACTS}/source-full.png"
  ```

- **NEVER copy the source site's actual CSS files or stylesheets into your project.**
  Rebuild every value from scratch using the extracted numbers.

- **NEVER use arbitrary Tailwind values (`bg-[#abc]`) for semantic colours.**
  Arbitrary values bypass the token system — use semantic utility classes instead.

  ```tsx
  // ❌ BAD — hardcoded, bypasses token system
  <button className="bg-[#0D9488] text-white">

  // ✅ GOOD — semantic token, single source of truth
  <button className="bg-primary text-primary-foreground">
  ```

## Reference Files

| File | Priority | When to load |
|------|----------|-------------|
| [references/extraction.md](references/extraction.md) | CRITICAL | Load at the start of Stage 1 — full JS toolkit and colour parsing helpers |
| [references/tailwind-mapping.md](references/tailwind-mapping.md) | CRITICAL | Load at the start of Stage 2 — token tables, HSL converter, Token Decision Tree, index.css template |
| [references/verification.md](references/verification.md) | HIGH | Load at the start of Stage 4 — diff thresholds, mismatch symptom→fix table |
