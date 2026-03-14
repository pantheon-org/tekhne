# {PROJECT_NAME} — Web Design Reference Sheet

> Single source of truth for all visual and interaction decisions.
> Use this document when designing in Figma, writing CSS, or reviewing code.
> Figma source: [Web Design Reference Sheet](https://www.figma.com/community/file/1318219306256490255/web-design-reference-sheet)

---

## Table of Contents

1. [Brand Identity](#1-brand-identity)
2. [Colour Palette](#2-colour-palette)
3. [Typography](#3-typography)
4. [Spacing System](#4-spacing-system)
5. [Layout & Grid](#5-layout--grid)
6. [Borders & Radius](#6-borders--radius)
7. [Shadows & Elevation](#7-shadows--elevation)
8. [Motion & Transitions](#8-motion--transitions)
9. [Component Patterns](#9-component-patterns)
10. [Accessibility](#10-accessibility)
11. [Dark Mode](#11-dark-mode)
12. [Figma Sync Notes](#12-figma-sync-notes)

---

## 1. Brand Identity

| Property      | Value                    |
| ------------- | ------------------------ |
| Project name  | {PROJECT_NAME}           |
| Tagline       | {TAGLINE}                |
| Personality   | {PERSONALITY_ADJECTIVES} |
| Target market | {TARGET_MARKET}          |
| Brand voice   | {BRAND_VOICE}            |

> If brand identity is not defined in code, ask the user to supply it or mark as TBD.

---

## 2. Colour Palette

All tokens are defined as CSS custom properties in `{TOKEN_FILE_PATH}`.

### Primary Colours

| Token | Hex | Usage |
| ----- | --- | ----- |
| `--{token}` | `#{hex}` | {usage} |

### Semantic Colours

| Token | Value | Usage |
| ----- | ----- | ----- |
| `--border` | `{value}` | Subtle borders |
| `--text-secondary` | `{value}` | Supporting copy |
| `--error` | `{value}` | Form validation |

### Colour Do / Don't

- **Do** use `--{accent}` only on `--{sufficient-contrast-bg}` text
- **Don't** use `--{over-saturated}` as a background

---

## 3. Typography

### Font Stack

| Token | Family | Weights | Usage |
| ----- | ------ | ------- | ----- |
| `--font-display` | {family} | {weights} | Headings (h1–h3) |
| `--font-main`    | {family} | {weights} | Body copy, paragraphs |
| `--font-nav`     | {family} | {weights} | Navigation, buttons, labels |
| `--font-editorial` | {family} | {weights} | Pull quotes, subtitles |

> Include only font roles that exist in the project.

### Type Scale

| Element | Size | Weight | Line height | Letter spacing | Font |
| ------- | ---- | ------ | ----------- | -------------- | ---- |
| h1      | `{size}` | {weight} | {lh} | {ls} | `--font-display` |
| h2      | `{size}` | {weight} | {lh} | {ls} | `--font-display` |
| h3      | `{size}` | {weight} | {lh} | {ls} | `--font-display` |
| body    | `{size}` | {weight} | {lh} | — | `--font-main`    |
| small   | `{size}` | {weight} | {lh} | — | `--font-main`    |
| label   | `{size}` | {weight} | — | `{ls}` uppercase | `--font-nav` |
| button  | `{size}` | {weight} | — | {ls} | `--font-nav` |

---

## 4. Spacing System

| Token | Value | Usage example |
| ----- | ----- | ------------- |
| `--spacing-{name}` | `{value}` | {usage} |

### Section Vertical Rhythm

| Breakpoint | Padding top/bottom |
| ---------- | ------------------ |
| Mobile     | `{value}`          |
| Desktop    | `{value}`          |

---

## 5. Layout & Grid

### Container

| Purpose | Max width | Padding |
| ------- | --------- | ------- |
| Navigation | `{value}` | `{value}` |
| Content    | `{value}` | `{value}` |
| Narrow prose | `{value}` | `{value}` |

### Column Grid

| Layout | Classes |
| ------ | ------- |
| {n}-col | `{classes}` |

### Breakpoints

| Name | Min width |
| ---- | --------- |
| sm | 640px |
| md | 768px |
| lg | 1024px |
| xl | 1280px |

---

## 6. Borders & Radius

| Use case | Value | Tailwind class |
| -------- | ----- | -------------- |
| Card border | `{value}` | `{class}` |
| Badge / pill | `{value}` | `{class}` |
| Card / module | `{value}` | `{class}` |
| Button | `{value}` | `{class}` |

---

## 7. Shadows & Elevation

| Level | CSS value | Usage |
| ----- | --------- | ----- |
| Default | none | Resting cards |
| Hover | `{value}` | Card hover |

---

## 8. Motion & Transitions

### Standard Transitions

| Property | Duration | Easing | Usage |
| -------- | -------- | ------ | ----- |
| Color | {duration} | {easing} | Links, buttons |
| Box shadow | {duration} | {easing} | Card hover |
| Transform | {duration} | {easing} | Lift effects |

### Keyframe Animations

List any named `@keyframes` found in CSS with their purpose.

### Reduced Motion

Document how `prefers-reduced-motion` is handled (global CSS guard, per-component, etc.).

---

## 9. Component Patterns

For each distinct interactive pattern found in components, document:

### Buttons

**Primary:**
```html
{copy-paste HTML with all classes}
```

**Secondary:**
```html
{copy-paste HTML}
```

**Ghost (on dark bg):**
```html
{copy-paste HTML}
```

### Badge / Section Label

```html
{copy-paste HTML}
```

### Card

```html
{copy-paste HTML}
```

### Section Structure

```html
{copy-paste HTML section skeleton}
```

> Add any additional patterns found (modals, toasts, tabs, accordions, etc.)

---

## 10. Accessibility

| Rule | Implementation |
| ---- | -------------- |
| WCAG AA contrast | {details} |
| Focus visible | {implementation} |
| Decorative icons | `aria-hidden="true"` |
| Screen reader text | {utility class used} |
| Landmark regions | {which elements used} |
| Heading hierarchy | {policy} |
| Touch targets | Minimum `44px` |
| Reduced motion | {implementation} |

---

## 11. Dark Mode

| Token | Light | Dark |
| ----- | ----- | ---- |
| `--{token}` | `{value}` | `{value}` |

> Document which tokens change in dark mode and which stay fixed (accent colours typically do not change).

Dark mode strategy: {class-based `.dark` / `prefers-color-scheme` / data attribute}.

---

## 12. Figma Sync Notes

### Keeping Figma and code in sync

1. All design tokens live in `{token file path}` — treat it as the **master token source**.
2. When updating a value: update CSS first, then reflect in Figma library.
3. The [Figma Web Design Reference Sheet](https://www.figma.com/community/file/1318219306256490255) provides the template structure. Use it to populate a project-specific Figma file.
4. Recommended Figma file structure: Colours → Typography → Spacing → Grid → Components → Accessibility notes.

### Naming convention alignment

| CSS token | Figma style name suggestion |
| --------- | --------------------------- |
| `--{background-token}` | `Color/Background/Base` |
| `--{primary-text-token}` | `Color/Text/Primary` |
| `--{accent-token}` | `Color/Accent/{Name}` |
| `--{brand-token}` | `Color/Brand/{Name}` |
| `--font-display` | `Text Style/Display/H1` etc. |
| `--font-main` | `Text Style/Body/Regular` |
| `--font-nav` | `Text Style/UI/Label` |

### Checklist when adding a new component in Figma

- [ ] Uses only tokens from Section 2 (Colours)?
- [ ] Type size, weight, and font match Section 3 (Typography)?
- [ ] Spacing uses values from Section 4 (Spacing)?
- [ ] Passes WCAG AA contrast (Section 10)?
- [ ] Added to Component Patterns (Section 9) in this doc?
