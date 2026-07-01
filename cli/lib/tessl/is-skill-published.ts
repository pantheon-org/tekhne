import { exec } from "../utils/exec";
import { readManifest } from "../utils/skill-manifest";

export const isSkillPublished = async (
  skillPath: string,
  workspace: string,
): Promise<boolean> => {
  const manifest = await readManifest(skillPath);
  if (!manifest || !manifest.name) {
    return false;
  }

  const { exitCode } = await exec(`tessl search ${workspace}/${manifest.name}`);
  return exitCode === 0;
};
