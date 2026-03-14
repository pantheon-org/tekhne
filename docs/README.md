# Tekhne Docs

Documentation site for [Tekhne](https://github.com/pantheon-org/tekhne) — a curated collection of
reusable agent skills for AI assistants.

Built with [Astro Starlight](https://starlight.astro.build) and deployed to GitHub Pages at
`https://pantheon-org.github.io/tekhne/`.

## How it works

Skills live under `skills/` in the repository root. The docs site renders them without copying files:
a symlink at `src/content/docs/skills` points to `../../../../skills`. The symlink is **not tracked in
git** — it is created automatically at build time by the `prelink` script in `package.json`.

## Local development

```sh
# From repository root
cd docs
bun install
bun run dev    # starts dev server at http://localhost:4321/tekhne/
```

`bun run dev` creates the symlink, then starts Astro. On Windows, run Node.js as Administrator or
enable Developer Mode so that `junction` symlinks can be created without elevated privileges.

## Building

```sh
bun run build     # outputs to docs/dist/
bun run preview   # preview the built site locally
```

## Deployment

The site is deployed automatically by `.github/workflows/deploy-docs.yml` on every push to `main`
that changes `docs/**` or `skills/**`.

To enable GitHub Pages for the first time, go to **Settings → Pages** and set the source to
**GitHub Actions**.

## Commands

| Command             | Action                                      |
| :------------------ | :------------------------------------------ |
| `bun install`       | Install dependencies                        |
| `bun run dev`       | Start local dev server at `localhost:4321`  |
| `bun run build`     | Build production site to `./dist/`          |
| `bun run preview`   | Preview the build locally before deploying  |
