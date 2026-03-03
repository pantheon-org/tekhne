import { existsSync } from "node:fs";
import { join } from "node:path";

export async function getTesslStatus(
  skillRelativePath: string,
): Promise<string> {
  const tileJsonPath = join("skills", skillRelativePath, "tile.json");

  if (!existsSync(tileJsonPath)) {
    return "-";
  }

  try {
    const tileData = await Bun.file(tileJsonPath).json();

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
}
