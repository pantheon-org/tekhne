import { describe, expect, test } from "bun:test";
import { readPackageSection } from "../../../docs/src/lib/crate-versions";

describe("readPackageSection", () => {
  test("reads name and version from the [package] section", () => {
    const toml = [
      "[package]",
      'name = "pantheon-skill-auditor"',
      'version = "0.2.0"',
      "edition.workspace = true",
      "",
      "[package.metadata.dist]",
      "dist = true",
    ].join("\n");

    expect(readPackageSection(toml)).toEqual({
      name: "pantheon-skill-auditor",
      version: "0.2.0",
    });
  });

  test("ignores a version declared in a later section", () => {
    const toml = [
      "[package]",
      'name = "pantheon-adr"',
      'version = "0.2.0"',
      "",
      "[dependencies]",
      'version = "9.9.9"',
    ].join("\n");

    expect(readPackageSection(toml).version).toBe("0.2.0");
  });

  test("returns nothing when there is no [package] section", () => {
    expect(readPackageSection('[workspace]\nmembers = ["a"]')).toEqual({});
  });
});
