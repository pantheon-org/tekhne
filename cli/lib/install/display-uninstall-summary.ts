import { logger } from "../utils/logger";

interface AgentStats {
  removed: number;
  skipped: number;
  failed: number;
}

export const displayUninstallSummary = (
  agents: string[],
  stats: Record<string, AgentStats>,
  dryRun: boolean,
): void => {
  logger.header("\nUninstall Summary");

  for (const agent of agents) {
    if (!stats[agent]) continue;

    const { removed, skipped, failed } = stats[agent];
    logger.info(`\n${agent}:`);
    if (removed > 0) logger.success(`  Removed: ${removed}`);
    if (skipped > 0) logger.debug(`  Skipped: ${skipped}`);
    if (failed > 0) logger.error(`  Failed: ${failed}`);
  }

  if (dryRun) {
    logger.warning("\nDry run completed. No changes made.");
  }
};
