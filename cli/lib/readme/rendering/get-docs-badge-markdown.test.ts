import { describe, expect, test } from "bun:test";
import { getDocsBadgeMarkdown } from "./get-docs-badge-markdown";

describe("getDocsBadgeMarkdown", () => {
  test("returns absolute path badge for a known grade", () => {
    expect(getDocsBadgeMarkdown("A")).toBe(
      "![A](/tekhne/.github/badges/A.svg)",
    );
  });

  test("encodes plus in filename", () => {
    const md = getDocsBadgeMarkdown("B+");
    expect(md).toContain("![B+]");
    expect(md).toContain("B-plus.svg");
    expect(md).toContain("/tekhne/.github/badges/");
  });

  test("question mark grade renders as unknown filename", () => {
    expect(getDocsBadgeMarkdown("?")).toBe(
      "![?](/tekhne/.github/badges/unknown.svg)",
    );
  });
});
