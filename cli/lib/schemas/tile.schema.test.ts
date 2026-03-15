import { describe, expect, test } from "bun:test";
import { TileSchema, TileSkillSchema } from "./tile.schema";

describe("TileSkillSchema", () => {
  test("parses valid skill", () => {
    expect(() =>
      TileSkillSchema.parse({ name: "my-skill", description: "Does things" }),
    ).not.toThrow();
  });

  test("rejects empty name", () => {
    expect(() =>
      TileSkillSchema.parse({ name: "", description: "x" }),
    ).toThrow();
  });
});

describe("TileSchema", () => {
  const valid = {
    name: "org/my-tile",
    version: "1.0.0",
    summary: "A tile",
  };

  test("parses valid tile", () => {
    expect(() => TileSchema.parse(valid)).not.toThrow();
  });

  test("rejects name without slash", () => {
    expect(() => TileSchema.parse({ ...valid, name: "notvalid" })).toThrow();
  });

  test("rejects non-semver version", () => {
    expect(() => TileSchema.parse({ ...valid, version: "v1.0" })).toThrow();
  });

  test("accepts optional private field", () => {
    expect(() => TileSchema.parse({ ...valid, private: true })).not.toThrow();
  });
});
