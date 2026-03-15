export const getSkillName = (skillPath: string): string => {
  const parts = skillPath.split("/").slice(1);
  return parts.join("--");
};
