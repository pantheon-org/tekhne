import { existsSync } from "node:fs";
import { join } from "node:path";
import { TileSchema } from "../schemas/tile.schema";
import { exec } from "../utils/exec";

export const isSkillPublished = async (
  skillPath: string,
  workspace: string,
): Promise<boolean> => {
  const tileJsonPath = join(skillPath, "tile.json");
  if (!existsSync(tileJsonPath)) {
    return false;
  }

  const rawData = await Bun.file(tileJsonPath).json();
  const tileData = TileSchema.parse(rawData);
  const tileName = tileData.name;

  const { exitCode } = await exec(`tessl search ${workspace}/${tileName}`);
  return exitCode === 0;
};
