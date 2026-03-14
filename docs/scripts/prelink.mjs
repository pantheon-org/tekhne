#!/usr/bin/env node
/**
 * prelink.mjs
 *
 * Creates the symlink docs/src/content/docs/skills → ../../../../skills
 * so Astro Starlight can render all SKILL.md pages without copying files.
 *
 * Uses 'junction' type on Windows (no admin rights required) and 'dir'
 * type on POSIX. Safe to run repeatedly — removes a stale link first.
 */

import { existsSync, symlinkSync, unlinkSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const target = resolve(__dirname, "../src/content/docs/skills");
const source = "../../../../skills";
const type = process.platform === "win32" ? "junction" : "dir";

if (existsSync(target)) {
  unlinkSync(target);
}

symlinkSync(source, target, type);
console.log(`Symlink created: ${target} → ${source}`);
