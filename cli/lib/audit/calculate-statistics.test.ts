import { describe, expect, test } from "bun:test";
import { calculateStatistics } from "./calculate-statistics";
import type { SkillAudit } from "./collect-audit-data";

const mkAudit = (score: number): SkillAudit => ({
  path: "x",
  score,
  grade: "A",
  dimensions: {},
});

describe("calculateStatistics", () => {
  test("single item returns all equal stats", () => {
    const r = calculateStatistics([mkAudit(80)]);
    expect(r.avgScore).toBe(80);
    expect(r.medianScore).toBe(80);
    expect(r.minScore).toBe(80);
    expect(r.maxScore).toBe(80);
  });

  test("computes avg, median, min, max for multiple items", () => {
    const r = calculateStatistics([mkAudit(60), mkAudit(80), mkAudit(100)]);
    expect(r.avgScore).toBeCloseTo(80);
    expect(r.medianScore).toBe(80);
    expect(r.minScore).toBe(60);
    expect(r.maxScore).toBe(100);
  });

  test("handles unsorted input", () => {
    const r = calculateStatistics([mkAudit(100), mkAudit(40), mkAudit(70)]);
    expect(r.minScore).toBe(40);
    expect(r.maxScore).toBe(100);
  });
});
