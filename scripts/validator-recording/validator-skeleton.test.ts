import { describe, expect, test } from "bun:test";
import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  realpathSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  findSchemaPath,
  formatPyValue,
  pyTypeName,
} from "../../skills/project-mgmt/plan-create/scripts/validator-skeleton";

const SKILLS = join(import.meta.dir, "../../skills/project-mgmt");
const COPIES = [
  join(SKILLS, "plan-create/scripts/validator-skeleton.ts"),
  join(SKILLS, "plan-review/scripts/validator-skeleton.ts"),
  join(SKILLS, "tech-evaluation/scripts/validator-skeleton.ts"),
];

describe("the three copies of validator-skeleton.ts", () => {
  test("are identical, byte for byte", () => {
    const [first, ...others] = COPIES.map((path) => readFileSync(path));
    for (const other of others) expect(other.equals(first)).toBe(true);
  });
});

describe("formatPyValue and pyTypeName", () => {
  test("write values and type names the way python does", () => {
    expect(formatPyValue(["a", 1, null, true])).toBe("['a', 1, None, True]");
    expect(pyTypeName([])).toBe("list");
    expect(pyTypeName("x")).toBe("str");
    expect(pyTypeName(null)).toBe("NoneType");
    expect(pyTypeName(1)).toBe("int");
    expect(pyTypeName(1.5)).toBe("float");
    expect(pyTypeName(true)).toBe("bool");
  });
});

describe("findSchemaPath", () => {
  const skill = (): string => {
    const dir = realpathSync(
      mkdtempSync(join(tmpdir(), "validator-skeleton-")),
    );
    writeFileSync(join(dir, "SKILL.md"), "# skill\n");
    mkdirSync(join(dir, "scripts"), { recursive: true });
    mkdirSync(join(dir, "assets/schemas"), { recursive: true });
    return dir;
  };

  test("walks up to SKILL.md and finds the schema by its exact name", () => {
    const dir = skill();
    const schema = join(dir, "assets/schemas/x.schema.json");
    writeFileSync(schema, "{}");
    writeFileSync(join(dir, "assets/schemas/other.schema.json"), "{}");
    expect(findSchemaPath(join(dir, "scripts"), "x.schema.json")).toBe(schema);
    rmSync(dir, { recursive: true, force: true });
  });

  test("says what it found when there is none, two, or no SKILL.md", () => {
    const dir = skill();
    expect(() => findSchemaPath(join(dir, "scripts"), "x.schema.json")).toThrow(
      "x.schema.json",
    );
    writeFileSync(join(dir, "assets/schemas/x.schema.json"), "{}");
    mkdirSync(join(dir, "references"), { recursive: true });
    writeFileSync(join(dir, "references/x.schema.json"), "{}");
    expect(() => findSchemaPath(join(dir, "scripts"), "x.schema.json")).toThrow(
      /2 /,
    );
    const bare = realpathSync(
      mkdtempSync(join(tmpdir(), "validator-skeleton-")),
    );
    expect(() => findSchemaPath(bare, "x.schema.json")).toThrow("SKILL.md");
    rmSync(dir, { recursive: true, force: true });
    rmSync(bare, { recursive: true, force: true });
  });

  test("follows a symlinked folder to the real skill", () => {
    const dir = skill();
    const schema = join(dir, "assets/schemas/x.schema.json");
    writeFileSync(schema, "{}");
    const link = join(
      mkdtempSync(join(tmpdir(), "validator-skeleton-")),
      "linked",
    );
    symlinkSync(join(dir, "scripts"), link);
    expect(findSchemaPath(link, "x.schema.json")).toBe(schema);
    rmSync(dir, { recursive: true, force: true });
  });
});
