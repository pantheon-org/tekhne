# P02T02 — refactor-starlight-route-from-components

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Remove all `Astro.locals.starlightRoute` usages from `BaseLayout.astro`, `DocsLayout.astro`, and `SkillLayout.astro`, replacing them with an explicit `entry: CollectionEntry<"docs">` prop passed from the page route.

## File to create / modify

```
docs/src/layouts/BaseLayout.astro
docs/src/layouts/DocsLayout.astro
docs/src/layouts/SkillLayout.astro
```

## Implementation

In each layout, change the Props interface and remove `Astro.locals.starlightRoute`:

**BaseLayout.astro** — add `title` and `description` to Props; remove any Starlight locals reference:

```astro
---
interface Props {
  title: string;
  description?: string;
}
const { title, description = "" } = Astro.props;
---
```

**DocsLayout.astro** — accept `entry: CollectionEntry<"docs">`:

```astro
---
import type { CollectionEntry } from "astro:content";
import BaseLayout from "./BaseLayout.astro";
import LeftNav from "../components/LeftNav.astro";

interface Props {
  entry: CollectionEntry<"docs">;
}
const { entry } = Astro.props;
---
<BaseLayout title={entry.data.title} description={entry.data.description}>
  <LeftNav slot="nav" />
  <slot />
</BaseLayout>
```

**SkillLayout.astro** — same pattern, plus sidebar data:

```astro
---
import type { CollectionEntry } from "astro:content";
import BaseLayout from "./BaseLayout.astro";
import LeftNav from "../components/LeftNav.astro";
import SkillSidebar from "../components/SkillSidebar.astro";

interface Props {
  entry: CollectionEntry<"docs">;
}
const { entry } = Astro.props;
---
<BaseLayout title={entry.data.title} description={entry.data.description}>
  <LeftNav slot="nav" />
  <slot />
  <SkillSidebar slot="sidebar" entry={entry} />
</BaseLayout>
```

## Notes

- `Astro.locals.starlightRoute` is a Starlight-internal API that will not exist after P02T07. Any remaining reference will cause a TypeScript error.
- The `entry` prop is passed down from the `[...slug].astro` page route (P02T06) via `getStaticPaths`.
- `SkillSidebar` receives `entry` to access `entry.data.tilePublishedUrl`, `entry.data.tileVersion`, and `entry.data.grade`.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -20
# Expect: zero lines referencing starlightRoute
grep -r "starlightRoute" src/layouts/ && echo "FAIL: starlightRoute still referenced" || echo "ok"
```
