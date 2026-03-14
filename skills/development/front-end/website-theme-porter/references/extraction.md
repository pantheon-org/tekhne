# CSS Extraction Toolkit

Reusable JavaScript snippets to extract design tokens from a live page.

Run via `agent-browser eval --stdin` **or** `playwright_browser_evaluate(function: '() => { … }')` — the JS payloads are identical; only the invocation differs.

**mcp-playwright advantage:** `playwright_browser_take_screenshot()` (no `filename`) returns the image inline in the agent's context for immediate visual inspection — no file I/O needed.

---

## agent-browser vs mcp-playwright Command Equivalents

| agent-browser | mcp-playwright |
|---|---|
| `agent-browser open <URL>` | `playwright_browser_navigate(url)` |
| `agent-browser wait --load networkidle` | `playwright_browser_wait_for(time: 2)` |
| `agent-browser screenshot --full <path>` | `playwright_browser_take_screenshot(filename, fullPage: true)` |
| `agent-browser set viewport 375 812` | `playwright_browser_resize(width: 375, height: 812)` |
| `agent-browser eval '<JS>'` | `playwright_browser_evaluate(function: '() => { <JS> }')` |
| `agent-browser eval --stdin <<'JS' … JS` | `playwright_browser_evaluate(function: '() => { … }')` |
| `agent-browser --session source open <URL>` | open a new tab: `playwright_browser_tabs(action: "new")` then `playwright_browser_navigate(url)` |
| `agent-browser diff url <A> <B>` | screenshot both URLs without `filename` — images are returned inline for direct visual comparison |

Omit `filename` on `playwright_browser_take_screenshot` during verification to leverage inline image context.

---

## Full Token Dump

Extracts all CSS custom properties from `:root` across all loaded stylesheets:

```bash
agent-browser eval --stdin <<'JS'
JSON.stringify((() => {
  const style = getComputedStyle(document.documentElement);
  const props = [];
  for (const sheet of document.styleSheets) {
    try {
      for (const rule of sheet.cssRules) {
        if (rule.selectorText === ':root') {
          for (const prop of rule.style) {
            props.push([prop, style.getPropertyValue(prop).trim()]);
          }
        }
      }
    } catch (_) { /* cross-origin sheet */ }
  }
  return Object.fromEntries(props);
})(), null, 2)
JS
```

---

## Colour Extraction

### All background and text colours across the page

```bash
agent-browser eval --stdin <<'JS'
JSON.stringify((() => {
  const seen = new Set();
  const result = [];
  for (const el of document.querySelectorAll('*')) {
    const s = getComputedStyle(el);
    for (const prop of ['color', 'backgroundColor', 'borderColor']) {
      const val = s[prop];
      if (val && val !== 'rgba(0, 0, 0, 0)' && val !== 'transparent' && !seen.has(val)) {
        seen.add(val);
        result.push({ property: prop, value: val, tag: el.tagName.toLowerCase() });
      }
    }
  }
  return result.slice(0, 40); // top 40 unique colours
})(), null, 2)
JS
```

### Convert rgb() to hex (run in browser context)

```bash
agent-browser eval --stdin <<'JS'
function rgbToHex(rgb) {
  const m = rgb.match(/\d+/g);
  if (!m || m.length < 3) return rgb;
  return '#' + m.slice(0,3).map(n => parseInt(n).toString(16).padStart(2,'0')).join('');
}
// Example usage:
rgbToHex('rgb(13, 148, 136)')  // '#0d9488'
JS
```

---

## Typography Extraction

### Full font stack for key elements

```bash
agent-browser eval --stdin <<'JS'
JSON.stringify(
  ['h1','h2','h3','h4','h5','p','li','a','button','input','label','span']
    .reduce((acc, tag) => {
      const el = document.querySelector(tag);
      if (!el) return acc;
      const s = getComputedStyle(el);
      acc[tag] = {
        fontFamily: s.fontFamily.split(',')[0].replace(/['"]/g, '').trim(),
        fontSize: s.fontSize,
        fontWeight: s.fontWeight,
        lineHeight: s.lineHeight,
        letterSpacing: s.letterSpacing,
        textTransform: s.textTransform,
        color: s.color,
      };
      return acc;
    }, {}),
null, 2)
JS
```

### Detect Google Fonts or font-face declarations

