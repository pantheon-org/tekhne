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
// release (mise.toml), the same binary the hk.pkl gate runs. It must be on PATH,
// which `mise install` alone does not do for a shell without mise activation.
const VALIDATOR_BIN = "pantheon-skill-auditor";

const MISSING_VALIDATOR = `${VALIDATOR_BIN} not found on PATH. Run \`mise install\`, then run the suite through mise: \`mise exec -- bun run test:integration\`.\n`;

// Run the pinned auditor, turning a missing binary into an actionable message
// instead of a bare spawn error.
const runAuditor = (cwd: string, args: string[]): CommandResult => {
  const result = spawnSync(VALIDATOR_BIN, args, { cwd, encoding: "utf-8" });
  if ((result.error as NodeJS.ErrnoException | undefined)?.code === "ENOENT") {
    return { stdout: "", stderr: MISSING_VALIDATOR, exitCode: 127 };
  }
  return capture(result);
};

export const runValidatorHelp = (cwd: string): CommandResult =>
  runAuditor(cwd, ["validate", "--help"]);

export const runValidatorStructure = (
  cwd: string,
  skill: string,
): CommandResult =>
  runAuditor(cwd, [
    "validate",
    "structure",
    "--allow-dirs=evals",
    `skills/${skill}`,
  ]);

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
