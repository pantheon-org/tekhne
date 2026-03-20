# Laws of UX: Theme Amber Critical Review

**Date:** 2026-03-20
**Scope:** `.context/previews/theme-amber/` (all 6 pages)

### Severity key: HIGH · MEDIUM · PASS

---

## HIGH: Fitts's Law — Touch targets below minimum

**Affected:** skill-detail, tile-detail (all pages with left-nav)

`.left-nav__item` has `padding: 0.35rem 1.25rem` — ~5.6px top+bottom. Combined with
`font-size: 0.8125rem` the rendered height is ~22–26px, well below the 44px minimum.
SVG icon buttons (`width: 0.875rem; height: 0.875rem` = 14px) throughout the nav and
toolbar have the same problem.

**Fix:** Raise to `padding: 0.6rem 1.25rem` for nav items and wrap icon-only controls
in a 44×44px hit area.

---

## HIGH: Aesthetic-Usability Effect — Focus ring stripped with no replacement

**Affected:** all pages (search input, install tabs, sort buttons)

```css
.toolbar__search input { outline: none; }
```

No `:focus-visible` style replaces the native ring anywhere in the amber overrides.
Keyboard users get zero indication of where focus is. This also fails WCAG 2.4.11.

**Fix:**
```css
:focus-visible { outline: 2px solid var(--tk-accent); outline-offset: 2px; }
.toolbar__search:focus-within { border-color: var(--tk-accent); }
```

---

## MEDIUM: Jakob's Law — Inconsistent sidebar presence breaks spatial memory

**Affected:** landing / skills-list / tiles-list hide left-nav; skill-detail / tile-detail show it

Users build a mental model of "this site has no sidebar" on browse pages, then encounter
a persistent left-nav on detail pages. The navigation context shifts without warning.
Either commit to sidebar everywhere or remove it everywhere and rely on breadcrumbs.

---

## MEDIUM: Jakob's Law / Law of Proximity — Horizontal pill strip has no overflow affordance

**Affected:** landing, skills-list (domain strip)

```css
.domain-grid { display: flex; flex-wrap: nowrap; gap: 0.625rem; overflow-x: auto; }
```

`flex-wrap: nowrap` with `overflow-x: auto` scrolls, but there is no fade gradient or
scroll arrow to hint that more pills exist. Users stop scanning at the visible edge.
On narrow viewports, all secondary domains are invisible.

**Fix:** Add a right-edge gradient mask and/or allow wrap at ≤768px.

---

## MEDIUM: Von Restorff Effect — Install tab active state is low-contrast

**Affected:** skill-detail install block

The active tab uses only a bottom border (`border-bottom: 2px solid var(--tk-accent)`)
to signal selection. Against a dark surface, this is subtle and the inactive tabs are
visually very similar to the active one.

**Fix:** Apply a background fill or a bolder weight change to the active tab so the
selected state is unambiguous at a glance.

---

## MEDIUM: Hick's Law — No recommended/default highlighted in install tabs

Three equal-weight tabs (tessl.io / skills.sh / agentskills.sh) present three paths
with no visual signal about which to use first. First-time users are left to guess.

**Fix:** Add a `(recommended)` label or a small badge to the primary tab.

---

## Passes

| Law | Evidence |
|---|---|
| Miller's Law | Top nav: 2 items only. Domain grouping well-chunked. |
| Serial Position | "Skills" leads nav; settings/secondary items trail. |
| Tesler's Law | Install commands pre-populated, copy button toggles to checkmark. |
| Peak-End Rule | Copy button → checkmark feedback is a positive micro-moment. |
| Law of Similarity | Consistent pill/badge styling across pages. |
| Uniform Connectedness | Breadcrumb on all detail pages; sticky headers orient users in long lists. |
| Occam's Razor | Hero stripped of decorative backgrounds — clean. |

---

## Priority order for fixes

1. **Focus ring** — accessibility blocker, trivial CSS fix
2. **Touch target heights** — affects all keyboard and touch users
3. **Pill strip overflow affordance** — content discovery issue
4. **Sidebar consistency** — spatial disorientation across page types
5. **Install tab prominence** — conversion/onboarding friction

---

---

# Nielsen Heuristic Evaluation — Theme Amber Previews

**Date**: 2026-03-20
**Scope**: All 6 preview pages
**Method**: Nielsen's 10 Usability Heuristics

## Heuristic Evaluation Summary

| Severity | Count |
|----------|-------|
| Critical | 0     |
| Major    | 4     |
| Minor    | 5     |
| Advisory | 3     |
| **Total** | **12** |

---

### [H7-1] No skip-to-content link on any page

- **Severity**: Major
- **Heuristic**: #7 — Flexibility and Efficiency of Use
- **Location**: All 6 pages — `<body>` opening, before `<nav class="top-nav">`
- **Description**: Keyboard users must tab through the entire top nav plus ~15 left-nav items on inner pages before reaching main content. Every page reload resets focus to the top.
- **Evidence**:
  ```html
  <body>
    <!-- No skip link -->
    <nav class="top-nav">…</nav>
    <nav class="left-nav">… ~15 items …</nav>
    <main>…</main>
  ```
