import { describe, expect, test } from "bun:test";
import { getBadgeMarkdown } from "./get-badge-markdown";

describe("getBadgeMarkdown", () => {
  test("returns shield.io badge markdown for a known grade", () => {
    expect(getBadgeMarkdown("A")).toBe(
      "![A](https://img.shields.io/badge/Rating-A-green)",
    );
  });

  test("uses grade in both alt text and badge label", () => {
    const md = getBadgeMarkdown("B+");
    expect(md).toContain("![B+]");
    expect(md).toContain("Rating-B+");
  });

  test("unknown grade falls back to lightgrey color", () => {
    expect(getBadgeMarkdown("Z")).toContain("-lightgrey)");
  });
});
