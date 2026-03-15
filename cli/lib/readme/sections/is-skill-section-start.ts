export const isSkillSectionStart = (
  line: string,
  domainHeaders: string[],
): boolean => {
  return domainHeaders.some((h) => line.startsWith(`## ${h}`));
};
