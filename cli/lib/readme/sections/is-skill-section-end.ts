export const isSkillSectionEnd = (
  line: string,
  domainHeaders: string[],
): boolean => {
  return (
    line.startsWith("## ") &&
    !domainHeaders.some((h) => line.startsWith(`## ${h}`))
  );
};
