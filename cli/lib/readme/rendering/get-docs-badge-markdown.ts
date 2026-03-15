import { getBadgeFilename } from "./get-badge-filename";

const DOCS_BADGE_BASE = "/tekhne/.github/badges";

export const getDocsBadgeMarkdown = (grade: string): string => {
  const filename = getBadgeFilename(grade);
  return `![${grade}](${DOCS_BADGE_BASE}/${filename}.svg)`;
};
