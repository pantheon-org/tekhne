import { getBadgeFilename } from "./get-badge-filename";

export const getDocsBadgeMarkdown = (grade: string): string => {
  const cssClass = getBadgeFilename(grade).toLowerCase();
  return `<span class="skill-badge skill-badge--${cssClass}">${grade}</span>`;
};
