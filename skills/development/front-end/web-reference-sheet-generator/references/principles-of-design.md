# Principles of Design

The principles of design are the rules and guidelines that govern how the elements of design are
arranged and combined to create effective visual communication. They are the "how" of design —
while elements are the raw materials, principles are how you use them.

---

## 1. Balance

Visual equilibrium — the distribution of visual weight across a composition.

**Symmetrical balance** — identical or near-identical elements mirrored across an axis. Conveys
stability, formality, trustworthiness. Common in corporate and institutional design.

**Asymmetrical balance** — different elements balanced by compensating weight (size, colour,
texture). More dynamic and modern; requires careful judgement. Common in editorial and startup
design.

**Radial balance** — elements radiate from a central point. Creates a sense of movement and energy.
Used in logos, icons, and decorative elements.

**Design application:** In web UI, asymmetric layouts (e.g. hero image left + text right) feel
contemporary while remaining stable if visual weights are managed.

---

## 2. Contrast

The juxtaposition of opposing elements to create visual interest, hierarchy, and readability.

Contrast can be achieved through:
- **Colour** — dark vs light, complementary colours
- **Size** — large heading vs small body text
- **Weight** — bold vs regular
- **Shape** — geometric vs organic
- **Texture** — rough vs smooth
- **Space** — dense vs open areas

**Design application:** Contrast is the primary driver of visual hierarchy. Without contrast,
everything competes equally and nothing is emphasised.

**Accessibility note:** Colour contrast must meet WCAG AA (4.5:1 for body text, 3:1 for large text
and UI components).

---

## 3. Emphasis (Dominance / Focal Point)

The deliberate prioritisation of one element to draw the eye first.

Techniques:
- Size (larger = more important)
- Colour (saturated/bright against muted surroundings)
- Isolation (surrounded by whitespace)
- Position (top-left, centre, or above the fold)
- Contrast
- Unique shape or texture

**Design application:** Every page needs a clear primary focal point — the hero headline, the CTA
button, the product image. If everything is emphasised, nothing is.

---

## 4. Movement (Flow)

The path the viewer's eye travels through a composition.

**Z-pattern** — natural reading path for sparse/marketing layouts: top-left → top-right → diagonal →
bottom-left → bottom-right.

**F-pattern** — natural reading path for text-heavy content: horizontal scan across top → down the
left edge → shorter horizontal scans.

**Circular movement** — used in logo design and illustration to keep the eye engaged within the
composition.

Directional cues that guide movement:
- Lines and arrows
- Character eye gaze direction
- Repetition and rhythm
- White space channels

**Design application:** Hero sections typically use Z-pattern. Blog/article content uses F-pattern.
Design layouts to match the expected reading pattern of the content type.

---

## 5. Pattern & Repetition

The repeated use of the same or similar elements throughout a design.

**Repetition** creates consistency and unity — repeated colours, fonts, shapes, and spacing build
a recognisable visual language.

**Pattern** is organised repetition — a motif repeated at regular intervals to create texture or
background interest.

**Design application:** A consistent component library (buttons, cards, badges using the same
radius, padding, shadow) is repetition in practice. Repeated typographic scales and spacing tokens
enforce visual rhythm.

---

## 6. Rhythm

The sense of movement created by repeating elements at regular or varied intervals — analogous to
rhythm in music.

**Regular rhythm** — uniform intervals, predictable. Creates order and calm.

**Flowing rhythm** — organic, natural intervals. Creates a relaxed, elegant feel.

**Progressive rhythm** — elements gradually increase or decrease (e.g. a typographic scale or
gradient progression). Creates a sense of direction and energy.

**Design application:** Consistent vertical rhythm (spacing between sections, between text blocks)
creates a composed, professional feel. Irregular spacing feels accidental and unpolished.

---

## 7. Unity (Harmony)

The sense that all elements belong together — that the design is a coherent whole rather than a
collection of parts.

Achieved through:
- **Proximity** — grouping related elements (see Gestalt)
- **Repetition** — consistent use of colour, type, and spacing
- **Alignment** — all elements connect to an invisible grid
- **Continuation** — visual lines carry the eye across elements

**Design application:** A design system enforces unity by codifying tokens and patterns. Unity
breaks down when one-off colours, rogue font sizes, or inconsistent spacing creep in.

---

## 8. Variety

Deliberate introduction of difference to prevent monotony and maintain interest.

Variety is the counterbalance to unity — too much sameness is boring, too much variety is chaotic.

**Design application:** Vary section layouts alternately (image-left/image-right), use accent
colours sparingly for emphasis, mix typographic weights to create texture in body content.

---

## 9. Proportion & Scale

The size relationship between elements, and the size of elements relative to the overall
composition.

**Golden ratio (φ ≈ 1.618)** — a naturally pleasing proportion used in layout, typography, and
spacing scales.

**Rule of thirds** — divide a composition into a 3×3 grid; place focal points at intersections.

**Typographic scale** — a modular scale (e.g. 1.25× or Major Third) applied to font sizes creates
harmonious hierarchy.

**Design application:** Scale is the fastest way to communicate importance. A 48px heading above
16px body text immediately communicates hierarchy via proportion.

---

## 10. White Space (Negative Space)

The empty space around and between elements — not wasted space, but an active design element.

**Macro white space** — large gaps between major sections and layout regions. Creates breathing room and structure.

**Micro white space** — small gaps between lines, letters, list items, and UI components. Directly affects readability and perceived quality.

**Design application:** Generous white space signals premium positioning. Cramped layouts feel cheap or overwhelming. Padding, line-height, and margin tokens are the primary tools for controlling white space systematically.

---

## 11. Hierarchy

The visual organisation of elements by order of importance, guiding the viewer through a deliberate reading sequence.

Distinct from *Emphasis* (which concerns a single focal point), hierarchy is about the full stack of importance — primary, secondary, tertiary, and supporting content.

Created through:
- **Size** — larger elements read first
- **Weight** — bold before regular
- **Colour** — high contrast before low contrast
- **Position** — top and left before bottom and right
- **Spacing** — isolated elements feel more important

**Design application:** A well-structured typographic scale (H1 → H2 → H3 → body → caption) communicates hierarchy without decoration. If a user cannot identify the most important content within 3 seconds, hierarchy has failed.

---

## 12. Alignment

The positioning of elements along a common axis or grid line.

**Types:** Left, right, centre, justified.

**Edge alignment** — elements aligned to a shared invisible edge create visual connection and order.

**Centre alignment** — creates formality and symmetry; appropriate for short headings, modal
dialogs, and hero text. Avoid for long body copy.

**Design application:** Even a few pixels of misalignment are noticed subconsciously and reduce
perceived quality. Use an 8-point grid or consistent spacing tokens to enforce alignment.

---

## Quick Reference

| Principle | Core Question | Common Failure |
|---|---|---|
| Balance | Does it feel stable? | One side too heavy |
| Contrast | Can I tell what's important? | Everything looks the same |
| Emphasis | What do I look at first? | No clear focal point |
| Movement | Where does my eye go? | Eye wanders aimlessly |
| Pattern/Repetition | Does it feel consistent? | Inconsistent components |
| Rhythm | Does the spacing feel deliberate? | Random gaps and jumps |
| Unity | Does it belong together? | Mismatched styles |
| Variety | Is it interesting? | Monotonous sameness |
| Proportion | Do sizes make sense? | Wrong scale relationships |
| White Space | Does it breathe? | Cramped, cluttered layout |
| Hierarchy | Do I know what to read first? | All content feels equal |
| Alignment | Is it tidy? | Misaligned elements |
