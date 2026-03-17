# P01T07 — Create `LeftNav.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/components/LeftNav.astro` that server-renders a domain-grouped skill tree from the `docs` collection, with `localStorage`-persisted collapsed/expanded state per domain group and force-expansion of the active page's ancestor.

## File to create / modify

```
docs/src/components/LeftNav.astro
```

## Implementation

```astro
---
import { getCollection } from "astro:content";

interface Props {
  currentPath: string;
}
const { currentPath } = Astro.props;

const entries = await getCollection("docs");

type NavNode = {
  domain: string;
  slug: string;
  title: string;
  href: string;
};

const base = import.meta.env.BASE_URL.replace(/\/$/, "");

const nodes: NavNode[] = entries
  .filter((e) => e.id.endsWith("/skill"))
  .map((e) => {
    const parts = e.id.split("/");
    const domain = parts[1] ?? "other";
    const slug = e.id.replace(/^skills\//, "").replace(/\/skill$/, "");
    return {
      domain,
      slug,
      title: e.data.title,
      href: `${base}/skills/${slug}/`,
    };
  })
  .sort((a, b) => a.title.localeCompare(b.title));

const domains = [...new Set(nodes.map((n) => n.domain))].sort();

const grouped = Object.fromEntries(
  domains.map((d) => [d, nodes.filter((n) => n.domain === d)])
);
---

<nav class="left-nav" id="left-nav" aria-label="Skill navigation">
  {domains.map((domain) => {
    const items = grouped[domain];
    const isActive = items.some((item) => currentPath.startsWith(item.href));
    return (
      <div class="nav-group" data-domain={domain}>
        <button
          class="nav-group-toggle"
          aria-expanded={isActive ? "true" : "false"}
          data-domain-key={`tk-nav-${domain}`}
        >
          <span class="nav-group-label">{domain}</span>
          <span class="nav-chevron" aria-hidden="true">▾</span>
        </button>
        <ul class="nav-group-items">
          {items.map((item) => (
            <li>
              <a
                href={item.href}
                class={`nav-link${currentPath.startsWith(item.href) ? " nav-link--active" : ""}`}
                aria-current={currentPath.startsWith(item.href) ? "page" : undefined}
              >
                {item.title}
              </a>
            </li>
          ))}
        </ul>
      </div>
    );
  })}
</nav>

<script>
  const nav = document.getElementById("left-nav");
  if (nav) {
    nav.querySelectorAll<HTMLButtonElement>(".nav-group-toggle").forEach((btn) => {
      const key = btn.dataset.domainKey!;
      const group = btn.closest(".nav-group")!;
      const list = group.querySelector<HTMLElement>(".nav-group-items")!;
      const isActive = group.querySelector(".nav-link--active") !== null;

      const stored = localStorage.getItem(key);
      const expanded = isActive || stored === "true";
      btn.setAttribute("aria-expanded", String(expanded));
      list.style.display = expanded ? "" : "none";

      btn.addEventListener("click", () => {
        const next = btn.getAttribute("aria-expanded") !== "true";
        btn.setAttribute("aria-expanded", String(next));
        list.style.display = next ? "" : "none";
        localStorage.setItem(key, String(next));
      });

      btn.addEventListener("keydown", (e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          btn.click();
        }
      });
    });
  }
</script>

<style>
  .left-nav {
    border-right: 1px solid var(--tk-border);
    padding: 1rem 0;
    overflow-y: auto;
    background: var(--tk-bg-surface);
  }

  .nav-group-toggle {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.4rem 1rem;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--tk-text-muted);
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    gap: 0.5rem;
  }

  .nav-group-label {
    flex: 1;
    text-align: left;
  }

  .nav-group-items {
    list-style: none;
    padding: 0;
  }

  .nav-link {
    display: block;
    padding: 0.3rem 1rem 0.3rem 1.5rem;
    color: var(--tk-text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    border-left: 2px solid transparent;
  }

  .nav-link:hover {
    color: var(--tk-text);
    background: var(--tk-bg-subtle);
  }

  .nav-link--active {
    color: var(--tk-accent);
    border-left-color: var(--tk-accent);
    font-weight: 500;
  }
</style>
```

## Notes

- Port `buildSidebarNode` / `buildDomainSidebar` logic from `astro.config.mjs` rather than reimplementing from scratch.
- Active ancestors are always force-expanded regardless of stored `localStorage` state (see `isActive` check in script).
- The `currentPath` prop is `Astro.url.pathname` from the parent layout — do not read `window.location` server-side.

## Verification

```sh
test -f docs/src/components/LeftNav.astro
grep 'localStorage' docs/src/components/LeftNav.astro
grep 'aria-expanded' docs/src/components/LeftNav.astro
```
