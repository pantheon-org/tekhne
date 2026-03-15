import { describe, expect, test } from "bun:test";
import { parseReadmeSections } from "./parse-readme-sections";

const domainHeaders = ["Skills"];

describe("parseReadmeSections", () => {
  test("captures before/after sections around skill section", () => {
    const lines = [
      "# Title",
      "Intro line",
      "## Skills",
      "skill content",
      "## Contributing",
      "after content",
    ];
    const { beforeSkills, afterSkills } = parseReadmeSections(
      lines,
      domainHeaders,
    );
    expect(beforeSkills).toContain("# Title");
    expect(beforeSkills).toContain("Intro line");
    expect(afterSkills).toContain("## Contributing");
    expect(afterSkills).toContain("after content");
  });

  test("everything is before when no domain header is found", () => {
    const lines = ["# Title", "No skills section here"];
    const { beforeSkills, afterSkills } = parseReadmeSections(
      lines,
      domainHeaders,
    );
    expect(beforeSkills).toHaveLength(2);
    expect(afterSkills).toHaveLength(0);
  });

  test("excludes skill section start line from both arrays", () => {
    const lines = ["## Skills", "content", "## Other", "end"];
    const { beforeSkills, afterSkills } = parseReadmeSections(
      lines,
      domainHeaders,
    );
    expect(beforeSkills).toHaveLength(0);
    expect(afterSkills).toContain("## Other");
  });
});
