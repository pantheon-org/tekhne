export const formatSummary = (summary: string): string => {
  return summary.replace(/\n/g, " ").trim();
};
