# P01T04 — Create `BaseLayout.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/layouts/BaseLayout.astro` as the HTML shell for all pages, with FOUT-prevention inline script, stylesheet imports via Astro asset handling, and named slots for `header` and default content.

## File to create / modify

```
docs/src/layouts/BaseLayout.astro
```

## Implementation

```astro
---
import tokensUrl from "../styles/tokens.css?url";
import baseUrl from "../styles/base.css?url";

interface Props {
  title: string;
  description?: string;
}
const { title, description = "Tekhne — Agent skill library" } = Astro.props;
---
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="description" content={description} />
    <title>{title} | Tekhne</title>
    <link rel="stylesheet" href={tokensUrl} />
    <link rel="stylesheet" href={baseUrl} />
    <!-- Flash-of-wrong-theme prevention: runs synchronously before first paint -->
    <script is:inline>
      (function () {
        var stored = localStorage.getItem("tk-theme");
        var preferred = stored
          || (window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark");
        document.documentElement.setAttribute("data-theme", preferred);
      })();
    </script>
  </head>
  <body>
    <header>
      <slot name="header" />
    </header>
    <slot />
  </body>
</html>
```

## Notes

- Use `?url` suffix (Vite asset import) to resolve stylesheet hrefs through Vite's asset pipeline; this ensures the `base: "/tekhne"` prefix is applied correctly. Do **not** hardcode `/tekhne/styles/...` paths.
- The inline `<script is:inline>` runs synchronously before any CSS is parsed, preventing flash of wrong theme.
- The script reads `localStorage.getItem("tk-theme")` first; falls back to `prefers-color-scheme`.

## Verification

```sh
test -f docs/src/layouts/BaseLayout.astro
grep 'is:inline' docs/src/layouts/BaseLayout.astro
grep '?url' docs/src/layouts/BaseLayout.astro
cd docs && bun run build
```
