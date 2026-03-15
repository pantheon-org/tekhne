import { describe, expect, test } from "bun:test";
import { parsePublishedStatus } from "./parse-published-status";

describe("parsePublishedStatus", () => {
  test("returns 'public' when private is false", () => {
    expect(parsePublishedStatus({ private: false })).toBe("public");
  });

  test("returns 'private' when private is true", () => {
    expect(parsePublishedStatus({ private: true })).toBe("private");
  });

  test("returns 'unpublished' when private is undefined", () => {
    expect(parsePublishedStatus({})).toBe("unpublished");
  });

  test("returns 'unpublished' when private is a string", () => {
    expect(parsePublishedStatus({ private: "yes" })).toBe("unpublished");
  });

  test("returns 'unpublished' when private is null", () => {
    expect(parsePublishedStatus({ private: null })).toBe("unpublished");
  });
});
