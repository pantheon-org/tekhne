import { describe, expect, test } from "bun:test";
import { calculateDimensionAverages } from "./calculate-dimension-averages";
import type { SkillAudit } from "./collect-audit-data";

const mkAudit = (dims: Record<string, number>): SkillAudit => ({
  path: "x",
  score: 80,
  grade: "A",
  dimensions: dims,
});

describe("calculateDimensionAverages", () => {
  test("returns empty object for no audits", () => {
    expect(calculateDimensionAverages([])).toEqual({});
  });

  test("single audit returns its dimension values unchanged", () => {
    const r = calculateDimensionAverages([mkAudit({ D1: 10, D2: 12 })]);
    expect(r["D1"]).toBe(10);
    expect(r["D2"]).toBe(12);
  });

  test("averages across multiple audits", () => {
    const r = calculateDimensionAverages([
      mkAudit({ D1: 10 }),
      mkAudit({ D1: 14 }),
    ]);
    expect(r["D1"]).toBe(12);
  });

  test("rounds averages", () => {
    const r = calculateDimensionAverages([
      mkAudit({ D1: 10 }),
      mkAudit({ D1: 11 }),
      mkAudit({ D1: 12 }),
    ]);
    expect(r["D1"]).toBe(11);
  });
});
