# P02T03 — create-index-astro

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Create `docs/src/pages/index.astro` as the site landing page, replacing the Starlight-generated root with a custom hero section and feature cards — no `starlight/components` imports.

## File to create / modify

```
docs/src/pages/index.astro
```

## Implementation

```astro
---
import BaseLayout from "../layouts/BaseLayout.astro";

const BASE = import.meta.env.BASE_URL.replace(/\/$/, "");
---
<BaseLayout title="Tekhne — Agent Skills" description="Curated, production-ready skills for AI coding agents.">
  <main class="home">
    <section class="hero">
      <h1>Tekhne</h1>
      <p class="tagline">Curated, production-ready skills for AI coding agents.</p>
      <div class="hero-actions">
        <a href={`${BASE}/tiles`} class="btn-primary">Browse Skills</a>
        <a href="https://github.com/pantheon-org/tekhne" class="btn-secondary">GitHub</a>
      </div>
    </section>

    <section class="features">
      <div class="feature-card">
        <h2>500+ Skills</h2>
        <p>Domain-specific SKILL.md files covering CI/CD, infrastructure, testing, documentation, and more.</p>
      </div>
      <div class="feature-card">
        <h2>Agent Agnostic</h2>
        <p>Compatible with Claude Code, Cursor, Gemini CLI, and 40+ other AI coding agents via the Agent Skills spec.</p>
      </div>
      <div class="feature-card">
        <h2>Quality Graded</h2>
        <p>Each skill is scored on 9 dimensions. A-grade skills are publication-ready; all others show improvement paths.</p>
      </div>
    </section>
  </main>
</BaseLayout>
```

## Notes

- No `@astrojs/starlight` imports; this page is pure Astro + our own layouts.
- `BASE_URL` trailing slash is stripped to avoid double slashes in `${BASE}/tiles`.
- Feature card copy can be iterated in Phase 4 polish; this establishes the structural skeleton.
- The page is served at `/tekhne/` (GitHub Pages deployment with `base: "/tekhne"`).

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
grep -r "starlight" src/pages/index.astro && echo "FAIL: starlight import found" || echo "ok"
```
