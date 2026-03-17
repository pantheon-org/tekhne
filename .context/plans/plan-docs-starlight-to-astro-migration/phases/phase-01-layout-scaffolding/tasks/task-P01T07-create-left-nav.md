# P01T07 — Create `src/components/LeftNav.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/components/LeftNav.astro` — a server-side-rendered domain-grouped
skill navigation tree with client-side localStorage persistence and active-link
highlighting.

## File to create / modify

```
docs/src/components/LeftNav.astro
```

## Implementation

```astro
---
import { getCollection } from "astro:content";

const entries = await getCollection("docs");
const currentPath = Astro.url.pathname;

interface NavItem {
  label: string;
  href: string;
  active: boolean;
}

interface NavGroup {
  domain: string;
  slug: string;
  items: NavItem[];
}

const groups: NavGroup[] = [];
const domainMap = new Map<string, NavItem[]>();

for (const entry of entries) {
  if (!entry.id.endsWith("/skill")) continue;
  const parts = entry.id.split("/");
  const domain = parts[1] ?? "unknown";
  const href = `${import.meta.env.BASE_URL}/${entry.id.replace(/^skills\//, "")}`;
  const active = currentPath.startsWith(href);
  if (!domainMap.has(domain)) domainMap.set(domain, []);
  domainMap.get(domain)!.push({ label: entry.data.title, href, active });
}

for (const [domain, items] of domainMap) {
  groups.push({ domain, slug: domain.replace(/\s+/g, "-").toLowerCase(), items });
}
groups.sort((a, b) => a.domain.localeCompare(b.domain));
---

<nav id="left-nav" aria-label="Skill navigation">
  {groups.map((group) => {
    const hasActive = group.items.some((i) => i.active);
    return (
      <details
        data-domain={group.slug}
        open={hasActive || undefined}
      >
        <summary
          tabindex="0"
          role="button"
        >
          {group.domain}
        </summary>
        <ul>
          {group.items.map((item) => (
            <li>
              <a
                href={item.href}
                aria-current={item.active ? "page" : undefined}
              >
                {item.label}
              </a>
            </li>
          ))}
        </ul>
      </details>
    );
  })}
</nav>

<script>
  const nav = document.getElementById("left-nav");
  if (nav) {
    nav.querySelectorAll("details[data-domain]").forEach((details) => {
      const el = details as HTMLDetailsElement;
      const key = `tk-nav-${el.dataset.domain}`;
      const stored = localStorage.getItem(key);
      // Force-open if active page is inside; otherwise restore stored state
      const hasActive = el.querySelector("[aria-current='page']");
      if (!hasActive && stored !== null) {
        el.open = stored === "true";
      }
      el.addEventListener("toggle", () => {
        if (!el.querySelector("[aria-current='page']")) {
          localStorage.setItem(key, String(el.open));
        }
      });
    });
  }
</script>

<style>
  nav {
    border-right: 1px solid var(--tk-border);
    padding: 1rem 0;
    overflow-y: auto;
    background: var(--tk-bg-surface);
  }

  details > summary {
    padding: 0.4rem 1rem;
    cursor: pointer;
    font-weight: 600;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--tk-text-muted);
    list-style: none;
  }

  details > summary::-webkit-details-marker { display: none; }

  ul {
    list-style: none;
    padding: 0;
  }

  a {
    display: block;
    padding: 0.3rem 1rem 0.3rem 1.5rem;
    font-size: 0.875rem;
    color: var(--tk-text);
  }

  a:hover {
    background: var(--tk-bg-hover);
    text-decoration: none;
  }

  a[aria-current="page"] {
    color: var(--tk-accent);
    background: var(--tk-bg-hover);
    font-weight: 500;
  }
</style>
```

## Notes

- Ports `buildSidebarNode` / `buildDomainSidebar` logic from `astro.config.mjs` into a component.
- Uses native `<details>`/`<summary>` for collapse state — no JS required for the initial open/closed render.
- Active ancestor is force-opened server-side via the `hasActive` check, ensuring correct SSR output.
- `localStorage` key per domain slug: `tk-nav-<slug>`.

## Verification

```sh
test -f docs/src/components/LeftNav.astro && echo ok
grep "aria-current" docs/src/components/LeftNav.astro
grep "localStorage" docs/src/components/LeftNav.astro
```
