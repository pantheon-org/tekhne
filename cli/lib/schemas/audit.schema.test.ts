import { describe, expect, test } from "bun:test";
import { AuditResultSchema, DimensionScoreSchema } from "./audit.schema";

const validDim = { score: 12, rationale: "Good coverage" };

const validAudit = {
  skill: "dev/bun",
  timestamp: "2026-03-15T00:00:00Z",
  totalScore: 96,
  grade: "B" as const,
  dimensions: {
    D1_knowledge_delta: validDim,
    D2_mindset_procedures: validDim,
    D3_anti_patterns: validDim,
    D4_specification_compliance: validDim,
    D5_progressive_disclosure: validDim,
    D6_freedom_calibration: validDim,
    D7_pattern_recognition: validDim,
    D8_practical_usability: validDim,
  },
};

describe("DimensionScoreSchema", () => {
  test("parses valid dimension", () => {
    expect(() => DimensionScoreSchema.parse(validDim)).not.toThrow();
  });

  test("rejects score above 15", () => {
    expect(() =>
      DimensionScoreSchema.parse({ score: 16, rationale: "x" }),
    ).toThrow();
  });

  test("rejects missing rationale", () => {
    expect(() => DimensionScoreSchema.parse({ score: 10 })).toThrow();
  });
});

describe("AuditResultSchema", () => {
  test("parses valid audit", () => {
    expect(() => AuditResultSchema.parse(validAudit)).not.toThrow();
  });

  test("rejects invalid grade", () => {
    expect(() =>
      AuditResultSchema.parse({ ...validAudit, grade: "Z" }),
    ).toThrow();
  });

  test("rejects totalScore above 120", () => {
    expect(() =>
      AuditResultSchema.parse({ ...validAudit, totalScore: 121 }),
    ).toThrow();
  });
});
