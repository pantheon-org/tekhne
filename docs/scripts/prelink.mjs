#!/usr/bin/env node
/**
 * prelink.mjs  —  copy-and-transform
 *
 * Copies skills/ content into docs/src/content/docs/skills/ with two
 * transformations applied on the fly:
 *
 *   SKILL.md       — rewrites `references/foo.md` links to `../references/foo`
 *                    so they resolve from the rendered `…/skill/` page URL.
 *
 *   references/*.md — injects Starlight-required frontmatter:
 *                      title          (extracted from the first H1, or filename)
 *                      sidebar.hidden (true — keeps reference pages off the sidebar)
 *
 * All other files (AGENTS.md, tile.json, templates/, scripts/, schemas/ …)
 * are intentionally omitted to keep the built docs directory lean.
 *
 * The destination (docs/src/content/docs/skills/) is git-ignored and fully
 * regenerated on every run.
 */

import {
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import matter from "gray-matter";

const __dirname = dirname(fileURLToPath(import.meta.url));
const skillsRoot = resolve(__dirname, "../../skills");
const destRoot = resolve(__dirname, "../src/content/docs/skills");
const auditsRoot = resolve(__dirname, "../../.context/audits");

/** Directories inside a skill that contain non-renderable content. */
const SKIP_DIRS = new Set([
  "templates",
  "scripts",
  "schemas",
  "assets",
  "evals",
  "checklist",
  "template",
]);

/** Convert a kebab-case filename to Title Case. */
const toTitleCase = (str) =>
  str
    .split("-")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");

/** Extract the text of the first H1 heading from markdown content. */
const extractH1 = (content) => {
  const match = content.match(/^#\s+(.+)$/m);
  return match ? match[1].trim() : null;
};

/**
 * Rewrite  `[label](references/foo.md)`  →  `[label](../references/foo)`
 * so links resolve correctly when the page is served from `…/skill/`.
 */
const rewriteReferenceLinks = (content) =>
  content.replace(
    /\[([^\]]*)\]\(references\/([^)]+)\.md\)/g,
    "[$1](../references/$2)",
  );

/**
 * For SKILL.md files:
 *   1. Extracts the first H1 heading text and stores it as `metadata.displayTitle`
 *      so the custom SkillPageTitle component can render it as the canonical heading.
 *   2. Strips that H1 from the markdown content to avoid a duplicate h1 on the page.
 *   3. Embeds `skillRefs` (reference list) and `skillAudit` (audit data) into frontmatter.
 *
 * @param {string} raw - raw SKILL.md content
 * @param {string} skillSrcDir - absolute path to the skill source directory
 * @param {string} skillRelPath - relative path from skills root, e.g. "documentation/markdown-authoring"
 */
