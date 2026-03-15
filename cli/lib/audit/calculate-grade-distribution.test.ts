import { describe, expect, test } from "bun:test";
import { calculateGradeDistribution } from "./calculate-grade-distribution";
import type { SkillAudit } from "./collect-audit-data";

const mkAudit = (score: number): SkillAudit => ({
  path: "x",
  score,
  grade: "A",
  dimensions: {},
});

describe("calculateGradeDistribution", () => {
  test("all grade keys present even for empty input", () => {
    const r = calculateGradeDistribution([]);
    for (const g of ["A+", "A", "B+", "B", "C+", "C", "D", "F"]) {
      expect(r[g]).toBe(0);
    }
  });

  test("A+ for score 114", () => {
    expect(calculateGradeDistribution([mkAudit(114)])["A+"]).toBe(1);
  });

  test("A for score 108", () => {
    expect(calculateGradeDistribution([mkAudit(108)])["A"]).toBe(1);
  });

  test("F for score 50", () => {
    expect(calculateGradeDistribution([mkAudit(50)])["F"]).toBe(1);
  });

  test("multiple audits across grades", () => {
    const r = calculateGradeDistribution([
      mkAudit(115),
      mkAudit(50),
      mkAudit(115),
    ]);
    expect(r["A+"]).toBe(2);
    expect(r["F"]).toBe(1);
  });
});
