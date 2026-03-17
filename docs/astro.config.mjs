// @ts-check

import { readdirSync, readFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";
import matter from "gray-matter";

const __dirname = dirname(fileURLToPath(import.meta.url));
const skillsRoot = resolve(__dirname, "../skills");

/** Convert a kebab-case directory name to Title Case. */
const toTitleCase = (str) =>
  str
    .split("-")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");

/**
 * Recursively build a sidebar node for a directory.
 * - Returns a leaf `{ label, slug }` when the directory contains SKILL.md.
 * - Returns a group `{ label, items, collapsed }` for intermediate directories.
 * - Returns null when no SKILL.md files exist anywhere below.
 *
 * @param {string} dir  - absolute path to the directory
 * @returns {{ label: string, slug?: string, items?: unknown[], collapsed?: boolean } | null}
 */
const buildSidebarNode = (dir) => {
  /** @type {import("node:fs").Dirent[]} */
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return null;
  }

  // Leaf: directory directly contains SKILL.md
  const hasSkill = entries.some((e) => e.isFile() && e.name === "SKILL.md");
  if (hasSkill) {
    const raw = readFileSync(join(dir, "SKILL.md"), "utf-8");
    const { data } = matter(raw);
    const label = /** @type {string} */ (
      data.name ?? data.title ?? toTitleCase(basename(dir))
    );
    const rel = relative(skillsRoot, dir).toLowerCase().replace(/\\/g, "/");
    return { label, slug: `skills/${rel}/skill` };
  }

  // Group: recurse into sub-directories, skipping non-navigable content folders.
  const skip = new Set([
    "references",
    "templates",
    "scripts",
    "schemas",
    "assets",
    "evals",
    "checklist",
    "template",
  ]);
  const subDirs = entries.filter(
    (e) => e.isDirectory() && !e.name.startsWith(".") && !skip.has(e.name),
  );
  const items = subDirs
    .map((e) => buildSidebarNode(join(dir, e.name)))
    .filter(Boolean);

  if (items.length === 0) return null;

  items.sort((a, b) => (a?.label ?? "").localeCompare(b?.label ?? ""));
  return { label: toTitleCase(basename(dir)), items, collapsed: true };
};

/**
 * Build a nested, alphabetically-sorted sidebar group for a domain.
 * @param {string} domain  - directory name under skills/ (e.g. "ci-cd")
 * @param {string} label   - human-readable group label
 */
const buildDomainSidebar = (domain, label) => {
  const domainDir = join(skillsRoot, domain);
  let entries;
  try {
    entries = readdirSync(domainDir, { withFileTypes: true });
  } catch {
    return { label, items: [], collapsed: true };
  }

  const skip = new Set([
    "references",
    "templates",
    "scripts",
    "schemas",
    "assets",
    "evals",
  ]);
  const items = entries
    .filter(
      (e) => e.isDirectory() && !e.name.startsWith(".") && !skip.has(e.name),
    )
    .map((e) => buildSidebarNode(join(domainDir, e.name)))
    .filter(Boolean);

  items.sort((a, b) => (a?.label ?? "").localeCompare(b?.label ?? ""));
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
      components: {
        PageTitle: "./src/components/SkillPageTitle.astro",
        MarkdownContent: "./src/components/SkillTabs.astro",
        PageSidebar: "./src/components/SkillSidebar.astro",
      },
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
        {
          label: "References",
          items: [
            { label: "tessl.io", link: "https://tessl.io/registry" },
            { label: "vercel skills", link: "https://skills.sh/" },
          ],
        },
      ],
    }),
  ],
});