- **Recommendation**:
  ```html
  <body>
    <a href="#main-content" class="skip-link">Skip to main content</a>
    <nav class="top-nav">…</nav>
    …
    <main id="main-content">…</main>
  ```
  CSS: `.skip-link { position: absolute; left: -9999px; } .skip-link:focus { left: 1rem; top: 1rem; z-index: 9999; }`

---

### [H7-2] Skill tab switcher uses `<div>` — not keyboard accessible

- **Severity**: Major
- **Heuristic**: #7 — Flexibility and Efficiency of Use
- **Location**: `skill-preview.html` — `.sk-tabs`
- **Description**: The Skill / Audits / Evals / References tabs are `<div>` elements — not reachable by Tab key, not activatable with Enter/Space, and carry no ARIA tab semantics. Keyboard users cannot access audit results or reference docs.
- **Evidence**:
  ```html
  <div class="sk-tabs">
    <div class="sk-tab sk-tab--active" data-tab="skill">Skill</div>
    <div class="sk-tab" data-tab="audits">Audits <span class="sk-tab__count">4</span></div>
    <div class="sk-tab" data-tab="evals">Evals</div>
    <div class="sk-tab" data-tab="references">References</div>
  </div>
  ```
- **Recommendation**:
  ```html
  <div class="sk-tabs" role="tablist" aria-label="Skill sections">
    <button class="sk-tab sk-tab--active" role="tab" aria-selected="true"
            aria-controls="panel-skill" id="tab-skill">Skill</button>
    <button class="sk-tab" role="tab" aria-selected="false"
            aria-controls="panel-audits" id="tab-audits">
      Audits <span class="sk-tab__count">4</span>
    </button>
    …
  </div>
  <div id="panel-skill" role="tabpanel" aria-labelledby="tab-skill">…</div>
  ```

---

### [H6-1] Toolbar search inputs have no visible `<label>`

- **Severity**: Major
- **Heuristic**: #6 — Recognition Rather than Recall
- **Location**: `tiles-preview.html`, `skills-preview.html`, `domain-preview.html` — `.toolbar__search input`
- **Description**: The search field uses only `placeholder` text. Once typing starts, the label disappears and the field is semantically unlabelled — no `<label>`, no `aria-label`.
- **Evidence**:
  ```html
  <div class="toolbar__search">
    <svg>…</svg>
    <input placeholder="Search tiles…">
  </div>
  ```
- **Recommendation**:
  ```html
  <label for="search-tiles" class="sr-only">Search tiles</label>
  <div class="toolbar__search">
    <svg aria-hidden="true">…</svg>
    <input id="search-tiles" type="search" placeholder="Search tiles…">
  </div>
  ```

---

### [H1-1] Copy button provides no success feedback

- **Severity**: Major
- **Heuristic**: #1 — Visibility of System Status
- **Location**: `skill-preview.html`, `tile-preview.html` — `.install-copy` buttons
- **Description**: Clicking the copy button produces no visible or announced state change. Users have no confirmation the command was copied. (The `ux-review.md` passes section credits a checkmark feedback — this is not present in the HTML markup; it would need to be wired in JS.)
- **Evidence**:
  ```html
  <button class="install-copy" aria-label="Copy to clipboard">
    <svg>…clipboard icon…</svg>
  </button>
  <!-- No success state, no aria-live region -->
  ```
- **Recommendation**:
  ```html
  <button class="install-copy" aria-label="Copy to clipboard">
    <svg class="copy-icon" aria-hidden="true">…</svg>
    <svg class="check-icon" aria-hidden="true" hidden>…</svg>
  </button>
  <div aria-live="polite" class="sr-only" id="copy-status"></div>
  ```
  JS: on copy, swap icons, update `aria-label` to "Copied!", announce via live region, reset after 2 s.

---

### [H6-2] Breadcrumb terminal item missing `aria-current="page"`

- **Severity**: Minor
- **Heuristic**: #6 — Recognition Rather than Recall
- **Location**: `domain-preview.html`, `skill-preview.html` — `.breadcrumb`
- **Evidence**:
  ```html
  <span class="breadcrumb__sep">›</span>
  <span>Development</span>  <!-- missing aria-current -->
  ```
- **Recommendation**:
  ```html
  <span class="breadcrumb__sep" aria-hidden="true">›</span>
  <span aria-current="page">Development</span>
  ```

---

### [H1-2] Theme toggle button has no visible content or current-state label

- **Severity**: Minor
- **Heuristic**: #1 — Visibility of System Status
- **Location**: All pages — `#theme-btn`
- **Description**: The button is completely empty — no icon, no text. `aria-label="Switch theme"` names the action but not the current state. A sighted user cannot tell what the button does or what theme is active.
- **Evidence**:
  ```html
  <button class="theme-btn" id="theme-btn" aria-label="Switch theme"></button>
  ```
