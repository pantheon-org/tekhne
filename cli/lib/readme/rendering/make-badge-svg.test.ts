import { describe, expect, test } from "bun:test";
import { makeBadgeSvg } from "./make-badge-svg";

describe("makeBadgeSvg", () => {
  test("returns an SVG string for a known grade", () => {
    const svg = makeBadgeSvg("A");
    expect(svg).toContain("<svg");
    expect(svg).toContain("Rating");
    expect(svg).toContain("A");
  });

  test("returns an SVG string for an unknown grade", () => {
    const svg = makeBadgeSvg("Z");
    expect(svg).toContain("<svg");
  });
});
