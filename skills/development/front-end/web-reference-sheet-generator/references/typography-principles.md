# Typography Principles

Typography is simultaneously functional and expressive — it must be readable, establish hierarchy,
and convey personality. Good typography is largely invisible; the reader absorbs content without
noticing the type choices. Bad typography creates friction.

---

## The Six Core Principles

### 1. Hierarchy

Visual hierarchy communicates importance and guides reading order without requiring the reader to
consciously decide what to read first.

**Hierarchy tools:**
- **Size** — largest = most important. The simplest and most powerful hierarchy signal.
- **Weight** — bold draws attention; light recedes.
- **Colour** — high-contrast pops; muted recedes.
- **Case** — UPPERCASE commands attention (use sparingly); sentence case is readable.
- **Style** — italic for emphasis, contrast, or quotation.
- **Spacing** — more space above a heading than below ties it to the content it introduces.

**A clear hierarchy typically has 3–4 levels:**
1. Display / Hero headline (largest, boldest)
2. Section headings (H2)
3. Sub-headings (H3)
4. Body text
5. Supporting / caption text (smallest)

More than 4 distinct levels in a single layout creates confusion.

---

### 2. Readability & Legibility

**Legibility** — whether individual characters can be distinguished from one another.
**Readability** — whether text can be comfortably read at length.

**Line length (measure):** 45–75 characters per line is optimal for body text.
- Too short: eye jumps too frequently, creates choppy rhythm
- Too long: eye struggles to find the next line; fatigue sets in
- A practical rule: a paragraph container should be approximately `ch` units in CSS matching
  this range, e.g. `max-width: 65ch`

**Line height (leading):**
- Body text: 1.4–1.6× the font size
- Headings: 1.1–1.3× (tighter; they are read in short bursts)
- Very large display text: 1.0–1.1× or even below 1 for tight editorial effect

**Letter spacing (tracking):**
- Body text: 0 or very slight positive (0–0.02em)
- Uppercase labels/captions: +0.05–0.15em (uppercase needs more air to read well)
- Display/hero text: slight negative (-0.01 to -0.03em) tightens large type elegantly
- Never track body text negatively

**Font size:**
- Minimum body text: 16px (browser default is 16px for good reason)
- Captions/labels: 12–14px — use sparingly; never below 11px
- Mobile: same minimums; do not reduce body text on mobile

---

### 3. Type Pairing

Most designs use 2 typefaces maximum. 3 is occasionally justified; more creates noise.

**Pairing strategies:**

**Serif + Sans-serif** — the classic pairing. Serif for headings (personality), sans-serif for
body (readability), or vice versa.
- Playfair Display + Inter
- Lora + Source Sans Pro
- Georgia + Helvetica

**Sans-serif + Sans-serif** — works when typefaces have contrasting personalities or weights.
- A humanist sans (Gill Sans, Optima) + geometric sans (Futura, Avenir)
- A display sans (Clash Display) + neutral sans (Inter, DM Sans)

**Monospace + Anything** — monospace reserved for code blocks only in most non-technical contexts.

**Rules for pairing:**
- High contrast between the two faces (serif vs sans, heavy vs light, condensed vs regular)
- Both faces should share a similar x-height for visual harmony at similar sizes
- Never pair two faces with the same personality — the pair must offer contrast

---

### 4. Typographic Scale

A modular scale creates harmonious size relationships throughout a design.

**Common scale ratios:**

| Ratio | Name | Character |
|---|---|---|
| 1.067 | Minor Second | Very tight, minimal variation |
| 1.125 | Major Second | Subtle, conservative |
| 1.200 | Minor Third | Balanced, works well for UI |
| 1.250 | Major Third | Clear hierarchy, popular for web |
| 1.333 | Perfect Fourth | Strong hierarchy, editorial feel |
| 1.414 | Augmented Fourth | Bold jumps, display contexts |
| 1.500 | Perfect Fifth | Very dramatic — display/hero only |
| 1.618 | Golden Ratio | Classical, harmonious but large jumps |

