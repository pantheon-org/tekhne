# Visual Hierarchy

Visual hierarchy is the arrangement of elements to show their order of importance — guiding the
viewer's eye through a composition in a deliberate sequence. It answers the question: "What should
I look at first, second, third?"

Without hierarchy, all elements compete equally and the viewer is left to construct their own
reading order — which means most will not engage deeply.

---

## Why Hierarchy Matters

The human visual system performs pre-attentive processing before conscious attention kicks in.
Within 50–150ms of seeing a page, the brain has already identified the dominant focal point,
the rough structure of sections, and the approximate colour and size relationships. Hierarchy
operates at this pre-attentive level.

**The result:** Users decide within seconds whether a page is worth reading. Hierarchy is what
makes the content scannable enough to earn that decision.

---

## The Seven Hierarchy Variables

### 1. Size

The most powerful hierarchy signal. Larger elements receive attention before smaller ones —
regardless of position.

- A 60px headline at the bottom of the page commands more attention than a 14px caption at the top
- Use size contrast deliberately: the jump between heading levels should be perceptible at a glance
- Avoid "creeping sameness" — heading levels that are barely distinguishable

**Rule of thumb:** Each heading level should be at least 25% larger than the one below it.

---

### 2. Weight (Font Weight)

Bold text reads before regular text. Weight is the secondary hierarchy signal after size.

- Use weight to differentiate labels from values, headings from body text
- Bold sparingly within body text — if everything is bold, nothing is
- Variable fonts allow fine-grained weight control for subtle hierarchy within a single typeface

---

### 3. Colour & Value Contrast

High-contrast elements advance; low-contrast elements recede.

- A saturated accent colour on a muted background instantly signals importance
- Muted/grey text signals secondary or supporting information
- Colour alone must never be the **only** hierarchy signal (accessibility requirement)
- Dark-on-light and light-on-dark both work — contrast ratio matters, not which is "on top"

**Practical use:**
- Primary heading: full-contrast (e.g. `#111`)
- Body text: high contrast but softer (e.g. `#333`)
- Secondary/supporting text: reduced contrast (e.g. `#666` or `color-mix`)
- Disabled/placeholder: low contrast (e.g. `#999` — check WCAG for input placeholders)

---

### 4. Position

Where an element sits determines when it is seen.

**Western reading convention (left-to-right, top-to-bottom):**
- Top-left → first noticed
- Centre → formal, balanced, high attention
- Bottom-right → last noticed

**Above the fold:** Content visible without scrolling receives more attention. The most important
hierarchy decision on any page is what lives above the fold.

**Z-pattern vs F-pattern:**
- **Z-pattern:** For sparse marketing/landing pages. Eye scans top-left → top-right → diagonal →
  bottom-left → bottom-right.
- **F-pattern:** For text-heavy content. Eye scans across the top, down the left edge, with shorter
  horizontal scans lower down. Left-rail items receive significantly more attention.

---

### 5. Whitespace (Isolation)

An element surrounded by empty space receives more visual weight than one surrounded by other
elements. Isolation is a powerful hierarchy signal often overlooked.

- A single bold sentence with large margin above and below it commands more attention than the same
  sentence buried in a dense paragraph
- Pull quotes, stats, and key claims benefit from generous white space
- Dense information hierarchies (data tables, navigation menus) reduce whitespace to signal equal
  weight across items

See `whitespace-theory.md` for detailed whitespace guidance.

---

### 6. Contrast of Shape & Style

An element that differs from its surroundings in shape, form, or style will be noticed.

- A circular avatar in a layout of rectangles draws the eye
- A filled button among ghost buttons signals the primary action
- An italic pull quote in a Roman body text section creates hierarchy through style contrast
- Icons next to text items make those items scan faster and stand out from plain text

---

### 7. Sequence & Proximity

Proximity groups elements into visual units. Items close together are perceived as related; items
spaced apart are perceived as separate.

**Reading sequence** is established by combining proximity with size, weight, and position:
- A heading followed immediately by body text (small gap between) signals a section unit
- Generous space before a heading signals the start of a new section
- A CTA button immediately below a value proposition captures attention at the end of the reading
  sequence

---

## Building a Page Hierarchy

### Step 1 — Define the goal

Every page has one primary goal. What action or understanding should the visitor leave with?
This goal determines the primary focal point.

### Step 2 — Establish 3–4 levels

```
Level 1 (dominant)  → Primary headline, hero image, or main CTA
Level 2 (secondary) → Supporting headings, key claims, section titles
Level 3 (tertiary)  → Body text, supporting copy, card content
Level 4 (supporting)→ Labels, captions, metadata, footnotes
```

### Step 3 — Apply hierarchy variables

For each level, assign appropriate values of size, weight, colour, and spacing. The jumps between
levels must be perceptible — not gradual.

### Step 4 — Test the squint test

Squint at the design until it blurs. You should still be able to identify:
- The primary focal point (Level 1)
- The rough structure of sections (Level 2)

If multiple elements compete at the blurred view, the hierarchy is insufficient.

### Step 5 — Test the 5-second test

Show the design to someone for 5 seconds, then hide it. Ask:
- What was the page about?
- What were you supposed to do?

If they cannot answer both questions, the hierarchy is failing.

---

## Hierarchy in Web Component Patterns

### Cards
```
Card title         → Largest, full contrast, H3 or similar weight
Supporting text    → Body size, reduced contrast
CTA / link         → Accent colour, medium weight
Metadata/date      → Small, lowest contrast
```

### Navigation
Primary navigation items should be equal weight — no single item should dominate. The active/current
item may be differentiated by colour or weight (not size).

### Forms
```
Section heading    → Identifies form section
Label              → Names the field — should be clearly visible, not placeholder text
Input value        → Full contrast (user-entered content is highest priority)
Helper text        → Reduced contrast, small — supporting, not dominant
Error message      → Error colour (red), icon + text — must be noticeable
```

### Hero sections
```
Eyebrow / label    → Small, uppercase, accent colour — sets context
H1 headline        → Dominant — the largest text on the page
Sub-headline/body  → Supports headline, softer weight or reduced contrast
Primary CTA        → High contrast filled button
Secondary CTA      → Ghost/outline or text link — must feel secondary
```

---

## Hierarchy Anti-patterns

| Anti-pattern | Problem | Fix |
|---|---|---|
| Everything is bold | No differentiation; nothing is emphasised | Reserve bold for 1–2 hierarchy levels |
| Equal font sizes throughout | No visual structure | Apply a modular typographic scale |
| All caps body text | Removes weight differentiation; hard to read | Reserve caps for labels/eyebrows only |
| CTA buried at bottom without visual weight | Users don't see the action | Increase size, colour contrast, and isolation |
| Headers same size as subheaders | Hierarchy levels blur together | Increase size contrast between levels |
| Too many focal points | Eye doesn't know where to start | Choose one dominant element per section |
| Colour as the only differentiator | Inaccessible to colour-blind users | Combine colour with weight/size/position |
