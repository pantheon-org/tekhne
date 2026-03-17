# P01T04 — Create `src/layouts/BaseLayout.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/layouts/BaseLayout.astro` — the HTML shell with theme-init
script, CSS imports, slots, and Vite-safe asset paths.

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
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="description" content={description} />
    <title>{title} | Tekhne</title>
    <link rel="stylesheet" href={tokensUrl} />
    <link rel="stylesheet" href={baseUrl} />
    <!-- Inline theme-init: must run before any CSS renders to prevent FOUT -->
    <script is:inline>
      (function () {
        const stored = localStorage.getItem("tk-theme");
        const preferred = window.matchMedia("(prefers-color-scheme: dark)").matches
          ? "dark"
          : "light";
        document.documentElement.setAttribute("data-theme", stored || preferred);
      })();
    </script>
  </head>
  <body>
    <slot name="header" />
    <slot />
  </body>
</html>
```

## Notes

- Use `?url` Vite import suffix so paths go through Vite asset handling and respect `base: "/tekhne"` in `astro.config.mjs`.
- `is:inline` on the theme-init script prevents Astro from hoisting it — it must execute synchronously before the first CSS render.
- The `header` named slot is filled by `DocsLayout` / `SkillLayout` to inject the site header.

## Verification

```sh
grep "is:inline" docs/src/layouts/BaseLayout.astro
grep "data-theme" docs/src/layouts/BaseLayout.astro
grep "?url" docs/src/layouts/BaseLayout.astro
```
