# Accessibility Guidelines

Accessibility ensures that web content is usable by people with disabilities — visual, auditory,
motor, and cognitive. It is both a legal requirement in many jurisdictions and a quality indicator
for all users.

The primary standard is **WCAG (Web Content Accessibility Guidelines)**, published by the W3C.
The current version is WCAG 2.2. Most legal requirements reference WCAG 2.1 AA.

---

## WCAG Framework: POUR

All WCAG success criteria are organised under four principles:

**Perceivable** — Information must be presentable in ways users can perceive.  
**Operable** — UI components must be operable by all users.  
**Understandable** — Information and operation must be understandable.  
**Robust** — Content must be robust enough to be interpreted by assistive technologies.

---

## 1. Colour Contrast

The most commonly failed WCAG criterion in web design.

### Requirements

| Content type | WCAG AA | WCAG AAA |
|---|---|---|
| Normal text (< 18pt or < 14pt bold) | 4.5:1 | 7:1 |
| Large text (≥ 18pt or ≥ 14pt bold) | 3:1 | 4.5:1 |
| UI components (buttons, inputs, icons) | 3:1 | — |
| Decorative elements | No requirement | — |
| Logos | No requirement | — |

**Checking tools:**
- WebAIM Contrast Checker: webaim.org/resources/contrastchecker
- Figma plugins: Contrast (by Figma), Able, Stark
- Browser: Chrome DevTools → Accessibility → colour contrast indicator

### Common failures
- Light grey text on white backgrounds (e.g. `#999` on `#fff` = 2.85:1 — FAILS)
- Placeholder text in inputs (often intentionally low contrast — must still meet 4.5:1 if it
  conveys information)
- Button text on brand-coloured buttons (orange buttons with white text often fail)
- Disabled state text — technically exempt, but low-contrast disabled states confuse users

### Design approach
- Check contrast at the token level — build accessible values into the design system
- Never specify a colour combination without checking its ratio
- Slightly darken/desaturate light brand colours for text use

---

## 2. Keyboard Navigation

All interactive elements must be reachable and operable via keyboard alone.

