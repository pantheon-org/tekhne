import { exec } from "../utils/exec";
import { logger } from "../utils/logger";

export const runLintCheck = async (tilePath: string): Promise<boolean> => {
  logger.info("Running tessl skill lint...");
  const { exitCode, stdout, stderr } = await exec(
    `tessl skill lint ${tilePath}`,
  );

  if (exitCode !== 0) {
    logger.error("Lint failed");
    console.error(stderr);
    return false;
  }

  logger.success("Lint passed");

  if (stdout.includes("warning") || stdout.includes("orphaned")) {
    logger.warning("Warnings detected (not blocking)");
    console.log(stdout);
  }

  return true;
};
