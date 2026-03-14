# CSS to Tailwind Token Mapping

Translate raw CSS computed values extracted from a live site into Tailwind CSS variables and utility classes.

---

## Colours

### rgb() / rgba() → HSL CSS variable

Tailwind v4 uses HSL for semantic tokens. Convert extracted `rgb()` values:

| Extracted (rgb) | Hex | HSL | Token |
|-----------------|-----|-----|-------|
| `rgb(13, 148, 136)` | `#0D9488` | `174 90% 31%` | `--primary` |
| `rgb(255, 255, 255)` | `#FFFFFF` | `0 0% 100%` | `--background` |
| `rgb(17, 24, 39)` | `#111827` | `221 39% 11%` | `--foreground` |
| `rgb(107, 114, 128)` | `#6B7280` | `220 9% 46%` | `--muted-foreground` |
| `rgb(229, 231, 235)` | `#E5E7EB` | `220 14% 96%` | `--border` |

**Quick HSL converter** (run in browser):
```js
function toHsl(rgb) {
  const [r, g, b] = rgb.match(/\d+/g).map(Number).map(n => n / 255);
  const max = Math.max(r, g, b), min = Math.min(r, g, b);
  let h, s, l = (max + min) / 2;
  if (max === min) { h = s = 0; }
  else {
    const d = max - min;
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    switch (max) {
      case r: h = ((g - b) / d + (g < b ? 6 : 0)) / 6; break;
      case g: h = ((b - r) / d + 2) / 6; break;
      case b: h = ((r - g) / d + 4) / 6; break;
    }
  }
  return `${Math.round(h * 360)} ${Math.round(s * 100)}% ${Math.round(l * 100)}%`;
}
```

### Semantic token assignment

Map extracted colours to these standard tokens:

| Token | Description | Typical source element |
|-------|-------------|------------------------|
| `--background` | Page background | `body` bg |
| `--foreground` | Default text | `body` color |
| `--primary` | Main brand / CTA | Primary button bg |
| `--primary-foreground` | Text on primary | Primary button text |
| `--secondary` | Secondary actions | Secondary button bg |
| `--secondary-foreground` | Text on secondary | Secondary button text |
| `--muted` | Subtle backgrounds | Card, sidebar bg |
| `--muted-foreground` | De-emphasised text | Captions, placeholders |
| `--accent` | Highlight/hover | Link hover, tag bg |
| `--accent-foreground` | Text on accent | — |
| `--border` | Dividers, outlines | `border-color` of inputs |
| `--ring` | Focus ring | `:focus` outline |
| `--destructive` | Errors, delete | Error text / button |
| `--destructive-foreground` | Text on destructive | — |
| `--card` | Card surface | Card component bg |
| `--card-foreground` | Text on card | Card text color |

### Token Decision Tree

When a colour does not obviously map to a single token, walk this tree:

```
Is it the most prominent interactive colour (CTA button bg, main link)?
  YES → --primary
  NO ↓

Is it used on a second type of button or a less prominent interactive element?
  YES → --secondary
  NO ↓

Is it used for hover states, tag backgrounds, or inline highlights only?
  YES → --accent
  NO ↓

Is it a subtle background (sidebar, card surface, input field bg)?
  YES → --muted  (text on it → --muted-foreground)
  NO ↓

Is it used on borders, dividers, or form input outlines?
  YES → --border
  NO ↓

Is it the page-level background?
  YES → --background  (text on it → --foreground)
  NO → define a custom token (e.g. --surface, --panel, --hero-bg)
```

**Tie-breaking rules:**
- `--secondary` vs `--accent`: prefer `--accent` for decorative hover/highlight effects;
  use `--secondary` when the element has an interactive affordance (click, focus, etc.).
- `:hover` colours: `getComputedStyle` may silently return the resting state if the
  pointer is not physically over the element. Always verify against the base state.
- Dark mode: if extraction was performed with the OS in dark mode, all
  background/foreground values will be dark-mode variants. Re-extract in light mode
  or document which mode was active in `docs/theme/overview.md`.
- Colours appearing only in `@media (prefers-color-scheme: dark)` blocks belong in
  `docs/theme/dark-mode.md`, not the main token map.

---

## Typography

### Font family

| Extracted value | Tailwind token | CSS variable |
|-----------------|----------------|--------------|
| `"Inter", sans-serif` | `font-sans` | `--font-sans: 'Inter', ui-sans-serif, system-ui, sans-serif` |
| `"Playfair Display", serif` | `font-serif` | `--font-serif: 'Playfair Display', ui-serif, Georgia, serif` |
| `"JetBrains Mono", monospace` | `font-mono` | `--font-mono: 'JetBrains Mono', ui-monospace, monospace` |

Define in `@theme inline`:
```css
@theme inline {
  --font-sans: 'Inter', ui-sans-serif, system-ui, sans-serif;
}
```

### Font size scale

Map extracted `px` values to Tailwind size tokens:

