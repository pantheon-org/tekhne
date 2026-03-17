import { Checkbox } from "@cliffy/prompt";
import { logger } from "../utils/logger";
import type { InstalledSkill } from "./find-installed-skills";

export const selectSkillsToUninstallInteractively = async (
  skills: InstalledSkill[],
): Promise<InstalledSkill[]> => {
  if (!process.stdin.isTTY) {
    logger.warning(
      "--interactive ignored: stdin is not a TTY. Uninstalling all skills.",
    );
    return skills;
  }

  const selected = await Checkbox.prompt({
    message: "Select skills to uninstall (space to toggle, enter to confirm):",
    options: skills.map((skill) => ({
      name: skill.name,
      value: skill.name,
      checked: false,
    })),
  });

  if (selected.length === 0) {
    logger.info("No skills selected. Aborting.");
    return [];
  }

  return skills.filter((skill) => selected.includes(skill.name));
};
