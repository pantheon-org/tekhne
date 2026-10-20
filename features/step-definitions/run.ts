import { spawnSync } from "node:child_process";
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

// The validator is the `validate` subcommand of the pinned pantheon-skill-auditor
// release (mise.toml), the same binary the hk.pkl gate runs. It must be on PATH:
// the suite fails with the spawn error when it is not, rather than building one.
const VALIDATOR_BIN = "pantheon-skill-auditor";

export const runValidatorHelp = (cwd: string): CommandResult => {
  return capture(
    spawnSync(VALIDATOR_BIN, ["validate", "--help"], {
      cwd,
      encoding: "utf-8",
    }),
  );
};

export const runValidatorStructure = (
  cwd: string,
  skill: string,
): CommandResult => {
  return capture(
    spawnSync(
      VALIDATOR_BIN,
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
