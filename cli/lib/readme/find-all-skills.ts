import { $ } from "bun";
import type { SkillEntry } from "./readme-types";

export const findAllSkills = async (): Promise<SkillEntry[]> => {
  const output = await $`find skills -name "SKILL.md" -type f`.text();
  const files = output.trim().split("\n").filter(Boolean);

  return files.map((file) => {
    const relativePath = file.replace("skills/", "").replace("/SKILL.md", "");
    const domain = relativePath.split("/")[0];
    return { domain, relativePath };
  });
};
