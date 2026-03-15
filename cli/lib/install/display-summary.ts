import { logger } from "../utils/logger";

interface AgentStats {
  installed: number;
  skipped: number;
  failed: number;
}

export const displaySummary = (
  agents: string[],
  stats: Record<string, AgentStats>,
  dryRun: boolean,
): void => {
  logger.header("\nInstallation Summary");

  for (const agent of agents) {
    if (!stats[agent]) continue;

    const { installed, skipped, failed } = stats[agent];
    logger.info(`\n${agent}:`);
    if (installed > 0) logger.success(`  Installed: ${installed}`);
    if (skipped > 0) logger.debug(`  Skipped: ${skipped}`);
    if (failed > 0) logger.error(`  Failed: ${failed}`);
  }

  if (dryRun) {
    logger.warning("\nDry run completed. No changes made.");
  }
};
