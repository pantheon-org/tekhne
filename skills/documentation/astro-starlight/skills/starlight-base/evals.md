# Eval Scenarios: starlight-base

## Scenario 1: New project setup

**Prompt:** "I want to create a new Astro Starlight documentation site. Show me the setup steps."

**Expected outputs:**
- Runs `npm create astro@latest -- --template starlight` (or pnpm/yarn equivalent)
- Runs `npm run dev` to start dev server
- Does NOT manually install `@astrojs/starlight` separately (template handles it)

**Success criteria:**
- Correct scaffolding command used
- Dev server command included
- No unnecessary manual steps added

---

## Scenario 2: Add Starlight to existing Astro project

**Prompt:** "I have an existing Astro project and want to add Starlight to it."

**Expected outputs:**
- Runs `npx astro add starlight`
- Notes that this updates `astro.config.mjs` automatically
- Does NOT instruct manual `npm install @astrojs/starlight` + manual config editing

**Success criteria:**
- Uses `astro add` integration command
- Mentions automatic config update

---

## Scenario 3: Sidebar configuration

**Prompt:** "How do I configure the sidebar with explicit links and an auto-generated section?"

**Expected outputs:**
- Shows `sidebar` array in `starlight({})` config
- Demonstrates both `items` (explicit) and `autogenerate` approaches
- Uses `slug` values without leading slashes or `.md` extensions
- Does NOT use both `items` and `autogenerate` on the same group

**Success criteria:**
- Correct slug format (e.g., `'guides/getting-started'` not `'/guides/getting-started.md'`)
- `autogenerate: { directory: '...' }` syntax correct
- No mixed `items`/`autogenerate` in same group

---

## Scenario 4: Sitemap setup

**Prompt:** "How do I enable the sitemap for my Starlight site?"

**Expected outputs:**
- Sets `site: 'https://...'` on `defineConfig({})`, NOT inside `starlight({})`
- Notes that Starlight generates the sitemap automatically when `site` is set

**Success criteria:**
- `site` is at `defineConfig` level
- No manual sitemap plugin mentioned (not needed)

---

## Scenario 5: Draft pages

**Prompt:** "How do I mark a documentation page as a draft so it doesn't appear in production?"

**Expected outputs:**
- Adds `draft: true` to page frontmatter
- Notes that draft pages are visible in dev but excluded from production build and sitemap

**Success criteria:**
- Frontmatter syntax is correct
- Development vs. production behavior explained

---

## Scenario 6: Anti-pattern detection — wrong slug format

**Prompt:** "My sidebar link isn't working. I set `slug: '/guides/setup.md'`. What's wrong?"

**Expected outputs:**
- Identifies leading slash and `.md` extension as the problem
- Provides corrected slug: `'guides/setup'`
- Explains that slugs map to paths under `src/content/docs/` without slash or extension

**Success criteria:**
- Both errors (leading slash AND extension) identified
- Correct slug shown
- Cause explained
