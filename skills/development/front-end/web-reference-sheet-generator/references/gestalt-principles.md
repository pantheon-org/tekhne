# Gestalt Principles

Gestalt psychology (German: "shape" or "form") describes how the human brain organises visual
information into unified wholes rather than perceiving individual parts in isolation. The core
insight: **the whole is different from the sum of its parts.**

These principles are directly applicable to UI/UX layout because the browser renders a flat
collection of HTML elements — Gestalt explains how users perceive those elements as meaningful
groups, patterns, and structures.

---

## 1. Proximity

**Elements close together are perceived as belonging to the same group.**

The brain interprets spatial closeness as a relationship signal — before colour, shape, or any
other attribute is processed.

```
● ● ●   ■ ■ ■
Three circles and three squares — perceived as two groups based on proximity,
despite the shape difference.
```

**UI application:**
- A label and its form input must be closer to each other than to adjacent label-input pairs
- Navigation items grouped by function (primary nav, utility nav) should have more space between
  groups than within them
- Cards on a grid are perceived as related because the gap between cards is smaller than the gap
  between section blocks
- The gap above a heading should be larger than the gap below it — tying the heading to its body

**Practical rule:** If two elements are related, move them closer. If they are separate, move them
further apart. Never rely on labels or lines when spacing can do the work.

---

## 2. Similarity

**Elements that share visual attributes — colour, shape, size, orientation, texture — are perceived
as belonging together.**

Similarity creates implicit categorisation without borders, labels, or explicit grouping.

**UI application:**
- All primary buttons should share the same colour, shape, and weight so users learn the pattern
  instantly
- Navigation items at the same hierarchy level should be visually identical
- Interactive elements (links, buttons, clickable cards) should share a colour token so users know
  they are actionable
- Column headers in a data table look different from data rows — similarity groups each type

**Warning:** Similarity works against you when unrelated elements accidentally share attributes.
If your CTA button and your error message are both red, the error message inherits the "action"
connotation of the button colour.

---

## 3. Figure-Ground

**The brain separates visual input into a dominant "figure" (foreground) and a "ground" (background).**

The figure is perceived as solid, in front, with definite shape. The ground is perceived as
behind, shapeless, continuous.

Classic example: The Rubin vase — two face profiles or a vase, depending on which you perceive
as figure vs ground.

**UI application:**
- Modal dialogs rely on figure-ground: the modal is the figure; the dimmed overlay is the ground
- Dropdown menus elevate above the page via shadow — creating figure-ground separation
- Cards sit on a background — the card is figure, the page is ground. This is why card shadow and
  background colour must provide sufficient contrast
- High enough contrast between text and background establishes text as figure and background as ground
- A lack of contrast collapses the figure-ground relationship and makes text unreadable

**Ambiguous figure-ground:** Sometimes intentional (logo design, optical illusion art), but in UI
it creates confusion. Ensure your figure (interactive or important element) is always clearly
distinct from its ground.

---

## 4. Continuity (Common Fate)

**Elements arranged along a line, curve, or path are perceived as related and connected.**

The brain follows the path and groups the elements along it.

**UI application:**
- A horizontal navigation bar — items arranged along a line are perceived as a related set
- A timeline (vertical or horizontal) — items along the line are perceived as sequential
- A stepper component — numbered steps on a shared track form a connected sequence
- Scrolling content — items that appear along the user's scroll path are linked by the path
- Progress indicators — the filled portion of a progress bar reads as "completed"; the unfilled as
  "remaining" — connected along the same track

---

## 5. Closure

**The brain completes incomplete shapes and fills in missing information.**

Humans prefer complete, familiar shapes and will mentally "close" gaps to perceive them.

**UI application:**
- Icons are often constructed with negative space — the brain completes the shape (hamburger menu,
  check mark, loading spinners)
- A grid of cards cut off at the edge of the viewport signals "there are more items here" — the
  user's brain completes the row and understands to scroll
- Partially visible images at the edge of a carousel indicate more content is available
- Minimalist logos exploit closure — a simple incomplete shape reads as the full letter or symbol

---

## 6. Prägnanz (Law of Good Form / Simplicity)

**The brain perceives ambiguous or complex images as the simplest possible form.**

Given multiple possible interpretations, the simplest is preferred.

**UI application:**
- Users will interpret a complex set of elements as simple geometric structures (rows, columns,
  grids) before they process individual element details
- Overly complex layouts trigger cognitive load — users simplify to "this is confusing" before
  they try to parse the detail
- Clean, grid-aligned layouts conform to Prägnanz — they feel immediately comprehensible
- Every element added to a layout adds complexity; only add elements that earn their presence

---

## 7. Common Fate

**Elements that move together are perceived as belonging together.**

When multiple elements move in the same direction simultaneously, the brain groups them.

**UI application:**
- An accordion section — the heading and body content expand/collapse together, reinforcing their
  relationship
- A toast notification that slides in from the top-right is perceived as a single unit because all
  its parts move together
- Parallax effects — elements moving at different rates are perceived as existing on different
  depth layers (figure-ground)
- Loading skeleton screens — elements that pulse together communicate "these are loading together"

---

## 8. Uniform Connectedness

**Elements with an explicit visual connection (line, border, shared background) are perceived as
more strongly related than elements grouped by proximity or similarity alone.**

**UI application:**
- Cards use a border or background colour to visually bind their contents into a single unit
- A form section uses a card or background shade to group its fields
- A tag/badge binds a label and colour into an inseparable unit
- Breadcrumbs use dividers to connect sequential path items
- Stepper components use lines to connect numbered steps

---

## Applying Gestalt to Layouts

When building a page layout, work through these questions:

**Proximity check:** Is every related pair of elements closer to each other than to unrelated elements?

**Similarity check:** Do all interactive elements share a visual attribute that marks them as
interactive? Do all navigation items of the same level look the same?

**Figure-ground check:** Is the main content clearly distinct from the background? Do modals,
dropdowns, and elevated panels have enough shadow/contrast to separate from the page?

**Closure check:** Are any elements cut off that should signal "more content available"? Is that
cutoff intentional?

**Continuity check:** Are sequential or related items arranged along a common path or line?

**Prägnanz check:** Does the overall layout resolve to a simple structure (grid, columns, rows) that
the brain can interpret quickly? Is there unnecessary complexity?

---

## Quick Reference

| Principle | Core idea | Primary UI use case |
|---|---|---|
| Proximity | Close = related | Label/input pairing, card layout, section spacing |
| Similarity | Shared attributes = same group | Button styles, nav items, interactive affordance |
| Figure-Ground | Foreground vs background | Modals, dropdowns, card elevation, text contrast |
| Continuity | Along a path = connected | Navigation bars, timelines, steppers, progress bars |
| Closure | Brain completes incomplete shapes | Icons, carousel truncation, lazy borders |
| Prägnanz | Simplest interpretation wins | Overall layout structure, reducing cognitive load |
| Common Fate | Moving together = related | Accordion, notifications, parallax layers |
| Connectedness | Explicit connection = stronger grouping | Cards, form sections, badges, breadcrumbs |
