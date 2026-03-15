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
      // Skill entry point: copy with reference link rewriting
      const raw = readFileSync(srcPath, "utf-8");
      writeFileSync(destPath, rewriteReferenceLinks(raw));
    }
    // All other files are intentionally skipped.
  }
};

// Fully regenerate the destination directory on every run.
rmSync(destRoot, { recursive: true, force: true });
copyDir(skillsRoot, destRoot);

console.log(`Skills copied and transformed → ${destRoot}`);
