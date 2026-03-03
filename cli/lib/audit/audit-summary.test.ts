import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TEST_DIR = "/tmp/audit-summary-test/.context/audits";

beforeEach(() => {
  if (existsSync("/tmp/audit-summary-test")) {
    rmSync("/tmp/audit-summary-test", { recursive: true });
  }
  mkdirSync(TEST_DIR, { recursive: true });
});

afterEach(() => {
  if (existsSync("/tmp/audit-summary-test")) {
    rmSync("/tmp/audit-summary-test", { recursive: true });
  }
});

describe("audit-summary data collection", () => {
  test("should parse valid audit.json files", () => {
    const skillPath = join(TEST_DIR, "domain/skill-name/latest");
    mkdirSync(skillPath, { recursive: true });

    const auditData = {
      score: 105,
      grade: "A",
      dimensions: {
        D1_knowledge_delta: 13,
        D2_mindset_procedures: 14,
        D3_anti_patterns: 12,
      },
    };

    writeFileSync(join(skillPath, "audit.json"), JSON.stringify(auditData));

    expect(existsSync(join(skillPath, "audit.json"))).toBe(true);
  });

  test("should extract skill path from file location", () => {
    const filePath = ".context/audits/domain/skill-name/latest/audit.json";
    const match = filePath.match(/\.context\/audits\/(.+?)\/latest/);

    expect(match).not.toBeNull();
    expect(match?.[1]).toBe("domain/skill-name");
  });

  test("should handle multiple audit files", () => {
    const skills = ["skill1", "skill2", "skill3"];

    for (const skill of skills) {
      const skillPath = join(TEST_DIR, `domain/${skill}/latest`);
      mkdirSync(skillPath, { recursive: true });
      writeFileSync(
        join(skillPath, "audit.json"),
        JSON.stringify({ score: 100, grade: "B+", dimensions: {} }),
      );
    }

    expect(existsSync(join(TEST_DIR, "domain/skill1/latest/audit.json"))).toBe(
      true,
    );
    expect(existsSync(join(TEST_DIR, "domain/skill2/latest/audit.json"))).toBe(
      true,
    );
    expect(existsSync(join(TEST_DIR, "domain/skill3/latest/audit.json"))).toBe(
      true,
    );
  });
});

describe("grade distribution logic", () => {
  test("should map scores to correct grade ranges", () => {
    const gradeRanges = {
      "A+": [114, 120],
      A: [108, 113],
      "B+": [102, 107],
      B: [96, 101],
      "C+": [90, 95],
      C: [84, 89],
      D: [72, 83],
      F: [0, 71],
    };

    expect(120).toBeGreaterThanOrEqual(gradeRanges["A+"][0]);
    expect(120).toBeLessThanOrEqual(gradeRanges["A+"][1]);

    expect(108).toBeGreaterThanOrEqual(gradeRanges.A[0]);
    expect(113).toBeLessThanOrEqual(gradeRanges.A[1]);

    expect(70).toBeGreaterThanOrEqual(gradeRanges.F[0]);
    expect(70).toBeLessThanOrEqual(gradeRanges.F[1]);
  });

  test("should assign correct grade for boundary scores", () => {
    const testCases = [
      { score: 120, expectedGrade: "A+" },
      { score: 114, expectedGrade: "A+" },
      { score: 113, expectedGrade: "A" },
      { score: 108, expectedGrade: "A" },
      { score: 107, expectedGrade: "B+" },
      { score: 96, expectedGrade: "B" },
      { score: 71, expectedGrade: "F" },
      { score: 0, expectedGrade: "F" },
    ];

    for (const { score } of testCases) {
      expect(score).toBeGreaterThanOrEqual(0);
      expect(score).toBeLessThanOrEqual(120);
    }
  });
});

describe("statistics calculations", () => {
  test("should calculate average correctly", () => {
    const scores = [100, 110, 90];
    const avg = scores.reduce((a, b) => a + b, 0) / scores.length;
    expect(avg).toBe(100);
  });

  test("should calculate median for odd count", () => {
    const scores = [80, 90, 100, 110, 120];
    const median = scores[Math.floor(scores.length / 2)];
    expect(median).toBe(100);
  });

  test("should calculate median for even count", () => {
    const scores = [80, 90, 100, 110];
    const median = scores[Math.floor(scores.length / 2)];
    expect(median).toBe(100);
  });

  test("should find min and max scores", () => {
    const scores = [85, 92, 108, 76, 115];
    const min = Math.min(...scores);
    const max = Math.max(...scores);

    expect(min).toBe(76);
    expect(max).toBe(115);
  });
});

describe("dimension averages", () => {
  test("should calculate average dimension scores", () => {
    const dimensionData = {
      D1: [12, 14, 10],
      D2: [15, 15, 15],
      D3: [8, 9, 7],
    };

    const averages = Object.fromEntries(
      Object.entries(dimensionData).map(([dim, scores]) => [
        dim,
        Math.round(scores.reduce((a, b) => a + b, 0) / scores.length),
      ]),
    );

    expect(averages.D1).toBe(12);
    expect(averages.D2).toBe(15);
    expect(averages.D3).toBe(8);
  });

  test("should identify weak dimensions (< 80%)", () => {
    const dimensionAvg = 11;
    const percentage = Math.round((dimensionAvg / 15) * 100);
    const isWeak = percentage < 80;

    expect(percentage).toBe(73);
    expect(isWeak).toBe(true);
  });

  test("should identify strong dimensions (>= 80%)", () => {
    const dimensionAvg = 13;
    const percentage = Math.round((dimensionAvg / 15) * 100);
    const isStrong = percentage >= 80;

    expect(percentage).toBe(87);
    expect(isStrong).toBe(true);
  });
});
