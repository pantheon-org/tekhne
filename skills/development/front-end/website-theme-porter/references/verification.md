# Verification Guide

How to interpret visual diffs and fix mismatches when comparing the original site to your local or deployed build.

---

## Quick Start

```bash
# Start local dev server first
bun run dev
```

### agent-browser

```bash
# Screenshot both sides
agent-browser --session source open <TARGET_URL> && \
  agent-browser --session source wait --load networkidle && \
  agent-browser --session source screenshot --full "${ARTIFACTS}/verify-source.png"

agent-browser --session local open http://localhost:5173 && \
  agent-browser --session local wait --load networkidle && \
  agent-browser --session local screenshot --full "${ARTIFACTS}/verify-local.png"

# Side-by-side pixel diff
agent-browser diff url <TARGET_URL> http://localhost:5173 \
  --screenshot "${ARTIFACTS}/diff-homepage.png"
```

### mcp-playwright

With mcp-playwright, omit `filename` to receive screenshots **inline in context** — the agent can read and reason about the images directly without saving files or running a diff tool.

```
playwright_browser_navigate(url: "<TARGET_URL>")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # source — visible inline

playwright_browser_navigate(url: "http://localhost:5173")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # local — visible inline
```

Both images appear in the agent's context. Describe observed differences (colour, font, spacing, layout) directly — no pixel-diff tooling required.

---

## Reading the diff output

### agent-browser diff

`agent-browser diff url` produces:

1. **A diff image** — changed pixels highlighted in red/pink. Unchanged areas appear greyed out.
2. **A mismatch percentage** — percentage of pixels that differ.

### mcp-playwright (inline)

There is no mismatch percentage. Instead, visually inspect both screenshots in context and note:
- Colour differences (wrong shade, missing tint)
- Font rendering differences (wrong family, weight, size)
- Spacing differences (too tight, too loose, wrong section height)
- Missing or misaligned components

### Mismatch thresholds (agent-browser diff)

| Percentage | Interpretation |
|------------|----------------|
| < 5% | Acceptable — minor rendering differences |
| 5–20% | Review needed — likely font, spacing, or colour drift |
| 20–50% | Significant — section layout or component structure differs |
| > 50% | Major — probably wrong page, wrong viewport, or missing content |

---

## Common Mismatches and Fixes

### Wrong font rendering

**Symptom:** Text looks wider/narrower, slightly different weight.

**Cause:** Font not loaded, fallback font used, or font-display: swap delay.

**Fix:**
1. Check `<link>` in `index.html` includes the Google Fonts URL with `display=swap`
2. Verify `--font-sans` in `@theme inline` matches the extracted font family name exactly
3. Check the weight range: Google Fonts URLs must include all needed weights (e.g. `wght@400;600;700`)

```html
<!-- Correct -->
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet" />
```

---

### Colour drift

**Symptom:** Buttons, backgrounds, or text appear in the wrong shade.

**Cause:** HSL conversion was off, or wrong token assigned.

**Fix:**
1. Re-run the colour extraction snippet from `references/extraction.md`
2. Convert the `rgb()` value to hex using the HSL converter
3. Check the assigned token — verify `--primary` is really the CTA colour, not a background

Quick browser check:

**agent-browser:**
```bash
agent-browser eval --stdin <<'JS'
const btn = document.querySelector('button, [class*="btn"]');
getComputedStyle(btn).backgroundColor
JS
```

**mcp-playwright:**
```
playwright_browser_evaluate(function: '() => { const btn = document.querySelector("button, [class*=\\"btn\\"]"); return getComputedStyle(btn).backgroundColor; }')
```

---

### Spacing too tight or too loose

**Symptom:** Sections have wrong padding/margin compared to original.

**Cause:** Extracted spacing values weren't translated to the correct Tailwind scale.

**Fix:**
1. Re-run the spacing extraction snippet
2. Convert px to rem (divide by 16), then map to the Tailwind spacing scale
3. For irregular values, use arbitrary syntax temporarily: `py-[72px]`, then define a token

---

### Missing section or wrong layout order

**Symptom:** A section visible on the original doesn't appear locally.

**Cause:** Unimplemented component, wrong route, or different page structure.

**Fix:** This is a content/structure issue, not a theme issue. Skip for the theme porting phase — focus on visual fidelity of what is present.

---

### Border radius mismatch

**Symptom:** Buttons/cards are more or less rounded.

