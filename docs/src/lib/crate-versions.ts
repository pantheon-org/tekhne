import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const CRATES_ROOT = resolve(process.cwd(), "../crates");

/**
 * Read `name` and `version` out of a Cargo manifest's [package] section,
 * stopping at the next section header so a later [package.metadata.*] table
 * cannot contribute keys.
 */
export const readPackageSection = (
  toml: string,
): { name?: string; version?: string } => {
  const result: { name?: string; version?: string } = {};
  let inPackage = false;

  for (const line of toml.split("\n")) {
    const trimmed = line.trim();
    if (trimmed.startsWith("[")) {
      if (inPackage) break;
      inPackage = trimmed === "[package]";
      continue;
    }
    if (!inPackage) continue;
    result.name ??= trimmed.match(/^name = "(.+)"$/)?.[1];
    result.version ??= trimmed.match(/^version = "(.+)"$/)?.[1];
  }

  return result;
};

/**
 * Cargo package version keyed by package name, read from the workspace crates.
 *
 * The tool install URLs are built from these versions, and a release tag is
 * `tool/<package>-v<version>`. A hand-maintained copy here silently 404s the
 * moment a crate is renamed or released, which is exactly what happened when
 * the binaries were namespaced to `pantheon-*`.
 */
export const crateVersions = (): Map<string, string> => {
  const versions = new Map<string, string>();
  if (!existsSync(CRATES_ROOT)) return versions;

  for (const dir of readdirSync(CRATES_ROOT)) {
    const manifest = join(CRATES_ROOT, dir, "Cargo.toml");
    if (!existsSync(manifest)) continue;
    const { name, version } = readPackageSection(
      readFileSync(manifest, "utf-8"),
    );
    if (name && version) versions.set(name, version);
  }

  return versions;
};
