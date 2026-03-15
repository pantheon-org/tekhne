import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getEvalCount } from "./get-eval-count";

let tmp: string;

beforeEach(() => {
  tmp = mkdtempSync(join(tmpdir(), "eval-count-test-"));
});

afterEach(() => {
  Bun.spawnSync(["rm", "-rf", tmp]);
});

describe("getEvalCount", () => {
  test("returns 0 when neither evals/ nor evaluation-scenarios/ exist", () => {
    expect(getEvalCount(tmp)).toBe(0);
  });

  test("counts scenario-N/ subdirectories in evals/", () => {
    const evalsDir = join(tmp, "evals");
    mkdirSync(evalsDir);
    mkdirSync(join(evalsDir, "scenario-1"));
    mkdirSync(join(evalsDir, "scenario-2"));
    mkdirSync(join(evalsDir, "other-dir")); // should not count
    expect(getEvalCount(tmp)).toBe(2);
  });

  test("counts scenario-NN.md files in evaluation-scenarios/", () => {
    const scenariosDir = join(tmp, "evaluation-scenarios");
    mkdirSync(scenariosDir);
    writeFileSync(join(scenariosDir, "scenario-01.md"), "");
    writeFileSync(join(scenariosDir, "scenario-02.md"), "");
    writeFileSync(join(scenariosDir, "README.md"), ""); // should not count
    expect(getEvalCount(tmp)).toBe(2);
  });

  test("sums counts from both conventions", () => {
    mkdirSync(join(tmp, "evals"));
    mkdirSync(join(tmp, "evals", "scenario-1"));
    mkdirSync(join(tmp, "evaluation-scenarios"));
    writeFileSync(join(tmp, "evaluation-scenarios", "scenario-01.md"), "");
    expect(getEvalCount(tmp)).toBe(2);
  });
});
