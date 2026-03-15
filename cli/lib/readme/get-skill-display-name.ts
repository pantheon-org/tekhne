export const getSkillDisplayName = (skillRelativePath: string): string => {
  const parts = skillRelativePath.split("/");
  const pathWithoutDomain = parts.slice(1).join("/");

  if (pathWithoutDomain.includes("/")) {
    return pathWithoutDomain.replace(/\//g, "-");
  }

  return pathWithoutDomain;
};
