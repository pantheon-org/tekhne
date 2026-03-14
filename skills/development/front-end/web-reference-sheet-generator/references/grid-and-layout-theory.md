# Grid and Layout Theory

Grids are the invisible structure that organises content into coherent, predictable, and
professional layouts. They create alignment, rhythm, and consistency — the qualities that
separate designed pages from assembled pages.

---

## What a Grid Does

A grid provides:
1. **Alignment** — elements connect to shared invisible lines, creating visual order
2. **Consistency** — the same grid applied across pages creates coherence
3. **Efficiency** — designers and developers share a common spatial language
4. **Flexibility** — grid systems allow variety within structure

A well-applied grid is invisible to the viewer — they simply feel that the layout is "right".

---

## Types of Grid

### 1. Column Grid

The most common grid type for web layout. The page width is divided into a fixed number of equal
columns with consistent gutter (space between columns) and margin (space on the outer edges).

**Standard web column grids:**

| Context | Columns | Typical gutter | Notes |
|---|---|---|---|
| Mobile (320–767px) | 4 | 16px | Most content spans full width |
| Tablet (768–1023px) | 8 | 24px | 2–4 column spans common |
| Desktop (1024–1279px) | 12 | 24–32px | Standard 12-column system |
| Wide desktop (1280px+) | 12–16 | 32px | Max content width typically 1200–1440px |

**12-column grid** is the de facto standard because 12 divides evenly into 1, 2, 3, 4, 6, and 12
columns — providing maximum layout flexibility.

**How to use column spans:**
- Full-width section: 12 columns
- Two equal halves: 6+6
- Sidebar + content: 3+9 or 4+8
- Three equal thirds: 4+4+4
- Four cards: 3+3+3+3
- Featured + two supporting: 6+3+3

### 2. Baseline Grid

A horizontal grid with uniform row height (the baseline interval) applied to the entire page.
All text elements align to the baseline grid, creating vertical rhythm.

**The baseline unit** is typically equal to the body text's line height (e.g. 24px for 16px body
at 1.5 line height).

**In web practice:** Pure baseline grids are difficult to maintain with CSS. Instead, establish
a **spacing scale** based on a base unit (8px is standard) that approximates baseline rhythm:

```
4px  → micro (icon gaps, border offsets)
8px  → xs
16px → sm
24px → md
32px → lg
48px → xl
64px → 2xl
96px → 3xl
```

All padding, margin, and gap values should come from this scale.

### 3. Modular Grid

Combines column and row grids to create a matrix of cells. Common in editorial design
(magazine layouts), image galleries, and complex dashboard layouts.

**Web application:** CSS Grid with defined row heights creates modular grids.

### 4. Hierarchical Grid

A flexible, rule-based approach where elements are placed intuitively but still align to implied
axes and proportional relationships. Not a strict grid — more a set of alignment rules.

Common in highly creative, editorial, or asymmetric layouts. Requires more design judgement.

---

## The 8-Point Grid System

The most widely adopted spacing system for digital product design.

**Principle:** All spacing values (padding, margin, gap, width, height) are multiples of 8px.

**Why 8?**
- Most common screen densities (1×, 1.5×, 2×, 3×) scale 8px evenly — no blurry sub-pixel rendering
- 8px divides into 4px and 2px for fine adjustments while maintaining the system

**Practical scale:**
```
4px   → hairline spacing (badge padding, tight icon gaps)
8px   → tight spacing (within-component padding)
16px  → default spacing (standard component padding, gap between related elements)
24px  → comfortable spacing
32px  → section breathing room
48px  → section separation
64px  → major section gaps
96px  → hero/section vertical padding
```

**Applying in CSS:**
```css
:root {
  --space-1: 4px;
  --space-2: 8px;
  --space-3: 16px;
  --space-4: 24px;
  --space-5: 32px;
  --space-6: 48px;
  --space-7: 64px;
  --space-8: 96px;
}
```

---

## Responsive Layout Strategies

### Fluid vs Fixed

**Fixed-width layout:** A `max-width` container centred on the page. Common values: 1200px, 1280px,
1440px. Clean, predictable, easy to reason about.

**Fluid layout:** Container stretches with the viewport up to a maximum. Used when content should
expand with available space (data tables, dashboards).

