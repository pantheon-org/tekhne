# Eval Scenarios: starlight-custom-component

## Scenario 1: Override the footer component

**Prompt:** "How do I replace the Starlight footer with my own custom footer?"

**Expected outputs:**
1. Creates `src/components/CustomFooter.astro` with custom markup
2. Registers `Footer: './src/components/CustomFooter.astro'` in `components: {}` in `astro.config.mjs`
3. Uses path without leading slash

**Success criteria:**
- Correct path format (`'./src/...'` not `'/src/...'`)
- Exact component name `Footer` as key in `components: {}`
- `astro.config.mjs` shows full integration registration

---

## Scenario 2: Add a banner above the header

**Prompt:** "I want to add an announcement banner above the Starlight header without replacing it."

**Expected outputs:**
- Creates a component that imports `Default` from `@astrojs/starlight/components/Header.astro`
- Renders custom banner before `<Default><slot /></Default>`
- Includes `<slot />` inside `<Default>`

**Success criteria:**
- Uses wrapping pattern (not full replacement)
- `<slot />` present inside `<Default>`
- Banner rendered above (before) the `<Default>` call

---

## Scenario 3: Access page title in a custom component

**Prompt:** "How do I access the current page's title inside a custom Starlight component?"

**Expected outputs:**
- Uses `Astro.locals.starlightRoute.entry.data.title`
- Does NOT use `Astro.props.title`

**Success criteria:**
- `Astro.locals.starlightRoute` path used
- `Astro.props` approach not suggested
- Working code example provided

---

## Scenario 4: Conditional override — banner only on guides pages

**Prompt:** "How do I show an upgrade notice banner on all pages under /guides/ but not elsewhere?"

**Expected outputs:**
- Checks `Astro.locals.starlightRoute.id.startsWith('guides/')`
- Conditionally renders banner
- Still renders default header with `<Default><slot /></Default>`

**Success criteria:**
- `starlightRoute.id` used for condition check
- Default component still rendered for non-guide pages
- `<slot />` included in `<Default>`

---

## Scenario 5: Override `TwoColumnContent` without losing the right sidebar

**Prompt:** "I need to wrap TwoColumnContent but the right sidebar disappeared. What did I do wrong?"

**Expected outputs:**
- Identifies missing named slot transfer as the problem
- Provides fix: `<slot name="right-sidebar" slot="right-sidebar" />`
- Shows full corrected component

**Success criteria:**
- Named slot transfer identified as the fix
- Correct slot syntax shown: `<slot name="right-sidebar" slot="right-sidebar" />`
- Both default `<slot />` and named slot included

---

## Scenario 6: Anti-pattern detection — props access

**Prompt:** "I have a custom PageTitle component but `Astro.props.title` is undefined. Why?"

**Expected outputs:**
- Explains that override components don't receive page data as props
- Provides correct path: `Astro.locals.starlightRoute.entry.data.title`

**Success criteria:**
- Root cause explained (override components don't get page data via props)
- `Astro.locals.starlightRoute.entry.data` path shown
- TypeScript type hint optional but appreciated

---

## Scenario 7: Anti-pattern detection — wrong registration key

**Prompt:** "I created `Footer.astro` and put it in `src/components/` but the default footer still shows. I didn't register anything in config. Is naming it Footer.astro enough?"

**Expected outputs:**
- Explains that file name has no effect on which slot is overridden
- Instructs registering `Footer: './src/components/Footer.astro'` in `components: {}` in `astro.config.mjs`

**Success criteria:**
- Clarifies file name is irrelevant
- Correct `components: {}` registration shown
- No suggestion that naming conventions alone work
