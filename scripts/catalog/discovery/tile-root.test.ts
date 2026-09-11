import { describe, expect, test } from "bun:test";
import { tileRoot } from "./tile-root";

describe("tileRoot", () => {
  test("unwraps the .tessl-plugin directory", () => {
    expect(
      tileRoot("skills/repository-mgmt/nx/.tessl-plugin/plugin.json"),
    ).toBe("skills/repository-mgmt/nx");
  });

  test("unwraps a nested child tile manifest", () => {
    expect(
      tileRoot("skills/infrastructure/terraform/.tessl-plugin/plugin.json"),
    ).toBe("skills/infrastructure/terraform");
  });

  test("returns the containing directory for a legacy tile.json", () => {
    expect(tileRoot("skills/ci-cd/github-actions/tile.json")).toBe(
      "skills/ci-cd/github-actions",
    );
  });

  test("does not unwrap a directory merely containing the marker name", () => {
    expect(tileRoot("skills/foo/.tessl-plugin-backup/plugin.json")).toBe(
      "skills/foo/.tessl-plugin-backup",
    );
  });
});
