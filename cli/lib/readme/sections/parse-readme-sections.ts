import type { ReadmeSections } from "../types";
import { isSkillSectionEnd } from "./is-skill-section-end";
import { isSkillSectionStart } from "./is-skill-section-start";

export const parseReadmeSections = (
  lines: string[],
  domainHeaders: string[],
): ReadmeSections => {
  const beforeSkills: string[] = [];
  const afterSkills: string[] = [];
  let inSkillsSection = false;
  let foundEndOfSkills = false;

  for (const line of lines) {
    if (!inSkillsSection && isSkillSectionStart(line, domainHeaders)) {
      inSkillsSection = true;
      continue;
    }

    if (
      inSkillsSection &&
      !foundEndOfSkills &&
      isSkillSectionEnd(line, domainHeaders)
    ) {
      foundEndOfSkills = true;
      afterSkills.push(line);
      continue;
    }

    if (!inSkillsSection) {
      beforeSkills.push(line);
    } else if (foundEndOfSkills) {
      afterSkills.push(line);
    }
  }

  return { beforeSkills, afterSkills };
};
