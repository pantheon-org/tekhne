export const getBadgeFilename = (grade: string): string =>
  grade.replace("+", "-plus").replace("?", "unknown");
