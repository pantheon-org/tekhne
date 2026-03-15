import { getGradeColor } from "../parsing";

export const getBadgeMarkdown = (grade: string): string => {
  const color = getGradeColor(grade);
  return `![${grade}](https://img.shields.io/badge/Rating-${grade}-${color})`;
};