const processSkillContent = (raw, skillSrcDir, skillRelPath) => {
  const parsed = matter(raw);

  // Match the first H1 at the start of the content (possibly after blank lines)
  const h1Match = parsed.content.match(/^\s*# ([^\n]+)/);
  if (!h1Match) return raw; // no leading H1 — leave unchanged

  const displayTitle = h1Match[1].trim();

  // Strip the H1 line (and a single following blank line if present)
  const strippedContent = parsed.content.replace(/^\s*# [^\n]+\n(\n)?/, "");

  const skillRefs = skillSrcDir
    ? buildReferencesList(skillSrcDir, skillRelPath)
    : [];
  const skillAudit = skillRelPath ? loadAuditData(skillRelPath) : null;
  const skillAudits = skillRelPath ? loadAllAuditSnapshots(skillRelPath) : [];
  const skillEvals = skillSrcDir ? loadEvalsData(skillSrcDir) : [];

  // Merge displayTitle into metadata (preserving any existing metadata fields)
  const newData = {
    ...parsed.data,
    metadata: { ...(parsed.data.metadata || {}), displayTitle },
    ...(skillRefs.length > 0 ? { skillRefs } : {}),
    ...(skillAudit ? { skillAudit } : {}),
    ...(skillAudits.length > 0 ? { skillAudits } : {}),
    ...(skillEvals.length > 0 ? { skillEvals } : {}),
  };

  return matter.stringify(strippedContent, newData);
};

/**
 * Escape `<` characters in flowing text that MDX would misinterpret as JSX.
 *
 * MDX parses `<letter…>`, `</…>`, `<digit…>`, `<https://…>` etc. as JSX
 * or namespaced elements. SKILL.md files use angle brackets for placeholders
 * (`<TARGET_URL>`), URL autolinks (`<https://…>`), and comparison operators
 * (`<80%`) — none of which are valid MDX JSX syntax.
 *
 * Strategy: escape ALL `<` outside of fenced code blocks and inline code spans.
 * SKILL.md files do not use raw HTML in flowing text, so this is safe.
 */
const escapeMdxText = (raw) => {
  const parsed = matter(raw);
  let inFence = false;

  const escapeLineAngles = (line) => {
    // Walk character-by-character; skip content inside inline backtick spans
    let out = "";
    let inCode = false;
    for (let i = 0; i < line.length; i++) {
      const ch = line[i];
      if (ch === "`") {
        inCode = !inCode;
        out += ch;
      } else if (!inCode && ch === "<") {
        out += "&lt;";
      } else {
        out += ch;
      }
    }
    return out;
  };

  const escaped = parsed.content
    .split("\n")
    .map((line) => {
      if (/^```/.test(line)) {
        inFence = !inFence;
        return line;
      }
      if (inFence) return line;
      return escapeLineAngles(line);
    })
    .join("\n");

  return matter.stringify(escaped, parsed.data);
};

/**
 * Build a list of reference entries for a skill directory.
 * Returns an array of { slug, title } objects, or an empty array if none exist.
 * @param {string} skillSrcDir - absolute path to the skill's source directory
 * @param {string} skillRelPath - e.g. "documentation/markdown-authoring"
 */
const buildReferencesList = (skillSrcDir, skillRelPath) => {
  const refsDir = join(skillSrcDir, "references");
  if (!existsSync(refsDir)) return [];

  const refs = [];
  for (const ref of readdirSync(refsDir, { withFileTypes: true })) {
    if (!ref.isFile() || !ref.name.endsWith(".md")) continue;
    const raw = readFileSync(join(refsDir, ref.name), "utf-8");
    const parsed = matter(raw);
    const baseName = ref.name.replace(/\.md$/, "");
    const title =
      parsed.data.title ?? extractH1(parsed.content) ?? toTitleCase(baseName);
    refs.push({ slug: baseName, title });
  }
  refs.sort((a, b) => a.slug.localeCompare(b.slug));
  return refs;
};

/**
 * Load all audit snapshots for a skill from .context/audits/<domain>/<skill>/.
 * Returns an array of { date, ...auditData } objects sorted by date descending,
 * or an empty array if none exist.
 * @param {string} skillRelPath - e.g. "documentation/markdown-authoring"
 */
const loadAllAuditSnapshots = (skillRelPath) => {
  const auditSkillDir = join(auditsRoot, skillRelPath);
  if (!existsSync(auditSkillDir)) return [];

  const snapshots = [];
  let entries;
  try {
    entries = readdirSync(auditSkillDir, { withFileTypes: true });
  } catch {
    return [];
  }

  for (const entry of entries) {
    if (!entry.isDirectory() || entry.name === "latest") continue;
    const auditPath = join(auditSkillDir, entry.name, "audit.json");
    if (!existsSync(auditPath)) continue;
    try {
      const data = JSON.parse(readFileSync(auditPath, "utf-8"));
      snapshots.push({ date: entry.name, ...data });
    } catch {
      // skip malformed audit files
    }
  }

  snapshots.sort((a, b) => b.date.localeCompare(a.date));
  return snapshots;
};

/**
 * Load audit data for a skill from .context/audits/<domain>/<skill>/latest/audit.json.
 * Returns the parsed JSON object, or null if not found.
 * @param {string} skillRelPath - e.g. "documentation/markdown-authoring"
 */
const loadAuditData = (skillRelPath) => {
  const auditPath = join(auditsRoot, skillRelPath, "latest", "audit.json");
  if (!existsSync(auditPath)) return null;
  try {
    return JSON.parse(readFileSync(auditPath, "utf-8"));
  } catch {
    return null;
  }
};

/**
 * Load evals data for a skill from skills/<domain>/<skill>/evals/.
 * Returns an array of { id, capability, feasible } objects from summary.json,
 * or an empty array if evals don't exist.
 * @param {string} skillSrcDir - absolute path to the skill's source directory
 */
const loadEvalsData = (skillSrcDir) => {
  const evalsDir = join(skillSrcDir, "evals");
  if (!existsSync(evalsDir)) return [];

  const summaryPath = join(evalsDir, "summary.json");
  if (!existsSync(summaryPath)) return [];

  try {
    const data = JSON.parse(readFileSync(summaryPath, "utf-8"));
    if (!Array.isArray(data.scenarios)) return [];
    return data.scenarios.map((s) => ({
      ...s,
      id: s.id ?? s.scenario_id ?? "unknown",
    }));
  } catch {
    return [];
  }
};

/**
 * Ensure a reference file has `title` and `sidebar.hidden: true` frontmatter,
 * which Starlight requires for rendering.  Source files are never modified.
 */
const withReferenceFrontmatter = (content, fileName) => {
  const parsed = matter(content);

  if (parsed.data.title && parsed.data.sidebar?.hidden === true) {
    return content; // already complete
  }

  const derivedTitle =
    parsed.data.title ??
    extractH1(parsed.content) ??
    toTitleCase(fileName.replace(/\.md$/, ""));

  const newData = {
    ...parsed.data,
    title: derivedTitle,
    sidebar: { hidden: true },
  };

  return matter.stringify(parsed.content, newData);
};

/**
 * Recursively copy a skills directory, transforming content as needed.
 * @param {string} src
 * @param {string} dest
 */
// biome-ignore lint/complexity/noExcessiveCognitiveComplexity: recursive directory walker with intentional branching
const copyDir = (src, dest) => {
  mkdirSync(dest, { recursive: true });

  for (const entry of readdirSync(src, { withFileTypes: true })) {
    if (entry.name.startsWith(".")) continue;

    const srcPath = join(src, entry.name);
    const destPath = join(dest, entry.name);

    if (entry.isDirectory()) {
      if (entry.name === "references") {
        // Reference docs: copy with frontmatter injection
        mkdirSync(destPath, { recursive: true });
        for (const ref of readdirSync(srcPath, { withFileTypes: true })) {
          if (!ref.isFile() || !ref.name.endsWith(".md")) continue;
          const raw = readFileSync(join(srcPath, ref.name), "utf-8");
          writeFileSync(
            join(destPath, ref.name),
            withReferenceFrontmatter(raw, ref.name),
          );
        }
      } else if (!SKIP_DIRS.has(entry.name)) {
        // Regular skill subdirectory: recurse
        copyDir(srcPath, destPath);
      }
    } else if (entry.isFile() && entry.name === "SKILL.md") {
      // Skill entry point: rewrite reference links, extract displayTitle, strip H1.
      // Written as SKILL.mdx so Astro treats it as MDX (enables JSX/component use).
      const raw = readFileSync(srcPath, "utf-8");
      const skillRelPath = src.startsWith(skillsRoot)
        ? src.slice(skillsRoot.length + 1).replace(/\\/g, "/")
        : undefined;
      writeFileSync(
        join(dest, "SKILL.mdx"),
        escapeMdxText(
          processSkillContent(rewriteReferenceLinks(raw), src, skillRelPath),
        ),
      );
    }
    // All other files are intentionally skipped.
  }
};

// Fully regenerate the destination directory on every run.
rmSync(destRoot, { recursive: true, force: true });
copyDir(skillsRoot, destRoot);

console.log(`Skills copied and transformed → ${destRoot}`);
