import { spawnSync } from "node:child_process";

export interface CliResult {
  stdout: string;
  stderr: string;
  exitCode: number;
}

export const runCli = (args: string[], cwd: string): CliResult => {
  const result = spawnSync("bun", ["cli/index.ts", ...args], {
    cwd,
    encoding: "utf-8",
  });
  return {
    stdout: result.stdout ?? "",
    stderr: result.stderr ?? "",
    exitCode: result.status ?? 1,
  };
};
