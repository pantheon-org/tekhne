import { existsSync } from "node:fs";
import { join } from "node:path";

export const getTesslStatus = async (
  skillRelativePath: string,
): Promise<string> => {
  const pluginJsonPath = join(
    "skills",
    skillRelativePath,
    ".tessl-plugin",
    "plugin.json",
  );
  const tileJsonPath = join("skills", skillRelativePath, "tile.json");

  const manifestPath = existsSync(pluginJsonPath)
    ? pluginJsonPath
    : existsSync(tileJsonPath)
      ? tileJsonPath
      : null;

  if (!manifestPath) {
    return "-";
  }

  try {
    const tileData = await Bun.file(manifestPath).json();

    if (tileData.private === false) {
      const tileName = tileData.name || "";
      if (tileName) {
        return `[Public](https://tessl.io/registry/skills/pantheon-ai/${tileName})`;
      }
      return "Public";
    }

    return "Private";
  } catch {
    return "Configured";
  }
};