**Example scale at 1.25 (Major Third), base 16px:**
```
xs:   10px  (0.64rem)
sm:   12px  (0.8rem)
base: 16px  (1rem)
md:   20px  (1.25rem)
lg:   25px  (1.563rem)
xl:   31px  (1.953rem)
2xl:  39px  (2.441rem)
3xl:  48px  (3.052rem)
4xl:  61px  (3.815rem)
```

**In practice:** Use a tool like typescale.com or set tokens in your design system. Apply the scale
consistently — do not create arbitrary intermediate sizes.

---

### 5. Typographic Colour & Texture

At a distance, a block of body text creates a grey mass with its own texture and "colour" — even
when printed in black.

**Factors that affect typographic colour:**
- Font weight (heavier = darker texture)
- Letter spacing (wider = lighter, airier texture)
- Line height (more leading = lighter, more open texture)
- Punctuation density
- The typeface itself (some faces have more variation between thick and thin strokes)

**Design application:** Vary typographic colour to create visual interest in text-heavy layouts.
A pull quote in a heavier weight creates a dark spot that breaks up the grey mass of body text.

---

### 6. Emotional Tone

Every typeface has personality. The type choice signals brand values before a word is read.

| Typeface category | Signals | Cautions |
|---|---|---|
| Old-style serif (Garamond, Caslon) | Heritage, editorial, craftsmanship | Can feel dated without modern pairing |
| Transitional serif (Times, Baskerville) | Authority, trustworthiness, academic | Can feel formal or stiff |
| Modern/Didone serif (Didot, Bodoni) | Luxury, fashion, high-end editorial | Poor at small sizes; high contrast strokes |
| Slab serif (Rockwell, Clarendon) | Bold, friendly, artisanal, confident | Strong personality; dominates |
| Humanist sans (Gill Sans, Frutiger) | Friendly, warm, approachable, readable | Less distinctive |
| Geometric sans (Futura, Avenir, Circular) | Modern, clean, minimal, progressive | Can feel cold |
| Grotesque sans (Helvetica, Akzidenz) | Neutral, Swiss, industrial, objective | Very neutral — needs strong colour/layout |
| Variable/contemporary sans (Inter, DM Sans) | Digital-native, accessible, modern | Can lack personality |

---

## Type Hierarchy in Web UI

### Semantic HTML mapping
```
h1  → Display / page hero headline
h2  → Section headings
h3  → Sub-section headings
h4  → Card headings, list group headings
p   → Body text
small / .caption → Supporting text, metadata
label → Form labels
```

### Practical CSS starting point
```css
:root {
  --font-size-xs:    0.75rem;   /* 12px */
  --font-size-sm:    0.875rem;  /* 14px */
  --font-size-base:  1rem;      /* 16px */
  --font-size-lg:    1.125rem;  /* 18px */
  --font-size-xl:    1.25rem;   /* 20px */
  --font-size-2xl:   1.5rem;    /* 24px */
  --font-size-3xl:   1.875rem;  /* 30px */
  --font-size-4xl:   2.25rem;   /* 36px */
  --font-size-5xl:   3rem;      /* 48px */
  --font-size-6xl:   3.75rem;   /* 60px */

  --line-height-tight:  1.25;
  --line-height-normal: 1.5;
  --line-height-relaxed: 1.75;
}
```

---

## Common Typographic Mistakes

| Mistake | Why it fails | Fix |
|---|---|---|
| Too many typefaces | Creates visual noise and fragmented personality | Max 2 typefaces |
| Tight line height on body text | Stacks lines into an unreadable wall | 1.4–1.6× for body |
| Very long lines | Eye loses its place; reading fatigue | `max-width: 65ch` on body containers |
| All caps body text | Dramatically reduces reading speed | Uppercase for labels/short text only |
| Default browser font size overridden below 16px | Accessibility failure; user intention overridden | Never set `font-size` < 16px on body |
| Untracked uppercase | Uppercase characters need more space | `letter-spacing: 0.08em` on uppercase |
| Inconsistent hierarchy | User cannot determine content structure | Use a defined scale; apply consistently |
| Font loading without fallbacks | FOUT/FOIT creates jarring layout shift | Always define a system font fallback stack |