| px | rem | Tailwind class |
|----|-----|----------------|
| 12px | 0.75rem | `text-xs` |
| 14px | 0.875rem | `text-sm` |
| 16px | 1rem | `text-base` |
| 18px | 1.125rem | `text-lg` |
| 20px | 1.25rem | `text-xl` |
| 24px | 1.5rem | `text-2xl` |
| 30px | 1.875rem | `text-3xl` |
| 36px | 2.25rem | `text-4xl` |
| 48px | 3rem | `text-5xl` |
| 60px | 3.75rem | `text-6xl` |

If the site uses a non-standard scale, define custom tokens:
```css
@theme inline {
  --text-display: 3.5rem;
}
```

### Font weight

| Extracted | Tailwind class |
|-----------|----------------|
| 300 | `font-light` |
| 400 | `font-normal` |
| 500 | `font-medium` |
| 600 | `font-semibold` |
| 700 | `font-bold` |
| 800 | `font-extrabold` |
| 900 | `font-black` |

### Line height

| Extracted | Tailwind class |
|-----------|----------------|
| 1 | `leading-none` |
| 1.25 | `leading-tight` |
| 1.375 | `leading-snug` |
| 1.5 | `leading-normal` |
| 1.625 | `leading-relaxed` |
| 2 | `leading-loose` |

---

## Spacing

Map extracted `px` values to Tailwind spacing scale (1 unit = 0.25rem = 4px):

| px | Tailwind | Common use |
|----|----------|------------|
| 4px | `p-1` | Tight padding |
| 8px | `p-2` | Small padding |
| 12px | `p-3` | — |
| 16px | `p-4` | Default padding |
| 20px | `p-5` | — |
| 24px | `p-6` | Section padding |
| 32px | `p-8` | Card padding |
| 40px | `p-10` | — |
| 48px | `p-12` | Section gap |
| 64px | `p-16` | Large section |
| 80px | `p-20` | Hero padding |
| 96px | `p-24` | Extra large |
| 128px | `p-32` | — |

For non-standard values, use CSS variables:
```css
@theme inline {
  --spacing-section: 5rem; /* 80px */
}
```

---

## Border Radius

| Extracted | Tailwind class |
|-----------|----------------|
| 2px | `rounded-sm` |
| 4px | `rounded` |
| 6px | `rounded-md` |
| 8px | `rounded-lg` |
| 12px | `rounded-xl` |
| 16px | `rounded-2xl` |
| 9999px | `rounded-full` |

Define a project-wide radius token:
```css
:root {
  --radius: 0.5rem; /* matches the site's rounded-lg */
}
@theme inline {
  --radius-default: var(--radius);
}
```

---

## Shadows

| Description | Tailwind class |
|-------------|----------------|
| None | `shadow-none` |
| Subtle card | `shadow-sm` |
| Card / panel | `shadow` or `shadow-md` |
| Elevated modal | `shadow-lg` |
| Overlay / dropdown | `shadow-xl` |
| Dramatic | `shadow-2xl` |

For custom shadows, extract the exact `box-shadow` value and define:
```css
@theme inline {
  --shadow-card: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
}
```

---

## Complete `src/index.css` Template

```css
@import "tailwindcss";

/* ── Semantic tokens ── */
:root {
  /* Colours (fill from extraction) */
  --background:           hsl(0 0% 100%);
  --foreground:           hsl(221 39% 11%);
  --primary:              hsl(174 90% 31%);
  --primary-foreground:   hsl(0 0% 100%);
  --secondary:            hsl(210 40% 96%);
  --secondary-foreground: hsl(222 47% 11%);
  --muted:                hsl(210 40% 96%);
  --muted-foreground:     hsl(215 16% 47%);
  --accent:               hsl(210 40% 96%);
  --accent-foreground:    hsl(222 47% 11%);
  --destructive:          hsl(0 84% 60%);
  --destructive-foreground: hsl(0 0% 100%);
  --border:               hsl(214 32% 91%);
  --ring:                 hsl(174 90% 31%);
  --card:                 hsl(0 0% 100%);
  --card-foreground:      hsl(221 39% 11%);

  /* Shape */
  --radius: 0.5rem;
}

/* ── Map to Tailwind utilities ── */
@theme inline {
  --color-background:           var(--background);
  --color-foreground:           var(--foreground);
  --color-primary:              var(--primary);
  --color-primary-foreground:   var(--primary-foreground);
  --color-secondary:            var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted:                var(--muted);
  --color-muted-foreground:     var(--muted-foreground);
  --color-accent:               var(--accent);
  --color-accent-foreground:    var(--accent-foreground);
  --color-destructive:          var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-border:               var(--border);
  --color-ring:                 var(--ring);
  --color-card:                 var(--card);
  --color-card-foreground:      var(--card-foreground);

  /* Typography */
  --font-sans: 'Inter', ui-sans-serif, system-ui, sans-serif;
}

/* ── Base styles ── */
@layer base {
  *, *::before, *::after { box-sizing: border-box; }

  body {
    background-color: var(--background);
    color: var(--foreground);
    font-family: var(--font-sans);
    line-height: 1.6;
    -webkit-font-smoothing: antialiased;
  }
}
```
