import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";

export const getEvalCount = (skillDir: string): number => {
  let count = 0;

  // Convention 1: evals/scenario-N/ subdirectories
  const evalsDir = join(skillDir, "evals");
  if (existsSync(evalsDir)) {
    count += readdirSync(evalsDir).filter((f) =>
      f.startsWith("scenario-"),
    ).length;
  }

  // Convention 2: evaluation-scenarios/scenario-NN.md files
  const evalScenariosDir = join(skillDir, "evaluation-scenarios");
  if (existsSync(evalScenariosDir)) {
    count += readdirSync(evalScenariosDir).filter(
      (f) => f.startsWith("scenario-") && f.endsWith(".md"),
    ).length;
  }

  return count;
};
