import { existsSync } from "node:fs";
import { join } from "node:path";
import { $ } from "bun";
import { CLIError } from "../utils/errors";

export const findSkills = async (cwd: string): Promise<string[]> => {
  if (!existsSync(join(cwd, "skills"))) {
    throw new CLIError(
      "skills/ directory not found. Run this command from the repository root.",
      1,
    );
  }

  const output = await $`find skills -name "SKILL.md" -type f`.cwd(cwd).text();
  return output
    .trim()
    .split("\n")
    .filter(Boolean)
    .map((path) => path.replace("/SKILL.md", ""));
};
