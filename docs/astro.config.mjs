// @ts-check

import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";
import matter from "gray-matter";

const __dirname = dirname(fileURLToPath(import.meta.url));
const skillsRoot = resolve(__dirname, "../skills");

/**
 * Recursively collect sidebar items from all SKILL.md files under a directory.
 * @param {string} dir
 * @returns {{ label: string, slug: string }[]}
 */
const collectSkillItems = (dir) => {
  /** @type {{ label: string, slug: string }[]} */
  const items = [];
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return items;
  }
  for (const entry of entries) {
    if (entry.isDirectory()) {
      items.push(...collectSkillItems(join(dir, entry.name)));
    } else if (entry.name === "SKILL.md") {
      const fullPath = join(dir, entry.name);
      const raw = readFileSync(fullPath, "utf-8");
      const { data } = matter(raw);
      const label = /** @type {string} */ (
        data.name ?? data.title ?? "Untitled"
      );
      // Slug matches the lowercased URL path: skills/<domain>/…/<tool>/skill
      const rel = relative(skillsRoot, dir).toLowerCase().replace(/\\/g, "/");
      items.push({ label, slug: `skills/${rel}/skill` });
    }
  }
  return items;
};

/**
 * Build a flat, alphabetically-sorted sidebar group for a domain.
 * @param {string} domain  - directory name under skills/ (e.g. "ci-cd")
 * @param {string} label   - human-readable group label
 */
const buildDomainSidebar = (domain, label) => {
  const items = collectSkillItems(join(skillsRoot, domain));
  items.sort((a, b) => a.label.localeCompare(b.label));
  return { label, items, collapsed: true };
};

// https://astro.build/config
export default defineConfig({
  site: "https://pantheon-org.github.io",
  base: "/tekhne",
  integrations: [
    starlight({
      title: "Tekhne Skills",
      description:
        "A curated collection of reusable agent skills for AI assistants.",
      logo: {
        src: "./src/assets/houston.webp",
        alt: "Tekhne",
      },
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/pantheon-org/tekhne",
        },
      ],
      editLink: {
        // Base must include the docs/ subdirectory so edit links resolve to
        // the correct file path within the monorepo on GitHub.
        baseUrl: "https://github.com/pantheon-org/tekhne/edit/main/docs/",
      },
      customCss: ["./src/styles/custom.css"],
      sidebar: [
        {
          label: "Overview",
          items: [{ label: "All Skills", slug: "tiles" }],
        },
        buildDomainSidebar("agentic-harness", "Agentic Harness"),
        buildDomainSidebar("ci-cd", "CI / CD"),
        buildDomainSidebar("development", "Development"),
        buildDomainSidebar("documentation", "Documentation"),
        buildDomainSidebar("infrastructure", "Infrastructure"),
        buildDomainSidebar("observability", "Observability"),
        buildDomainSidebar("package-mgmt", "Package Management"),
        buildDomainSidebar("project-mgmt", "Project Management"),
        buildDomainSidebar("repository-mgmt", "Repository Management"),
        buildDomainSidebar("software-engineering", "Software Engineering"),
        buildDomainSidebar("specialized", "Specialized"),
        buildDomainSidebar("testing", "Testing"),
      ],
    }),
  ],
});
