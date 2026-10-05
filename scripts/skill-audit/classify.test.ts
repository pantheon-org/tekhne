import { describe, expect, test } from "bun:test";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const SCRIPT = join(import.meta.dir, "classify.sh");

type Row = { skill: string; grade: string; total: number };

const withDims = (rows: Row[]) =>
  rows.map((r) => ({ ...r, maxTotal: 140, dimensions: {} }));

const classify = (head: Row[], base: Row[] | string) => {
  const dir = mkdtempSync(join(tmpdir(), "skill-classify-"));
  writeFileSync(join(dir, "head.json"), JSON.stringify(withDims(head)));
  writeFileSync(
    join(dir, "base.json"),
    typeof base === "string" ? base : JSON.stringify(withDims(base)),
  );
  const r = Bun.spawnSync([
    "bash",
    SCRIPT,
    join(dir, "head.json"),
    join(dir, "base.json"),
  ]);
  return {
    exitCode: r.exitCode,
    out: r.exitCode === 0 ? JSON.parse(r.stdout.toString()) : [],
  };
};

const tierOf = (out: { skill: string; tier: string }[], skill: string) =>
  out.find((e) => e.skill === skill)?.tier;

describe("classify.sh", () => {
  test("A and A+ pass, B and B+ warn", () => {
    const { out } = classify(
      [
        { skill: "a/one", grade: "A", total: 130 },
        { skill: "a/two", grade: "A+", total: 138 },
        { skill: "a/three", grade: "B+", total: 120 },
        { skill: "a/four", grade: "B", total: 114 },
      ],
      [],
    );
    expect(tierOf(out, "a/one")).toBe("pass");
    expect(tierOf(out, "a/two")).toBe("pass");
    expect(tierOf(out, "a/three")).toBe("warn");
    expect(tierOf(out, "a/four")).toBe("warn");
  });

  test("a skill below B with a score no lower than base is known debt", () => {
    const { out } = classify(
      [{ skill: "a/one", grade: "C", total: 100 }],
      [{ skill: "a/one", grade: "C", total: 100 }],
    );
    expect(tierOf(out, "a/one")).toBe("debt");
  });

  test("a skill below B that scores higher than base is known debt", () => {
    const { out } = classify(
      [{ skill: "a/one", grade: "C+", total: 108 }],
      [{ skill: "a/one", grade: "C", total: 100 }],
    );
    expect(tierOf(out, "a/one")).toBe("debt");
  });

  test("a skill below B that scores lower than base fails", () => {
    const { out } = classify(
      [{ skill: "a/one", grade: "C", total: 99 }],
      [{ skill: "a/one", grade: "C", total: 100 }],
    );
    expect(tierOf(out, "a/one")).toBe("fail");
    expect(out[0].reason).toBe("lower than base");
  });

  test("a new skill below B fails", () => {
    const { out } = classify([{ skill: "a/new", grade: "C", total: 100 }], []);
    expect(tierOf(out, "a/new")).toBe("fail");
    expect(out[0].reason).toBe("new skill");
    expect(out[0].baseTotal).toBeNull();
  });

  test("a skill that drops from A to B is not blocked", () => {
    const { out } = classify(
      [{ skill: "a/one", grade: "B", total: 114 }],
      [{ skill: "a/one", grade: "A", total: 130 }],
    );
    expect(tierOf(out, "a/one")).toBe("warn");
  });

  test("an unreadable base audit makes every skill below B fail", () => {
    const { exitCode, out } = classify(
      [{ skill: "a/one", grade: "C", total: 100 }],
      "not json",
    );
    expect(exitCode).toBe(0);
    expect(tierOf(out, "a/one")).toBe("fail");
  });

  test("matches skills by name when the audit reports a full path", () => {
    const { out } = classify(
      [{ skill: "/work/skills/a/one/SKILL.md", grade: "C", total: 100 }],
      [{ skill: "/base/skills/a/one/SKILL.md", grade: "C", total: 100 }],
    );
    expect(out[0].skill).toBe("a/one");
    expect(out[0].tier).toBe("debt");
  });
});