- **Recommendation**:
  ```html
  <button class="theme-btn" id="theme-btn"
          aria-label="Switch to light theme"
          title="Switch theme">
    <svg aria-hidden="true">…moon icon…</svg>
  </button>
  ```
  Update `aria-label` dynamically on toggle to reflect the new target theme.

---

### [H4-1] Sort buttons lack `aria-pressed` state

- **Severity**: Minor
- **Heuristic**: #4 — Consistency and Standards
- **Location**: `tiles-preview.html`, `skills-preview.html`, `domain-preview.html` — `.toolbar__sort-btn`
- **Evidence**:
  ```html
  <button class="toolbar__sort-btn toolbar__sort-btn--active">Popular</button>
  <button class="toolbar__sort-btn">Recent</button>
  ```
- **Recommendation**:
  ```html
  <button class="toolbar__sort-btn toolbar__sort-btn--active" aria-pressed="true">Popular</button>
  <button class="toolbar__sort-btn" aria-pressed="false">Recent</button>
  ```

---

### [H5-1] Global search bar is a `<div>` styled as an input

- **Severity**: Minor
- **Heuristic**: #5 — Error Prevention
- **Location**: All pages — `.nav-search` in `<nav class="top-nav">`
- **Description**: Visually identical to a text input (border, padding, search icon, placeholder), but it is a `<div>` with no `role`, no `tabindex`, and no keyboard handler. A user who clicks expecting to type will experience a false affordance.
- **Evidence**:
  ```html
  <div class="nav-search">
    <svg>…</svg>
    Search tiles…
    <kbd>⌘K</kbd>
  </div>
  ```
- **Recommendation**:
  ```html
  <button class="nav-search" aria-label="Search (⌘K)" aria-keyshortcuts="Meta+K">
    <svg aria-hidden="true">…</svg>
    <span>Search tiles…</span>
    <kbd aria-hidden="true">⌘K</kbd>
  </button>
  ```

---

### [H6-3] Active left-nav item missing `aria-current="page"`

- **Severity**: Minor
- **Heuristic**: #6 — Recognition Rather than Recall
- **Location**: All inner pages — `.left-nav__item--active`
- **Evidence**:
  ```html
  <a href="tiles-preview.html" class="left-nav__item left-nav__item--active">Tiles</a>
  ```
- **Recommendation**:
  ```html
  <a href="tiles-preview.html" class="left-nav__item left-nav__item--active"
     aria-current="page">Tiles</a>
  ```

---

### [H10-1] No empty state for zero search results

- **Severity**: Advisory
- **Heuristic**: #10 — Help and Documentation
- **Location**: `tiles-preview.html`, `skills-preview.html`, `domain-preview.html`
- **Description**: When a search produces no matches the list area goes blank — no guidance, no CTA, no suggestion to clear the query.
- **Recommendation**:
  ```html
  <div class="empty-state" id="search-empty" hidden>
    <p>No results for "<strong id="empty-query"></strong>"</p>
    <button class="btn-outline" id="clear-search">Clear search</button>
  </div>
  ```

---

### [H7-3] Reference file selector not grouped as `role="toolbar"`

- **Severity**: Advisory
- **Heuristic**: #7 — Flexibility and Efficiency of Use
- **Location**: `skill-preview.html` — `.audit-selector`
- **Recommendation**:
  ```html
  <div class="audit-selector" role="toolbar" aria-label="Reference files">…</div>
  ```

---

### [H1-3] Install method tabs lack `role="tab"` and `aria-selected`

- **Severity**: Advisory
- **Heuristic**: #1 — Visibility of System Status
- **Location**: `skill-preview.html`, `tile-preview.html` — `.install-tab` buttons
- **Evidence**:
  ```html
  <button class="install-tab install-tab--active" data-target="tessl">tessl</button>
  <button class="install-tab" data-target="npm">npm</button>
  ```
- **Recommendation**:
  ```html
  <div role="tablist" aria-label="Install method">
    <button role="tab" aria-selected="true" class="install-tab install-tab--active"
            aria-controls="install-panel-tessl">tessl</button>
    <button role="tab" aria-selected="false" class="install-tab"
            aria-controls="install-panel-npm">npm</button>
  </div>
  ```

---

## Heuristic Recommendations Priority

1. **Fix soon** (Major):
   - **H7-1** — Skip-to-content link (all pages, ~10 min)
   - **H7-2** — Convert `.sk-tab` divs to `<button role="tab">` with ARIA tablist
   - **H6-1** — Add `<label>` / `aria-label` to toolbar search inputs
   - **H1-1** — Wire copy success state (visual swap + `aria-live`)

2. **Next iteration** (Minor):
   - **H6-2 / H6-3** — `aria-current="page"` on breadcrumb + left-nav active items
   - **H1-2** — Theme button visible icon + dynamic state label
   - **H4-1** — `aria-pressed` on sort buttons
   - **H5-1** — Convert `.nav-search` div to `<button>`

3. **Consider for enhancement** (Advisory):
   - **H10-1** — Empty state for zero search results
   - **H7-3** — `role="toolbar"` on reference selector
   - **H1-3** — Full ARIA tab pattern on install method tabs
