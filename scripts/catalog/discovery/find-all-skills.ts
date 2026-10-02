import { $ } from "bun";
import type { SkillEntry } from "../types";
import { parseFileList } from "./parse-file-list";

export const findAllSkills = async (): Promise<SkillEntry[]> => {
  const output = await $`find skills -name "SKILL.md" -type f`.text();
  const files = parseFileList(output);

  return files.map((file) => {
    const relativePath = file.replace("skills/", "").replace("/SKILL.md", "");
    const domain = relativePath.split("/")[0];
    return { domain, relativePath };
  });
};