**Full-bleed sections:** Background images, colours, and video that extend edge-to-edge, while
inner content remains within the max-width container.

```css
.section-wrapper {
  width: 100%;
  background-color: var(--color-surface-alt);
}
.section-inner {
  max-width: 1200px;
  margin-inline: auto;
  padding-inline: 24px;
}
```

### Breakpoints

Define breakpoints based on content needs, not device names.

**Common breakpoints:**
```css
/* Mobile-first default styles: 0px and up */
@media (min-width: 640px)  { /* sm  — large phone / small tablet */ }
@media (min-width: 768px)  { /* md  — tablet portrait */ }
@media (min-width: 1024px) { /* lg  — tablet landscape / small desktop */ }
@media (min-width: 1280px) { /* xl  — desktop */ }
@media (min-width: 1536px) { /* 2xl — wide desktop */ }
```

**Mobile-first approach:** Write default styles for mobile, then use `min-width` media queries to
add complexity for larger screens. This is preferable to desktop-first because:
- Forces prioritisation of essential content
- Reduces the amount of CSS to override
- Better for performance on constrained devices

### CSS Grid for Page Layout

```css
.page-grid {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: 24px;
  max-width: 1200px;
  margin-inline: auto;
}

.hero-content    { grid-column: 1 / 8; }
.hero-image      { grid-column: 8 / 13; }

@media (max-width: 768px) {
  .hero-content,
  .hero-image    { grid-column: 1 / -1; }
}
```

### CSS Flexbox for Component Layout

Flexbox is suited to one-dimensional layouts (row or column):
```css
.card-row {
  display: flex;
  gap: 24px;
  flex-wrap: wrap;
}
.card {
  flex: 1 1 280px; /* grow, shrink, min-width */
}
```

---

## Proportional Systems

### Golden Ratio (φ ≈ 1.618)
A proportion found throughout nature and classical architecture. Rectangles, containers, and
typographic hierarchies built on this ratio feel naturally harmonious.

```
If short side = 1, long side = 1.618
If container width = 960px, a golden sidebar = 960 / 1.618 ≈ 593px content + 367px sidebar
```

### Rule of Thirds
Divide the composition into a 3×3 grid. Place focal points and key elements at the intersection
points ("power points"). Avoids static centred compositions.

**Web application:** Off-centre hero layouts, image composition guidelines for photography.

### Silver Ratio (1:√2 ≈ 1:1.414)
Used in ISO paper sizes (A4, A3, etc.). Useful for maintaining proportional relationships when
scaling components.

---

## Layout Patterns for the Web

### Hero section
```
[Full-width background]
  [Max-width container]
    [Left: headline + sub-text + CTA] [Right: image or illustration]
```
Common variants: centred text with background image, full-bleed video, split 50/50.

### Feature/Benefits grid
```
[Section heading — centred or left-aligned]
[3-column or 4-column card grid]
```

### Alternating content rows
```
[Image left | Text right]
[Text left  | Image right]
[Image left | Text right]
```
Creates visual rhythm and variety while maintaining a consistent structure.

### Sidebar layout
```
[Main content — 8 columns] [Sidebar — 4 columns]
```
Common for blog posts, documentation, product pages.

### Masonry / Pinterest grid
Variable height items in a multi-column layout. CSS `columns` or JS masonry library.
Use when items have unpredictable heights (images, user-generated content).

---

## Common Layout Mistakes

| Mistake | Problem | Fix |
|---|---|---|
| No max-width on content | Lines become too long on wide screens | `max-width: 1200px; margin: 0 auto` |
| Inconsistent gutters | Misaligned elements, uneven rhythm | Use a single gutter token throughout |
| No horizontal padding on mobile | Content touches screen edges | `padding-inline: 16px` as a minimum |
| Centred text on long paragraphs | Hard to read; eye struggles to find next line | Centre only for short headings/labels |
| Mixing grid systems | Columns and spacing clash | One grid, applied everywhere |
| Fixed heights on content containers | Content overflows or creates dead space | Avoid fixed heights; let content dictate |
| Assuming desktop layout works on mobile | Horizontal overflow, tiny text, broken layouts | Mobile-first; test at 375px width |
