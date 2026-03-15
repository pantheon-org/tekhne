import { Checkbox } from "@cliffy/prompt";
import { logger } from "../utils/logger";
import { getSkillName } from "./get-skill-name";

export const selectSkillsInteractively = async (
  skills: string[],
): Promise<string[]> => {
  if (!process.stdin.isTTY) {
    logger.warning(
      "--interactive ignored: stdin is not a TTY. Installing all skills.",
    );
    return skills;
  }

  const selected = await Checkbox.prompt({
    message: "Select skills to install (space to toggle, enter to confirm):",
    options: skills.map((skillPath) => ({
      name: getSkillName(skillPath),
      value: skillPath,
      checked: true,
    })),
  });

  if (selected.length === 0) {
    logger.info("No skills selected. Aborting.");
    return [];
  }

  return selected;
};
