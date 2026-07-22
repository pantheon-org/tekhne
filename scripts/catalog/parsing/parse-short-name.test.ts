import { describe, expect, test } from "bun:test";
import { parseShortName } from "./parse-short-name";

describe("parseShortName", () => {
  test("strips org prefix when fullName contains a slash", () => {
    expect(parseShortName("pantheon-ai/terraform-toolkit")).toBe(
      "terraform-toolkit",
    );
  });

  test("returns fullName unchanged when no slash present", () => {
    expect(parseShortName("terraform-toolkit")).toBe("terraform-toolkit");
  });

  test("handles multiple slashes by stripping only the first segment", () => {
    expect(parseShortName("org/sub/skill-name")).toBe("sub/skill-name");
  });

  test("handles empty string", () => {
    expect(parseShortName("")).toBe("");
  });

  test("handles name that is only a slash", () => {
    expect(parseShortName("/")).toBe("");
  });
});