```bash
agent-browser eval --stdin <<'JS'
const fonts = [];
for (const sheet of document.styleSheets) {
  try {
    for (const rule of sheet.cssRules) {
      if (rule.type === CSSRule.FONT_FACE_RULE) {
        fonts.push({
          family: rule.style.getPropertyValue('font-family'),
          src: rule.style.getPropertyValue('src').split(',')[0].substring(0, 100),
        });
      }
    }
  } catch (_) {}
}
// Also check link tags
const links = Array.from(document.querySelectorAll('link[href*="fonts.googleapis"]'))
  .map(l => l.href);
JSON.stringify({ fontFace: fonts, googleFonts: links }, null, 2)
JS
```

---

## Spacing Extraction

### Common padding/margin values used across layout elements

```bash
agent-browser eval --stdin <<'JS'
JSON.stringify((() => {
  const freq = {};
  for (const el of document.querySelectorAll('section, div, article, header, footer, nav, main')) {
    const s = getComputedStyle(el);
    for (const prop of ['paddingTop','paddingBottom','paddingLeft','paddingRight','marginTop','marginBottom']) {
      const v = s[prop];
      if (v && v !== '0px') freq[v] = (freq[v] || 0) + 1;
    }
  }
  return Object.entries(freq).sort((a,b) => b[1]-a[1]).slice(0,20).map(([v,n]) => `${v} (×${n})`);
})(), null, 2)
JS
```

---

## Component Style Extraction

### Button variants

```bash
agent-browser eval --stdin <<'JS'
const selectors = ['button','[class*="btn"]','[class*="button"]','a[class*="cta"]'];
const results = [];
for (const sel of selectors) {
  const els = document.querySelectorAll(sel);
  for (const el of Array.from(els).slice(0, 3)) {
    const s = getComputedStyle(el);
    results.push({
      selector: sel,
      text: el.textContent?.trim().slice(0, 30),
      bg: s.backgroundColor,
      color: s.color,
      border: s.border,
      borderRadius: s.borderRadius,
      padding: `${s.paddingTop} ${s.paddingRight} ${s.paddingBottom} ${s.paddingLeft}`,
      fontSize: s.fontSize,
      fontWeight: s.fontWeight,
      boxShadow: s.boxShadow,
    });
  }
}
JSON.stringify(results, null, 2)
JS
```

### Navigation bar

```bash
agent-browser eval --stdin <<'JS'
const nav = document.querySelector('nav, header, [class*="nav"], [class*="header"]');
if (!nav) 'no nav found';
else {
  const s = getComputedStyle(nav);
  JSON.stringify({
    height: nav.getBoundingClientRect().height,
    backgroundColor: s.backgroundColor,
    borderBottom: s.borderBottom,
    boxShadow: s.boxShadow,
    position: s.position,
    padding: `${s.paddingTop} ${s.paddingRight}`,
    fontFamily: s.fontFamily.split(',')[0].replace(/['"]/g,'').trim(),
    fontSize: s.fontSize,
    fontWeight: s.fontWeight,
    color: s.color,
  }, null, 2);
}
JS
```

### Card / content block

```bash
agent-browser eval --stdin <<'JS'
const card = document.querySelector('[class*="card"], article, .post, [class*="tile"]');
if (!card) 'no card found';
else {
  const s = getComputedStyle(card);
  JSON.stringify({
    backgroundColor: s.backgroundColor,
    border: s.border,
    borderRadius: s.borderRadius,
    boxShadow: s.boxShadow,
    padding: `${s.paddingTop} ${s.paddingRight}`,
  }, null, 2);
}
JS
```

---

## Page-by-Page Capture

For multi-page sites, snapshot each route.

**agent-browser:**
```bash
# ARTIFACTS must be set before running (see SKILL.md Artifact Storage Convention)
PAGES=("" "/about" "/services" "/pricing" "/contact")
for PAGE in "${PAGES[@]}"; do
  agent-browser open "${TARGET_URL}${PAGE}" && \
  agent-browser wait --load networkidle && \
  agent-browser screenshot --full "${ARTIFACTS}/source${PAGE//\//-}.png"
done
```

**mcp-playwright** (screenshots returned inline — no filename needed for visual inspection):
```
# Repeat for each page
playwright_browser_navigate(url: "<TARGET_URL>")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # visible inline in context

playwright_browser_navigate(url: "<TARGET_URL>/about")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)
```

Provide `filename` only if you need to persist screenshots for later comparison.

---

## Tips

- Run extractions on the **fully loaded** page — wait for `networkidle` before eval
- If CSS vars return empty, the site may use inline styles or a CSS-in-JS library — use the computed style approach instead
- Cross-origin stylesheets will throw on `cssRules` access — wrap in `try/catch` (already done in snippets above)
- For SPAs, navigate to each route and re-run extractions — styles may vary per page
