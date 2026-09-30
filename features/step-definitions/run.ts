import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { CommandResult } from "./world";

interface Spawned {
  stdout: string | null;
  stderr: string | null;
  status: number | null;
}

const capture = (result: Spawned): CommandResult => ({
  stdout: result.stdout ?? "",
  stderr: result.stderr ?? "",
  exitCode: result.status ?? 1,
});

// Run the relocated skill catalog generator (scripts/catalog) in dry-run mode.
export const runCatalogDryRun = (cwd: string): CommandResult =>
  capture(
    spawnSync("bun", ["scripts/catalog/update.ts", "--dry-run"], {
      cwd,
      encoding: "utf-8",
    }),
  );

const VALIDATOR_BIN = "target/release/skill-validator-rs";

// Build the Rust validator from source once (cached), mirroring the hk.pkl gate
// so the integration suite never depends on a pre-built binary.
const ensureValidator = (cwd: string): void => {
  if (existsSync(join(cwd, VALIDATOR_BIN))) {
    return;
  }
  spawnSync("cargo", ["build", "--release", "-q", "-p", "skill-validator-rs"], {
    cwd,
    encoding: "utf-8",
  });
};

export const runValidatorHelp = (cwd: string): CommandResult => {
  ensureValidator(cwd);
  return capture(
    spawnSync(join(cwd, VALIDATOR_BIN), ["validate", "--help"], {
      cwd,
      encoding: "utf-8",
    }),
  );
};

export const runValidatorStructure = (
  cwd: string,
  skill: string,
): CommandResult => {
  ensureValidator(cwd);
  return capture(
    spawnSync(
      join(cwd, VALIDATOR_BIN),
      ["validate", "structure", "--allow-dirs=evals", `skills/${skill}`],
      { cwd, encoding: "utf-8" },
    ),
  );
};

const GOAL_SCRIPT = "skills/project-mgmt/goal-tracker/scripts/goal.sh";

// Run the goal-tracker script. CLAUDE_PROJECT_DIR and GOAL_SESSION are cleared
// so the scenario's --root and --session are the only inputs.
export const runGoalScript = (cwd: string, args: string[]): CommandResult => {
  const env = { ...process.env };
  delete env.CLAUDE_PROJECT_DIR;
  delete env.GOAL_SESSION;
  return capture(
    spawnSync("bash", [GOAL_SCRIPT, ...args], { cwd, encoding: "utf-8", env }),
  );
};
