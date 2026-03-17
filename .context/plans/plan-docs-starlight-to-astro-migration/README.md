# Implementation Plan — Docs Starlight to Astro Migration

## Goal

Replace the Starlight wrapper in `docs/` with a bespoke Astro layout, removing
the framework constraint while preserving the content pipeline, skill pages, and
tabs UI. All 537+ pages must continue to render; the GitHub Pages deploy workflow
remains unchanged.

## Phases

| # | Phase | Status | Tasks |
|---|---|---|---|
| 01 | [Layout Scaffolding](phases/phase-01-layout-scaffolding/README.md) | pending | 10 |
| 02 | [Route Migration & Starlight Cutover](phases/phase-02-route-migration-starlight-cutover/README.md) | pending | 10 |
| 03 | [CSS Token Migration](phases/phase-03-css-token-migration/README.md) | pending | 5 |
| 04 | [Polish and Feature Parity](phases/phase-04-polish-and-feature-parity/README.md) | pending | 7 |
| 05 | [Registry Enhancements](phases/phase-05-registry-enhancements/README.md) | pending | 4 |

## Constraints

- `base: "/tekhne"` in `astro.config.mjs` — all asset paths must use Astro `import` + Vite asset handling; never hardcode absolute paths.
- Phase 2 is a **hard cutover** — Starlight and new routes do not coexist.
- Phase 2 must be executed on a feature branch; merge only after CI build passes.
- `@astrojs/mdx` is currently only available transitively via Starlight — must be added explicitly in Phase 1.
- Theme default: system preference (`prefers-color-scheme`), user-overridable, persisted in `localStorage`.
- Left nav state persisted per domain group in `localStorage`; active page's ancestor always force-expanded.

## References

- Source plan: `.context/plans/docs-starlight-to-astro-migration.md`
- Risk register: see source plan `## Risk areas` table
- Definition of done: see source plan `## Definition of done`
