import type { InstallOptions } from "./install-skills";

export const filterSkills = (
  skills: string[],
  options: Pick<InstallOptions, "skillDomain" | "skillSubdomain">,
): string[] => {
  const { skillDomain, skillSubdomain } = options;
  const hasDomainFilter = skillDomain && skillDomain.length > 0;
  const hasSubdomainFilter = skillSubdomain && skillSubdomain.length > 0;

  if (!hasDomainFilter && !hasSubdomainFilter) {
    return skills;
  }

  return skills.filter((skillPath) => {
    // skillPath format: "skills/<domain>/<subdomain>[/leaf]"
    const parts = skillPath.split("/");
    const domain = parts[1];
    const subdomain = parts[2];

    const domainMatch = hasDomainFilter ? skillDomain?.includes(domain) : true;
    const subdomainMatch = hasSubdomainFilter
      ? skillSubdomain?.includes(subdomain)
      : true;

    return domainMatch && subdomainMatch;
  });
};
