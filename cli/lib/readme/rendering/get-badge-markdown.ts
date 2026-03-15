import { getBadgeFilename } from "./get-badge-filename";

const BADGE_DIR = ".github/badges";

export const getBadgeMarkdown = (grade: string): string => {
  const filename = getBadgeFilename(grade);
  return `![${grade}](${BADGE_DIR}/${filename}.svg)`;
};
