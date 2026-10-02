import { describe, expect, test } from "bun:test";
import { parseFileList } from "./parse-file-list";

describe("parseFileList", () => {
  test("sorts the paths, so the order never depends on the filesystem", () => {
    const scrambled = [
      "skills/ci-cd/jenkinsfile/generator/SKILL.md",
      "skills/ci-cd/azure-pipelines/validator/SKILL.md",
      "skills/ci-cd/helm/generator/SKILL.md",
      "skills/ci-cd/gitlab-ci/generator/SKILL.md",
    ].join("\n");
    expect(parseFileList(scrambled)).toEqual([
      "skills/ci-cd/azure-pipelines/validator/SKILL.md",
      "skills/ci-cd/gitlab-ci/generator/SKILL.md",
      "skills/ci-cd/helm/generator/SKILL.md",
      "skills/ci-cd/jenkinsfile/generator/SKILL.md",
    ]);
  });

  test("gives the same result for any input order", () => {
    const a = "skills/b/SKILL.md\nskills/a/SKILL.md\nskills/c/SKILL.md\n";
    const b = "skills/c/SKILL.md\nskills/a/SKILL.md\nskills/b/SKILL.md\n";
    expect(parseFileList(a)).toEqual(parseFileList(b));
  });

  test("drops blank lines and surrounding whitespace", () => {
    expect(
      parseFileList("\n  skills/a/SKILL.md\n\nskills/b/SKILL.md\n\n"),
    ).toEqual(["skills/a/SKILL.md", "skills/b/SKILL.md"]);
  });

  test("is empty for empty output", () => {
    expect(parseFileList("")).toEqual([]);
    expect(parseFileList("\n")).toEqual([]);
  });

  test("sorts by code unit, not locale, so upper case sorts before lower case", () => {
    expect(parseFileList("skills/b/x\nskills/B/x")).toEqual([
      "skills/B/x",
      "skills/b/x",
    ]);
  });
});