### Requirements
- **Tab order** must follow a logical sequence (typically DOM order)
- All interactive elements (buttons, links, inputs, toggles, dropdowns) must receive keyboard focus
- Custom interactive components (accordions, modals, carousels) must implement keyboard patterns
  as defined in the [ARIA Authoring Practices Guide](https://www.w3.org/WAI/ARIA/apg/)
- No keyboard traps (except modal dialogs, which should trap focus intentionally until closed)

### Focus visibility
- Focus indicators must be visible. WCAG 2.2 SC 2.4.11 (AA) requires a focus indicator with:
  - At least 2px thick outline
  - Contrast ratio of at least 3:1 against adjacent colours
- Never `outline: none` or `outline: 0` without providing a custom focus indicator
- `:focus-visible` is preferred over `:focus` for pointer devices (avoids showing focus ring
  on mouse click while preserving it for keyboard)

### Common keyboard patterns

| Component | Keys |
|---|---|
| Button | Enter / Space to activate |
| Link | Enter to activate |
| Checkbox | Space to toggle |
| Radio group | Arrow keys to move between options |
| Select/Dropdown | Arrow keys to move; Enter to select |
| Modal | Escape to close; Tab/Shift+Tab cycles within |
| Accordion | Enter/Space to toggle; arrows to move between headers |
| Menu | Arrow keys to navigate; Escape to close; Enter to select |

---

## 3. Touch Targets

Sufficient size for users with motor impairments or those on mobile devices.

| Standard | Minimum target size |
|---|---|
| WCAG 2.2 SC 2.5.8 (AA) | 24×24 CSS pixels (with spacing) |
| WCAG 2.2 SC 2.5.5 (AAA) | 44×44 CSS pixels |
| Apple HIG | 44×44 points |
| Google Material | 48×48dp |

**Practical guidance:** Aim for 44×44px as a minimum for all interactive elements. If a smaller
visual element is required, increase the hit area using padding, `::after`, or a transparent
overlay — not by increasing the visible element.

**Spacing:** Touch targets should have at least 8px of non-interactive space between them.

---

## 4. Semantic HTML

Use the correct HTML element for the correct purpose. Semantic HTML provides free accessibility
without requiring ARIA.

| Correct | Avoid |
|---|---|
| `<button>` for actions | `<div onclick>` for actions |
| `<a href>` for navigation | `<span onclick>` for links |
| `<h1>`–`<h6>` for headings | `<div class="heading">` for headings |
| `<nav>` for navigation landmarks | `<div class="nav">` |
| `<main>`, `<header>`, `<footer>` | Non-semantic containers |
| `<ul>`, `<ol>`, `<li>` for lists | `<div>` lists |
| `<table>` with `<th scope>` for tabular data | CSS-faked tables |
| `<form>`, `<label for>`, `<fieldset>` | Custom form widgets without ARIA |

**Heading structure:**
- One `<h1>` per page — the primary page title
- Headings must not skip levels (h1 → h3 without h2)
- Heading levels indicate document structure, not visual size — use CSS for size

---

## 5. ARIA (Accessible Rich Internet Applications)

ARIA attributes supplement semantic HTML when HTML alone cannot convey the required semantics.

**First rule of ARIA: don't use it if you don't need it.** Native HTML semantics are always
preferred.

### Common ARIA patterns

**`aria-label`** — provides a text label when visible text is absent:
```html
<button aria-label="Close dialog">×</button>
```

**`aria-labelledby`** — links to an existing element's text as the label:
```html
<section aria-labelledby="section-heading">
  <h2 id="section-heading">Our Services</h2>
```

**`aria-describedby`** — links to additional descriptive text:
```html
<input aria-describedby="email-hint" type="email">
<span id="email-hint">We'll never share your email.</span>
```

**`aria-hidden="true"`** — hides decorative elements from screen readers:
```html
<span aria-hidden="true">🎉</span> Success!
```

**`aria-live`** — announces dynamic content updates:
```html
<div aria-live="polite" aria-atomic="true"><!-- toast messages --></div>
```

**`role`** — provides semantic role when HTML element doesn't:
```html
<div role="alert">Your session has expired.</div>
```

### ARIA landmark roles
- `role="banner"` → `<header>`
- `role="navigation"` → `<nav>`
- `role="main"` → `<main>`
- `role="contentinfo"` → `<footer>`
- `role="search"` → search form container
- `role="complementary"` → `<aside>`

---

## 6. Images & Media

**Descriptive alt text:**
```html
<!-- Informative image -->
<img src="team-photo.jpg" alt="The Design & Bloom team gathered around a table">

<!-- Decorative image (no information conveyed) -->
<img src="background-squiggle.svg" alt="">

<!-- Functional image (button/link) -->
<a href="/home"><img src="logo.svg" alt="Design & Bloom — return to homepage"></a>
```

**Rules for alt text:**
- Describe the *meaning* and *function*, not the appearance, unless appearance is the content
- Do not start with "Image of" or "Photo of" — screen readers announce it is an image
- Empty `alt=""` for decorative images — never omit the attribute entirely
- Background images (CSS) carry no alt text — never use CSS background for meaningful images

**Video and audio:**
- Videos require captions (prerecorded: AA requirement)
- Audio-only content requires a transcript
- Auto-playing media with sound is prohibited unless the user can stop it within 3 key presses

---

## 7. Reduced Motion

Some users have vestibular disorders; animations can cause nausea, dizziness, and distress.

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

- Parallax scrolling, auto-playing animations, and large motion should be disabled or significantly
  reduced under `prefers-reduced-motion: reduce`
- Subtle fade-ins are acceptable even for users with reduced motion preferences — it is large,
  rapid, or looping motion that causes problems

---

## 8. Cognitive Accessibility

Often overlooked but critically important.

**Clear language:**
- Use plain language — aim for a reading level appropriate for the audience
- Avoid jargon without explanation
- Use active voice

**Predictable patterns:**
- Navigation must be consistent across pages
- Components must behave consistently (a button that is sometimes a link creates confusion)
- Do not change context without user initiation (e.g. do not auto-submit on select change)

**Error prevention and recovery:**
- Label all form fields visibly (not placeholder-only)
- Show inline validation errors with specific, actionable messages
- Provide confirmation before irreversible actions
- Allow undo where possible

**Timeouts:**
- Warn users before sessions expire
- Provide at least 20 seconds to extend a timeout

---

## 9. Testing Checklist

### Automated (catches ~30–40% of issues)
- [ ] Run axe DevTools browser extension on each page
- [ ] Run Lighthouse accessibility audit (Chrome DevTools)
- [ ] Check colour contrast for all text/background combinations

### Manual keyboard testing
- [ ] Tab through entire page — all interactive elements reachable in logical order
- [ ] Visible focus indicator present on all focused elements
- [ ] All interactive actions completable via keyboard alone
- [ ] Modal dialogs trap focus and close on Escape

### Screen reader testing
- [ ] NVDA + Firefox (Windows) or VoiceOver + Safari (macOS/iOS)
- [ ] Page title is descriptive
- [ ] All images have appropriate alt text
- [ ] Form labels are announced with their inputs
- [ ] Dynamic content updates are announced (aria-live regions)

### Visual testing
- [ ] Page usable at 200% browser zoom without horizontal scrolling
- [ ] Page functional with OS high contrast mode enabled
- [ ] No information conveyed by colour alone

---

## Quick Reference: WCAG 2.1 AA Key Criteria

| Criterion | Requirement |
|---|---|
| 1.1.1 Non-text Content | Alt text for all informative images |
| 1.3.1 Info and Relationships | Semantic HTML for structure |
| 1.4.1 Use of Colour | Colour not the sole differentiator |
| 1.4.3 Contrast (Minimum) | 4.5:1 text, 3:1 large text/UI |
| 1.4.4 Resize Text | 200% zoom without loss of content |
| 1.4.10 Reflow | Content reflows at 320px width |
| 2.1.1 Keyboard | All functionality keyboard accessible |
| 2.4.3 Focus Order | Logical tab order |
| 2.4.7 Focus Visible | Keyboard focus always visible |
| 3.1.1 Language of Page | `<html lang>` set correctly |
| 3.3.1 Error Identification | Form errors described in text |
| 3.3.2 Labels or Instructions | All inputs have visible labels |
| 4.1.2 Name, Role, Value | All UI components have ARIA semantics |
