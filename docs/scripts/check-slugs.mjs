#!/usr/bin/env node
// Dry-run: reads the preprocessed content directory and lists all slug mappings.
// Run AFTER prelink.mjs has copied skills/ → docs/src/content/docs/skills/

import { readdir } from "node:fs/promises";
import { join } from "node:path";

const ROOT = new URL("../src/content/docs/", import.meta.url).pathname;

async function walk(dir, base = "") {
  const entries = await readdir(dir, { withFileTypes: true });
  const results = [];
  for (const e of entries) {
    const rel = base ? `${base}/${e.name}` : e.name;
    if (e.isDirectory()) {
      results.push(...(await walk(join(dir, e.name), rel)));
    } else if (e.name.endsWith(".md") || e.name.endsWith(".mdx")) {
      const id = rel.replace(/\.(md|mdx)$/, "");
      const slug = id.replace(/^skills\//, "");
      results.push({ id, slug });
    }
  }
  return results;
}

const pages = await walk(ROOT);
const slugs = pages.map((p) => p.slug);
const unique = new Set(slugs);

console.log(`Total pages: ${pages.length}`);
console.log(`Unique slugs: ${unique.size}`);
if (unique.size !== pages.length) {
  console.error("COLLISION DETECTED:");
  const seen = new Map();
  for (const p of pages) {
    if (seen.has(p.slug)) {
      console.error(`  ${p.slug} (${p.id} vs ${seen.get(p.slug)})`);
    }
    seen.set(p.slug, p.id);
  }
  process.exit(1);
}
console.log("No collisions.");
