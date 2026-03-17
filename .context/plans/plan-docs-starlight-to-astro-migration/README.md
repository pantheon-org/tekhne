# Implementation Plan — docs-starlight-to-astro-migration

## Goal

Replace the Starlight wrapper in `docs/` with a bespoke Astro layout, removing
the framework constraint while preserving the existing content pipeline, skill
pages, tabs UI, and GitHub Pages deployment. The result is a fully custom site
with `--tk-*` CSS tokens, dark/light theming, Pagefind search, and a dynamic
tiles catalog.

## Phases

| # | Phase | Status | Tasks |
|---|---|---|---|
| [01](phases/phase-01-layout-scaffolding/README.md) | Layout scaffolding (Starlight still active) | pending | 10 |
| [02](phases/phase-02-route-migration-and-starlight-cutover/README.md) | Route migration and Starlight cutover | pending | 11 |
| [03](phases/phase-03-css-token-migration/README.md) | CSS token migration | pending | 5 |
| [04](phases/phase-04-polish-and-feature-parity/README.md) | Polish and feature parity | pending | 7 |
| [05](phases/phase-05-registry-enhancements/README.md) | Registry enhancements | pending | 4 |

## Constraints

- `base: "/tekhne"` must be respected — no hardcoded absolute asset paths.
- Phase 2 is a hard cutover on a feature branch; Starlight and new routes never coexist.
- All 537+ pages must render after Phase 2; no regressions.
- GitHub Pages deploy workflow must remain untouched.
- `@astrojs/mdx` must be an explicit dependency before Starlight is removed.

## References

- Source plan: `.context/plans/docs-starlight-to-astro-migration.md`
- Current `docs/` entry: `docs/astro.config.mjs`, `docs/src/content/content.config.ts`
- Pagefind standalone docs: https://pagefind.app/docs/astro/
