import { describe, expect, test } from "bun:test";
import { getDocsBadgeMarkdown } from "./get-docs-badge-markdown";

describe("getDocsBadgeMarkdown", () => {
  test("returns a CSS span for a known grade", () => {
    expect(getDocsBadgeMarkdown("A")).toBe(
      '<span class="skill-badge skill-badge--a">A</span>',
    );
  });

  test("encodes plus as -plus in the CSS class", () => {
    const md = getDocsBadgeMarkdown("B+");
    expect(md).toBe('<span class="skill-badge skill-badge--b-plus">B+</span>');
  });

  test("question mark grade renders as unknown CSS class", () => {
    expect(getDocsBadgeMarkdown("?")).toBe(
      '<span class="skill-badge skill-badge--unknown">?</span>',
    );
  });
});