**Cause:** `--radius` token set to wrong value.

**Fix:** Extract the exact value:

**agent-browser:**
```bash
agent-browser eval 'getComputedStyle(document.querySelector("button")).borderRadius'
```

**mcp-playwright:**
```
playwright_browser_evaluate(function: '() => getComputedStyle(document.querySelector("button")).borderRadius')
```

Then update `--radius` in `:root`.

---

### Shadow too heavy or missing

**Symptom:** Cards look flat or over-shadowed.

**Cause:** Tailwind's built-in shadow scale doesn't match.

**Fix:** Extract the exact `box-shadow`:

**agent-browser:**
```bash
agent-browser eval --stdin <<'JS'
const card = document.querySelector('[class*="card"], article');
card ? getComputedStyle(card).boxShadow : 'not found'
JS
```

**mcp-playwright:**
```
playwright_browser_evaluate(function: '() => { const card = document.querySelector("[class*=\\"card\\"], article"); return card ? getComputedStyle(card).boxShadow : "not found"; }')
```
Then define a custom token:
```css
@theme inline {
  --shadow-card: 0 1px 3px rgb(0 0 0 / 0.08);
}
```

---

### Icon or image sizes different

**Symptom:** Logo or icons are bigger/smaller than original.

**Cause:** Missing `width`/`height` constraints, or SVG viewBox differences.

**Fix:** This is a component-level issue. Note the pixel dimensions from the source screenshot and apply them directly in the component.

---

## Multi-Page Verification

Check each route independently.

**agent-browser:**
```bash
PAGES=("" "/about" "/services" "/pricing" "/contact")
for PAGE in "${PAGES[@]}"; do
  SLUG="${PAGE//\//-}"
  SLUG="${SLUG:-home}"
  agent-browser diff url "<TARGET_URL>${PAGE}" "http://localhost:5173${PAGE}" \
    --screenshot "${ARTIFACTS}/diff${SLUG}.png"
done
```

**mcp-playwright** (inline — navigate to each page pair in sequence):
```
# For each page: screenshot source, then local — inspect inline
playwright_browser_navigate(url: "<TARGET_URL>")
playwright_browser_take_screenshot(fullPage: true)
playwright_browser_navigate(url: "http://localhost:5173")
playwright_browser_take_screenshot(fullPage: true)

# Repeat for /about, /services, etc.
```

---

## Mobile Verification

**agent-browser:**
```bash
# Source at 375px
agent-browser --session source set viewport 375 812
agent-browser --session source open <TARGET_URL>
agent-browser --session source wait --load networkidle
agent-browser --session source screenshot --full "${ARTIFACTS}/verify-source-mobile.png"

# Local at 375px
agent-browser --session local set viewport 375 812
agent-browser --session local open http://localhost:5173
agent-browser --session local wait --load networkidle
agent-browser --session local screenshot --full "${ARTIFACTS}/verify-local-mobile.png"

# Diff
agent-browser diff screenshot \
  --baseline "${ARTIFACTS}/verify-source-mobile.png" \
  "${ARTIFACTS}/verify-local-mobile.png"
```

**mcp-playwright:**
```
playwright_browser_resize(width: 375, height: 812)
playwright_browser_navigate(url: "<TARGET_URL>")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # source mobile — inline

playwright_browser_navigate(url: "http://localhost:5173")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # local mobile — inline
```

---

## Deployed Verification (Vercel)

When a preview URL is available.

**agent-browser:**
```bash
PREVIEW_URL="https://your-project-abc123.vercel.app"

agent-browser diff url <TARGET_URL> "${PREVIEW_URL}" \
  --screenshot "${ARTIFACTS}/diff-deployed-homepage.png"
```

**mcp-playwright:**
```
playwright_browser_navigate(url: "<TARGET_URL>")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # source — inline

playwright_browser_navigate(url: "https://your-project-abc123.vercel.app")
playwright_browser_wait_for(time: 2)
playwright_browser_take_screenshot(fullPage: true)   # deployed — inline
```

---

## Iteration Loop

1. Run diff → note red areas
2. Identify which token/component causes the mismatch (use the quick-fix table above)
3. Update `src/index.css` or the affected component
4. Rebuild: `bun run build && bun run preview`
5. Re-run diff
6. Repeat until mismatch % is acceptable

Aim for < 10% mismatch on primary pages (home, about) before considering the port complete.
