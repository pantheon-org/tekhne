# Elements of Design

The elements of design are the fundamental visual building blocks — the raw vocabulary of any
visual composition. Every design, from a logo to a webpage, is constructed from some combination
of these elements.

---

## 1. Line

A mark connecting two points; the most fundamental element.

**Types:**
- **Straight** — order, precision, strength
- **Curved** — flow, softness, elegance
- **Diagonal** — dynamism, tension, movement
- **Zigzag** — energy, nervousness, excitement

**Line weight:** Thick lines feel heavy and bold; thin lines feel delicate and refined.

**Implied lines** — not drawn, but suggested by alignment of elements, eye gaze direction, or the
edge of a shape. Strongly guide eye movement.

**Design application:** Horizontal rules, border separators, underlines, and grid lines are all
lines. Diagonal photography compositions create energy. Text alignment creates implied lines that
organise content.

---

## 2. Shape

A two-dimensional area defined by a boundary (a closed line, contrast, or colour).

**Geometric shapes** — circles, squares, triangles, polygons. Convey structure, order, logic.
- Square/Rectangle: stability, trust, reliability
- Circle: wholeness, community, continuity
- Triangle: direction, energy, dynamism (point up = growth; point down = instability)

**Organic shapes** — irregular, free-form, natural. Convey warmth, creativity, approachability.

**Abstract shapes** — simplified or stylised representations (icons, logos, pictograms).

**Positive vs negative shape:** The filled shape (positive) and the space around it (negative/white
space) are equally important. Logos exploit figure-ground relationships.

**Design application:** Button shapes, card containers, avatar crops, hero section blobs, icon
design — all are shape decisions. Rounded corners soften; sharp corners are more assertive.

---

## 3. Form

Shape extended into three dimensions — or the illusion of three dimensions on a 2D surface.

**Actual form** — physical 3D objects (packaging, product design, signage).

**Implied form** — the illusion of depth and volume created through:
- Shading and highlights (light source)
- Perspective and foreshortening
- Overlap and occlusion
- Gradient and shadow

**Design application:** Drop shadows, box shadows, gradients, and depth layers (elevation system)
all create implied form in flat UI design. Skeuomorphic design relies heavily on form to imply
physicality.

---

## 4. Space (Positive & Negative)

The area within and around elements.

**Positive space** — the area occupied by subjects and content.

**Negative space (white space)** — the empty area around and between elements. Not absence — it is
an active element that defines and supports the positive content.

**Depth cues:**
- Overlap — objects in front obscure those behind
- Size — smaller = further away
- Atmospheric perspective — distant objects are lower contrast and slightly blue-shifted
- Placement — higher on the picture plane = further away (landscape convention)

**Design application:** See `whitespace-theory.md` for detailed application guidance. In UI, the
space between a heading and its paragraph, between a card and its neighbours, and the padding
inside a button are all active space decisions.

---

## 5. Texture

The surface quality of an element — tactile or visual.

**Actual texture** — physically touchable (print, packaging, embossing, fabric).

**Visual texture** — the illusion of texture on screen:
- Photographic textures (grain, paper, concrete)
- Pattern-based textures (dot grids, cross-hatch, noise)
- Typographic texture — dense body text creates its own visual texture at a distance

**Design application:** Subtle noise overlays add warmth to flat colour backgrounds. Paper texture
backgrounds evoke analogue craft. Avoiding texture creates the clean, digital aesthetic. Most
modern web UI uses texture sparingly — a background grain, a subtle pattern.

---

## 6. Value (Light & Dark)

The relative lightness or darkness of an element — independent of hue.

**Value scale:** Runs from pure white (lightest) through grey to pure black (darkest).

**High contrast** — strong differentiation between light and dark areas. Dramatic, bold, readable.

**Low contrast** — subtle tonal differences. Soft, refined, sophisticated — but can reduce
readability.

**Tint** — a colour mixed with white (lighter value).
**Shade** — a colour mixed with black (darker value).
**Tone** — a colour mixed with grey (reduced saturation, adjusted value).

**Design application:** Value is the foundation of readability. Text must have sufficient value
contrast against its background (WCAG contrast ratios are essentially value contrast ratios).
Dark mode inverts the value hierarchy.

---

## 7. Colour

The visual property arising from the spectrum of light reflected or emitted. The most emotionally
powerful element of design.

**Colour properties:**
- **Hue** — the pure colour (red, blue, yellow, etc.)
- **Saturation (Chroma)** — the intensity or purity of the colour. Fully saturated = vivid;
  desaturated = grey.
- **Value (Lightness/Brightness)** — how light or dark the colour is.

**Colour temperature:**
- **Warm** — reds, oranges, yellows. Advance visually, feel energetic, urgent.
- **Cool** — blues, greens, purples. Recede visually, feel calm, trustworthy.

**Colour relationships (harmony):** See `psychology-of-color.md` for full palette strategy.

**Additive colour (RGB/screen)** — red + green + blue = white. Used for all digital design.

**Subtractive colour (CMYK/print)** — cyan + magenta + yellow + black = black. Used for print.

**Design application:** Colour is never used in isolation — it is always experienced relative to
surrounding colours. Establish a palette of primary, secondary, and neutral colours, then use
them consistently through design tokens.

---

## 8. Typography (as Element)

Typography is simultaneously a functional tool and a visual element with its own texture, rhythm,
and personality.

**Typeface categories:**
- **Serif** — traditional, editorial, trustworthy (Times New Roman, Georgia, Playfair Display)
- **Sans-serif** — modern, clean, approachable (Inter, Helvetica, Roboto)
- **Monospace** — technical, code, precise (JetBrains Mono, Courier)
- **Display/Decorative** — expressive, personality-driven — use sparingly for headings only
- **Script/Handwritten** — personal, elegant, artisanal — use very sparingly

**The typographic elements:**
- **Letterform** — the shape of individual characters
- **Counter** — the enclosed or partially enclosed spaces within letters (e.g. inside an 'o')
- **Baseline** — the invisible line on which letters sit
- **X-height** — the height of lowercase letters (higher x-height = more readable at small sizes)
- **Ascender/Descender** — strokes above/below the cap height and baseline

**Design application:** See `typography-principles.md` for hierarchy, scale, and pairing guidance.

---

## Quick Reference

| Element | What it is | Key design consideration |
|---|---|---|
| Line | Mark connecting two points | Weight, direction, and implied lines |
| Shape | 2D enclosed area | Geometric = structured; organic = warm |
| Form | 3D or illusion of depth | Shadow, gradient, and elevation |
| Space | Area in/around elements | Negative space is an active design element |
| Texture | Surface quality | Visual texture via pattern/grain/noise |
| Value | Lightness/darkness | Foundation of contrast and readability |
| Colour | Hue + saturation + value | Most emotionally powerful element |
| Typography | Letterforms as visual elements | Texture, rhythm, and personality |
