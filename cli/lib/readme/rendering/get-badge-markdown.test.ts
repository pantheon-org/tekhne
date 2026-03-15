import { describe, expect, test } from "bun:test";
import { getBadgeMarkdown } from "./get-badge-markdown";

describe("getBadgeMarkdown", () => {
  test("returns local SVG badge markdown for a known grade", () => {
    expect(getBadgeMarkdown("A")).toBe("![A](.github/badges/A.svg)");
  });

  test("uses grade in alt text and encodes plus in filename", () => {
    const md = getBadgeMarkdown("B+");
    expect(md).toContain("![B+]");
    expect(md).toContain("B-plus.svg");
  });

  test("unknown grade renders as unknown filename", () => {
    expect(getBadgeMarkdown("Z")).toBe("![Z](.github/badges/Z.svg)");
  });

  test("question mark grade renders as unknown filename", () => {
    expect(getBadgeMarkdown("?")).toBe("![?](.github/badges/unknown.svg)");
  });
});
